//! The side of the host: the Engine calls through the socket of `gateway-engine-host`.

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use lexlean::Selection;
use serde_json::{json, Value};

use crate::engine::{Answer, Call, Unavailable, RESULT_MAXIMUM};
use crate::service::{call_name, selection_value, TIMEOUT};

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
