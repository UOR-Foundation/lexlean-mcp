//! A draft project: a directory of the `drafts` area that holds a LexLean project.

use std::path::{Path, PathBuf};

use camino::Utf8PathBuf;
use lexlean::Selection;

use crate::store::{self, StoreError};

/// The draft project `name` of the store at `root`, when it is one: a directory of the
/// `drafts` area, reached through no link, that holds `lexlean.toml` as a file.
pub fn resolve(root: &Path, name: &str) -> Option<PathBuf> {
    if !store::name_is_valid(name) {
        return None;
    }
    let drafts = store::area_directory(root, 1).ok()?;
    match store::directory_exists(&drafts, name) {
        Ok(true) => {}
        _ => return None,
    }
    let directory = drafts.join(name);
    match store::read_in(&directory, "lexlean.toml", 1 << 20) {
        Ok(Some(_)) => Some(directory),
        Ok(None) | Err(StoreError::Area | StoreError::Key | StoreError::Io) => None,
    }
}

/// The selection a request names: 0 the entrypoints, 1 every module, 2 the files listed one
/// to a line. The error is the message of the configuration error it is.
pub fn selection(kind: u32, files: &str) -> Result<Selection, String> {
    match kind {
        0 => Ok(Selection::Entrypoints),
        1 => Ok(Selection::All),
        2 => {
            let mut selected = std::collections::BTreeSet::new();
            for line in files.split('\n') {
                if !store::key_is_valid(line) {
                    return Err(format!("`{line}` is not a project-relative file"));
                }
                selected.insert(Utf8PathBuf::from(line));
            }
            Ok(Selection::Files(selected))
        }
        other => Err(format!(
            "{other} is not a selection: 0 entrypoints, 1 all, 2 files"
        )),
    }
}
