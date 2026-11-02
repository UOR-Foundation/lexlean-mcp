//! The signer as a service: the one place the signing key is read.
//!
//! One line of JSON in, one line of JSON out, one connection for each request:
//!
//! ```text
//! client -> service   {"op":"key"}
//! service -> client   {"keyid":"<hex>"}
//! client -> service   {"op":"sign","subject":"<base64>","predicate":{...}}
//! service -> client   {"envelope":"<base64>"}
//! service -> client   {"refused":"<the name of a SignError>"}
//! ```
//!
//! The key file is read for each request, so the key is in memory only while it is used, and an
//! operator who puts the file right or rotates the key does not restart anything. Requests are
//! answered one at a time: a record is signed once per verification, and that order is what makes
//! the read and the write of the history one step. The time of a first signature is this process's
//! clock, not something a client can choose.

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
use serde_json::{json, Value};

use crate::signer::{now, HistoryError, KeyError, SignError, Signer};

/// The environment variable that names the file of the signing key. Only this program reads it;
/// the host removes it from the environment of the verifier in case an operator set it there too.
pub const SIGNING_KEY_VARIABLE: &str = "LEXLEAN_MCP_SIGNING_KEY";

/// The environment variable that names the socket of the signer, for the program and for the host.
pub const SIGNER_VARIABLE: &str = "LEXLEAN_MCP_SIGNER";

/// The environment variable that names the store, whose `keys/history` the program writes.
pub const STORE_VARIABLE: &str = "LEXLEAN_MCP_STORE";

/// The most bytes of one request line. A subject is one `attestation.json`, which is far smaller.
pub const MAX_REQUEST_BYTES: u64 = 16_777_216;

/// How long one side waits for the other to read or write, so that a client that stops talking
/// cannot hold the requests behind it for longer than this.
pub const TIMEOUT: Duration = Duration::from_secs(30);

/// The name a refusal travels by.
pub fn refusal_name(error: SignError) -> &'static str {
    match error {
        SignError::History(HistoryError::Unreadable) => "history-unreadable",
        SignError::History(HistoryError::Unwritable) => "history-unwritable",
        SignError::History(HistoryError::Malformed) => "history-malformed",
        SignError::History(HistoryError::UnknownKey) => "history-unknown-key",
        SignError::KeyRevoked => "key-revoked",
        SignError::Key(KeyError::Missing) => "key-missing",
        SignError::Key(KeyError::Unreadable) => "key-unreadable",
        SignError::Key(KeyError::Malformed) => "key-malformed",
        SignError::RequestRefused => "request-refused",
        SignError::Unavailable => "unavailable",
    }
}

/// The refusal a name stands for; a name no refusal has is `None`.
pub fn refusal_named(name: &str) -> Option<SignError> {
    Some(match name {
        "history-unreadable" => SignError::History(HistoryError::Unreadable),
        "history-unwritable" => SignError::History(HistoryError::Unwritable),
        "history-malformed" => SignError::History(HistoryError::Malformed),
        "history-unknown-key" => SignError::History(HistoryError::UnknownKey),
        "key-revoked" => SignError::KeyRevoked,
        "key-missing" => SignError::Key(KeyError::Missing),
        "key-unreadable" => SignError::Key(KeyError::Unreadable),
        "key-malformed" => SignError::Key(KeyError::Malformed),
        "request-refused" => SignError::RequestRefused,
        "unavailable" => SignError::Unavailable,
        _ => return None,
    })
}

/// Why the program did not start.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartError {
    /// `LEXLEAN_MCP_SIGNING_KEY` is not set.
    NoKey,
    /// `LEXLEAN_MCP_SIGNER` is not set.
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
            StartError::NoKey => write!(formatter, "{SIGNING_KEY_VARIABLE} is not set"),
            StartError::NoSocket => write!(formatter, "{SIGNER_VARIABLE} is not set"),
            StartError::NoStore => write!(formatter, "{STORE_VARIABLE} is not set"),
            StartError::PathTaken => {
                formatter.write_str("the path of the socket holds something that is not a socket")
            }
            StartError::Bind(error) => write!(formatter, "the socket cannot be made: {error}"),
        }
    }
}

impl std::error::Error for StartError {}

/// The service: where the key is and where its history goes.
#[derive(Debug, Clone)]
pub struct Service {
    key: PathBuf,
    history: PathBuf,
}

/// A service with its socket made, not yet answering.
#[derive(Debug)]
pub struct Listening {
    service: Service,
    listener: UnixListener,
}

impl Service {
    /// The service of the key file `key` and the history `history`. Neither is read yet.
    pub fn new(key: impl Into<PathBuf>, history: impl Into<PathBuf>) -> Service {
        Service {
            key: key.into(),
            history: history.into(),
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

    /// The answer to one request line.
    fn answer(&self, line: &[u8]) -> Value {
        self.answered(line)
            .unwrap_or_else(|error| json!({"refused": refusal_name(error)}))
    }

    fn answered(&self, line: &[u8]) -> Result<Value, SignError> {
        let request: Value = serde_json::from_slice(line).map_err(|_| SignError::RequestRefused)?;
        match request["op"].as_str() {
            Some("key") => Ok(json!({"keyid": self.signer()?.keyid()})),
            Some("sign") => {
                let subject = request["subject"]
                    .as_str()
                    .and_then(|text| STANDARD.decode(text).ok())
                    .ok_or(SignError::RequestRefused)?;
                let predicate = request.get("predicate").ok_or(SignError::RequestRefused)?;
                let envelope = self.signer()?.sign(&subject, predicate, now())?;
                Ok(json!({"envelope": STANDARD.encode(envelope)}))
            }
            _ => Err(SignError::RequestRefused),
        }
    }

    fn signer(&self) -> Result<Signer, SignError> {
        Signer::load(&self.key, self.history.clone()).map_err(SignError::Key)
    }

    /// Read one request from `stream` and answer it. A failure of the connection is the client's:
    /// it gets no envelope, and the next request is served.
    fn serve(&self, stream: &UnixStream) -> io::Result<()> {
        stream.set_read_timeout(Some(TIMEOUT))?;
        stream.set_write_timeout(Some(TIMEOUT))?;
        let mut line = Vec::new();
        BufReader::new(stream.take(MAX_REQUEST_BYTES + 1)).read_until(b'\n', &mut line)?;
        let answer = if line.len() as u64 > MAX_REQUEST_BYTES {
            json!({"refused": refusal_name(SignError::RequestRefused)})
        } else {
            self.answer(&line)
        };
        let mut text = serde_json::to_vec(&answer).unwrap_or_default();
        text.push(b'\n');
        let mut writer = stream;
        writer.write_all(&text)?;
        writer.flush()?;
        stream.shutdown(std::net::Shutdown::Write)?;
        // What is left of a request that was too long is read, within the bound, so that the
        // client can finish writing and read its refusal: closing on unread input resets the
        // connection, which would turn the refusal into a lost signer.
        io::copy(&mut stream.take(MAX_REQUEST_BYTES), &mut io::sink())?;
        Ok(())
    }
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
    let key = variable(SIGNING_KEY_VARIABLE).ok_or(StartError::NoKey)?;
    let socket = variable(SIGNER_VARIABLE).ok_or(StartError::NoSocket)?;
    let store = variable(STORE_VARIABLE).ok_or(StartError::NoStore)?;
    let history = PathBuf::from(store).join("keys").join("history");
    Ok((Service::new(key, history), PathBuf::from(socket)))
}

/// Run the program: start from the environment and answer until the process ends.
pub fn run_from_environment() -> Result<Infallible, StartError> {
    let (service, socket) = from_variables(|name| std::env::var_os(name))?;
    let listening = service.bind(&socket)?;
    // The line a supervisor or a test waits for: the socket is there and its mode is set.
    println!("gateway-signer listening on {}", socket.display());
    listening.run()
}
