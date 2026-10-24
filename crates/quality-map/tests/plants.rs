//! QM-08: each gate rejects its planted defect. `QUALITY.md` records the same
//! defects applied to the working tree, with the command and the diagnostic.

use quality_map::{
    check_explained, check_map, check_registry, check_returned, check_sources, check_surface,
    derive_surface, in_scope_of_model,
};

mod common;
use common::{
    files, map, none, quality, registry, reports, returned_codes, rev_commit, sources, text_of,
};

/// The controls: nothing planted, nothing reported.
#[test]
fn the_gates_report_nothing_on_the_repository_as_it_is_qm_08() {
    let (sources, files, map) = (sources(), files(), map());
    let found = derive_surface(&sources, &files).expect("derives");
    let in_scope = in_scope_of_model(&quality()).expect("in scope");
    none("sources", check_sources(&sources, &files, &rev_commit()));
    none("surface", check_surface(&map, &found));
    none("map", check_map(&map, &in_scope));
    let registry = registry(&files);
    none("registry", check_registry(&registry));
    none(
        "entries",
        check_explained(&registry, &text_of(&files, "ERRORS.md")),
    );
    none("returned", check_returned(&registry, &returned_codes()));
}

/// The gates of Q2 reject a function removed from the map, a tool misspelled,
/// a function with neither tool nor reason, a function with both, a function
/// added or re-signed upstream, a vendored byte altered and a pin moved.
#[test]
fn the_gates_of_q2_reject_their_planted_defects_qm_08() {
    let (sources, files, map) = (sources(), files(), map());
    let found = derive_surface(&sources, &files).expect("derives");
    let in_scope = in_scope_of_model(&quality()).expect("in scope");

    // A function removed from the map.
    let mut planted = map.clone();
    planted.function.retain(|f| f.path != "Engine::snapshot");
    let v = check_surface(&planted, &found);
    assert!(
        reports(
            &v,
            "Engine::snapshot is public in the pinned source and is not in map.toml"
        ),
        "{v:?}"
    );
    let v = check_map(&planted, &in_scope);
    assert!(
        reports(&v, "Engine::snapshot is in scope of Q2 and reaches no tool"),
        "{v:?}"
    );

    // A mapped tool name misspelled.
    let mut planted = map.clone();
    let f = planted
        .function
        .iter_mut()
        .find(|f| f.path == "Engine::check")
        .expect("mapped");
    f.tool = Some("lexlean_chek".to_owned());
    let v = check_map(&planted, &in_scope);
    assert!(
        reports(
            &v,
            "Engine::check is mapped to lexlean_chek, which no issue defines"
        ),
        "{v:?}"
    );

    // A function with neither a tool nor a reason.
    let mut planted = map.clone();
    let f = planted
        .function
        .iter_mut()
        .find(|f| f.path == "Engine::lock")
        .expect("mapped");
    f.reason = None;
    f.issue = None;
    let v = check_map(&planted, &in_scope);
    assert!(
        reports(&v, "Engine::lock is unmapped: no tool and no reason"),
        "{v:?}"
    );

    // A function that both reaches a tool and is not exposed.
    let mut planted = map.clone();
    let f = planted
        .function
        .iter_mut()
        .find(|f| f.path == "Engine::lock")
        .expect("mapped");
    f.tool = Some("lexlean_check".to_owned());
    let v = check_map(&planted, &in_scope);
    assert!(
        reports(
            &v,
            "Engine::lock is mapped to a tool and also states it is not exposed"
        ),
        "{v:?}"
    );

    // A function added to LexLean's surface.
    let mut changed = files.clone();
    let api = changed.get_mut("api.rs.txt").expect("vendored");
    let mut text = String::from_utf8(api.clone()).expect("UTF-8");
    text = text.replacen(
        "impl Engine {\n",
        "impl Engine {\n    pub fn export(&self) -> Result<(), LexLeanError> {\n        Ok(())\n    }\n\n",
        1,
    );
    *api = text.into_bytes();
    let v = check_surface(&map, &derive_surface(&sources, &changed).expect("derives"));
    assert!(
        reports(
            &v,
            "Engine::export is public in the pinned source and is not in map.toml"
        ),
        "{v:?}"
    );
    let v = check_sources(&sources, &changed, &rev_commit());
    assert!(reports(&v, "api.rs.txt: the git blob id is"), "{v:?}");

    // A function re-signed upstream.
    let mut changed = files.clone();
    let sdk = changed.get_mut("sdk.rs.txt").expect("vendored");
    let text = String::from_utf8(sdk.clone()).expect("UTF-8").replacen(
        "pub fn is_notation(text: &str)",
        "pub fn is_notation(text: &[u8])",
        1,
    );
    *sdk = text.into_bytes();
    let v = check_surface(&map, &derive_surface(&sources, &changed).expect("derives"));
    assert!(
        reports(&v, "sdk::is_notation: the pinned source declares"),
        "{v:?}"
    );

    // A vendored byte altered.
    let mut changed = files.clone();
    changed.get_mut("sdk.rs.txt").expect("vendored").push(b'\n');
    let v = check_sources(&sources, &changed, &rev_commit());
    assert!(reports(&v, "sdk.rs.txt: the git blob id is"), "{v:?}");

    // A vendored file removed.
    let mut changed = files.clone();
    changed.remove("errors.toml");
    let v = check_sources(&sources, &changed, &rev_commit());
    assert!(reports(&v, "errors.toml is not vendored"), "{v:?}");

    // The pin moved without a new import.
    let v = check_sources(&sources, &files, "0000000000000000000000000000000000000000");
    assert!(reports(&v, "import again"), "{v:?}");
}

/// The gates of Q8 reject an unexplained code in each place one can appear.
#[test]
fn the_gates_of_q8_reject_their_planted_defects_qm_08() {
    let files = files();
    let registry = registry(&files);
    let errors_md = text_of(&files, "ERRORS.md");

    // A code returned that the registry does not hold.
    let mut returned = returned_codes();
    returned.insert("LLX9999".to_owned());
    let v = check_returned(&registry, &returned);
    assert!(
        reports(
            &v,
            "LLX9999 is returned and the registry does not explain it"
        ),
        "{v:?}"
    );

    // A code added to the registry with no statement.
    let mut planted = registry.clone();
    let mut row = planted.error.last().expect("rows").clone();
    row.code = "LLV7999".to_owned();
    row.statement = String::new();
    planted.error.push(row);
    let v = check_registry(&planted);
    assert!(
        reports(&v, "LLV7999 has no statement: it is not explained"),
        "{v:?}"
    );
    let v = check_explained(&planted, &errors_md);
    assert!(
        reports(&v, "LLV7999 is registered and has no entry in ERRORS.md"),
        "{v:?}"
    );

    // An entry removed from ERRORS.md.
    let start = errors_md.find("## `LLB6002`").expect("entry");
    let end = errors_md.find("## `LLB6003`").expect("entry");
    let without = format!("{}{}", &errors_md[..start], &errors_md[end..]);
    let v = check_explained(&registry, &without);
    assert!(
        reports(&v, "LLB6002 is registered and has no entry in ERRORS.md"),
        "{v:?}"
    );

    // An entry whose text is not what explain prints.
    let altered = errors_md.replacen("Exit code: 3.", "Exit code: 4.", 1);
    let v = check_explained(&registry, &altered);
    assert!(
        reports(&v, "the entry of ERRORS.md is not what explain prints"),
        "{v:?}"
    );

    // A class with another class's exit status.
    let mut planted = registry.clone();
    planted.error[0].exit = 2;
    let v = check_registry(&planted);
    assert!(reports(&v, "class language exits 1, not 2"), "{v:?}");

    // Rows out of order.
    let mut planted = registry.clone();
    planted.error.swap(0, 1);
    let v = check_registry(&planted);
    assert!(reports(&v, "rows are sorted by code and unique"), "{v:?}");
}
