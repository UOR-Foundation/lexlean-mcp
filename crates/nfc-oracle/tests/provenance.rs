//! NF-07: the packages the oracle runs are the recorded exports of the NFC root and
//! the admission root.

mod common;

use std::path::Path;

use common::{hex, root, sha256};
use sha2::{Digest, Sha256};

/// The two packages: the export of `nfcRun` and the export of `admitRun`, with
/// the text of each as the oracle compiled it.
const PACKAGES: [(&str, &str); 2] = [
    ("lexlean_mcp_nfc", nfc_oracle::GENERATED_SOURCE),
    ("lexlean_mcp_admission", nfc_oracle::ADMISSION_SOURCE),
];

fn generated(package: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join(package)
}

fn provenance(package: &str) -> serde_json::Value {
    serde_json::from_slice(
        &std::fs::read(generated(package).join("provenance.json"))
            .expect("provenance.json is committed"),
    )
    .expect("provenance.json is JSON")
}

#[test]
fn the_generated_packages_are_the_recorded_exports_of_the_roots_nf_07() {
    for (package, compiled) in PACKAGES {
        let provenance = provenance(package);
        assert_eq!(provenance["name"], package);
        assert_eq!(provenance["profile"], "rust-std");
        assert_eq!(provenance["spec"], "lexlean/rust-provenance/1");
        let exports = provenance["exports"].as_array().expect("exports is a list");
        assert_eq!(exports.len(), 1, "{package}: one root is exported");
        assert_eq!(exports[0]["name"], "run");
        assert_eq!(
            provenance["effects"],
            serde_json::json!(["allocation", "overflow"]),
            "{package}: the export states the effects the root admits"
        );
        // The export records the digest of every file it wrote; the oracle compiled
        // exactly the recorded src/lib.rs, not an edited copy.
        let source = std::fs::read(generated(package).join("src").join("lib.rs"))
            .expect("src/lib.rs is committed");
        let recorded = provenance["files"]["src/lib.rs"]
            .as_str()
            .expect("the export records src/lib.rs");
        assert_eq!(
            recorded,
            hex(&Sha256::digest(&source)),
            "{package}: src/lib.rs is not the file the export recorded: regenerate it, never edit it"
        );
        assert_eq!(
            recorded,
            sha256(compiled.as_bytes()),
            "{package}: the oracle compiled a different file from the one recorded"
        );
        let manifest =
            std::fs::read(generated(package).join("Cargo.toml")).expect("Cargo.toml is committed");
        assert_eq!(
            provenance["files"]["Cargo.toml"]
                .as_str()
                .expect("the export records Cargo.toml"),
            sha256(&manifest),
            "{package}: Cargo.toml is not the file the export recorded"
        );
    }
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
    for (package, _) in PACKAGES {
        assert_eq!(
            provenance(package)["compiler_semantics"].as_str(),
            Some(id),
            "{package} was exported by another LexLean than the lock names"
        );
    }
}

#[test]
fn the_oracle_reaches_the_generated_package_nf_07() {
    // A run through the package, not a re-implementation: the verdicts differ
    // on a string only the tables can tell apart.
    assert!(!nfc_oracle::is_nfc(&[0x65, 0x301]));
    assert!(nfc_oracle::is_nfc(&[0xE9]));
}

#[test]
fn the_oracle_reaches_the_generated_admission_package_nf_07() {
    // A run through the admission package: the verdicts differ on a path only the
    // tables can tell apart.
    assert_eq!(
        nfc_oracle::admit(&[b"src"], "src/e\u{301}.lex.tex".as_bytes()),
        vec![1, 1]
    );
    assert_eq!(
        nfc_oracle::admit(&[b"src"], "src/\u{e9}.lex.tex".as_bytes()),
        vec![0]
    );
}
