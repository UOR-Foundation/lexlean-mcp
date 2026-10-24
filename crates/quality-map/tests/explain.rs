//! QM-04 to QM-06: Q8, every diagnostic code explained.

use quality_map::{
    cardinality_of_model, check_explained, check_registry, check_returned, check_sources,
    entries_of, manifest_of, ENTRIES, REGISTRY,
};

mod common;
use common::{
    authority, files, none, quality, registry, returned_codes, rev_commit, sources, text_of,
};

/// The number of codes the pinned registry holds.
const CODES: usize = 86;

/// `QM-04`: the registry and its generated entries are the files of the pinned
/// commit, and the registry holds the counted codes.
#[test]
fn the_registry_and_its_entries_are_the_pinned_blobs_qm_04() {
    let (sources, files) = (sources(), files());
    none(
        "the vendored files",
        check_sources(&sources, &files, &rev_commit()),
    );
    let authority = authority("A-LEXLEAN-ERRORS");
    assert_eq!(
        authority.checksum,
        manifest_of(&sources, &files, &[REGISTRY, ENTRIES]),
        "the checksum of A-LEXLEAN-ERRORS is the manifest of the registry and its entries"
    );
    assert!(authority.citation.contains(&sources.commit));

    let registry = registry(&files);
    assert_eq!(registry.error.len(), CODES);
    let model = common::model();
    let row = model
        .ids
        .id
        .iter()
        .find(|r| r.id == "QM-04")
        .expect("QM-04 is registered");
    assert!(row.statement.contains(&format!("{CODES} diagnostic codes")));
    assert_eq!(
        cardinality_of_model(&quality(), 7),
        Some(registry.error.len() as u64),
        "the model counts the codes of the registry"
    );
}

/// `QM-05`: every registered code is explained: its row is complete and
/// `ERRORS.md`, which `lexlean explain` prints from, holds exactly its entry.
#[test]
fn every_registered_code_has_one_complete_explanation_qm_05() {
    let files = files();
    let registry = registry(&files);
    none("the registry", check_registry(&registry));
    let errors_md = text_of(&files, "ERRORS.md");
    none("ERRORS.md", check_explained(&registry, &errors_md));
    assert_eq!(entries_of(&errors_md).len(), CODES);
}

/// `QM-06`: every code the negative fixtures of AC-07 return resolves to an
/// entry of `ERRORS.md`.
#[test]
fn every_code_the_fixtures_return_resolves_to_an_entry_qm_06() {
    let files = files();
    let registry = registry(&files);
    let returned = returned_codes();
    assert!(!returned.is_empty(), "the fixtures return codes");
    none("the returned codes", check_returned(&registry, &returned));
    let entries: std::collections::BTreeSet<String> = entries_of(&text_of(&files, "ERRORS.md"))
        .into_iter()
        .map(|(code, _)| code)
        .collect();
    for code in &returned {
        assert!(entries.contains(code), "{code} has no entry in ERRORS.md");
    }
    eprintln!(
        "QM-06: {} of {} registered codes are returned by the fixtures, all explained",
        returned.len(),
        registry.error.len()
    );
}
