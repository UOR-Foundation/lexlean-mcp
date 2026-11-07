//! The side of the host: the Engine calls through the socket of `gateway-engine-host`.

use std::ffi::OsString;
use std::fmt;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use lexlean::Selection;
use serde_json::{json, Value};

use crate::engine::{Answer, Call, Unavailable, RESULT_MAXIMUM};
use crate::service::{call_name, selection_value, ENGINE_VARIABLE, MAX_REQUEST_BYTES, TIMEOUT};

/// Make `call` on the project in `directory` through the socket `socket`. Anything but an answer the
/// service could have given is `Unavailable`, and so is every refusal: the host judged the request
/// before it asked, so a refusal is a host and a service that do not agree, not a fault of the client.
///
/// Only the sending has a time limit. The call takes as long as the limits of LexLean let it, and a
/// lost service ends the connection.
pub fn call(
    socket: &Path,
    directory: &Path,
    selection: &Result<Selection, String>,
    call: Call,
) -> Result<Answer, Unavailable> {
    let lost = |_| Unavailable;
    let request = json!({
        "directory": directory.to_str().ok_or(Unavailable)?,
        "call": call_name(call),
        "selection": selection_value(selection),
    });
    let stream = UnixStream::connect(socket).map_err(lost)?;
    stream.set_write_timeout(Some(TIMEOUT)).map_err(lost)?;
    let mut text = serde_json::to_vec(&request).map_err(|_| Unavailable)?;
    text.push(b'\n');
    let mut writer = &stream;
    writer.write_all(&text).map_err(lost)?;
    writer.flush().map_err(lost)?;
    let mut line = Vec::new();
    // A result is at most `RESULT_MAXIMUM` bytes, and its base64 is less than twice that.
    BufReader::new((&stream).take(2 * RESULT_MAXIMUM as u64))
        .read_until(b'\n', &mut line)
        .map_err(lost)?;
    answer_of(&line)
}

/// The answer a line holds. A class 3 or 5 is not an answer of these calls: it is `Unavailable`
/// before it travels (`engine::run`), so one that arrives did not come from the service.
fn answer_of(line: &[u8]) -> Result<Answer, Unavailable> {
    let answer: Value = serde_json::from_slice(line).map_err(|_| Unavailable)?;
    let class = answer["class"].as_u64().ok_or(Unavailable)?;
    if !matches!(class, 0 | 1 | 2 | 4) {
        return Err(Unavailable);
    }
    let result = answer["result"]
        .as_str()
        .and_then(|text| STANDARD.decode(text).ok())
        .filter(|result| result.len() <= RESULT_MAXIMUM)
        .ok_or(Unavailable)?;
    Ok(Answer {
        class: u32::try_from(class).map_err(|_| Unavailable)?,
        result,
    })
}

/// Why `gateway-engine-host ready`, the readiness command of the deployment, said no. It exits 1
/// with it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotReady {
    /// `LEXLEAN_MCP_ENGINE` is not set, or is empty.
    NoSocket,
    /// Nothing answered at the socket, or the answer was not one the service gives.
    Unavailable,
    /// The service answered with a refusal; the text is its name.
    Refused(String),
}

impl fmt::Display for NotReady {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotReady::NoSocket => write!(formatter, "{ENGINE_VARIABLE} is not set"),
            NotReady::Unavailable => {
                formatter.write_str("the engine host is not ready: unavailable")
            }
            NotReady::Refused(name) => write!(formatter, "the engine host is not ready: {name}"),
        }
    }
}

impl std::error::Error for NotReady {}

/// Whether the engine host at `socket` has its store, so that it can make a call.
pub fn ready(socket: &Path) -> Result<(), NotReady> {
    let lost = |_| NotReady::Unavailable;
    let stream = UnixStream::connect(socket).map_err(lost)?;
    stream.set_read_timeout(Some(TIMEOUT)).map_err(lost)?;
    stream.set_write_timeout(Some(TIMEOUT)).map_err(lost)?;
    let mut writer = &stream;
    writer.write_all(b"{\"op\":\"ready\"}\n").map_err(lost)?;
    writer.flush().map_err(lost)?;
    let mut line = Vec::new();
    BufReader::new((&stream).take(MAX_REQUEST_BYTES))
        .read_until(b'\n', &mut line)
        .map_err(lost)?;
    let answer: Value = serde_json::from_slice(&line).map_err(|_| NotReady::Unavailable)?;
    if answer["ready"] == true {
        return Ok(());
    }
    match answer["refused"].as_str() {
        Some(name) => Err(NotReady::Refused(name.to_string())),
        None => Err(NotReady::Unavailable),
    }
}

/// `ready` at the socket the variables `lookup` gives name. A variable that is empty is not set.
pub fn ready_from_variables(lookup: impl Fn(&str) -> Option<OsString>) -> Result<(), NotReady> {
    let socket = lookup(ENGINE_VARIABLE)
        .filter(|value| !value.is_empty())
        .ok_or(NotReady::NoSocket)?;
    ready(Path::new(&socket))
}
