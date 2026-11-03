//! The verifier host as a service: where the process of each verification runs.
//!
//! A process ID and a session mean something only in the container they live in, so whatever
//! starts, kills or sweeps a verifier process has to be there too. One connection is one task:
//!
//! ```text
//! client -> service   {"op":"ready"}
//! service -> client   {"ready":true}
//!
//! client -> service   {"op":"run","width":<n>,"memory_kib":<n>,"grace_ms":<n>,"job":<the job line>}
//! service -> client   {"event":"accepted"}
//! service -> client   each line of the process, as it wrote it (VERIFIER.md, "The protocol")
//! client -> service   cancel
//! client -> service   kill
//! service -> client   {"event":"ended","swept":<processes>,"log":"<the tail of its standard error>"}
//!
//! service -> client   {"refused":"<the name of a Refusal>"}, in place of an answer
//! ```
//!
//! `ended` is always the last line, after the process has gone and its session has been swept. The
//! end of the connection is the host gone: the input of the process closes, which ends it, and one
//! that does not end within the grace is killed.

use std::convert::Infallible;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::{ChildStdin, Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc, Mutex, PoisonError};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

use serde_json::{json, Value};

use crate::sweep;

/// The environment variable that names the socket of the verifier host, for the program and the host.
pub const VERIFIER_HOST_VARIABLE: &str = "LEXLEAN_MCP_VERIFIER_HOST";

/// The environment variable that names the store, below which the program runs.
pub const STORE_VARIABLE: &str = "LEXLEAN_MCP_STORE";

/// The environment variable that names the verifier program. Without it the program is
/// `gateway-verifier` beside the running executable.
pub const PROGRAM_VARIABLE: &str = "LEXLEAN_MCP_VERIFIER";

/// The environment variable of the signing key, which only the signer reads: it is removed in case
/// an operator set it here too.
pub const SIGNING_KEY_VARIABLE: &str = "LEXLEAN_MCP_SIGNING_KEY";

/// The most bytes of the first line of a connection. A job is a directory and a list of files.
pub const MAX_REQUEST_BYTES: u64 = 16_777_216;

/// How long a new connection may take to send its first line.
pub const TIMEOUT: Duration = Duration::from_secs(30);

/// The most bytes of the standard error of a process that `ended` carries.
const KEPT: usize = 8192;

/// Why the service did not run a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The line is not a request, or is over `MAX_REQUEST_BYTES`.
    Request,
    /// The directory of the job is not below the store of the service.
    Directory,
    /// The verifier program is not a file, or could not be started.
    Program,
}

/// The name a refusal travels by.
pub fn refusal_name(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::Request => "request-refused",
        Refusal::Directory => "directory-refused",
        Refusal::Program => "program-unavailable",
    }
}

/// The refusal a name stands for; a name no refusal has is `None`.
pub fn refusal_named(name: &str) -> Option<Refusal> {
    Some(match name {
        "request-refused" => Refusal::Request,
        "directory-refused" => Refusal::Directory,
        "program-unavailable" => Refusal::Program,
        _ => return None,
    })
}

/// Why the program did not start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartError {
    /// `LEXLEAN_MCP_VERIFIER_HOST` is not set.
    NoSocket,
    /// `LEXLEAN_MCP_STORE` is not set.
    NoStore,
    /// Something that is not a socket is at the path of the socket, and it is left there.
    PathTaken,
    /// The socket could not be made; the text is the error of the system.
    Bind(String),
}

impl fmt::Display for StartError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StartError::NoSocket => write!(formatter, "{VERIFIER_HOST_VARIABLE} is not set"),
            StartError::NoStore => write!(formatter, "{STORE_VARIABLE} is not set"),
            StartError::PathTaken => {
                formatter.write_str("the path of the socket holds something that is not a socket")
            }
            StartError::Bind(error) => write!(formatter, "the socket cannot be made: {error}"),
        }
    }
}

impl std::error::Error for StartError {}

/// The service: the store below which it runs, and the program it starts.
#[derive(Debug, Clone)]
pub struct Service {
    store: PathBuf,
    program: PathBuf,
}

/// A service with its socket made, not yet answering.
#[derive(Debug)]
pub struct Listening {
    service: Service,
    listener: UnixListener,
}

/// What one task asks for.
struct Run {
    width: u64,
    memory_kib: u64,
    grace: Duration,
    job: Value,
}

fn run_of(request: &Value) -> Result<Run, Refusal> {
    let number = |field: &str| request[field].as_u64().ok_or(Refusal::Request);
    Ok(Run {
        width: number("width")?,
        memory_kib: number("memory_kib")?,
        grace: Duration::from_millis(number("grace_ms")?),
        job: request.get("job").cloned().ok_or(Refusal::Request)?,
    })
}

fn say(mut stream: &UnixStream, line: &[u8]) {
    let _ = stream
        .write_all(line)
        .and_then(|()| stream.write_all(b"\n"))
        .and_then(|()| stream.flush());
}

fn say_value(stream: &UnixStream, value: &Value) {
    say(stream, value.to_string().as_bytes());
}

impl Service {
    /// The service of the store `store` and the verifier program `program`, neither read yet.
    pub fn new(store: impl Into<PathBuf>, program: impl Into<PathBuf>) -> Service {
        Service {
            store: store.into(),
            program: program.into(),
        }
    }

    /// Make the socket at `socket`, for this user only, in place of the socket an earlier run
    /// left there.
    pub fn bind(self, socket: &Path) -> Result<Listening, StartError> {
        match fs::symlink_metadata(socket) {
            Ok(found) if found.file_type().is_socket() => {
                fs::remove_file(socket).map_err(|error| StartError::Bind(error.to_string()))?;
            }
            Ok(_) => return Err(StartError::PathTaken),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(StartError::Bind(error.to_string())),
        }
        let listener =
            UnixListener::bind(socket).map_err(|error| StartError::Bind(error.to_string()))?;
        fs::set_permissions(socket, fs::Permissions::from_mode(0o600))
            .map_err(|error| StartError::Bind(error.to_string()))?;
        Ok(Listening {
            service: self,
            listener,
        })
    }

    /// Whether `directory` is below the store, after any link in either path is followed.
    fn admits(&self, directory: &Path) -> bool {
        match (self.store.canonicalize(), directory.canonicalize()) {
            (Ok(store), Ok(directory)) => directory != store && directory.starts_with(store),
            _ => false,
        }
    }

    /// Answer one connection.
    fn serve(&self, stream: &UnixStream) -> io::Result<()> {
        stream.set_read_timeout(Some(TIMEOUT))?;
        let mut line = Vec::new();
        BufReader::new(stream.take(MAX_REQUEST_BYTES + 1)).read_until(b'\n', &mut line)?;
        stream.set_read_timeout(None)?;
        let refused = |refusal| say_value(stream, &json!({"refused": refusal_name(refusal)}));
        if line.len() as u64 > MAX_REQUEST_BYTES {
            refused(Refusal::Request);
            return Ok(());
        }
        let Ok(request) = serde_json::from_slice::<Value>(&line) else {
            refused(Refusal::Request);
            return Ok(());
        };
        match request["op"].as_str() {
            Some("ready") if self.program.is_file() => say_value(stream, &json!({"ready": true})),
            Some("ready") => refused(Refusal::Program),
            Some("run") => {
                if let Err(refusal) = run_of(&request).and_then(|run| self.run(stream, &run)) {
                    refused(refusal);
                }
            }
            _ => refused(Refusal::Request),
        }
        Ok(())
    }

    /// Run the process of one task until it has gone. A refusal is returned before it starts.
    fn run(&self, stream: &UnixStream, run: &Run) -> Result<(), Refusal> {
        let directory = run.job["directory"].as_str().ok_or(Refusal::Request)?;
        if !self.admits(Path::new(directory)) {
            return Err(Refusal::Directory);
        }
        if !self.program.is_file() {
            return Err(Refusal::Program);
        }
        let mut child = Command::new(&self.program)
            .env("LEXLEAN_VERIFY_WIDTH", run.width.to_string())
            .env("LEXLEAN_VERIFY_MEMORY_KIB", run.memory_kib.to_string())
            .env_remove(SIGNING_KEY_VARIABLE)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| Refusal::Program)?;
        let (Some(mut input), Some(output), Some(errors)) =
            (child.stdin.take(), child.stdout.take(), child.stderr.take())
        else {
            let _ = child.kill();
            let _ = child.wait();
            return Err(Refusal::Program);
        };
        say_value(stream, &json!({"event": "accepted"}));
        // A process that is already gone is found below.
        let _ = writeln!(input, "{}", run.job).and_then(|()| input.flush());

        let input = Arc::new(Mutex::new(Some(input)));
        let kill = Arc::new(AtomicBool::new(false));
        let gone = Arc::new(AtomicBool::new(false));
        if let Ok(from_host) = stream.try_clone() {
            let (input, kill, gone) = (Arc::clone(&input), Arc::clone(&kill), Arc::clone(&gone));
            thread::spawn(move || relay_input(&from_host, &input, &kill, &gone));
        }
        let tail = Arc::new(Mutex::new(Vec::new()));
        {
            let tail = Arc::clone(&tail);
            thread::spawn(move || drain(errors, &tail));
        }
        let (send, lines) = mpsc::channel::<String>();
        thread::spawn(move || {
            for line in BufReader::new(output).lines().map_while(Result::ok) {
                if send.send(line).is_err() {
                    break;
                }
            }
        });

        let mut session = None;
        let mut verdict_at = None;
        let mut gone_at = None;
        let mut killed = false;
        let mut swept = 0;
        // The end of the process is found by asking the process and not by the end of its output: a
        // child that outlived it may hold the pipe open.
        loop {
            match lines.recv_timeout(Duration::from_millis(25)) {
                Ok(line) => {
                    note(&line, &mut session, &mut verdict_at);
                    say(stream, line.as_bytes());
                    continue;
                }
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    thread::sleep(Duration::from_millis(25));
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {}
            }
            if !matches!(child.try_wait(), Ok(None)) {
                // The process is gone: take what it had written, then stop.
                while let Ok(line) = lines.recv_timeout(Duration::from_millis(100)) {
                    note(&line, &mut session, &mut verdict_at);
                    say(stream, line.as_bytes());
                }
                break;
            }
            if gone.load(Ordering::SeqCst) && gone_at.is_none() {
                gone_at = Some(Instant::now());
            }
            let overdue =
                |since: Option<Instant>| since.is_some_and(|at| at.elapsed() >= run.grace);
            if !killed && (kill.load(Ordering::SeqCst) || overdue(verdict_at) || overdue(gone_at)) {
                killed = true;
                let _ = child.kill();
                if let Some(session) = session {
                    swept += sweep::kill_session(session);
                }
            }
        }
        let _ = child.wait();
        drop(input.lock().unwrap_or_else(PoisonError::into_inner).take());
        if verdict_at.is_none() {
            // A lost verifier: let its last words reach the tail.
            thread::sleep(Duration::from_millis(100));
        }
        if let Some(session) = session {
            swept += sweep::kill_session(session);
        }
        let log = String::from_utf8_lossy(&tail.lock().unwrap_or_else(PoisonError::into_inner))
            .into_owned();
        say_value(
            stream,
            &json!({"event": "ended", "swept": swept, "log": log}),
        );
        let _ = stream.shutdown(std::net::Shutdown::Write);
        Ok(())
    }
}

/// What a line of the process says of the session it leads and of its verdict.
fn note(line: &str, session: &mut Option<u32>, verdict_at: &mut Option<Instant>) {
    let Ok(event) = serde_json::from_str::<Value>(line) else {
        return;
    };
    match event["event"].as_str() {
        Some("started") => {
            *session = event["sid"]
                .as_u64()
                .and_then(|sid| u32::try_from(sid).ok());
        }
        Some("verified" | "failed" | "cancelled") => *verdict_at = Some(Instant::now()),
        _ => {}
    }
}

/// Pass the lines of the host to the process until the host lets go, which ends its input.
fn relay_input(
    from_host: &UnixStream,
    input: &Mutex<Option<ChildStdin>>,
    kill: &AtomicBool,
    gone: &AtomicBool,
) {
    for line in BufReader::new(from_host).lines().map_while(Result::ok) {
        match line.trim() {
            "cancel" => {
                if let Some(input) = input
                    .lock()
                    .unwrap_or_else(PoisonError::into_inner)
                    .as_mut()
                {
                    let _ = writeln!(input, "cancel").and_then(|()| input.flush());
                }
            }
            "kill" => kill.store(true, Ordering::SeqCst),
            _ => {}
        }
    }
    drop(input.lock().unwrap_or_else(PoisonError::into_inner).take());
    gone.store(true, Ordering::SeqCst);
}

/// Keep the last bytes of the standard error of a process.
fn drain(mut errors: impl Read, tail: &Mutex<Vec<u8>>) {
    let mut buffer = [0u8; 1024];
    while let Ok(count) = errors.read(&mut buffer) {
        if count == 0 {
            break;
        }
        let mut kept = tail.lock().unwrap_or_else(PoisonError::into_inner);
        kept.extend_from_slice(&buffer[..count]);
        if kept.len() > KEPT {
            let excess = kept.len() - KEPT;
            kept.drain(..excess);
        }
    }
}

impl Listening {
    /// Answer connections until the process ends, each on a thread of its own, because each lasts
    /// as long as its verification. How many run at once is the host's: `verify.concurrency`.
    pub fn run(self) -> ! {
        let service = Arc::new(self.service);
        loop {
            if let Ok((stream, _)) = self.listener.accept() {
                let service = Arc::clone(&service);
                thread::spawn(move || {
                    let _ = service.serve(&stream);
                });
            }
        }
    }

    /// Answer connections on a thread of this process, for the tests of a host: the same service,
    /// over the same socket.
    pub fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }
}

/// `gateway-verifier` beside the running executable, which is where an image puts it.
fn program_beside_this_one() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|exe| {
            exe.parent()
                .map(|directory| directory.join("gateway-verifier"))
        })
        .unwrap_or_else(|| PathBuf::from("gateway-verifier"))
}

/// The settings of the program, from the variables `lookup` gives; a variable that is empty is not
/// set. The result is the service and its socket.
pub fn from_variables(
    lookup: impl Fn(&str) -> Option<OsString>,
) -> Result<(Service, PathBuf), StartError> {
    let variable = |name: &str| lookup(name).filter(|value| !value.is_empty());
    let socket = variable(VERIFIER_HOST_VARIABLE).ok_or(StartError::NoSocket)?;
    let store = variable(STORE_VARIABLE).ok_or(StartError::NoStore)?;
    let program = variable(PROGRAM_VARIABLE).map_or_else(program_beside_this_one, PathBuf::from);
    Ok((Service::new(store, program), PathBuf::from(socket)))
}

/// Run the program: start from the environment and answer until the process ends.
pub fn run_from_environment() -> Result<Infallible, StartError> {
    let (service, socket) = from_variables(|name| std::env::var_os(name))?;
    let listening = service.bind(&socket)?;
    // The line a supervisor or a test waits for: the socket is there and its mode is set.
    println!("gateway-verifier-host listening on {}", socket.display());
    listening.run()
}
