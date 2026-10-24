//! QM-01 to QM-03: Q2, the API-to-tool map.

use std::collections::BTreeSet;

use quality_map::{
    cardinality_of_model, check_map, check_sources, check_surface, derive_surface,
    in_scope_of_model, manifest_of, SURFACE,
};

mod common;
use common::{authority, files, map, none, quality, rev_commit, root, sources};

/// The functions SPEC 24.1 declares for `Engine`.
const SPEC_ENGINE: [&str; 7] = [
    "load", "lock", "check", "snapshot", "build", "verify", "format",
];

/// The functions SPEC 24.8 declares for `lexlean::sdk`.
const SPEC_SDK: [&str; 8] = [
    "module_kind",
    "is_notation",
    "read_module",
    "write_module",
    "to_data_form",
    "to_notation",
    "declaration_spans",
    "assemble",
];

/// The functions SPEC 24.7 declares for `lexlean::api::production`.
const SPEC_PRODUCTION: [&str; 5] = [
    "api::production::targets",
    "api::production::targets_document",
    "ProductionRequest::from_json",
    "api::production::build",
    "api::production::verify_artifact",
];

/// The functions SPEC 24.6 declares for the embedding host.
const SPEC_LIFECYCLE: [&str; 2] = ["verify::end_live_children", "verify::allow_children"];

/// The tools issues 8 to 12 define, with the issue that defines each.
const ISSUE_TOOLS: [(&str, u32); 14] = [
    ("lexlean_profiles", 8),
    ("lexlean_lexicon_search", 8),
    ("lexlean_draft_write", 9),
    ("lexlean_draft_assemble", 9),
    ("lexlean_draft_read", 9),
    ("lexlean_check", 10),
    ("lexlean_snapshot", 10),
    ("lexlean_format", 10),
    ("lexlean_build", 10),
    ("lexlean_explain", 10),
    ("lexlean_verify", 11),
    ("lexlean_present", 12),
    ("lexlean_attestation", 12),
    ("lexlean_export", 12),
];

/// The number of public functions the pinned surface files declare.
const PUBLIC_FUNCTIONS: usize = 33;

/// `QM-01`: the vendored files are the files of the pinned commit, and they
/// declare the public functions the statement counts.
#[test]
fn the_surface_files_are_the_pinned_blobs_and_declare_the_counted_functions_qm_01() {
    let (sources, files) = (sources(), files());
    none(
        "the vendored files",
        check_sources(&sources, &files, &rev_commit()),
    );

    let authority = authority("A-LEXLEAN-API");
    assert_eq!(
        authority.checksum,
        manifest_of(&sources, &files, &[SURFACE]),
        "the checksum of A-LEXLEAN-API is the manifest of the surface files"
    );
    assert!(
        authority.citation.contains(&sources.commit),
        "the authority names the pinned commit"
    );

    let found = derive_surface(&sources, &files).expect("the surface derives");
    assert_eq!(found.len(), PUBLIC_FUNCTIONS, "{found:#?}");
    let model = common::model();
    let row = model
        .ids
        .id
        .iter()
        .find(|r| r.id == "QM-01")
        .expect("QM-01 is registered");
    assert!(
        row.statement
            .contains(&format!("{PUBLIC_FUNCTIONS} public functions")),
        "the statement of QM-01 counts {PUBLIC_FUNCTIONS} public functions"
    );
}

/// `QM-02`: the map has a row for exactly the functions the pinned source
/// declares, with their signatures, and the source declares every function
/// SPEC 24 promises.
#[test]
fn the_map_has_a_row_for_each_public_function_of_the_pinned_source_qm_02() {
    let (sources, files, map) = (sources(), files(), map());
    let found = derive_surface(&sources, &files).expect("the surface derives");
    none("the map against the source", check_surface(&map, &found));

    let derived: BTreeSet<String> = found.iter().map(|f| f.path.clone()).collect();
    let promised = SPEC_ENGINE
        .iter()
        .map(|n| format!("Engine::{n}"))
        .chain(SPEC_SDK.iter().map(|n| format!("sdk::{n}")))
        .chain(SPEC_PRODUCTION.iter().map(|n| (*n).to_owned()))
        .chain(SPEC_LIFECYCLE.iter().map(|n| (*n).to_owned()));
    for path in promised {
        assert!(
            derived.contains(&path),
            "SPEC 24 declares {path} and the pinned source does not"
        );
    }
    // The methods of Engine and the functions of sdk are exactly the ones SPEC
    // 24.1 and 24.8 list: neither grew nor shrank.
    let engine: BTreeSet<&str> = derived
        .iter()
        .filter_map(|p| p.strip_prefix("Engine::"))
        .collect();
    assert_eq!(engine, SPEC_ENGINE.into_iter().collect::<BTreeSet<_>>());
    let sdk: BTreeSet<&str> = derived
        .iter()
        .filter_map(|p| p.strip_prefix("sdk::"))
        .collect();
    assert_eq!(sdk, SPEC_SDK.into_iter().collect::<BTreeSet<_>>());
}

/// `QM-03`: the measure of Q2. No public function is unmapped, each reaches one
/// tool or states why none does, every tool is one the issues define, and the
/// functions that reach a tool are the in-scope ones of the model.
#[test]
fn every_public_function_reaches_one_defined_tool_or_states_why_not_qm_03() {
    let map = map();
    let quality = quality();
    let in_scope = in_scope_of_model(&quality).expect("the model states the in-scope functions");
    none("the map", check_map(&map, &in_scope));

    let defined: BTreeSet<(String, u32)> = ISSUE_TOOLS
        .iter()
        .map(|(n, i)| ((*n).to_owned(), *i))
        .collect();
    let mapped: BTreeSet<(String, u32)> =
        map.tool.iter().map(|t| (t.name.clone(), t.issue)).collect();
    assert_eq!(
        mapped, defined,
        "map.toml names the tools the issues define"
    );

    assert_eq!(
        cardinality_of_model(&quality, 1),
        Some(in_scope.len() as u64),
        "the model counts its in-scope functions"
    );
    assert_eq!(in_scope.len(), 9);

    let unreached: Vec<&str> = map
        .tool
        .iter()
        .map(|t| t.name.as_str())
        .filter(|n| !map.function.iter().any(|f| f.tool.as_deref() == Some(n)))
        .collect();
    eprintln!("QM-03: tools no API function backs: {unreached:?}");
    assert!(root().join("QUALITY.md").exists());
}
