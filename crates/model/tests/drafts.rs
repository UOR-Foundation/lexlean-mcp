//! DR-01 to DR-11: the draft tools of F3 (issue #9).
//!
//! The behavior is tested where the core and the host run: `gateway/tool-tests` calls
//! `lexlean_draft_write`, `lexlean_draft_read` and an Engine tool on a draft through the
//! production core with the real host, and `gateway/bindings/gateway-binding` tests the
//! materialization. These tests read the sources as text and hold them against each other: the
//! core's modules against the model's (`src/Identity.lex.tex`, `src/Admission.lex.tex`), the
//! tests against the rows they answer, the tool layer against the closed effect set, and the
//! planted defects against `DRAFTS.md`. Nothing here runs LexLean.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn bytes(relative: &str) -> Vec<u8> {
    std::fs::read(root().join(relative)).unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn tight(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn walk(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| {
                ["target", ".prism", ".lexlean", "registry"]
                    .iter()
                    .any(|skipped| name == *skipped)
            }) {
                continue;
            }
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// The names of the `#[test]` functions of the Rust files beneath `relative`.
fn test_names(relative: &str) -> BTreeSet<String> {
    let mut files = Vec::new();
    walk(&root().join(relative), &mut files);
    let mut names = BTreeSet::new();
    for file in files
        .iter()
        .filter(|file| file.extension().is_some_and(|e| e == "rs"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        let mut in_test = false;
        for line in text.lines() {
            let line = line.trim();
            if line == "#[test]" {
                in_test = true;
            } else if in_test && line.starts_with("fn ") {
                names.insert(line[3..].split('(').next().expect("a name").to_string());
                in_test = false;
            } else if in_test && !line.starts_with("#[") && !line.starts_with("///") {
                in_test = false;
            }
        }
    }
    names
}

/// The tests of the tool tests and of the binding that answer the row `id`.
fn answering(id: &str) -> BTreeSet<String> {
    let suffix = id.to_lowercase().replace('-', "_");
    let mut names = test_names("gateway/tool-tests");
    names.extend(test_names("gateway/bindings/gateway-binding"));
    names
        .into_iter()
        .filter(|name| name.ends_with(&suffix))
        .collect()
}

fn needs(id: &str, tests: &[&str]) {
    let names = answering(id);
    for test in tests {
        assert!(
            names.contains(*test),
            "a test of gateway/ has the name {test} for {id}: {names:?}"
        );
    }
}

fn drafts() -> String {
    tight(&read("gateway/src/Drafts.lex.tex"))
}

fn tools() -> String {
    tight(&read("gateway/src/GatewayTools.lex.tex"))
}

/// DR-01: the identity of the core is the model's.
#[test]
fn the_core_encodes_the_preimage_of_the_model_dr_01() {
    let model = tight(&read("src/Identity.lex.tex"));
    for name in ["draftPreimageVector", "draftIdVector", "encDraft"] {
        assert!(model.contains(name), "the model has {name}");
    }
    let octets: Vec<String> = "lexlean-mcp-draft-v1"
        .bytes()
        .map(|octet| octet.to_string())
        .collect();
    let spelled = format!(
        "{}{}",
        octets.join(" (cons "),
        " (nil Nat)".to_owned() + &")".repeat(octets.len() - 1)
    );
    assert!(
        model.contains(&format!("body: (cons {spelled})")),
        "the model spells the domain lexlean-mcp-draft-v1 in octets"
    );
    let core = drafts();
    for needle in [
        "body: 0x6c65786c65616e2d6d63702d64726166742d7631",
        "frameOf \"domain\"",
        "frameOf \"profile\"",
        "headFor \"sources\"",
        "frameOf \"source\"",
        "sortRows",
        "Sha256::",
    ] {
        assert!(core.contains(needle), "the core encodes with {needle}");
    }
    needs(
        "DR-01",
        &[
            "the_core_reproduces_the_models_draft_identity_dr_01",
            "the_rows_of_a_draft_in_any_order_have_one_identity_dr_01",
        ],
    );
}

/// DR-02: the identity covers the profile and every path and byte.
#[test]
fn the_identity_depends_on_the_profile_and_on_every_row_dr_02() {
    let core = drafts();
    assert!(
        core.contains("frameOf \"profile\" profile"),
        "the preimage holds the profile ID: a draft identity that ignores the profile fails here"
    );
    needs(
        "DR-02",
        &[
            "the_same_files_under_two_profiles_have_two_identities_dr_02",
            "a_byte_and_a_path_each_change_the_identity_dr_02",
            "a_draft_written_twice_is_one_key_of_the_drafts_area_dr_02",
        ],
    );
}

/// DR-03: the admission of the core is the exported admission, and it runs before any effect
/// beyond the profile record.
#[test]
fn the_admission_of_the_core_is_the_exported_one_dr_03() {
    let model = read("src/Admission.lex.tex");
    let copy = read("gateway/src/DraftAdmission.lex.tex");
    let model_lines: Vec<&str> = model.lines().collect();
    let copy_lines: Vec<&str> = copy.lines().collect();
    assert_eq!(model_lines.len(), copy_lines.len(), "one line for one line");
    let differing: Vec<usize> = (0..model_lines.len())
        .filter(|&at| model_lines[at] != copy_lines[at])
        .collect();
    assert_eq!(
        differing,
        [0, 6],
        "the copy differs from src/Admission.lex.tex in its module name and its title only"
    );
    assert_eq!(model_lines[0], "\\begin{lexlean}{Admission}");
    assert_eq!(copy_lines[0], "\\begin{lexlean}{DraftAdmission}");
    assert_eq!(model_lines[6], "\\title{LexLean MCP path admission}");
    assert_eq!(copy_lines[6], "\\title{LexLean MCP draft path admission}");
    for module in ["Nfc", "NfcTables"] {
        assert_eq!(
            bytes(&format!("src/{module}.lex.tex")),
            bytes(&format!("gateway/src/{module}.lex.tex")),
            "gateway/src/{module}.lex.tex is the model's"
        );
    }
    for table in [
        "nfc-sample.bin",
        "nfc-tables-high.bin",
        "nfc-tables-low.bin",
    ] {
        assert_eq!(
            bytes(&format!("artifacts/{table}")),
            bytes(&format!("gateway/artifacts/{table}")),
            "gateway/artifacts/{table} is the model's"
        );
    }
    let core = drafts();
    assert!(
        core.contains("DraftAdmission::admitDraft"),
        "the core admits with the exported admission and not with a check of its own"
    );
    needs(
        "DR-03",
        &[
            "a_path_with_a_dot_dot_component_is_refused_before_the_store_and_the_engine_dr_03",
            "a_path_that_is_not_lex_tex_is_refused_before_the_store_and_the_engine_dr_03",
            "a_path_that_is_not_nfc_is_refused_before_the_store_and_the_engine_dr_03",
            "a_path_the_form_of_which_admission_refuses_is_refused_before_the_store_dr_03",
            "a_path_that_is_admitted_is_stored_with_the_lexicon_read_and_the_put_dr_03",
        ],
    );
}

/// DR-04: the limits.
#[test]
fn the_limits_of_a_draft_are_the_stated_ones_dr_04() {
    let core = drafts();
    let owner = read("gateway/vectors/generate.py");
    for (name, value) in [("maxFiles", "256"), ("maxBytes", "8388608")] {
        assert!(
            core.contains(&format!(
                "name: {name}, parameters: [], result: Nat, body: {value}"
            )),
            "the core states {name} as {value}"
        );
    }
    assert!(
        owner.contains("8388608") || read("model/ids.toml").contains("draft.max_bytes"),
        "the limit is an owner value"
    );
    needs(
        "DR-04",
        &[
            "a_draft_of_256_files_is_written_and_a_257th_is_refused_dr_04",
            "a_draft_of_8_mib_is_written_and_one_byte_more_is_refused_dr_04",
            "the_bytes_of_a_draft_are_counted_over_all_its_files_dr_04",
            "a_repeated_path_is_refused_dr_04",
        ],
    );
}

/// DR-05: the drafts area holds the preimage, and a read checks it.
#[test]
fn a_read_checks_the_bytes_against_the_identity_dr_05() {
    let core = drafts();
    for needle in [
        "Sha256::",
        "DraftCorrupt",
        "DraftUnknown",
        "PathUnknown",
        "storeGetTag",
    ] {
        assert!(core.contains(needle), "the reader uses {needle}");
    }
    needs(
        "DR-05",
        &[
            "a_draft_is_the_one_key_of_its_profile_and_its_content_is_the_preimage_dr_05",
            "the_text_and_the_listing_of_a_draft_are_read_back_dr_05",
            "an_empty_file_and_a_large_file_are_read_back_whole_dr_05",
        ],
    );
}

/// DR-06: the profile record.
#[test]
fn the_profile_record_is_the_first_effect_of_a_write_dr_06() {
    let core = drafts();
    for needle in ["ProfileRecordMalformed", "lexiconTag", "recordOf", "rootOk"] {
        assert!(core.contains(needle), "the core uses {needle}");
    }
    needs(
        "DR-06",
        &[
            "the_profile_record_is_read_by_the_first_effect_of_a_write_dr_06",
            "a_second_root_admits_its_files_dr_06",
            "an_unknown_profile_is_the_host_failure_profile_unknown_dr_06",
            "a_record_that_is_not_a_profile_record_is_refused_without_a_put_dr_06",
            "a_profile_without_a_record_is_refused_without_a_put_dr_06",
        ],
    );
}

/// DR-07: materialization copies.
#[test]
fn a_draft_is_materialized_by_copy_dr_07() {
    let host = read("gateway/bindings/gateway-binding/src/materialize.rs");
    assert!(
        !host.contains("os::unix::fs::symlink") && !host.contains("hard_link"),
        "the materialization creates no link"
    );
    assert!(
        host.contains("create_new(true)"),
        "every file of a project is created, not overwritten"
    );
    needs(
        "DR-07",
        &[
            "an_engine_tool_on_the_handle_of_a_draft_agrees_with_the_directory_dr_07",
            "the_directory_of_an_engine_call_is_gone_after_it_dr_07",
            "a_draft_the_engine_refuses_is_the_refusal_of_the_engine_dr_07",
            "a_handle_whose_draft_or_profile_is_absent_is_project_refused_dr_07",
            "a_draft_of_another_profile_than_its_record_is_project_refused_dr_07",
            "a_changed_draft_is_project_refused_dr_07",
            "a_verification_of_a_draft_starts_on_a_copy_dr_07",
            "the_bundle_and_the_draft_are_copied_into_a_directory_of_their_own_dr_07",
            "two_materializations_of_one_draft_are_two_directories_dr_07",
            "an_absent_draft_and_a_changed_draft_are_refused_dr_07",
            "a_draft_with_a_path_that_leaves_the_project_is_refused_dr_07",
        ],
    );
}

/// DR-08: a symbolic link in a bundle is refused, never followed.
#[test]
fn a_symbolic_link_in_a_bundle_is_refused_dr_08() {
    let host = read("gateway/bindings/gateway-binding/src/materialize.rs");
    assert!(
        host.contains("symlink_metadata") && host.contains("is_symlink()"),
        "the host inspects an entry without following it and refuses a link"
    );
    assert!(
        !host.contains("fs::metadata(") && !host.contains(".canonicalize("),
        "nothing in the materialization follows a link to inspect it"
    );
    assert!(
        host.contains("open_for_reading"),
        "a file is opened without following a link that appeared since"
    );
    needs(
        "DR-08",
        &[
            "a_profile_bundle_with_a_link_to_a_file_is_project_refused_dr_08",
            "a_profile_bundle_with_a_link_to_a_directory_is_project_refused_dr_08",
            "a_profile_bundle_with_a_dangling_link_is_project_refused_dr_08",
            "a_profile_bundle_with_a_link_below_a_subdirectory_is_project_refused_dr_08",
            "no_byte_is_read_through_a_link_of_a_bundle_dr_08",
            "a_profile_bundle_without_a_link_is_materialized_dr_08",
            "a_link_to_a_file_is_refused_dr_08",
            "a_link_to_a_directory_is_refused_dr_08",
            "a_dangling_link_is_refused_dr_08",
            "a_link_below_a_subdirectory_is_refused_dr_08",
            "a_link_to_the_record_is_refused_dr_08",
            "a_link_to_a_pipe_does_not_block_and_is_refused_dr_08",
        ],
    );
}

/// DR-09: the draft tools are the two that E1 to E10 reach.
#[test]
fn the_draft_tools_are_the_two_the_ten_effects_reach_dr_09() {
    let source = tools();
    assert!(source.contains("\"lexlean_draft_write\""));
    assert!(source.contains("\"lexlean_draft_read\""));
    assert!(
        !source.contains("lexlean_draft_assemble"),
        "no tool is declared for the functions of lexlean::sdk that no effect reaches"
    );
    let effects = tight(&read("gateway/src/Effects.lex.tex"));
    assert!(
        !effects.contains("draft"),
        "the set of effects is the model's ten: no effect is added for drafts"
    );
    let gateway = read("GATEWAY.md");
    assert!(
        gateway.contains("lexlean_draft_assemble"),
        "GATEWAY.md records why the assemble tool is not declared"
    );
    needs(
        "DR-09",
        &[
            "the_two_draft_tools_are_listed_with_closed_schemas_and_no_assemble_dr_09",
            "the_effects_of_the_draft_tools_are_the_ones_stated_dr_09",
            "a_call_that_breaks_a_schema_of_the_draft_tools_asks_for_no_effect_dr_09",
        ],
    );
}

/// DR-10: every refusal is typed and has a negative test.
#[test]
fn every_refusal_of_the_draft_tools_has_a_negative_test_dr_10() {
    let model = tight(&read("src/Admission.lex.tex"));
    let constructors = [
        ("DotDotComponent", "dot_dot_component"),
        ("NotLexTex", "not_lex_tex"),
        ("NotNfc", "not_nfc"),
        ("AbsolutePath", "absolute_path"),
        ("EmptyComponent", "empty_component"),
        ("DotComponent", "dot_component"),
        ("OutsideRoots", "outside_roots"),
        ("TooManyFiles", "too_many_files"),
        ("TooManyBytes", "too_many_bytes"),
        ("DuplicatePath", "duplicate_path"),
        ("NotUtf8", "not_utf8"),
    ];
    assert_eq!(constructors.len(), 11, "the eleven constructors of Refusal");
    for (constructor, _) in constructors {
        assert!(model.contains(constructor), "the model has {constructor}");
    }
    let core = drafts();
    let mut tests = Vec::new();
    for (constructor, snake) in constructors {
        if constructor == "NotUtf8" {
            continue;
        }
        assert!(
            core.contains(constructor),
            "the core names the refusal {constructor}"
        );
        tests.push(format!("{snake}_is_a_typed_refusal_dr_10"));
    }
    let names = answering("DR-10");
    for test in &tests {
        assert!(names.contains(test), "a negative test {test}: {names:?}");
    }
    needs(
        "DR-10",
        &[
            "not_utf8_is_refused_by_the_json_reader_before_the_tool_is_reached_dr_10",
            "profile_record_malformed_is_a_typed_refusal_dr_10",
            "draft_unknown_is_a_typed_refusal_dr_10",
            "path_unknown_is_a_typed_refusal_dr_10",
            "draft_corrupt_is_a_typed_refusal_dr_10",
            "bytes_that_are_not_a_preimage_are_a_corrupt_draft_dr_10",
            "a_preimage_with_trailing_bytes_is_a_corrupt_draft_dr_10",
            "profile_unknown_is_a_typed_host_failure_dr_10",
            "project_refused_is_a_typed_host_failure_dr_10",
            "a_key_the_store_refuses_is_a_typed_host_failure_dr_10",
            "a_bundle_file_is_never_a_file_of_a_draft_dr_10",
        ],
    );
}

/// DR-11: each gate rejects its planted defect, and DRAFTS.md records the command, the
/// diagnostic and the restore.
#[test]
fn every_gate_has_a_recorded_planted_defect_dr_11() {
    let document = read("DRAFTS.md");
    let rows: Vec<Vec<String>> = document
        .lines()
        .filter(|line| line.starts_with("| D") && !line.starts_with("| Defect"))
        .map(|line| {
            line.trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect()
        })
        .collect();
    assert!(
        rows.len() >= 3,
        "at least the three planted defects are recorded"
    );
    for row in &rows {
        assert_eq!(row.len(), 5, "a row has five cells: {row:?}");
        for cell in row {
            assert!(!cell.is_empty(), "an empty cell in {row:?}");
        }
        assert!(
            row[2].contains("cargo test") || row[2].contains("lexlean"),
            "the command of {} is one that runs here: {row:?}",
            row[0]
        );
    }
    let defects: String = rows
        .iter()
        .map(|row| row[1].to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    for needle in ["`..`", "symbolic link", "ignores the profile"] {
        assert!(
            defects.contains(needle),
            "a planted defect is named {needle}: {defects}"
        );
    }
}
