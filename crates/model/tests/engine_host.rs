//! EH-01 to EH-04: the engine host as a program of its own (issue #17).
//!
//! The behavior is tested where it runs: `gateway/bindings/gateway-engine-host/tests/service.rs` and
//! `gateway/bindings/gateway-binding/tests/effects.rs`. These tests read the sources as text and hold
//! them against each other: the rows against the tests that answer them, every constructor of every
//! public failure against a negative test (R5), the host against any Engine call of its own, and the
//! planted defects against `ENGINE-HOST.md`.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

const SERVICE: &str = "gateway/bindings/gateway-engine-host/src/service.rs";
const SERVICE_TESTS: &str = "gateway/bindings/gateway-engine-host/tests/service.rs";
const HOST_TESTS: &str = "gateway/bindings/gateway-binding/tests/effects.rs";
const DOCUMENT: &str = "ENGINE-HOST.md";

/// The names of the `#[test]` functions that answer the rows of the engine host.
fn tests_of(id: &str) -> BTreeSet<String> {
    let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
    let mut names = BTreeSet::new();
    let mut in_test = false;
    let sources = [read(SERVICE_TESTS), read(HOST_TESTS)];
    for line in sources.iter().flat_map(|source| source.lines()) {
        let line = line.trim();
        if line == "#[test]" {
            in_test = true;
        } else if in_test && line.starts_with("fn ") {
            let name = line[3..].split('(').next().expect("a name");
            if name.ends_with(&suffix) {
                names.insert(name.to_string());
            }
            in_test = false;
        }
    }
    names
}

fn answered_by(id: &str, expected: &[&str]) {
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(tests_of(id), expected, "the tests that answer {id}");
    assert!(read(DOCUMENT).contains(id), "{DOCUMENT} does not name {id}");
}

/// The constructors of `pub enum <name>` of the service, as written.
fn constructors(name: &str) -> Vec<String> {
    let source = read(SERVICE);
    let head = format!("pub enum {name} {{");
    let start = source
        .find(&head)
        .unwrap_or_else(|| panic!("{SERVICE} has no {head}"))
        + head.len();
    let end = start + source[start..].find("\n}\n").expect("the end of the enum");
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
fn the_engine_host_is_a_program_of_its_own_and_the_host_makes_no_call_eh_01() {
    answered_by(
        "EH-01",
        &[
            "the_program_answers_on_a_socket_of_mode_0600_eh_01",
            "the_program_replaces_the_socket_of_an_earlier_run_eh_01",
            "a_host_with_no_engine_host_makes_no_engine_call_eh_01",
            "the_environment_names_the_engine_host_eh_01",
        ],
    );
    let manifest = read("gateway/bindings/gateway-engine-host/Cargo.toml");
    for package in ["gateway-host-api", "gateway-binding", "gateway-signer"] {
        assert!(
            !manifest.contains(package),
            "the engine host links {package}"
        );
    }
    // The four calls are made in one place of the host, and that place is the socket.
    let host = read("gateway/bindings/gateway-binding/src/lib.rs");
    assert!(
        host.contains("gateway_engine_host::client::call("),
        "the host calls the engine host"
    );
    assert!(
        !host.contains("engine::run(") && !host.contains("gateway_engine_host::engine::run"),
        "the host makes an Engine call itself"
    );
    assert!(
        !root()
            .join("gateway/bindings/gateway-binding/src/engine.rs")
            .exists(),
        "the Engine calls are back in the host"
    );
    for call in [".check(", ".snapshot(", ".format(", ".build("] {
        assert!(
            !host.contains(&format!("engine{call}")),
            "the host makes the call {call}"
        );
    }
}

#[test]
fn the_vectors_are_answered_by_the_program_and_ci_runs_its_tests_eh_01() {
    let lib = read(".github/lexlean-ci/lib.sh");
    let verify = &lib[lib.find("gateway_verify() {").expect("gateway_verify")..];
    let verify = &verify[..verify.find("\n}\n").expect("its end")];
    assert!(
        verify.contains("gateway/target/debug/gateway-engine-host")
            && verify.contains("--env LEXLEAN_MCP_ENGINE="),
        "the replay of the vectors reaches the program"
    );
    // The step runs under `bash -e`, and `wait` answers 143 for the program it has just ended.
    assert!(
        verify.contains("wait \"$engine_pid\" 2>/dev/null || true")
            && verify.contains("return \"$status\""),
        "the verdict of the step is that of the replay, not of ending the program"
    );
    let workflow = read(".github/workflows/lexlean.yml");
    assert!(
        workflow.contains("gateway_cargo test -p gateway-engine-host --locked --offline"),
        "the gateway job runs the tests of the engine host"
    );
    let generator = read("gateway/vectors/generate.py");
    assert!(
        generator.contains("LEXLEAN_MCP_ENGINE") && generator.contains("[ENGINE_HOST]"),
        "the recorder of the vectors reaches the program"
    );
}

#[test]
fn an_answer_through_the_socket_is_the_engines_eh_02() {
    answered_by(
        "EH-02",
        &[
            "each_call_through_the_socket_is_the_answer_of_the_engine_eh_02",
            "each_selection_travels_as_itself_eh_02",
            "a_refusal_of_the_engine_is_an_answer_with_its_class_eh_02",
        ],
    );
}

#[test]
fn every_failure_of_the_engine_host_is_typed_and_tested_eh_03() {
    answered_by(
        "EH-03",
        &[
            "every_refusal_and_call_travels_by_its_name_eh_03",
            "a_directory_outside_the_store_is_refused_eh_03",
            "a_request_that_is_not_one_is_refused_and_the_next_is_served_eh_03",
            "a_request_over_the_bound_is_refused_and_the_next_is_served_eh_03",
            "no_answer_at_the_socket_is_unavailable_eh_03",
            "an_answer_that_is_not_one_is_unavailable_eh_03",
            "a_service_lost_before_it_answers_is_unavailable_eh_03",
            "each_setting_the_program_lacks_stops_it_eh_03",
            "a_host_whose_engine_host_has_gone_is_unavailable_and_judges_the_project_first_eh_03",
        ],
    );
    let tests = read(SERVICE_TESTS);
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
fn each_gate_of_the_engine_host_rejects_its_planted_defect_eh_04() {
    let document = read(DOCUMENT);
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("ENGINE-HOST.md has a section Planted defects")
        .split("\n## ")
        .next()
        .expect("the section");
    let rows = section
        .lines()
        .filter(|line| {
            line.starts_with("| ") && !line.starts_with("| Defect") && !line.starts_with("| ---")
        })
        .count();
    assert_eq!(rows, 7, "the planted defects of EH-04");
    for answering in [
        "the_program_answers_on_a_socket_of_mode_0600_eh_01",
        "a_directory_outside_the_store_is_refused_eh_03",
        "each_call_through_the_socket_is_the_answer_of_the_engine_eh_02",
        "every_refusal_and_call_travels_by_its_name_eh_03",
        "a_request_over_the_bound_is_refused_and_the_next_is_served_eh_03",
        "an_answer_that_is_not_one_is_unavailable_eh_03",
        "the_engine_host_is_a_program_of_its_own_and_the_host_makes_no_call_eh_01",
    ] {
        assert!(
            section.contains(answering),
            "ENGINE-HOST.md does not name {answering}"
        );
    }
}
