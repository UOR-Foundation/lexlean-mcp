//! NF-01: the pinned files are the Unicode 17.0.0 files the authorities record,
//! and 17.0.0 is the Unicode version of the crate LexLean pins.

mod common;

use common::{files, root, sha256_of, ucd_dir, PINS, VERSION};
use nfc_tables::ucd::{Data, Files, FILES};
use repo_model::Model;

#[test]
fn every_pinned_file_has_the_recorded_digest_nf_01() {
    let model = Model::load(&root().join("model")).expect("the model loads");
    for (name, pin) in PINS {
        assert_eq!(
            sha256_of(&ucd_dir().join(name)),
            pin,
            "{name} is not the pinned file"
        );
        let id = format!(
            "A-UCD-{}",
            name.trim_end_matches(".txt").to_ascii_uppercase()
        );
        let authority = model
            .authorities
            .authority
            .iter()
            .find(|a| a.id == id)
            .unwrap_or_else(|| panic!("{id} is a cited authority"));
        assert_eq!(authority.checksum, pin, "{id} records a different digest");
        assert!(
            authority.citation.contains("17.0.0"),
            "{id} names the version"
        );
    }
    let listed: Vec<&str> = PINS.iter().map(|(name, _)| *name).collect();
    assert_eq!(listed, FILES, "the pins cover exactly the files read");
}

#[test]
fn every_pinned_file_names_unicode_17_0_0_nf_01() {
    // Parsing refuses a file whose first line names another version; the
    // unversioned UnicodeData.txt is held by its digest alone.
    Data::parse(&files()).expect("the pinned files name 17.0.0");
    for (name, _) in &PINS[1..] {
        let text = std::fs::read_to_string(ucd_dir().join(name)).expect("the file reads");
        let expected = format!("# {}-17.0.0.txt", name.trim_end_matches(".txt"));
        assert_eq!(text.lines().next(), Some(expected.as_str()), "{name}");
    }
}

#[test]
fn the_unicode_version_is_that_of_the_pinned_crate_nf_01() {
    assert_eq!(
        unicode_normalization::UNICODE_VERSION,
        VERSION,
        "unicode-normalization 0.1.25 implements another Unicode version"
    );
    let model = Model::load(&root().join("model")).expect("the model loads");
    let authority = model
        .authorities
        .authority
        .iter()
        .find(|a| a.id == "A-UNICODE-NORMALIZATION")
        .expect("the crate is a cited authority");
    assert!(authority.citation.contains("0.1.25") && authority.citation.contains("(17, 0, 0)"));
    // The version is the one LexLean's own lock names: the lock of the pinned
    // release is not in this repository, so the digest recorded here is the one
    // in LexLean's Cargo.lock for the 0.1.25 .crate file.
    assert_eq!(
        authority.checksum,
        "5fd4f6878c9cb28d874b009da9e8d183b5abc80117c40bbd187a1fde336be6e8"
    );
    let lock = std::fs::read_to_string(root().join("Cargo.lock")).expect("Cargo.lock reads");
    let section = lock
        .split("[[package]]")
        .find(|p| p.contains("name = \"unicode-normalization\""))
        .expect("the oracle depends on unicode-normalization");
    assert!(
        section.contains("version = \"0.1.25\"")
            && section.contains(&format!("checksum = \"{}\"", authority.checksum)),
        "the locked crate is not the recorded 0.1.25"
    );
}

#[test]
fn a_file_that_names_another_version_is_refused_nf_01() {
    for (index, label) in [
        (1, "CompositionExclusions"),
        (2, "DerivedNormalizationProps"),
        (3, "NormalizationTest"),
    ] {
        let mut other: Files = files();
        let slot = match index {
            1 => &mut other.composition_exclusions,
            2 => &mut other.derived_normalization_props,
            _ => &mut other.normalization_test,
        };
        *slot = slot.replacen("17.0.0", "16.0.0", 1);
        let error = Data::parse(&other).expect_err("another version is refused");
        assert!(
            error.0.contains(label) && error.0.contains("17.0.0"),
            "the refusal names the file and the version: {error}"
        );
    }
}
