//! The Unicode 17.0.0 normalization tables of the LexLean NFC decision (issue #25).
//!
//! Dev and CI only. The decision is LexLean source in `src/Nfc.lex.tex`; the
//! tables it reads are generated here from the pinned Unicode Character Database
//! files by [`tables::pack`] and [`tables::module`], and the test suite compares
//! the committed `artifacts/nfc-tables-low.bin`, `nfc-tables-high.bin`,
//! `nfc-sample.bin` and `src/NfcTables.lex.tex` with a fresh generation byte for byte. Correctness is not established by the
//! generator: it is established by running the package LexLean exports for the
//! decision (crate `nfc-oracle`) over every row of `NormalizationTest.txt`.

use std::path::{Path, PathBuf};

pub mod tables;
pub mod ucd;

/// The Unicode version the decision implements: that of LexLean's pinned
/// `unicode-normalization` 0.1.25 (`UNICODE_VERSION` in its `src/tables.rs`).
pub const VERSION: (u8, u8, u8) = (17, 0, 0);

/// SHA-256 of each pinned file, in the order of [`ucd::FILES`].
pub const PINS: [(&str, &str); 4] = [
    (
        "UnicodeData.txt",
        "2e1efc1dcb59c575eedf5ccae60f95229f706ee6d031835247d843c11d96470c",
    ),
    (
        "CompositionExclusions.txt",
        "2f239196ef3b5b61db5cc476e9bd80f534d15aa1b74e1be1dea5d042a344c85f",
    ),
    (
        "DerivedNormalizationProps.txt",
        "71fd6a206a2c0cdd41feb6b7f656aa31091db45e9cedc926985d718397f9e488",
    ),
    (
        "NormalizationTest.txt",
        "5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db",
    ),
];

/// The directory holding the pinned Unicode Character Database files.
pub fn ucd_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("ucd")
}
