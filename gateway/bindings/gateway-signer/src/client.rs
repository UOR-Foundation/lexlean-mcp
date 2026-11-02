//! The side of the host: signing through the socket of `gateway-signer`, with no key in this process.

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::{Path, PathBuf};

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use serde_json::{json, Value};

use crate::service::{refusal_named, MAX_REQUEST_BYTES, TIMEOUT};
use crate::signer::{RecordSigner, SignError};

/// A signer that answered at its socket.
#[derive(Debug, Clone)]
pub struct SignerClient {
    socket: PathBuf,
}

impl SignerClient {
    /// The signer at `socket`, after it has said that it can load its key: a verification is not
    /// begun whose record could not be signed.
    pub fn ready(socket: impl Into<PathBuf>) -> Result<SignerClient, SignError> {
        let socket = socket.into();
        let answer = ask(&socket, &json!({"op": "key"}))?;
        if answer["keyid"].as_str().is_none() {
            return Err(SignError::Unavailable);
        }
        Ok(SignerClient { socket })
    }
}

impl RecordSigner for SignerClient {
    fn sign_record(&self, subject: &[u8], predicate: &Value) -> Result<Vec<u8>, SignError> {
        let answer = ask(
            &self.socket,
            &json!({"op": "sign", "subject": STANDARD.encode(subject), "predicate": predicate}),
        )?;
        answer["envelope"]
            .as_str()
            .and_then(|text| STANDARD.decode(text).ok())
            .ok_or(SignError::Unavailable)
    }
}

/// One request and its answer. Anything but an answer the service could have given is
/// `Unavailable`; a refusal comes back as what it is.
fn ask(socket: &Path, request: &Value) -> Result<Value, SignError> {
    let lost = |_| SignError::Unavailable;
    let stream = UnixStream::connect(socket).map_err(lost)?;
    stream.set_read_timeout(Some(TIMEOUT)).map_err(lost)?;
    stream.set_write_timeout(Some(TIMEOUT)).map_err(lost)?;
    let mut text = serde_json::to_vec(request).map_err(|_| SignError::Unavailable)?;
    text.push(b'\n');
    let mut writer = &stream;
    writer.write_all(&text).map_err(lost)?;
    writer.flush().map_err(lost)?;
    let mut line = Vec::new();
    // An envelope is the base64 of what the request carried, and a little more.
    BufReader::new((&stream).take(2 * MAX_REQUEST_BYTES))
        .read_until(b'\n', &mut line)
        .map_err(lost)?;
    let answer: Value = serde_json::from_slice(&line).map_err(|_| SignError::Unavailable)?;
    match answer.get("refused") {
        Some(name) => Err(name
            .as_str()
            .and_then(refusal_named)
            .unwrap_or(SignError::Unavailable)),
        None => Ok(answer),
    }
}
