//! VS-01 to VS-05: the verifier host, the third service of issue #17.
//!
//! The behavior is tested where it runs: `gateway/bindings/gateway-verifier-host/tests/service.rs`
//! with a stand-in for the program, and `gateway/bindings/gateway-binding/tests/verifier_tasks.rs`
//! for the host above it. These tests read the sources as text and hold them against each other: the
//! rows against the tests that answer them, every constructor of every public failure against a
//! negative test (R5), the host against any process of its own, and the planted defects against
//! `VERIFIER.md`.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

const CRATE: &str = "gateway/bindings/gateway-verifier-host";
const SERVICE: &str = "gateway/bindings/gateway-verifier-host/src/service.rs";
const TESTS: &str = "gateway/bindings/gateway-verifier-host/tests/service.rs";
const HOST: &str = "gateway/bindings/gateway-binding/src/verifier.rs";

fn answered_by(id: &str, expected: &[&str]) {
    let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
    let mut found = BTreeSet::new();
    let mut in_test = false;
    for line in read(TESTS).lines().map(str::trim) {
        if line == "#[test]" {
            in_test = true;
        } else if in_test && line.starts_with("fn ") {
            let name = line[3..].split('(').next().expect("a name");
            if name.ends_with(&suffix) {
                found.insert(name.to_string());
            }
            in_test = false;
        }
    }
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests that answer {id}");
    assert!(
        read("VERIFIER.md").contains(id),
        "VERIFIER.md does not name {id}"
    );
}

/// The constructors of `pub enum <name>` of the service, as written.
fn constructors(name: &str) -> Vec<String> {
    let source = read(SERVICE);
    let head = format!("pub enum {name} {{");
    let start = source.find(&head).expect("the enum") + head.len();
    let end = start + source[start..].find("\n}\n").expect("its end");
    source[start..end]
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .map(|line| {
            line.split(['(', ','])
                .next()
                .expect("a constructor")
                .to_string()
        })
        .collect()
}

#[test]
fn the_host_has_no_process_of_its_own_vs_01() {
    answered_by(
        "VS-01",
        &[
            "the_program_answers_on_a_socket_of_mode_0600_vs_01",
            "the_program_replaces_the_socket_of_an_earlier_run_vs_01",
            "ready_says_whether_there_is_a_program_to_start_vs_01",
            "a_program_named_by_its_variable_is_the_one_that_starts_vs_01",
        ],
    );
    let manifest = read(&format!("{CRATE}/Cargo.toml"));
    for package in [
        "lexlean",
        "gateway-host-api",
        "gateway-binding",
        "gateway-signer",
    ] {
        assert!(
            !manifest.contains(package),
            "the verifier host links {package}"
        );
    }
    let host = read(HOST);
    for word in [
        "Command::new",
        "std::process",
        "kill_session",
        "kill_process",
    ] {
        assert!(!host.contains(word), "the host has {word} in verifier.rs");
    }
    assert!(
        host.contains("client::run("),
        "the host reaches its processes through the socket"
    );
    assert!(
        !root()
            .join("gateway/bindings/gateway-binding/src/sweep.rs")
            .exists(),
        "the sweep is back in the host"
    );
    // The name of the key's variable is the signer's, spelled again so that this crate need not link it.
    assert!(
        read(SERVICE).contains("SIGNING_KEY_VARIABLE: &str = \"LEXLEAN_MCP_SIGNING_KEY\"")
            && read("gateway/bindings/gateway-signer/src/service.rs")
                .contains("SIGNING_KEY_VARIABLE: &str = \"LEXLEAN_MCP_SIGNING_KEY\""),
        "both name the same variable"
    );
    let workflow = read(".github/workflows/lexlean.yml");
    assert!(
        workflow.contains("gateway_cargo test -p gateway-verifier-host --locked --offline"),
        "the gateway job runs the tests of the verifier host"
    );
}

#[test]
fn one_connection_is_one_task_in_one_process_vs_02() {
    answered_by(
        "VS-02",
        &[
            "the_process_gets_the_job_and_the_settings_and_nothing_of_the_key_vs_02",
            "ended_is_last_and_carries_the_tail_of_a_lost_process_vs_02",
            "two_tasks_run_in_two_processes_at_once_vs_02",
        ],
    );
}

#[test]
fn the_service_ends_what_it_started_vs_03() {
    answered_by(
        "VS-03",
        &[
            "a_cancel_reaches_the_process_and_its_answer_comes_back_vs_03",
            "kill_ends_a_process_that_ignores_everything_vs_03",
            "the_end_of_the_input_ends_the_process_when_the_host_has_gone_vs_03",
            "a_process_that_ignores_the_end_of_the_input_is_killed_after_the_grace_vs_03",
            "a_process_that_does_not_end_after_its_verdict_is_killed_after_the_grace_vs_03",
            "what_a_lost_process_leaves_in_its_session_is_swept_vs_03",
            "a_child_that_holds_the_pipe_does_not_hold_the_end_of_the_task_vs_03",
        ],
    );
}

#[test]
fn every_failure_of_the_verifier_host_is_typed_and_tested_vs_04() {
    answered_by(
        "VS-04",
        &[
            "every_refusal_travels_by_its_name_vs_04",
            "a_directory_outside_the_store_starts_no_process_vs_04",
            "a_program_that_cannot_start_is_refused_vs_04",
            "a_request_that_is_not_one_is_refused_and_the_next_is_served_vs_04",
            "a_request_over_the_bound_is_refused_vs_04",
            "an_answer_that_is_not_one_is_lost_vs_04",
            "no_answer_at_the_socket_is_lost_vs_04",
            "a_service_lost_during_a_task_ends_the_events_without_ended_vs_04",
            "each_setting_the_program_lacks_stops_it_vs_04",
        ],
    );
    let tests = read(TESTS);
    for constructor in constructors("Refusal") {
        assert!(
            tests.contains(&format!("Refusal::{constructor}")),
            "no test names Refusal::{constructor}"
        );
    }
    for constructor in constructors("StartError") {
        assert!(
            tests.contains(&format!("StartError::{constructor}")),
            "no test names StartError::{constructor}"
        );
    }
    assert_eq!(constructors("Refusal").len(), 3);
    assert_eq!(constructors("StartError").len(), 4);
}

#[test]
fn each_gate_of_the_verifier_host_rejects_its_planted_defect_vs_05() {
    let document = read("VERIFIER.md");
    let section = document
        .split("## Planted defects of the verifier host")
        .nth(1)
        .expect("VERIFIER.md has the section")
        .split("\n## ")
        .next()
        .expect("the section");
    let rows = section
        .lines()
        .filter(|line| {
            line.starts_with("| ") && !line.starts_with("| Defect") && !line.starts_with("| ---")
        })
        .count();
    assert_eq!(rows, 11, "the planted defects of VS-05");
    for answering in [
        "the_program_answers_on_a_socket_of_mode_0600_vs_01",
        "a_directory_outside_the_store_starts_no_process_vs_04",
        "the_process_gets_the_job_and_the_settings_and_nothing_of_the_key_vs_02",
        "kill_ends_a_process_that_ignores_everything_vs_03",
        "what_a_lost_process_leaves_in_its_session_is_swept_vs_03",
        "a_process_that_ignores_the_end_of_the_input_is_killed_after_the_grace_vs_03",
        "a_process_that_does_not_end_after_its_verdict_is_killed_after_the_grace_vs_03",
        "every_refusal_travels_by_its_name_vs_04",
        "a_request_over_the_bound_is_refused_vs_04",
        "a_service_lost_during_a_task_ends_the_events_without_ended_vs_04",
        "the_host_has_no_process_of_its_own_vs_01",
    ] {
        assert!(
            section.contains(answering),
            "VERIFIER.md does not name {answering}"
        );
    }
}
