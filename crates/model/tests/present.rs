//! PT-01 to PT-06 and EX-01 to EX-06: the present and export tools of the gateway (issue #12, F9 and F12).
//!
//! The behavior is tested where the tool runs: `gateway/tool-tests/tests/present.rs` calls
//! `lexlean_present` and `lexlean_export` through the production core with the real host on a
//! store that holds a signed record. These tests read the sources as text and hold the rows against the tests that
//! answer them and the planted defects against `PRESENT.md`. Nothing here runs the tool.

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
/// `PRESENT.md` names the row.
fn answered_by(id: &str, expected: &[&str]) {
    let suffix = id.to_lowercase().replace('-', "_");
    let found: BTreeSet<String> = tests_of("gateway/tool-tests/tests/present.rs")
        .into_iter()
        .filter(|name| name.ends_with(&format!("_{suffix}")))
        .collect();
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests that answer {id}");
    assert!(
        read("PRESENT.md").contains(id),
        "PRESENT.md does not name {id}"
    );
}

#[test]
fn the_presented_resource_is_the_rendering_of_the_record_pt_01() {
    answered_by(
        "PT-01",
        &["the_tool_presents_the_attested_rendering_as_an_embedded_resource_pt_01"],
    );
}

#[test]
fn j1_for_nat_add_zero_ends_in_the_expected_latex_pt_02() {
    answered_by(
        "PT-02",
        &["j1_for_nat_add_zero_ends_in_the_expected_canonical_latex_pt_02"],
    );
}

#[test]
fn the_module_selects_the_rendering_pt_03() {
    answered_by(
        "PT-03",
        &["by_digest_and_by_module_the_tool_presents_the_chosen_rendering_pt_03"],
    );
}

#[test]
fn a_miss_and_a_refusal_write_nothing_pt_04() {
    answered_by(
        "PT-04",
        &["a_miss_is_not_found_a_bad_value_is_refused_and_nothing_is_written_pt_04"],
    );
}

#[test]
fn only_what_is_attested_is_presented_by_the_host_pt_05() {
    answered_by("PT-05", &["only_what_is_attested_is_presented_pt_05"]);
}

#[test]
fn the_bundle_holds_the_files_of_the_record_ex_01() {
    answered_by("EX-01", &["the_tool_writes_the_bundle_of_the_record_ex_01"]);
}

#[test]
fn a_reader_checks_the_bundle_offline_by_its_files_ex_02() {
    answered_by("EX-02", &["a_reader_checks_the_bundle_offline_ex_02"]);
}

#[test]
fn the_renderings_digest_matches_the_manifest_ex_03() {
    answered_by(
        "EX-03",
        &["the_renderings_digest_of_a_bundle_matches_the_attested_manifest_ex_03"],
    );
}

#[test]
fn a_record_that_is_not_whole_leaves_no_bundle_ex_04() {
    answered_by(
        "EX-04",
        &["a_miss_or_a_refusal_writes_nothing_and_a_record_that_is_not_whole_leaves_no_bundle_ex_04"],
    );
}

#[test]
fn an_export_is_repeatable_ex_05() {
    answered_by(
        "EX-05",
        &["an_export_is_repeatable_and_a_bundle_is_apart_from_another_ex_05"],
    );
}

#[test]
fn each_gate_of_present_rejects_its_planted_defect_pt_06() {
    let document = read("PRESENT.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("PRESENT.md has a section Planted defects");
    for defect in [
        "the rendering read from the wrong file",
        "the digest of the rendering not checked",
        "the module not selected",
        "a present that writes",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("PRESENT.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("_pt_0") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}

#[test]
fn each_gate_of_export_rejects_its_planted_defect_ex_06() {
    let document = read("PRESENT.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("PRESENT.md has a section Planted defects");
    for defect in [
        "a rendering left out of the bundle",
        "the key history left out",
        "the envelope replaced by attestation.json",
        "the evidence not checked before the first write",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("PRESENT.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("_ex_0") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}
