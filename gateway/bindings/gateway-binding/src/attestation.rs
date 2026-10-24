//! The records of verifications, and their lookup.
//!
//! A verification that succeeds leaves a record in the `verifications` area, named by its
//! attestation ID, and an entry of the `index` area for the digest of each source file it
//! verified, whose content is the attestation ID. The signed envelope of the record is the
//! work of the signing issue; the record is the unsigned document it will wrap.

use std::path::Path;

use lexlean::Sha256Digest;
use serde_json::Value;

use crate::config::DRAFT_MAX_BYTES;
use crate::store::{self, StoreError};

/// The most bytes of a record.
pub const RECORD_MAXIMUM: usize = 1 << 20;

/// Whether `text` is a SHA-256 in lowercase hexadecimal.
pub fn is_digest(text: &str) -> bool {
    text.len() == 64 && text.bytes().all(|byte| matches!(byte, b'0'..=b'9' | b'a'..=b'f'))
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

/// Write the record of a verification of the project in `directory`, and the index entry of
/// each of its source files.
pub fn record(
    root: &Path,
    directory: &Path,
    attestation: &str,
    record: &Value,
) -> Result<(), ()> {
    if !is_digest(attestation) {
        return Err(());
    }
    let mut files = Vec::new();
    sources(directory, &mut files, 0);
    let digests: Vec<String> = files
        .iter()
        .map(|bytes| Sha256Digest::of(bytes).to_hex())
        .collect();
    let mut document = record.clone();
    if let Some(object) = document.as_object_mut() {
        object.insert("files".to_owned(), Value::from(digests.clone()));
    }
    let bytes = serde_json::to_vec(&document).map_err(|_| ())?;
    store::write(root, 3, &format!("{attestation}.json"), &bytes, RECORD_MAXIMUM).map_err(|_| ())?;
    for digest in digests {
        store::write(root, 4, &format!("by-file/{digest}"), attestation.as_bytes(), 64)
            .map_err(|_| ())?;
    }
    Ok(())
}

/// Why a lookup was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The value is not a digest, or `by` is neither 0 nor 1.
    Value,
    /// The store failed.
    Store,
}

/// The record that `value` names: by attestation ID (`by` 0) or by the digest of a source
/// file it verified (`by` 1).
pub fn lookup(root: &Path, by: u32, value: &str) -> Result<Option<Vec<u8>>, Refused> {
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
    match store::read(root, 3, &format!("{identifier}.json"), RECORD_MAXIMUM) {
        Ok(found) => Ok(found),
        Err(StoreError::Area | StoreError::Key | StoreError::Io) => Err(Refused::Store),
    }
}
