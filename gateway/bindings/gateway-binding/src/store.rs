//! The store: seven directories below the configured root, one for each area of
//! `STORE.md`, holding files that are named by keys.
//!
//! A key is a relative path of one or more segments. A segment is 1 to 255 letters, digits,
//! `.`, `_` or `-`, and does not begin with `.`, so a key cannot name the parent, a hidden
//! file or the temporary file of a write. The host never follows a symbolic link below the
//! root: a component that is one is refused, and a file is opened without following one.

use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

/// The seven areas, by number: the order of the layout of `src/Store.lex.tex`.
pub const AREAS: [&str; 7] = [
    "profiles",
    "drafts",
    "builds",
    "verifications",
    "index",
    "exports",
    "keys",
];

/// The longest key, in bytes.
pub const KEY_MAXIMUM: usize = 1024;

/// Why the store did not do what it was asked.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    /// The area number is not one of the seven.
    Area,
    /// The key is not a key.
    Key,
    /// The file system refused, or the path crosses a link, or the entry is not a file.
    Io,
}

/// The name of an area.
pub fn area_name(number: u32) -> Option<&'static str> {
    AREAS.get(usize::try_from(number).ok()?).copied()
}

fn segment_is_valid(segment: &str) -> bool {
    !segment.is_empty()
        && segment.len() <= 255
        && !segment.starts_with('.')
        && segment
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

/// Whether `key` is a key of the store.
pub fn key_is_valid(key: &str) -> bool {
    !key.is_empty() && key.len() <= KEY_MAXIMUM && key.split('/').all(segment_is_valid)
}

/// Whether `name` is a key of one segment: the name of a profile or of a draft project.
pub fn name_is_valid(name: &str) -> bool {
    segment_is_valid(name) && name.len() <= 64
}

/// What lies at a path that is below a directory the host trusts.
enum Entry {
    Missing,
    Directory,
    File,
}

fn entry(path: &Path) -> Result<Entry, StoreError> {
    match fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(StoreError::Io),
        Ok(metadata) if metadata.is_dir() => Ok(Entry::Directory),
        Ok(metadata) if metadata.is_file() => Ok(Entry::File),
        Ok(_) => Err(StoreError::Io),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(Entry::Missing),
        Err(_) => Err(StoreError::Io),
    }
}

/// The directory of an area, or why there is none.
pub fn area_directory(root: &Path, area: u32) -> Result<PathBuf, StoreError> {
    area_name(area)
        .map(|name| root.join(name))
        .ok_or(StoreError::Area)
}

/// The path of a key below a directory, when no component that exists is a link and every
/// parent that exists is a directory; `None` when a component is absent.
fn existing_path(directory: &Path, key: &str) -> Result<Option<PathBuf>, StoreError> {
    let mut path = directory.to_path_buf();
    let segments: Vec<&str> = key.split('/').collect();
    match entry(&path)? {
        Entry::Missing => return Ok(None),
        Entry::Directory => {}
        Entry::File => return Err(StoreError::Io),
    }
    for (index, segment) in segments.iter().enumerate() {
        path.push(segment);
        let last = index + 1 == segments.len();
        match (entry(&path)?, last) {
            (Entry::Missing, _) => return Ok(None),
            (Entry::Directory, false) | (Entry::File, true) => {}
            _ => return Err(StoreError::Io),
        }
    }
    Ok(Some(path))
}

/// Whether `key` below `directory` is an existing directory.
pub fn directory_exists(directory: &Path, key: &str) -> Result<bool, StoreError> {
    if !key_is_valid(key) {
        return Err(StoreError::Key);
    }
    let mut path = directory.to_path_buf();
    if !matches!(entry(&path)?, Entry::Directory) {
        return Ok(false);
    }
    for segment in key.split('/') {
        path.push(segment);
        if !matches!(entry(&path)?, Entry::Directory) {
            return Ok(false);
        }
    }
    Ok(true)
}

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const NOFOLLOW: i32 = 0o400000;
#[cfg(all(target_os = "linux", target_arch = "aarch64"))]
const NOFOLLOW: i32 = 0o100000;
#[cfg(not(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
)))]
const NOFOLLOW: i32 = 0;

pub(crate) fn open_for_reading(path: &Path) -> std::io::Result<fs::File> {
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        // O_NONBLOCK, so that a pipe put in place of a file cannot block the host.
        options.custom_flags(0o4000 | NOFOLLOW);
    }
    options.open(path)
}

/// Read the file `key` below `directory`: `None` when there is none, and an error when it
/// is longer than `limit`.
pub fn read_in(directory: &Path, key: &str, limit: usize) -> Result<Option<Vec<u8>>, StoreError> {
    if !key_is_valid(key) {
        return Err(StoreError::Key);
    }
    let Some(path) = existing_path(directory, key)? else {
        return Ok(None);
    };
    let file = match open_for_reading(&path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(_) => return Err(StoreError::Io),
    };
    if !file.metadata().map_err(|_| StoreError::Io)?.is_file() {
        return Err(StoreError::Io);
    }
    let mut content = Vec::new();
    let bound = u64::try_from(limit).unwrap_or(u64::MAX).saturating_add(1);
    file.take(bound)
        .read_to_end(&mut content)
        .map_err(|_| StoreError::Io)?;
    if content.len() > limit {
        return Err(StoreError::Io);
    }
    Ok(Some(content))
}

/// Read the file `key` of an area of the store at `root`.
pub fn read(
    root: &Path,
    area: u32,
    key: &str,
    limit: usize,
) -> Result<Option<Vec<u8>>, StoreError> {
    let directory = area_directory(root, area)?;
    read_in(&directory, key, limit)
}

fn ensure_directory(path: &Path) -> Result<(), StoreError> {
    match entry(path)? {
        Entry::Directory => Ok(()),
        Entry::File => Err(StoreError::Io),
        Entry::Missing => match fs::create_dir(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => match entry(path)? {
                Entry::Directory => Ok(()),
                _ => Err(StoreError::Io),
            },
            Err(_) => Err(StoreError::Io),
        },
    }
}

static WRITES: AtomicU64 = AtomicU64::new(0);

/// Write `content` as the file `key` below `directory`, all of it or none of it: the bytes
/// go to a file of the same directory that is renamed over the key. The directories of the
/// key are made, never through a link. Returns the length written.
pub fn write_in(
    directory: &Path,
    key: &str,
    content: &[u8],
    limit: usize,
) -> Result<u64, StoreError> {
    if !key_is_valid(key) {
        return Err(StoreError::Key);
    }
    if content.len() > limit {
        return Err(StoreError::Io);
    }
    if let Some(parent) = directory.parent() {
        fs::create_dir_all(parent).map_err(|_| StoreError::Io)?;
    }
    ensure_directory(directory)?;
    let segments: Vec<&str> = key.split('/').collect();
    let mut parent = directory.to_path_buf();
    for segment in &segments[..segments.len() - 1] {
        parent.push(segment);
        ensure_directory(&parent)?;
    }
    let target = parent.join(segments[segments.len() - 1]);
    match entry(&target)? {
        Entry::Directory => return Err(StoreError::Io),
        Entry::File | Entry::Missing => {}
    }
    let temporary = parent.join(format!(
        ".write-{}-{}",
        std::process::id(),
        WRITES.fetch_add(1, Ordering::Relaxed)
    ));
    let written = (|| -> std::io::Result<()> {
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)?;
        file.write_all(content)?;
        file.sync_all()?;
        fs::rename(&temporary, &target)
    })();
    if written.is_err() {
        let _ = fs::remove_file(&temporary);
        return Err(StoreError::Io);
    }
    Ok(content.len() as u64)
}

/// Write the file `key` of an area of the store at `root`.
pub fn write(
    root: &Path,
    area: u32,
    key: &str,
    content: &[u8],
    limit: usize,
) -> Result<u64, StoreError> {
    let directory = area_directory(root, area)?;
    write_in(&directory, key, content, limit)
}
