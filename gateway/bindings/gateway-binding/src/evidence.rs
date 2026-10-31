//! What a record keeps beside its envelope: the evidence that the `attestation.json` it signs binds.
//!
//! `attestation.json` names the build manifest of the verification by its SHA-256
//! (`build_manifest.sha256`); the manifest names each output of the build and each source file
//! by theirs. A record keeps the manifest, the canonical LaTeX of each module (the outputs of kind
//! `tex`) and the source files (the inputs of kind `source`), so that a reader can follow the
//! digests from the signed envelope down to a rendering and to the file it came from. Everything is
//! read and checked here, before anything is written: a verification whose evidence cannot be
//! kept whole leaves no record (SIGNING.md, SG-15).

use std::path::Path;

use lexlean::Sha256Digest;
use serde_json::Value;

use crate::attestation::{is_digest, RECORD_MAXIMUM};
use crate::store;

/// The name of the build manifest in a verified directory, and in a record.
pub const MANIFEST: &str = "build-manifest.json";

/// The directory of a record that holds the source files, under their paths in the project.
pub const SOURCES: &str = "sources";

/// The most files a record keeps beside its envelope and its `attestation.json`.
pub const FILES_MAXIMUM: usize = 1024;

/// The files a record keeps, as keys below `<attestation ID>/` and their bytes, in the order
/// they are written: the manifest, the renderings, then the sources.
pub type Evidence = Vec<(String, Vec<u8>)>;

fn digest_of(bytes: &[u8]) -> String {
    Sha256Digest::of(bytes).to_hex()
}

/// The file `path` below `directory`, as the store reads a key: refused when `path` is not a key
/// (an absolute path, a `..` or a hidden segment), when it crosses a link, or when it is larger
/// than a record may be.
fn read_below(directory: &Path, path: &str, what: &str) -> Result<Vec<u8>, String> {
    match store::read_in(directory, path, RECORD_MAXIMUM) {
        Ok(Some(bytes)) => Ok(bytes),
        Ok(None) => Err(format!("{what} {path} is missing")),
        Err(_) => Err(format!("{what} {path} cannot be read")),
    }
}

/// The rows of `field` in the manifest whose `kind` is `kind`, as their path and their digest.
fn rows(manifest: &Value, field: &str, kind: &str) -> Result<Vec<(String, String)>, String> {
    let Some(all) = manifest[field].as_array() else {
        return Err(format!("the build manifest has no {field}"));
    };
    let mut found = Vec::new();
    for row in all.iter().filter(|row| row["kind"] == kind) {
        let (Some(path), Some(sha256)) = (row["path"].as_str(), row["sha256"].as_str()) else {
            return Err(format!(
                "a {kind} row of the build manifest has no path or digest"
            ));
        };
        if !is_digest(sha256) {
            return Err(format!("the {kind} row {path} has no digest"));
        }
        found.push((path.to_owned(), sha256.to_owned()));
    }
    Ok(found)
}

/// The file `path` below `directory`, whose SHA-256 must be `sha256`.
fn bound(directory: &Path, path: &str, sha256: &str, what: &str) -> Result<Vec<u8>, String> {
    let bytes = read_below(directory, path, what)?;
    if digest_of(&bytes) != sha256 {
        return Err(format!(
            "{what} {path} is not the file the build manifest names"
        ));
    }
    Ok(bytes)
}

/// The evidence of a verification whose verified directory is `verified`, whose project is
/// `project` and whose `attestation.json` is `subject`: the manifest that `subject` names, the
/// canonical LaTeX of each module and the source files, each checked against its digest. An
/// error says why the evidence cannot be kept.
pub fn gather(verified: &Path, project: &Path, subject: &[u8]) -> Result<Evidence, String> {
    let attestation: Value =
        serde_json::from_slice(subject).map_err(|_| "attestation.json is not JSON".to_owned())?;
    let Some(named) = attestation["build_manifest"]["sha256"].as_str() else {
        return Err("attestation.json names no build manifest".to_owned());
    };
    let manifest_bytes = bound(verified, MANIFEST, named, "the build manifest")?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes)
        .map_err(|_| "the build manifest is not JSON".to_owned())?;
    let renderings = rows(&manifest, "outputs", "tex")?;
    let sources = rows(&manifest, "inputs", "source")?;
    if renderings.is_empty() {
        return Err("the build manifest names no rendering".to_owned());
    }
    if sources.is_empty() {
        return Err("the build manifest names no source file".to_owned());
    }
    if renderings.len() + sources.len() + 1 > FILES_MAXIMUM {
        return Err("the build manifest names more files than a record keeps".to_owned());
    }
    let mut evidence = vec![(MANIFEST.to_owned(), manifest_bytes)];
    for (path, sha256) in &renderings {
        if path == MANIFEST
            || path == "attestation.json"
            || path.starts_with(&format!("{SOURCES}/"))
        {
            return Err(format!(
                "the rendering {path} has the name of another file of a record"
            ));
        }
        evidence.push((
            path.clone(),
            bound(verified, path, sha256, "the rendering")?,
        ));
    }
    for (path, sha256) in &sources {
        let bytes = bound(project, path, sha256, "the source file")?;
        evidence.push((format!("{SOURCES}/{path}"), bytes));
    }
    Ok(evidence)
}
