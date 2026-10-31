//! AT-01 to AT-05: the attestation tool of the gateway (issue #12, F10).
//!
//! The behavior is tested where the tool runs: `gateway/tool-tests/tests/attestation.rs` calls
//! `lexlean_attestation` through the production core with the real host on a store that holds a
//! signed record. These tests read the sources as text and hold the rows against the tests that
//! answer them and the planted defects against `ATTESTATION.md`. Nothing here runs the tool.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The names of the `#[test]` functions of the file.
fn tests_of(relative: &str) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let mut in_test = false;
    for line in read(relative).lines() {
        let line = line.trim();
        if line == "#[test]" {
            in_test = true;
        } else if in_test && line.starts_with("fn ") {
            names.insert(line[3..].split('(').next().expect("a name").to_string());
            in_test = false;
        }
    }
    names
}

/// The tests of the tool tests that answer the row `id` are exactly `expected`, and
/// `ATTESTATION.md` names the row.
fn answered_by(id: &str, expected: &[&str]) {
    let suffix = id.to_lowercase().replace('-', "_");
    let found: BTreeSet<String> = tests_of("gateway/tool-tests/tests/attestation.rs")
        .into_iter()
        .filter(|name| name.ends_with(&format!("_{suffix}")))
        .collect();
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests that answer {id}");
    assert!(
        read("ATTESTATION.md").contains(id),
        "ATTESTATION.md does not name {id}"
    );
}

#[test]
fn the_envelope_is_returned_by_id_at_01() {
    answered_by(
        "AT-01",
        &["the_tool_returns_the_envelope_by_attestation_id_at_01"],
    );
}

#[test]
fn the_envelope_is_returned_by_digest_at_02() {
    answered_by(
        "AT-02",
        &["the_tool_returns_the_same_envelope_by_the_digest_of_a_source_file_at_02"],
    );
}

#[test]
fn a_miss_is_not_an_error_and_a_bad_value_is_refused_at_03() {
    answered_by(
        "AT-03",
        &[
            "an_absent_record_is_not_found_and_a_malformed_value_is_refused_at_03",
            "the_tool_on_an_empty_store_finds_nothing_at_03",
        ],
    );
}

#[test]
fn a_lookup_writes_nothing_at_04() {
    answered_by("AT-04", &["a_lookup_writes_nothing_to_the_store_at_04"]);
}

#[test]
fn each_gate_of_the_tool_rejects_its_planted_defect_at_05() {
    let document = read("ATTESTATION.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("ATTESTATION.md has a section Planted defects");
    for defect in [
        "the lookup reading the file that the envelope signs",
        "the digest read as an attestation ID",
        "the check of the value dropped",
        "a lookup that writes",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("ATTESTATION.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("_at_0") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}
