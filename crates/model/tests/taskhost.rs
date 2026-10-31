//! TH-01 to TH-06: the task host of the gateway (issue #11, slice 4a).
//!
//! The behavior is tested where the host runs: `gateway/bindings/gateway-binding/tests/
//! taskhost.rs` asks the host for the time, an identifier and the table, and
//! `gateway/tool-tests/tests/taskhost.rs` serves the generated adapter over a socket and
//! records what it asks of the host. These tests read the sources as text and hold them
//! against each other: the declaration against the order the frame of the core is read in, the
//! tag of the store against the provider information, the rows against the tests that answer
//! them, and the planted defects against `TASKHOST.md`. Nothing here runs the host.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

const GENERATE: &str = "gateway/vectors/generate.py";
const GATEWAY: &str = "gateway/src/Gateway.lex.tex";

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

/// The tests of the binding and of the tool tests that answer the row `id` are exactly
/// `expected`, and `TASKHOST.md` names the row.
fn answered_by(id: &str, expected: &[&str]) {
    let suffix = id.to_lowercase().replace('-', "_");
    let mut found: BTreeSet<String> = BTreeSet::new();
    for file in [
        "gateway/bindings/gateway-binding/tests/taskhost.rs",
        "gateway/tool-tests/tests/taskhost.rs",
    ] {
        found.extend(
            tests_of(file)
                .into_iter()
                .filter(|name| name.ends_with(&format!("_{suffix}"))),
        );
    }
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests that answer {id}");
    assert!(
        read("TASKHOST.md").contains(id),
        "TASKHOST.md does not name {id}"
    );
}

/// The names of the effects of the declaration, in the order generate.py lists them.
fn declared_effects() -> Vec<String> {
    let source = read(GENERATE);
    let start = source.find("EFFECTS = [").expect("the list of effects");
    let end = start + source[start..].find("\n]\n").expect("its end");
    source[start..end]
        .lines()
        .filter(|line| line.starts_with("    (\""))
        .map(|line| {
            line["    (\"".len()..]
                .split('"')
                .next()
                .expect("a name")
                .to_string()
        })
        .collect()
}

#[test]
fn the_declaration_lists_the_entries_the_core_needs_at_the_tags_th_01() {
    let effects = declared_effects();
    let model = read("src/Effects.lex.tex");
    assert!(model.contains("\"store.get\"") && model.contains("\"attestation.lookup\""));
    assert_eq!(
        effects.len(),
        15,
        "ten effects, rate.calls, and four more: {effects:?}"
    );
    assert_eq!(
        effects[10..],
        [
            "rate.calls",
            "clock.now",
            "task.id",
            "task.table",
            "task.store"
        ],
        "the tags 10 to 14"
    );
    // The provider information names the tag of the store.
    let gateway = read(GATEWAY);
    assert!(
        gateway.contains("taskStoreTag := 14"),
        "taskStoreTag is the tag of task.store"
    );
    // The sections of the frame, in the order the core reads them (LIFECYCLE.md in MCP).
    let source = read(GENERATE);
    let sections = source
        .find("FRAME_SECTIONS = [")
        .expect("the frame sections");
    let end = sections + source[sections..].find("\n]\n").expect("their end");
    let names: Vec<&str> = source[sections..end]
        .lines()
        .filter(|line| line.starts_with("    (\""))
        .map(|line| line["    (\"".len()..].split('"').next().expect("a name"))
        .collect();
    assert_eq!(
        names,
        ["tool_calls", "now_ms", "now", "fresh_task_id", "task_table"]
    );
    // The maximums are those of MCP's own conformance declaration.
    for (effect, maximum) in [
        ("clock.now", "64"),
        ("task.id", "128"),
        ("task.table", "524_296"),
        ("task.store", "524_296"),
    ] {
        let at = source.find(&format!("(\"{effect}\"")).expect("the effect");
        let row = &source[at..at + source[at..].find("\n    (").unwrap_or(source.len() - at)];
        assert!(
            row.contains(maximum),
            "{effect} states the maximum {maximum}: {row}"
        );
    }
}

#[test]
fn the_time_is_answered_by_its_tests_th_02() {
    answered_by(
        "TH-02",
        &[
            "the_text_of_an_instant_is_iso_8601_in_utc_with_milliseconds_th_02",
            "the_counter_never_goes_back_and_the_text_agrees_with_it_th_02",
            "the_effect_answers_the_time_th_02",
        ],
    );
}

#[test]
fn the_identifiers_are_answered_by_their_tests_th_03() {
    answered_by(
        "TH-03",
        &[
            "an_identifier_is_32_lowercase_hexadecimal_digits_and_never_repeats_th_03",
            "hosts_do_not_share_identifiers_and_none_is_made_from_the_clock_th_03",
        ],
    );
    let host = read("gateway/bindings/gateway-binding/src/taskhost.rs");
    assert!(
        host.contains("getrandom"),
        "the identifier comes from the operating system's source"
    );
}

#[test]
fn the_table_is_answered_by_its_tests_th_04() {
    answered_by(
        "TH-04",
        &[
            "the_table_is_empty_until_one_is_stored_and_then_it_is_what_was_stored_th_04",
            "a_store_replaces_the_whole_table_and_an_empty_store_empties_it_th_04",
            "bytes_of_every_value_and_the_largest_table_come_back_unchanged_th_04",
            "two_hosts_do_not_share_a_table_th_04",
        ],
    );
}

#[test]
fn the_core_serves_the_vectors_with_the_new_frame_th_05() {
    answered_by(
        "TH-05",
        &[
            "a_listing_is_given_the_time_an_identifier_and_the_table_and_stores_nothing_th_05",
            "a_tool_call_that_creates_no_task_asks_the_core_for_no_store_th_05",
        ],
    );
    // The pin is the one MCP's tasks are in.
    let pin = read(".github/mcp.rev");
    assert!(
        pin.contains("rev = c53816e"),
        "the pin of MCP is the commit that serves tasks"
    );
}

#[test]
fn a_verification_is_a_task_for_a_client_that_declares_them_th_07() {
    answered_by(
        "TH-07",
        &[
            "a_client_that_declares_tasks_is_given_a_task_for_a_verification_th_07",
            "a_client_that_does_not_declare_tasks_is_answered_as_before_th_07",
            "a_verification_that_is_refused_creates_no_task_th_07",
        ],
    );
    // The terms of the handle are the ones TASKHOST.md and the row state.
    let tools = read("gateway/src/TaskTools.lex.tex");
    assert!(
        tools.contains("body: (86400000 : Int)"),
        "a time to live of 24 hours"
    );
    assert!(
        tools.contains("body: (5000 : Int)"),
        "a poll interval of 5000 ms"
    );
}

#[test]
fn a_task_is_polled_with_tasks_get_th_08() {
    answered_by(
        "TH-08",
        &[
            "a_working_task_stays_working_while_the_verifier_runs_th_08",
            "a_verified_task_is_completed_with_the_result_of_the_tool_th_08",
            "a_failed_verification_completes_the_task_with_an_error_result_th_08",
            "a_task_that_the_verifier_lost_is_failed_th_08",
            "a_terminal_task_is_answered_from_the_table_th_08",
            "a_task_that_the_table_does_not_hold_is_unknown_th_08",
        ],
    );
}

#[test]
fn a_task_is_cancelled_with_tasks_cancel_th_09() {
    answered_by(
        "TH-09",
        &[
            "a_cancelled_task_is_stored_as_cancelled_th_09",
            "a_second_cancel_is_answered_from_the_table_th_09",
            "cancelling_a_task_that_the_table_does_not_hold_is_refused_th_09",
        ],
    );
}

#[test]
fn a_cancel_says_what_became_of_the_child_groups_th_10() {
    answered_by(
        "TH-10",
        &[
            "a_cancel_reports_the_child_groups_that_ended_th_10",
            "a_cancel_reports_a_child_group_that_failed_to_end_th_10",
        ],
    );
}

#[test]
fn the_task_path_runs_with_the_real_verifier_th_11() {
    answered_by(
        "TH-11",
        &[
            "a_real_verification_runs_as_a_task_to_completed_th_11",
            "a_real_verification_is_cancelled_through_the_task_th_11",
        ],
    );
    // The gateway job requires the count of the ignored tests that ran, which includes these two
    // and the one of TH-13.
    let workflow = read(".github/workflows/lexlean.yml");
    assert!(
        workflow.contains("gateway_ignored_tests gateway-tool-tests 11"),
        "the job requires the eleven ignored tests of the tool tests"
    );
}

#[test]
fn calls_over_the_rate_are_refused_th_12() {
    answered_by(
        "TH-12",
        &["calls_over_the_rate_are_refused_before_any_tool_runs_th_12"],
    );
    let test = read("gateway/tool-tests/tests/taskhost.rs");
    assert!(
        test.contains("const RATE_TOOL_CALLS: u64 = 100;"),
        "the test counts the calls the owner value admits"
    );
    assert!(
        read("OWNER-VALUES.md").contains("| `rate.tool_calls` | `100` |"),
        "the count is the owner value rate.tool_calls"
    );
}

#[test]
fn a_draft_over_the_limit_fails_alone_th_13() {
    answered_by(
        "TH-13",
        &["a_draft_over_the_limit_fails_with_lls8002_and_no_other_task_is_affected_th_13"],
    );
}

#[test]
fn each_gate_of_the_task_path_rejects_its_planted_defect_th_11() {
    let document = read("TASKHOST.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("TASKHOST.md has a section Planted defects");
    for defect in [
        "the counts dropped",
        "an ended group counted as failed",
        "an ended group not counted",
        "a cancel that ends no group",
    ] {
        assert!(
            section
                .lines()
                .any(|line| line.starts_with('|') && line.contains(defect)),
            "TASKHOST.md records no planted defect: {defect}"
        );
    }
}

#[test]
fn each_gate_of_the_task_host_rejects_its_planted_defect_th_06() {
    let document = read("TASKHOST.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("TASKHOST.md has a section Planted defects");
    for defect in [
        "the text of the time disagreeing with its counter",
        "an identifier made from the counter",
        "a store that appends to the table instead of replacing it",
        "a provider that names the wrong tag for the store",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("TASKHOST.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("th_0") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}
