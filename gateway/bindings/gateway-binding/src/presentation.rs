//! What a reader is given of a record: the attested canonical LaTeX of a module, and the bundle
//! that can be checked offline.
//!
//! Both read the record that a verification left (`evidence`): the files below
//! `verifications/<attestation ID>/` are followed from `attestation.json` to the build manifest
//! and from the manifest to each rendering and source file, and a file whose digest is not the one
//! that names it is not given. A record that cannot be followed whole is a fault of the store, not
//! a miss, because it was written whole (SG-15).

use serde_json::Value;

use crate::attestation::{Refused, ATTESTATION_MAXIMUM, RECORD_MAXIMUM};
use crate::evidence::{digest_of, rows, source_key, MANIFEST};
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
    let subject = match store::read(
        root,
        VERIFICATIONS,
        &format!("{id}/{SUBJECT}"),
        ATTESTATION_MAXIMUM,
    ) {
        Ok(Some(bytes)) => bytes,
        _ => return Err(Refused::Store),
    };
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

/// The most bytes of the files of a bundle that an export hands the client as embedded resources
/// (#94); a larger bundle is written to the store all the same, and not embedded.
pub const EMBED_MAXIMUM: usize = 4 << 20;

/// The most bytes of the list of the renderings of a record, as JSON.
pub const MODULES_MAXIMUM: usize = 256 << 10;

/// The module a rendering path of the manifest is of: `modules/<Prefix>/<Module>.tex` is of
/// `<Module>` (which holds `/` for a module below a directory).
fn module_of(path: &str) -> Option<&str> {
    let rest = path.strip_prefix("modules/")?;
    let (_, module) = rest.split_once('/')?;
    module.strip_suffix(".tex")
}

/// Whether `named`, a module as a client may write it, names the rendering `path`: the module
/// (`Main`, `Foundation/Arch` or `Foundation.Arch`), or the path of its source file
/// (`src/Main.lex.tex`), whose source root is the components before the module's.
fn names_module(named: &str, path: &str) -> bool {
    let Some(module) = module_of(path) else {
        return false;
    };
    let named = named.strip_suffix(".lex.tex").unwrap_or(named);
    named == module
        || named.replace('.', "/") == module
        || named
            .strip_suffix(module)
            .is_some_and(|root| root.ends_with('/'))
}

/// The rendering that `module` names: a path of the build manifest, or else the one rendering of
/// the module or of the source file it names; an empty `module` names the only rendering of a
/// record that has one. `None` when it names none, or several.
fn chosen<'a>(renderings: &'a [(String, String)], module: &str) -> Option<&'a (String, String)> {
    match (module, renderings) {
        ("", [only]) => Some(only),
        ("", _) => None,
        (named, _) => renderings
            .iter()
            .find(|(path, _)| path == named)
            .or_else(|| {
                let mut matching = renderings
                    .iter()
                    .filter(|(path, _)| names_module(named, path));
                match (matching.next(), matching.next()) {
                    (Some(one), None) => Some(one),
                    _ => None,
                }
            }),
    }
}

/// The paths of the renderings of a record, as a JSON array of strings, in the order of the
/// manifest: what a client may name as `module`.
fn modules_json(renderings: &[(String, String)]) -> Result<String, Refused> {
    let paths: Vec<&str> = renderings.iter().map(|(path, _)| path.as_str()).collect();
    let text = serde_json::to_string(&paths).map_err(|_| Refused::Store)?;
    if text.len() > MODULES_MAXIMUM {
        return Err(Refused::Store);
    }
    Ok(text)
}

/// What a present gives of a record: the path of the rendering it chose and its canonical LaTeX,
/// or an empty path and no bytes when `module` names no rendering; and the paths of every
/// rendering of the record, as a JSON array.
pub struct Presented {
    pub module: String,
    pub rendering: Vec<u8>,
    pub modules: String,
}

/// The canonical LaTeX of one module of the record `id`, and the path the manifest gives it. A
/// `module` that names no rendering, or several, is answered with the renderings there are, which
/// the core gives the client with its refusal `module-unknown` (#94).
pub fn present(root: &std::path::Path, id: &str, module: &str) -> Result<Presented, Refused> {
    let record = follow(root, id)?;
    let modules = modules_json(&record.renderings)?;
    let Some((path, sha256)) = chosen(&record.renderings, module) else {
        return Ok(Presented {
            module: String::new(),
            rendering: Vec::new(),
            modules,
        });
    };
    let rendering = read_bound(root, id, path, sha256)?;
    // The core puts the bytes in a text block: LaTeX that is not UTF-8 is not presentable.
    if std::str::from_utf8(&rendering).is_err() {
        return Err(Refused::Store);
    }
    Ok(Presented {
        module: path.clone(),
        rendering,
        modules,
    })
}

/// What an export gives of a record: the key of the bundle in the store, the paths of the
/// renderings as a JSON array, and the files of the bundle framed for the core (`frame_files`), or
/// nothing when the bundle is over `EMBED_MAXIMUM` or a file of it is not text.
pub struct Exported {
    pub bundle: String,
    pub modules: String,
    pub files: Vec<u8>,
}

/// The files of a bundle as the core reads them: for each, its path and its bytes, each with its
/// length in four octets, big-endian.
fn frame_files(files: &[(String, Vec<u8>)]) -> Vec<u8> {
    let mut out = Vec::new();
    for (name, bytes) in files {
        for part in [name.as_bytes(), bytes.as_slice()] {
            out.extend_from_slice(&(part.len() as u32).to_be_bytes());
            out.extend_from_slice(part);
        }
    }
    out
}

/// Write the bundle of the record `id` below `exports/<id>/` and return its key, and its files
/// for the client (#94). Everything is read and checked before the first file is written, so a
/// record that is not whole leaves no bundle.
pub fn export(root: &std::path::Path, id: &str, envelope: &[u8]) -> Result<Exported, Refused> {
    let record = follow(root, id)?;
    let modules = modules_json(&record.renderings)?;
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
        let key = source_key(path);
        let bytes = read_bound(root, id, &key, sha256)?;
        files.push((key, bytes));
    }
    let mut names: Vec<&str> = files.iter().map(|(name, _)| name.as_str()).collect();
    names.sort_unstable();
    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(Refused::Store);
    }
    // Each file was bounded when it was read, and `attestation.json` is the largest a record holds.
    for (name, bytes) in &files {
        store::write(
            root,
            EXPORTS,
            &format!("{id}/{name}"),
            bytes,
            ATTESTATION_MAXIMUM,
        )
        .map_err(|_| Refused::Store)?;
    }
    // The bundle is handed to the client as well, for a client that cannot reach the store: as
    // text, since the core makes a text resource of each file.
    let size: usize = files
        .iter()
        .map(|(name, bytes)| name.len() + bytes.len())
        .sum();
    let text = files
        .iter()
        .all(|(_, bytes)| std::str::from_utf8(bytes).is_ok());
    Ok(Exported {
        bundle: format!("exports/{id}"),
        modules,
        files: if text && size <= EMBED_MAXIMUM {
            frame_files(&files)
        } else {
            Vec::new()
        },
    })
}
