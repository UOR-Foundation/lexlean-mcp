//! What a reader is given of a record: the attested canonical LaTeX of a module, and the bundle
//! that can be checked offline.
//!
//! Both read the record that a verification left (`evidence`): the files below
//! `verifications/<attestation ID>/` are followed from `attestation.json` to the build manifest
//! and from the manifest to each rendering and source file, and a file whose digest is not the one
//! that names it is not given. A record that cannot be followed whole is a fault of the store, not
//! a miss, because it was written whole (SG-15).

use serde_json::Value;

use crate::attestation::{Refused, RECORD_MAXIMUM};
use crate::evidence::{digest_of, rows, MANIFEST, SOURCES};
use crate::store;

/// The area of the verifications, and of the key history, and of the exports.
const VERIFICATIONS: u32 = 3;
const EXPORTS: u32 = 5;
const KEYS: u32 = 6;

/// The files of a bundle that are not the record's own evidence.
const SUBJECT: &str = "attestation.json";
const ENVELOPE: &str = "envelope.json";
const HISTORY: &str = "keys.json";

/// The evidence of a record, followed from its `attestation.json` and checked against the digests.
struct Record {
    subject: Vec<u8>,
    manifest: Vec<u8>,
    renderings: Vec<(String, String)>,
    sources: Vec<(String, String)>,
}

fn read(root: &std::path::Path, id: &str, name: &str) -> Result<Vec<u8>, Refused> {
    match store::read(root, VERIFICATIONS, &format!("{id}/{name}"), RECORD_MAXIMUM) {
        Ok(Some(bytes)) => Ok(bytes),
        _ => Err(Refused::Store),
    }
}

fn read_bound(
    root: &std::path::Path,
    id: &str,
    name: &str,
    sha256: &str,
) -> Result<Vec<u8>, Refused> {
    let bytes = read(root, id, name)?;
    if digest_of(&bytes) == sha256 {
        Ok(bytes)
    } else {
        Err(Refused::Store)
    }
}

fn follow(root: &std::path::Path, id: &str) -> Result<Record, Refused> {
    let subject = read(root, id, SUBJECT)?;
    let attestation: Value = serde_json::from_slice(&subject).map_err(|_| Refused::Store)?;
    let named = attestation["build_manifest"]["sha256"]
        .as_str()
        .ok_or(Refused::Store)?;
    let manifest_bytes = read_bound(root, id, MANIFEST, named)?;
    let manifest: Value = serde_json::from_slice(&manifest_bytes).map_err(|_| Refused::Store)?;
    Ok(Record {
        subject,
        manifest: manifest_bytes,
        renderings: rows(&manifest, "outputs", "tex").map_err(|_| Refused::Store)?,
        sources: rows(&manifest, "inputs", "source").map_err(|_| Refused::Store)?,
    })
}

/// The rendering that `module` names, a path of the build manifest; an empty `module` names the
/// only rendering of a record that has one.
fn chosen<'a>(
    renderings: &'a [(String, String)],
    module: &str,
) -> Result<&'a (String, String), Refused> {
    match (module, renderings) {
        ("", [only]) => Ok(only),
        ("", _) => Err(Refused::Module),
        (named, _) => renderings
            .iter()
            .find(|(path, _)| path == named)
            .ok_or(Refused::Module),
    }
}

/// The canonical LaTeX of one module of the record `id`, and the path the manifest gives it.
pub fn present(
    root: &std::path::Path,
    id: &str,
    module: &str,
) -> Result<(String, Vec<u8>), Refused> {
    let record = follow(root, id)?;
    let (path, sha256) = chosen(&record.renderings, module)?;
    let rendering = read_bound(root, id, path, sha256)?;
    // The core puts the bytes in a text block: LaTeX that is not UTF-8 is not presentable.
    if std::str::from_utf8(&rendering).is_err() {
        return Err(Refused::Store);
    }
    Ok((path.clone(), rendering))
}

/// Write the bundle of the record `id` below `exports/<id>/` and return its key. Everything is
/// read and checked before the first file is written, so a record that is not whole leaves no
/// bundle.
pub fn export(root: &std::path::Path, id: &str, envelope: &[u8]) -> Result<String, Refused> {
    let record = follow(root, id)?;
    let history = match store::read(root, KEYS, "history", RECORD_MAXIMUM) {
        Ok(Some(bytes)) => bytes,
        _ => return Err(Refused::Store),
    };
    let mut files: Vec<(String, Vec<u8>)> = vec![
        (SUBJECT.to_owned(), record.subject.clone()),
        (ENVELOPE.to_owned(), envelope.to_vec()),
        (HISTORY.to_owned(), history),
        (MANIFEST.to_owned(), record.manifest.clone()),
    ];
    for (path, sha256) in &record.renderings {
        files.push((path.clone(), read_bound(root, id, path, sha256)?));
    }
    for (path, sha256) in &record.sources {
        files.push((
            format!("{SOURCES}/{path}"),
            read_bound(root, id, &format!("{SOURCES}/{path}"), sha256)?,
        ));
    }
    let mut names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(Refused::Store);
    }
    for (name, bytes) in &files {
        store::write(
            root,
            EXPORTS,
            &format!("{id}/{name}"),
            bytes,
            RECORD_MAXIMUM,
        )
        .map_err(|_| Refused::Store)?;
    }
    Ok(format!("exports/{id}"))
}
