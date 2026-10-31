//! The records of verifications, and their lookup.
//!
//! A verification that succeeds leaves a record in the `verifications` area: the signed envelope
//! `<attestation ID>.json` and the `attestation.json` it signs, `<attestation ID>/attestation.json`,
//! the evidence that `attestation.json` binds below `<attestation ID>/` (`evidence`: the build
//! manifest, the canonical LaTeX of each module and the source files), and an entry of the `index`
//! area for the digest of each source file it verified, whose content is the attestation ID. The lookup returns the envelope, which a reader validates against the
//! key history (`signer`).

use std::path::Path;

use lexlean::Sha256Digest;

use crate::config::DRAFT_MAX_BYTES;
use crate::presentation;
use crate::store::{self, StoreError};

/// The most bytes of a record.
pub const RECORD_MAXIMUM: usize = 1 << 20;

/// Whether `text` is a SHA-256 in lowercase hexadecimal.
pub fn is_digest(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
}

fn sources(directory: &Path, found: &mut Vec<Vec<u8>>, depth: usize) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    let mut paths: Vec<_> = entries.flatten().map(|entry| entry.path()).collect();
    paths.sort();
    for path in paths {
        let hidden = path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with('.'));
        let Ok(metadata) = std::fs::symlink_metadata(&path) else {
            continue;
        };
        if hidden || metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() && depth < 16 {
            sources(&path, found, depth + 1);
        } else if metadata.is_file()
            && path.to_string_lossy().ends_with(".lex.tex")
            && metadata.len() <= DRAFT_MAX_BYTES as u64
        {
            if let Ok(bytes) = std::fs::read(&path) {
                found.push(bytes);
            }
        }
    }
}

/// The SHA-256 of each source file below `directory`, in the order of their paths.
pub fn source_digests(directory: &Path) -> Vec<String> {
    let mut files = Vec::new();
    sources(directory, &mut files, 0);
    files
        .iter()
        .map(|bytes| Sha256Digest::of(bytes).to_hex())
        .collect()
}

/// Write the record of a verification: the `attestation.json` that was verified, then the
/// `evidence` it binds, then the signed envelope that names it, then the index entry of each
/// source file in `digests`. The envelope is written after its subject and its evidence, so a
/// lookup never finds an envelope without the files it binds; a failure leaves no envelope, and
/// the caller reports the verification as failed.
pub fn record(
    root: &Path,
    attestation: &str,
    digests: &[String],
    subject: &[u8],
    evidence: &[(String, Vec<u8>)],
    envelope: &[u8],
) -> Result<(), ()> {
    if !is_digest(attestation) {
        return Err(());
    }
    store::write(
        root,
        3,
        &format!("{attestation}/attestation.json"),
        subject,
        RECORD_MAXIMUM,
    )
    .map_err(|_| ())?;
    for (key, bytes) in evidence {
        store::write(
            root,
            3,
            &format!("{attestation}/{key}"),
            bytes,
            RECORD_MAXIMUM,
        )
        .map_err(|_| ())?;
    }
    store::write(
        root,
        3,
        &format!("{attestation}.json"),
        envelope,
        RECORD_MAXIMUM,
    )
    .map_err(|_| ())?;
    for digest in digests {
        store::write(
            root,
            4,
            &format!("by-file/{digest}"),
            attestation.as_bytes(),
            64,
        )
        .map_err(|_| ())?;
    }
    Ok(())
}

/// Why a lookup was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The value is not a digest, or `by` is neither 0 nor 1.
    Value,
    /// The store failed, or the record it holds is not whole.
    Store,
    /// The module names no rendering of the record, or names none where the record has several.
    Module,
}

/// What a lookup returns of a record: the envelope (want 0), the rendering of a module (`PRESENT`)
/// or the key of the bundle written for it (`EXPORT`).
pub const PRESENT: u32 = 1;
pub const EXPORT: u32 = 2;

/// The attestation ID that `value` names: itself (`by` 0) or the one the index holds for the
/// digest of a source file it verified (`by` 1). `None` when the index holds none.
fn identify(root: &Path, by: u32, value: &str) -> Result<Option<String>, Refused> {
    if !is_digest(value) {
        return Err(Refused::Value);
    }
    let identifier = match by {
        0 => value.to_owned(),
        1 => match store::read(root, 4, &format!("by-file/{value}"), 64) {
            Ok(Some(found)) => String::from_utf8(found).map_err(|_| Refused::Store)?,
            Ok(None) => return Ok(None),
            Err(_) => return Err(Refused::Store),
        },
        _ => return Err(Refused::Value),
    };
    if !is_digest(&identifier) {
        return Err(Refused::Store);
    }
    Ok(Some(identifier))
}

/// The record that `value` names: by attestation ID (`by` 0) or by the digest of a source
/// file it verified (`by` 1).
pub fn lookup(root: &Path, by: u32, value: &str) -> Result<Option<Vec<u8>>, Refused> {
    let Some(identifier) = identify(root, by, value)? else {
        return Ok(None);
    };
    match store::read(root, 3, &format!("{identifier}.json"), RECORD_MAXIMUM) {
        Ok(found) => Ok(found),
        Err(StoreError::Area | StoreError::Key | StoreError::Io) => Err(Refused::Store),
    }
}

/// A record that was found, as far as the view asked.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Found {
    pub id: String,
    pub envelope: Vec<u8>,
    pub module: String,
    pub rendering: Vec<u8>,
    pub bundle: String,
}

/// The record `value` names, in a view: the envelope (`ENVELOPE`), the envelope and the canonical
/// LaTeX of `module` (`PRESENT`), or the envelope and the key of the bundle written for it
/// (`EXPORT`, the only view that writes).
pub fn find(
    root: &Path,
    by: u32,
    value: &str,
    view: u32,
    module: &str,
) -> Result<Option<Found>, Refused> {
    if view > EXPORT {
        return Err(Refused::Value);
    }
    let Some(id) = identify(root, by, value)? else {
        return Ok(None);
    };
    let Some(envelope) = lookup(root, 0, &id)? else {
        return Ok(None);
    };
    let mut found = Found {
        id: id.clone(),
        envelope,
        ..Found::default()
    };
    match view {
        PRESENT => (found.module, found.rendering) = presentation::present(root, &id, module)?,
        EXPORT => found.bundle = presentation::export(root, &id, &found.envelope)?,
        _ => {}
    }
    Ok(Some(found))
}
