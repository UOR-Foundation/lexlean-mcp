//! Materialization: a draft and the bundle of its profile, copied into a project directory.
//!
//! A draft is the one key `<profile>/<id>` of the `drafts` area. Its content is the preimage
//! whose SHA-256 is `<id>` (`src/Identity.lex.tex`, `encDraft`): the domain, the 32 octets of
//! the profile ID and the (path, bytes) rows. The profile is the directory `profiles/<profile>/`;
//! its entry `profile` is the record of the profile and is not part of the bundle. The project is
//! `builds/<id>/`: the files of the bundle and then the files of the draft, each **copied** into
//! a file of its own, so that nothing in the project is a link and LexLean (SPEC 25.1) sees none.
//!
//! A bundle that holds a symbolic link anywhere is refused before a byte of it is read, and so is
//! any entry that is neither a directory nor a regular file. Every materialization is a directory
//! of its own, so that two calls on one draft, or a verification that is still running, never see
//! each other's files: the `builds` area is a cache that can be dropped (`src/Store.lex.tex`).

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use crate::{lexicon, store};

static MATERIALIZED: AtomicU64 = AtomicU64::new(0);

/// The domain of the preimage of a draft.
pub const DOMAIN: &[u8] = b"lexlean-mcp-draft-v1";

/// The entry of a profile that holds its record, and is not a file of the bundle.
pub const RECORD: &str = "profile";

/// The entries of a profile that the host derives at install time: the record, the served
/// vocabulary and the review records. None of them is a file of the project a draft is checked in.
const DERIVED: [&str; 3] = [RECORD, "vocabulary", "reviews"];

/// Why a draft could not be materialized. The host reports all of them as `project-refused`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// There is no such draft, or its bytes are not those of its identity, or they do not decode.
    Draft,
    /// There is no such profile, its record is malformed, or it is not the profile of the draft.
    Profile,
    /// The bundle holds a symbolic link, or an entry that is neither a directory nor a file.
    Link,
    /// The file system refused.
    Io,
}

/// The profile and the identity of a handle `<profile>/<id>`, when it is one.
pub fn split_handle(handle: &str) -> Option<(&str, &str)> {
    let (profile, id) = handle.split_once('/')?;
    let lower_hex = id.len() == 64 && id.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f'));
    (store::name_is_valid(profile) && lower_hex).then_some((profile, id))
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let end = self.at.checked_add(count)?;
        let taken = self.bytes.get(self.at..end)?;
        self.at = end;
        Some(taken)
    }

    fn word(&mut self) -> Option<u64> {
        Some(u64::from_be_bytes(self.take(8)?.try_into().ok()?))
    }

    /// `head` is a length-prefixed name; the frame is the head, a 64-bit length and the payload.
    fn frame(&mut self, name: &str) -> Option<&'a [u8]> {
        let length = u32::try_from(name.len()).ok()?.to_be_bytes();
        if self.take(4)? != length || self.take(name.len())? != name.as_bytes() {
            return None;
        }
        let payload = usize::try_from(self.word()?).ok()?;
        self.take(payload)
    }
}

/// The (path, content) rows of a draft, as they lie in its preimage.
pub type Rows<'a> = Vec<(&'a [u8], &'a [u8])>;

/// The profile ID and the rows of a preimage, or `None` when the bytes are not a preimage.
pub fn decode(bytes: &[u8]) -> Option<([u8; 32], Rows<'_>)> {
    let mut reader = Reader { bytes, at: 0 };
    if reader.frame("domain")? != DOMAIN {
        return None;
    }
    let profile: [u8; 32] = reader.frame("profile")?.try_into().ok()?;
    let head = u32::try_from("sources".len()).ok()?.to_be_bytes();
    if reader.take(4)? != head || reader.take("sources".len())? != b"sources" {
        return None;
    }
    let fields = reader.word()?;
    if fields % 2 != 0 {
        return None;
    }
    let mut rows = Vec::new();
    for _ in 0..fields / 2 {
        let path = reader.frame("source")?;
        let content = reader.frame("source")?;
        rows.push((path, content));
    }
    (reader.at == bytes.len()).then_some((profile, rows))
}

/// Whether `path` is a path the host writes below a project: relative, in components that are
/// not empty, `.` or `..`, with no NUL, and a `.lex.tex` file.
fn path_is_safe(path: &str) -> bool {
    !path.is_empty()
        && !path.contains('\0')
        && path.ends_with(".lex.tex")
        && path
            .split('/')
            .all(|component| !matches!(component, "" | "." | ".."))
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The ID of the profile a record states: its first line, 64 hexadecimal digits.
fn record_id(record: &[u8]) -> Option<&str> {
    let text = std::str::from_utf8(record).ok()?;
    let line = text.split('\n').next()?;
    (line.len() == 64 && line.bytes().all(|b| matches!(b, b'0'..=b'9' | b'a'..=b'f')))
        .then_some(line)
}

/// Copy the regular files of `from` below `to`, refusing every link.
fn copy_bundle(from: &Path, to: &Path, top: bool) -> Result<(), Refused> {
    let entries = fs::read_dir(from).map_err(|_| Refused::Io)?;
    for entry in entries {
        let entry = entry.map_err(|_| Refused::Io)?;
        let name = entry.file_name();
        if top && DERIVED.iter().any(|derived| name == *derived) {
            continue;
        }
        let source = entry.path();
        let metadata = fs::symlink_metadata(&source).map_err(|_| Refused::Io)?;
        let kind = metadata.file_type();
        if kind.is_symlink() || !(kind.is_dir() || kind.is_file()) {
            return Err(Refused::Link);
        }
        let target = to.join(&name);
        if kind.is_dir() {
            fs::create_dir(&target).map_err(|_| Refused::Io)?;
            copy_bundle(&source, &target, false)?;
        } else {
            copy_file(&source, &target)?;
        }
    }
    Ok(())
}

/// Copy one file without following a link that appeared since it was inspected.
fn copy_file(source: &Path, target: &Path) -> Result<(), Refused> {
    let mut input = store::open_for_reading(source).map_err(|_| Refused::Link)?;
    if !input.metadata().map_err(|_| Refused::Io)?.is_file() {
        return Err(Refused::Link);
    }
    let mut output = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(target)
        .map_err(|_| Refused::Io)?;
    let mut buffer = [0u8; 1 << 16];
    loop {
        let count = input.read(&mut buffer).map_err(|_| Refused::Io)?;
        if count == 0 {
            return output.flush().map_err(|_| Refused::Io);
        }
        output
            .write_all(&buffer[..count])
            .map_err(|_| Refused::Io)?;
    }
}

/// Materialize the draft `<profile>/<id>` of the store at `root` as a directory of the `builds`
/// area, and return it. `limit` is the most bytes a draft may have.
pub fn materialize(root: &Path, profile: &str, id: &str, limit: usize) -> Result<PathBuf, Refused> {
    let blob = store::read(root, 1, &format!("{profile}/{id}"), limit)
        .map_err(|_| Refused::Draft)?
        .ok_or(Refused::Draft)?;
    if hex(&Sha256::digest(&blob)) != id {
        return Err(Refused::Draft);
    }
    let (profile_id, rows) = decode(&blob).ok_or(Refused::Draft)?;
    if rows
        .iter()
        .any(|(path, _)| std::str::from_utf8(path).map_or(true, |path| !path_is_safe(path)))
    {
        return Err(Refused::Draft);
    }
    let record = lexicon::read(root, profile, RECORD)
        .map_err(|_| Refused::Profile)?
        .ok_or(Refused::Profile)?;
    if record_id(&record) != Some(hex(&profile_id).as_str()) {
        return Err(Refused::Profile);
    }
    let bundle = store::area_directory(root, 0)
        .map_err(|_| Refused::Io)?
        .join(profile);
    let builds = store::area_directory(root, 2).map_err(|_| Refused::Io)?;
    fs::create_dir_all(&builds).map_err(|_| Refused::Io)?;
    let project = builds.join(format!(
        "{id}-{}-{}",
        std::process::id(),
        MATERIALIZED.fetch_add(1, Ordering::Relaxed)
    ));
    fs::create_dir(&project).map_err(|_| Refused::Io)?;
    let done = (|| -> Result<(), Refused> {
        copy_bundle(&bundle, &project, true)?;
        for (path, content) in &rows {
            let path = std::str::from_utf8(path).map_err(|_| Refused::Draft)?;
            let target = project.join(path);
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent).map_err(|_| Refused::Io)?;
            }
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&target)
                .map_err(|_| Refused::Draft)?;
            file.write_all(content).map_err(|_| Refused::Io)?;
            file.sync_all().map_err(|_| Refused::Io)?;
        }
        Ok(())
    })();
    match done {
        Ok(()) => Ok(project),
        Err(refused) => {
            let _ = fs::remove_dir_all(&project);
            Err(refused)
        }
    }
}
