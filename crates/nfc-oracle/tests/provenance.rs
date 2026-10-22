//! NF-07: the package the oracle runs is the recorded export of the NFC root.

mod common;

use std::path::Path;

use common::{hex, root, sha256};
use sha2::{Digest, Sha256};

fn generated() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join("lexlean_mcp_nfc")
}

fn provenance() -> serde_json::Value {
    serde_json::from_slice(
        &std::fs::read(generated().join("provenance.json")).expect("provenance.json is committed"),
    )
    .expect("provenance.json is JSON")
}

#[test]
fn the_generated_package_is_the_recorded_export_of_the_nfc_root_nf_07() {
    let provenance = provenance();
    assert_eq!(provenance["name"], "lexlean_mcp_nfc");
    assert_eq!(provenance["profile"], "rust-std");
    assert_eq!(provenance["spec"], "lexlean/rust-provenance/1");
    let exports = provenance["exports"].as_array().expect("exports is a list");
    assert_eq!(exports.len(), 1, "one root is exported");
    assert_eq!(exports[0]["name"], "run");
    assert_eq!(
        provenance["effects"],
        serde_json::json!(["allocation", "overflow", "recursion"]),
        "the export states the effects the root admits"
    );
    // The export records the digest of every file it wrote; the oracle compiled
    // exactly the recorded src/lib.rs, not an edited copy.
    let source =
        std::fs::read(generated().join("src").join("lib.rs")).expect("src/lib.rs is committed");
    let recorded = provenance["files"]["src/lib.rs"]
        .as_str()
        .expect("the export records src/lib.rs");
    assert_eq!(
        recorded,
        hex(&Sha256::digest(&source)),
        "src/lib.rs is not the file the export recorded: regenerate it, never edit it"
    );
    assert_eq!(
        recorded,
        sha256(nfc_oracle::GENERATED_SOURCE.as_bytes()),
        "the oracle compiled a different file from the one recorded"
    );
    let manifest = std::fs::read(generated().join("Cargo.toml")).expect("Cargo.toml is committed");
    assert_eq!(
        provenance["files"]["Cargo.toml"]
            .as_str()
            .expect("the export records Cargo.toml"),
        sha256(&manifest),
        "Cargo.toml is not the file the export recorded"
    );
}

#[test]
fn the_export_was_made_by_the_compiler_the_lock_names_nf_07() {
    let lock = std::fs::read_to_string(root().join("lexlean.lock")).expect("lexlean.lock reads");
    let line = lock
        .lines()
        .find(|l| l.starts_with("compiler_semantics = "))
        .expect("the lock records the compiler semantics ID");
    let id = line
        .trim_start_matches("compiler_semantics = ")
        .trim_matches('"');
    assert_eq!(
        provenance()["compiler_semantics"].as_str(),
        Some(id),
        "the package was exported by another LexLean than the lock names"
    );
}

#[test]
fn the_oracle_reaches_the_generated_package_nf_07() {
    // A run through the package, not a re-implementation: the verdicts differ
    // on a string only the tables can tell apart.
    assert!(!nfc_oracle::is_nfc(&[0x65, 0x301]));
    assert!(nfc_oracle::is_nfc(&[0xE9]));
}
