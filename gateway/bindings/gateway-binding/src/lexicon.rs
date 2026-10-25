//! The lexicon: an entry of a profile of the `profiles` area.

use std::path::Path;

use crate::store::{self, StoreError};

/// The most bytes of an entry.
pub const ENTRY_MAXIMUM: usize = 1 << 20;

/// Why a read was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// There is no such profile.
    Profile,
    /// The entry is not a key, or the store failed on it.
    Key,
}

/// The entry `entry` of the profile `profile`: `None` when the profile has no such entry.
pub fn read(root: &Path, profile: &str, entry: &str) -> Result<Option<Vec<u8>>, Refused> {
    if !store::name_is_valid(profile) {
        return Err(Refused::Profile);
    }
    let profiles = store::area_directory(root, 0).map_err(|_| Refused::Profile)?;
    match store::directory_exists(&profiles, profile) {
        Ok(true) => {}
        Ok(false) | Err(_) => return Err(Refused::Profile),
    }
    if !store::key_is_valid(entry) {
        return Err(Refused::Key);
    }
    match store::read_in(&profiles.join(profile), entry, ENTRY_MAXIMUM) {
        Ok(found) => Ok(found),
        Err(StoreError::Area | StoreError::Key | StoreError::Io) => Err(Refused::Key),
    }
}
