//! A project the Engine runs on: a draft, or a directory of the `drafts` area that holds a
//! LexLean project.

use std::path::{Path, PathBuf};

use camino::Utf8PathBuf;
use lexlean::Selection;

use crate::materialize;
use crate::store::{self, StoreError};
use crate::telemetry::Telemetry;

/// A project that has been resolved to a directory.
///
/// The directory of a draft is a copy made for this call and is removed with the value; a
/// directory project is the one in the `drafts` area and stays.
pub struct Resolved {
    directory: PathBuf,
    ephemeral: bool,
}

impl Resolved {
    /// The directory the Engine runs on.
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// The directory, kept after the value is gone.
    pub fn keep(mut self) -> PathBuf {
        self.ephemeral = false;
        std::mem::take(&mut self.directory)
    }
}

impl Drop for Resolved {
    fn drop(&mut self) {
        if self.ephemeral {
            let _ = std::fs::remove_dir_all(&self.directory);
        }
    }
}

/// The project `name` of the store at `root`, when it is one.
///
/// A name of the form `<profile>/<draft id>` is a draft: it is materialized by copying the
/// profile bundle and the files of the draft into a directory of the `builds` area, and
/// refused when the draft or the profile is absent or the bundle holds a symbolic link. Any
/// other name is a directory of the `drafts` area, reached through no link, that holds
/// `lexlean.toml` as a file. `limit` is the most bytes a stored draft may have.
pub fn resolve(root: &Path, name: &str, limit: usize, telemetry: &Telemetry) -> Option<Resolved> {
    if let Some((profile, id)) = materialize::split_handle(name) {
        let mut drifted = false;
        let materialized = materialize::materialize_noting(root, profile, id, limit, &mut drifted);
        if drifted {
            telemetry.profile_drift(profile);
        }
        return materialized.ok().map(|directory| Resolved {
            directory,
            ephemeral: true,
        });
    }
    directory_project(root, name).map(|directory| Resolved {
        directory,
        ephemeral: false,
    })
}

/// The draft project `name` of the store at `root`, when it is a directory project.
fn directory_project(root: &Path, name: &str) -> Option<PathBuf> {
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
