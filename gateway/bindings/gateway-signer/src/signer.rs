//! The signer of the verification records: a DSSE envelope over an in-toto statement whose subject
//! is the `attestation.json` of a verification, signed with Ed25519, and the key history that
//! lets a reader validate the envelope after the key has rotated or been revoked.
//!
//! LexLean signs nothing itself (SPEC 22.9), so this is where a verification becomes a record
//! that can be forged only with the key. The key is a file secret named by the operator; only the
//! program `gateway-signer` reads it (`service`), it is never copied into an envelope, a history or
//! a message, and the signing key of the library zeroizes itself on drop. An envelope is never
//! returned before the history that names its key is on disk, so a record cannot exist for a key a
//! reader cannot look up.
//!
//! The history is a JSON list in `keys/history`: one entry per key, in the order of first use,
//! with `revoked_at` set by the operator when the key is compromised. Rotation adds an entry and
//! revokes nothing, so earlier envelopes stay valid; revocation makes every envelope of that key
//! fail, because an envelope carries no time that an attacker holding the key could not choose.

use std::fmt;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use base64::engine::general_purpose::STANDARD;
use base64::Engine as _;
use ed25519_dalek::{Signature, Signer as _, SigningKey, VerifyingKey};
use serde_json::{json, Map, Value};
use sha2::{Digest, Sha256};

/// The payload type of the envelope: the media type of an in-toto statement.
pub const PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";

/// The type of the statement: in-toto Statement v1.
pub const STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";

/// The predicate type of a verification of this product.
pub const PREDICATE_TYPE: &str = "https://uor.foundation/lexlean-mcp/verification/v1";

/// The name of the one subject of a statement.
pub const SUBJECT_NAME: &str = "attestation.json";

/// The algorithm of every key of the history: the owner value `signing.algorithm`.
pub const ALGORITHM: &str = "Ed25519";

/// The pre-authentication encoding of DSSE: the payload type and the payload, each preceded by
/// its length in octets, so that no payload can be read as another payload type.
pub fn pae(payload_type: &str, payload: &[u8]) -> Vec<u8> {
    let mut encoded = format!(
        "DSSEv1 {} {} {} ",
        payload_type.len(),
        payload_type,
        payload.len()
    )
    .into_bytes();
    encoded.extend_from_slice(payload);
    encoded
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn keyid_of(public_key: &[u8]) -> String {
    hex(&Sha256::digest(public_key))
}

/// Why a key file was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyError {
    /// There is no file at the path.
    Missing,
    /// The file exists and cannot be read.
    Unreadable,
    /// The file is not 64 hexadecimal digits.
    Malformed,
}

impl fmt::Display for KeyError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            KeyError::Missing => "the signing key file does not exist",
            KeyError::Unreadable => "the signing key file cannot be read",
            KeyError::Malformed => "the signing key file is not 64 hexadecimal digits",
        })
    }
}

impl std::error::Error for KeyError {}

/// Why the key history was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HistoryError {
    /// The history exists and cannot be read.
    Unreadable,
    /// The history cannot be written.
    Unwritable,
    /// The history is not a list of well-formed entries with distinct keys.
    Malformed,
    /// The key to revoke is not in the history.
    UnknownKey,
}

impl fmt::Display for HistoryError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            HistoryError::Unreadable => "the key history cannot be read",
            HistoryError::Unwritable => "the key history cannot be written",
            HistoryError::Malformed => "the key history is not a list of well-formed key entries",
            HistoryError::UnknownKey => "the key is not in the key history",
        })
    }
}

impl std::error::Error for HistoryError {}

/// Why a signature was refused: no envelope exists for any of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignError {
    /// The history that names the key could not be read or written.
    History(HistoryError),
    /// The key of the signer has been revoked.
    KeyRevoked,
    /// The key file of the signer service was refused.
    Key(KeyError),
    /// The signer service was asked something that is not a request, or is over its bound.
    RequestRefused,
    /// No signer service answered, or what answered was not one.
    Unavailable,
}

impl fmt::Display for SignError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SignError::History(error) => write!(formatter, "no signature: {error}"),
            SignError::KeyRevoked => {
                formatter.write_str("no signature: the signing key is revoked")
            }
            SignError::Key(error) => write!(formatter, "no signature: {error}"),
            SignError::RequestRefused => {
                formatter.write_str("no signature: the signer refused the request")
            }
            SignError::Unavailable => formatter.write_str("no signature: no signer answered"),
        }
    }
}

impl std::error::Error for SignError {}

/// What makes the envelope of a record: `SignerClient` in a running host, or `Signer` itself.
pub trait RecordSigner: fmt::Debug + Send + Sync {
    /// The envelope for the verification whose `attestation.json` is `subject`.
    fn sign_record(&self, subject: &[u8], predicate: &Value) -> Result<Vec<u8>, SignError>;
}

impl RecordSigner for Signer {
    fn sign_record(&self, subject: &[u8], predicate: &Value) -> Result<Vec<u8>, SignError> {
        self.sign(subject, predicate, now())
    }
}

/// The seconds since the epoch, for `not_before`.
pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs())
}

/// The key ID an envelope names first: the one this host's signer signed with. A verification takes
/// minutes and the key may have rotated since it began, so the envelope says, not the start.
pub fn envelope_keyid(envelope: &[u8]) -> Option<String> {
    let envelope: Value = serde_json::from_slice(envelope).ok()?;
    envelope["signatures"][0]["keyid"]
        .as_str()
        .map(str::to_owned)
}

/// Why an envelope was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvelopeError {
    /// Not a JSON object with a payload type, a payload and a list of signatures.
    Malformed,
    /// The payload type is not [`PAYLOAD_TYPE`].
    PayloadType,
    /// The payload is not base64.
    Payload,
    /// The payload is not an in-toto statement of this product with one subject.
    Statement,
    /// The list of signatures is empty.
    NoSignature,
    /// No signature names a key of the history.
    UnknownKey,
    /// The key that signed is revoked.
    KeyRevoked,
    /// The signature does not verify with the key of the history.
    BadSignature,
    /// The digest of the subject is not that of the bytes given.
    SubjectMismatch,
}

impl fmt::Display for EnvelopeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            EnvelopeError::Malformed => "the envelope is not a DSSE envelope",
            EnvelopeError::PayloadType => "the payload type is not an in-toto statement",
            EnvelopeError::Payload => "the payload is not base64",
            EnvelopeError::Statement => "the payload is not a statement of a verification",
            EnvelopeError::NoSignature => "the envelope has no signature",
            EnvelopeError::UnknownKey => "no signature names a key of the key history",
            EnvelopeError::KeyRevoked => "the key that signed the envelope is revoked",
            EnvelopeError::BadSignature => "the signature does not verify",
            EnvelopeError::SubjectMismatch => "the subject is not the attestation that was given",
        })
    }
}

impl std::error::Error for EnvelopeError {}

/// One key of the history.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyEntry {
    /// The SHA-256 of the public key, in lowercase hexadecimal.
    pub keyid: String,
    /// The algorithm of the key: always [`ALGORITHM`].
    pub algorithm: String,
    /// The public key, 32 octets in base64.
    pub public_key: String,
    /// The time of the first signature with the key, in seconds since the epoch.
    pub not_before: u64,
    /// The time the operator revoked the key, when it did.
    pub revoked_at: Option<u64>,
}

impl KeyEntry {
    fn to_value(&self) -> Value {
        let mut entry = Map::new();
        entry.insert("keyid".to_owned(), json!(self.keyid));
        entry.insert("algorithm".to_owned(), json!(self.algorithm));
        entry.insert("public_key".to_owned(), json!(self.public_key));
        entry.insert("not_before".to_owned(), json!(self.not_before));
        if let Some(at) = self.revoked_at {
            entry.insert("revoked_at".to_owned(), json!(at));
        }
        Value::Object(entry)
    }

    /// An entry, or `None` when the value is not exactly the fields of an entry, its algorithm
    /// is not Ed25519, its key is not 32 octets of base64 or its keyid is not the key's digest.
    fn from_value(value: &Value) -> Option<KeyEntry> {
        let object = value.as_object()?;
        if object.keys().any(|name| {
            ![
                "keyid",
                "algorithm",
                "public_key",
                "not_before",
                "revoked_at",
            ]
            .contains(&name.as_str())
        }) {
            return None;
        }
        let keyid = object.get("keyid")?.as_str()?;
        let algorithm = object.get("algorithm")?.as_str()?;
        let public_key = object.get("public_key")?.as_str()?;
        let not_before = object.get("not_before")?.as_u64()?;
        let revoked_at = match object.get("revoked_at") {
            None => None,
            Some(at) => Some(at.as_u64()?),
        };
        let octets = STANDARD.decode(public_key).ok()?;
        if algorithm != ALGORITHM || octets.len() != 32 || keyid != keyid_of(&octets) {
            return None;
        }
        Some(KeyEntry {
            keyid: keyid.to_owned(),
            algorithm: algorithm.to_owned(),
            public_key: public_key.to_owned(),
            not_before,
            revoked_at,
        })
    }
}

/// The history at `path`; a history that does not exist yet is the empty list.
pub fn read_history(path: &Path) -> Result<Vec<KeyEntry>, HistoryError> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(_) => return Err(HistoryError::Unreadable),
    };
    let value: Value = serde_json::from_slice(&bytes).map_err(|_| HistoryError::Malformed)?;
    let mut entries: Vec<KeyEntry> = Vec::new();
    for item in value.as_array().ok_or(HistoryError::Malformed)? {
        let entry = KeyEntry::from_value(item).ok_or(HistoryError::Malformed)?;
        if entries.iter().any(|earlier| earlier.keyid == entry.keyid) {
            return Err(HistoryError::Malformed);
        }
        entries.push(entry);
    }
    Ok(entries)
}

/// Replace the history with `entries`: a complete file is written beside it and renamed over it,
/// so a reader sees the old list or the new one.
fn write_history(path: &Path, entries: &[KeyEntry]) -> Result<(), HistoryError> {
    let list = Value::Array(entries.iter().map(KeyEntry::to_value).collect());
    let mut text = serde_json::to_vec_pretty(&list).map_err(|_| HistoryError::Unwritable)?;
    text.push(b'\n');
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent).map_err(|_| HistoryError::Unwritable)?;
    }
    let mut name = path
        .file_name()
        .ok_or(HistoryError::Unwritable)?
        .to_os_string();
    name.push(".tmp");
    let temporary = path.with_file_name(name);
    let written = fs::File::create(&temporary)
        .and_then(|mut file| file.write_all(&text).and_then(|()| file.sync_all()))
        .and_then(|()| fs::rename(&temporary, path));
    if written.is_err() {
        // A directory in the place of the file is left to whoever put it there.
        let _ = fs::remove_file(&temporary);
        return Err(HistoryError::Unwritable);
    }
    Ok(())
}

/// Revoke the key `keyid` at `at`, in seconds since the epoch. A key that is revoked already
/// keeps its first time. The operator runs this; the signer does not.
pub fn revoke(path: &Path, keyid: &str, at: u64) -> Result<(), HistoryError> {
    let mut entries = read_history(path)?;
    let entry = entries
        .iter_mut()
        .find(|entry| entry.keyid == keyid)
        .ok_or(HistoryError::UnknownKey)?;
    entry.revoked_at.get_or_insert(at);
    write_history(path, &entries)
}

/// The signer: one Ed25519 key and the history that names it.
pub struct Signer {
    key: SigningKey,
    keyid: String,
    history: PathBuf,
    /// Held across the read of the history and the write of a new entry.
    lock: Mutex<()>,
}

impl fmt::Debug for Signer {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Signer")
            .field("keyid", &self.keyid)
            .field("history", &self.history)
            .finish_non_exhaustive()
    }
}

impl Signer {
    /// A signer whose key is made from `seed`, writing its history at `history`.
    pub fn from_seed(seed: [u8; 32], history: impl Into<PathBuf>) -> Signer {
        let key = SigningKey::from_bytes(&seed);
        let keyid = keyid_of(key.verifying_key().as_bytes());
        Signer {
            key,
            keyid,
            history: history.into(),
            lock: Mutex::new(()),
        }
    }

    /// A signer whose seed is the 64 hexadecimal digits in the file `secret`, with whitespace
    /// around them allowed.
    pub fn load(secret: &Path, history: impl Into<PathBuf>) -> Result<Signer, KeyError> {
        let bytes = fs::read(secret).map_err(|error| match error.kind() {
            io::ErrorKind::NotFound => KeyError::Missing,
            _ => KeyError::Unreadable,
        })?;
        let text = std::str::from_utf8(&bytes)
            .map_err(|_| KeyError::Malformed)?
            .trim();
        if text.len() != 64 || !text.bytes().all(|digit| digit.is_ascii_hexdigit()) {
            return Err(KeyError::Malformed);
        }
        let mut seed = [0u8; 32];
        for (octet, pair) in seed.iter_mut().zip(text.as_bytes().chunks(2)) {
            let digits = std::str::from_utf8(pair).map_err(|_| KeyError::Malformed)?;
            *octet = u8::from_str_radix(digits, 16).map_err(|_| KeyError::Malformed)?;
        }
        Ok(Signer::from_seed(seed, history))
    }

    /// The key ID: the SHA-256 of the public key, in lowercase hexadecimal.
    pub fn keyid(&self) -> &str {
        &self.keyid
    }

    /// The public key.
    pub fn public_key(&self) -> [u8; 32] {
        self.key.verifying_key().to_bytes()
    }

    /// The envelope for the verification whose `attestation.json` is `subject`, with the
    /// predicate `predicate`, signed at `now` (seconds since the epoch, recorded as `not_before`
    /// when this is the first signature with the key).
    ///
    /// The history that names the key is on disk before the envelope is returned, and a signature
    /// whose history could not be written returns nothing.
    pub fn sign(&self, subject: &[u8], predicate: &Value, now: u64) -> Result<Vec<u8>, SignError> {
        {
            let _held = self.lock.lock().unwrap_or_else(PoisonError::into_inner);
            let mut entries = read_history(&self.history).map_err(SignError::History)?;
            match entries.iter().find(|entry| entry.keyid == self.keyid) {
                Some(entry) if entry.revoked_at.is_some() => return Err(SignError::KeyRevoked),
                Some(_) => {}
                None => {
                    entries.push(KeyEntry {
                        keyid: self.keyid.clone(),
                        algorithm: ALGORITHM.to_owned(),
                        public_key: STANDARD.encode(self.public_key()),
                        not_before: now,
                        revoked_at: None,
                    });
                    write_history(&self.history, &entries).map_err(SignError::History)?;
                }
            }
        }
        let statement = json!({
            "_type": STATEMENT_TYPE,
            "subject": [{"name": SUBJECT_NAME, "digest": {"sha256": hex(&Sha256::digest(subject))}}],
            "predicateType": PREDICATE_TYPE,
            "predicate": predicate,
        });
        let payload = serde_json::to_vec(&statement).unwrap_or_default();
        let signature = self.key.sign(&pae(PAYLOAD_TYPE, &payload));
        let envelope = json!({
            "payloadType": PAYLOAD_TYPE,
            "payload": STANDARD.encode(&payload),
            "signatures": [{"keyid": self.keyid, "sig": STANDARD.encode(signature.to_bytes())}],
        });
        Ok(serde_json::to_vec(&envelope).unwrap_or_default())
    }
}

/// Whether one signature of an envelope verifies `message` with a key of the history.
fn check_signature(
    signature: &Value,
    history: &[KeyEntry],
    message: &[u8],
) -> Result<(), EnvelopeError> {
    let object = signature.as_object().ok_or(EnvelopeError::Malformed)?;
    let keyid = object
        .get("keyid")
        .and_then(Value::as_str)
        .ok_or(EnvelopeError::Malformed)?;
    let sig = object
        .get("sig")
        .and_then(Value::as_str)
        .ok_or(EnvelopeError::Malformed)?;
    let entry = history
        .iter()
        .find(|entry| entry.keyid == keyid)
        .ok_or(EnvelopeError::UnknownKey)?;
    if entry.revoked_at.is_some() {
        return Err(EnvelopeError::KeyRevoked);
    }
    let public: [u8; 32] = STANDARD
        .decode(&entry.public_key)
        .ok()
        .and_then(|octets| octets.try_into().ok())
        .ok_or(EnvelopeError::BadSignature)?;
    let key = VerifyingKey::from_bytes(&public).map_err(|_| EnvelopeError::BadSignature)?;
    let signature = STANDARD
        .decode(sig)
        .ok()
        .and_then(|octets| Signature::from_slice(&octets).ok())
        .ok_or(EnvelopeError::BadSignature)?;
    key.verify_strict(message, &signature)
        .map_err(|_| EnvelopeError::BadSignature)
}

/// The statement of an envelope that a key of `history` signed, when its one subject is the
/// attestation `subject`: the signature, then the statement, then the digest of the subject.
///
/// One valid signature is enough, whatever the others are; when none is valid the error is that
/// of the first.
pub fn validate(
    envelope: &[u8],
    history: &[KeyEntry],
    subject: &[u8],
) -> Result<Value, EnvelopeError> {
    let document: Value = serde_json::from_slice(envelope).map_err(|_| EnvelopeError::Malformed)?;
    let object = document.as_object().ok_or(EnvelopeError::Malformed)?;
    let text = |name: &str| object.get(name).and_then(Value::as_str);
    let payload_type = text("payloadType").ok_or(EnvelopeError::Malformed)?;
    let payload_text = text("payload").ok_or(EnvelopeError::Malformed)?;
    let signatures = object
        .get("signatures")
        .and_then(Value::as_array)
        .ok_or(EnvelopeError::Malformed)?;
    if payload_type != PAYLOAD_TYPE {
        return Err(EnvelopeError::PayloadType);
    }
    let payload = STANDARD
        .decode(payload_text)
        .map_err(|_| EnvelopeError::Payload)?;
    if signatures.is_empty() {
        return Err(EnvelopeError::NoSignature);
    }
    let message = pae(payload_type, &payload);
    let mut first = None;
    let mut accepted = false;
    for signature in signatures {
        match check_signature(signature, history, &message) {
            Ok(()) => {
                accepted = true;
                break;
            }
            Err(error) => {
                first.get_or_insert(error);
            }
        }
    }
    if !accepted {
        return Err(first.unwrap_or(EnvelopeError::NoSignature));
    }

    let statement: Value =
        serde_json::from_slice(&payload).map_err(|_| EnvelopeError::Statement)?;
    let fields = statement.as_object().ok_or(EnvelopeError::Statement)?;
    if fields.get("_type").and_then(Value::as_str) != Some(STATEMENT_TYPE)
        || fields.get("predicateType").and_then(Value::as_str) != Some(PREDICATE_TYPE)
        || !fields.contains_key("predicate")
    {
        return Err(EnvelopeError::Statement);
    }
    let subjects = fields
        .get("subject")
        .and_then(Value::as_array)
        .ok_or(EnvelopeError::Statement)?;
    let [only] = subjects.as_slice() else {
        return Err(EnvelopeError::Statement);
    };
    let digests = only
        .get("digest")
        .and_then(Value::as_object)
        .ok_or(EnvelopeError::Statement)?;
    let digest = match (
        only.get("name").and_then(Value::as_str),
        digests.len(),
        digests.get("sha256").and_then(Value::as_str),
    ) {
        (Some(SUBJECT_NAME), 1, Some(digest)) => digest,
        _ => return Err(EnvelopeError::Statement),
    };
    if digest != hex(&Sha256::digest(subject)) {
        return Err(EnvelopeError::SubjectMismatch);
    }
    Ok(statement)
}
