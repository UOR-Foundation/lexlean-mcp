//! Shared by the oracle tests: where the pinned files live and what they must be.
#![allow(dead_code)]

use std::path::{Path, PathBuf};

use nfc_tables::ucd::{self, Data, Files, Row};
#[allow(unused_imports)]
pub use nfc_tables::{PINS, VERSION};
use sha2::{Digest, Sha256};

/// The repository root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/nfc-oracle is two below the root")
        .to_path_buf()
}

/// The directory of the pinned files.
pub fn ucd_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ucd")
}

/// Lowercase hexadecimal of a digest.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// SHA-256 of a file's bytes.
pub fn sha256_of(path: &Path) -> String {
    hex(&Sha256::digest(
        std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display())),
    ))
}

/// The pinned files.
pub fn files() -> Files {
    Files::load(&ucd_dir()).expect("the pinned files read")
}

/// What the algorithms read from them.
pub fn data() -> Data {
    Data::parse(&files()).expect("the pinned files parse")
}

/// Every row of `NormalizationTest.txt`.
pub fn rows() -> Vec<Row> {
    ucd::rows(&files().normalization_test).expect("NormalizationTest.txt parses")
}

/// A scalar sequence as the `char`s it spells; the oracle encodes them to UTF-8
/// with the standard library, and the LexLean decision decodes that encoding.
pub fn text(scalars: &[u32]) -> String {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or_else(|| panic!("U+{c:04X} is not a scalar value")))
        .collect()
}
