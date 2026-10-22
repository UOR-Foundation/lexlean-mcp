//! Shared by the oracle tests.
#![allow(dead_code)]

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use nfc_tables::ucd::{self, Data, Files, Row};
use sha2::{Digest, Sha256};

/// The repository root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/nfc-oracle is two below the root")
        .to_path_buf()
}

/// Lowercase hexadecimal.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of bytes, as hexadecimal.
pub fn sha256(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// The pinned files.
pub fn files() -> Files {
    Files::load(&nfc_tables::ucd_dir()).expect("the pinned files read")
}

/// What the algorithms read from them.
pub fn data() -> Data {
    Data::parse(&files()).expect("the pinned files parse")
}

/// Every row of `NormalizationTest.txt`.
pub fn rows() -> Vec<Row> {
    ucd::rows(&files().normalization_test).expect("NormalizationTest.txt parses")
}

/// The text of a scalar sequence.
pub fn text(scalars: &[u32]) -> String {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or_else(|| panic!("U+{c:04X} is not a scalar value")))
        .collect()
}

/// `U+0041 U+0301`, for a message.
pub fn show(scalars: &[u32]) -> String {
    scalars
        .iter()
        .map(|c| format!("{c:04X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// A function of a scalar sequence, remembered, because the file repeats strings.
pub struct Memo<T> {
    seen: HashMap<Vec<u32>, T>,
    calls: usize,
}

impl<T: Clone> Memo<T> {
    /// An empty memo.
    pub fn new() -> Self {
        Memo {
            seen: HashMap::new(),
            calls: 0,
        }
    }

    /// `f(key)`, computed once per distinct key.
    pub fn get(&mut self, key: &[u32], f: impl FnOnce(&[u32]) -> T) -> T {
        if let Some(found) = self.seen.get(key) {
            return found.clone();
        }
        self.calls += 1;
        let value = f(key);
        self.seen.insert(key.to_vec(), value.clone());
        value
    }

    /// How many distinct keys were computed.
    pub fn distinct(&self) -> usize {
        self.calls
    }
}
