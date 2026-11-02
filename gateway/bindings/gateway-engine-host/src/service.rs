//! The engine host as a service: the one place the four Engine calls are made.
//!
//! One line of JSON in, one line of JSON out, one connection for each request:
//!
//! ```text
//! client -> service   {"directory":"<project>","call":"check"|"snapshot"|"format"|"format-check"|"build",
//!                      "selection":{"kind":0|1|2,"files":["..."]}|{"invalid":"<message>"}}
//! service -> client   {"class":<0|1|2|4>,"result":"<base64>"}
//! service -> client   {"refused":"<the name of a Refusal>"}
//! ```
//!
//! The host has already judged the project and made its directory: the service runs on a directory
//! it is named, and refuses one outside the store it serves. Requests are answered one at a time, as
//! the host asks them. A call runs on a thread of its own, so that a panic of the compiler is a
//! refusal and the next request is served.

use std::collections::BTreeSet;
use std::convert::Infallible;
use std::ffi::OsString;
use std::fmt;
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::thread::{self, JoinHandle};
use std::time::Duration;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use camino::Utf8PathBuf;
use lexlean::Selection;
use serde_json::{json, Value};

use crate::engine::{self, Call};

/// The environment variable that names the socket of the engine host, for the program and the host.
pub const ENGINE_VARIABLE: &str = "LEXLEAN_MCP_ENGINE";

/// The environment variable that names the store, below which the program runs.
pub const STORE_VARIABLE: &str = "LEXLEAN_MCP_STORE";

/// The most bytes of one request line. A request is a directory and a list of files.
pub const MAX_REQUEST_BYTES: u64 = 16_777_216;

/// How long one side waits for the other to read or write, so that a client that stops talking
/// cannot hold the requests behind it for longer than this. The call itself has no such bound:
/// the limits of LexLean bound it, as they did when it ran in the host.
pub const TIMEOUT: Duration = Duration::from_secs(30);

/// The stack of a call, the one the Engine ran on in the process of the core.
const STACK_BYTES: usize = 512 << 20;

/// Why the service did not answer a request with a class and a result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refusal {
    /// The line is not a request, or is over `MAX_REQUEST_BYTES`.
    Request,
    /// The directory is not below the store of the service.
    Directory,
    /// The Engine could not run at all.
    Engine,
}

/// The name a refusal travels by.
pub fn refusal_name(refusal: Refusal) -> &'static str {
    match refusal {
        Refusal::Request => "request-refused",
        Refusal::Directory => "directory-refused",
        Refusal::Engine => "engine-unavailable",
    }
}

/// The refusal a name stands for; a name no refusal has is `None`.
pub fn refusal_named(name: &str) -> Option<Refusal> {
    Some(match name {
        "request-refused" => Refusal::Request,
        "directory-refused" => Refusal::Directory,
        "engine-unavailable" => Refusal::Engine,
        _ => return None,
    })
}

/// The name a call travels by.
pub fn call_name(call: Call) -> &'static str {
    match call {
        Call::Check => "check",
        Call::Snapshot => "snapshot",
        Call::Format(false) => "format",
        Call::Format(true) => "format-check",
        Call::Build => "build",
    }
}

/// The call a name stands for; a name no call has is `None`.
pub fn call_named(name: &str) -> Option<Call> {
    Some(match name {
        "check" => Call::Check,
        "snapshot" => Call::Snapshot,
        "format" => Call::Format(false),
        "format-check" => Call::Format(true),
        "build" => Call::Build,
        _ => return None,
    })
}

/// The selection as it travels.
pub fn selection_value(selection: &Result<Selection, String>) -> Value {
    match selection {
        Ok(Selection::Entrypoints) => json!({"kind": 0}),
        Ok(Selection::All) => json!({"kind": 1}),
        Ok(Selection::Files(files)) => {
            let files: Vec<&str> = files.iter().map(|file| file.as_str()).collect();
            json!({"kind": 2, "files": files})
        }
        Err(message) => json!({"invalid": message}),
    }
}

fn selection_of(value: &Value) -> Result<Result<Selection, String>, Refusal> {
    if let Some(message) = value.get("invalid") {
        let message = message.as_str().ok_or(Refusal::Request)?;
        return Ok(Err(message.to_string()));
    }
    Ok(Ok(match value["kind"].as_u64() {
        Some(0) => Selection::Entrypoints,
        Some(1) => Selection::All,
        Some(2) => {
            let files = value["files"].as_array().ok_or(Refusal::Request)?;
            let mut selected = BTreeSet::new();
            for file in files {
                let file = file.as_str().ok_or(Refusal::Request)?;
                selected.insert(Utf8PathBuf::from(file));
            }
            Selection::Files(selected)
        }
        _ => return Err(Refusal::Request),
    }))
}

/// Why the program did not start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartError {
    /// `LEXLEAN_MCP_ENGINE` is not set.
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
            StartError::NoSocket => write!(formatter, "{ENGINE_VARIABLE} is not set"),
            StartError::NoStore => write!(formatter, "{STORE_VARIABLE} is not set"),
            StartError::PathTaken => {
                formatter.write_str("the path of the socket holds something that is not a socket")
            }
            StartError::Bind(error) => write!(formatter, "the socket cannot be made: {error}"),
        }
    }
}

impl std::error::Error for StartError {}

/// The service: the store below which it runs.
#[derive(Debug, Clone)]
pub struct Service {
    store: PathBuf,
}

/// A service with its socket made, not yet answering.
#[derive(Debug)]
pub struct Listening {
    service: Service,
    listener: UnixListener,
}

impl Service {
    /// The service of the store `store`, which is not read yet.
    pub fn new(store: impl Into<PathBuf>) -> Service {
        Service {
            store: store.into(),
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

    fn answered(&self, line: &[u8]) -> Result<Value, Refusal> {
        let request: Value = serde_json::from_slice(line).map_err(|_| Refusal::Request)?;
        let directory = request["directory"].as_str().ok_or(Refusal::Request)?;
        let call = request["call"]
            .as_str()
            .and_then(call_named)
            .ok_or(Refusal::Request)?;
        let selection = selection_of(&request["selection"])?;
        let directory = Path::new(directory);
        if !self.admits(directory) {
            return Err(Refusal::Directory);
        }
        let answer = run_on_a_stack(directory, selection, call)?;
        Ok(json!({"class": answer.class, "result": STANDARD.encode(answer.result)}))
    }

    /// The answer to one request line.
    fn answer(&self, line: &[u8]) -> Value {
        self.answered(line)
            .unwrap_or_else(|refusal| json!({"refused": refusal_name(refusal)}))
    }

    /// Read one request from `stream` and answer it. A failure of the connection is the client's:
    /// it gets no answer, and the next request is served.
    fn serve(&self, stream: &UnixStream) -> io::Result<()> {
        stream.set_read_timeout(Some(TIMEOUT))?;
        stream.set_write_timeout(Some(TIMEOUT))?;
        let mut line = Vec::new();
        BufReader::new(stream.take(MAX_REQUEST_BYTES + 1)).read_until(b'\n', &mut line)?;
        let answer = if line.len() as u64 > MAX_REQUEST_BYTES {
            json!({"refused": refusal_name(Refusal::Request)})
        } else {
            self.answer(&line)
        };
        let mut text = serde_json::to_vec(&answer).unwrap_or_default();
        text.push(b'\n');
        let mut writer = stream;
        writer.write_all(&text)?;
        writer.flush()?;
        stream.shutdown(std::net::Shutdown::Write)?;
        // What is left of a request that was too long is read, within the bound, so that the client
        // can finish writing and read its refusal: closing on unread input resets the connection.
        io::copy(&mut stream.take(MAX_REQUEST_BYTES), &mut io::sink())?;
        Ok(())
    }
}

/// Make `call` on a thread with the stack it needs: a panic there is `Refusal::Engine`.
fn run_on_a_stack(
    directory: &Path,
    selection: Result<Selection, String>,
    call: Call,
) -> Result<engine::Answer, Refusal> {
    thread::scope(|scope| {
        thread::Builder::new()
            .stack_size(STACK_BYTES)
            .spawn_scoped(scope, || engine::run(directory, selection, call))
            .map_err(|_| Refusal::Engine)?
            .join()
            .map_err(|_| Refusal::Engine)?
            .map_err(|engine::Unavailable| Refusal::Engine)
    })
}

impl Listening {
    /// Answer requests until the process ends.
    pub fn run(self) -> ! {
        loop {
            if let Ok((stream, _)) = self.listener.accept() {
                let _ = self.service.serve(&stream);
            }
        }
    }

    /// Answer requests on a thread of this process, for the tests of a host: the same service, over
    /// the same socket.
    pub fn spawn(self) -> JoinHandle<()> {
        thread::spawn(move || {
            self.run();
        })
    }
}

/// The settings of the program, from the variables `lookup` gives; a variable that is empty is not
/// set. The result is the service and its socket.
pub fn from_variables(
    lookup: impl Fn(&str) -> Option<OsString>,
) -> Result<(Service, PathBuf), StartError> {
    let variable = |name: &str| lookup(name).filter(|value| !value.is_empty());
    let socket = variable(ENGINE_VARIABLE).ok_or(StartError::NoSocket)?;
    let store = variable(STORE_VARIABLE).ok_or(StartError::NoStore)?;
    Ok((Service::new(store), PathBuf::from(socket)))
}

/// Run the program: start from the environment and answer until the process ends.
pub fn run_from_environment() -> Result<Infallible, StartError> {
    let (service, socket) = from_variables(|name| std::env::var_os(name))?;
    let listening = service.bind(&socket)?;
    // The line a supervisor or a test waits for: the socket is there and its mode is set.
    println!("gateway-engine-host listening on {}", socket.display());
    listening.run()
}
