//! VP-01 to VP-06: the verifier process (issue #11, slice 3a).
//!
//! The behavior is tested where the host runs: `gateway/bindings/gateway-binding/tests/
//! verifier_program.rs` runs the real program, and `verifier_tasks.rs` runs the host against a
//! script that speaks the protocol. These tests read the sources as text and hold them against
//! each other: the rows against the tests that answer them, the key against the environment of
//! the process, the order of the lock, the count of ignored tests against the workflow, and the
//! planted defects against `VERIFIER.md`. Nothing here runs a verifier.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

const BINDING: &str = "gateway/bindings/gateway-binding";
const VERIFIER: &str = "gateway/bindings/gateway-binding/src/verifier.rs";
const CHILD: &str = "gateway/bindings/gateway-binding/src/child.rs";

/// The names of the `#[test]` functions of the verifier tests of the binding.
fn gateway_tests() -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for file in ["verifier_program.rs", "verifier_tasks.rs"] {
        let mut in_test = false;
        for line in read(&format!("{BINDING}/tests/{file}")).lines() {
            let line = line.trim();
            if line == "#[test]" {
                in_test = true;
            } else if in_test && line.starts_with("fn ") {
                names.insert(line[3..].split('(').next().expect("a name").to_string());
                in_test = false;
            }
        }
    }
    names
}

/// The tests of the binding that answer the row `id` are exactly `expected`, and `VERIFIER.md`
/// names the row.
fn answered_by(id: &str, expected: &[&str]) {
    let suffix = id.to_lowercase().replace('-', "_");
    let found: BTreeSet<String> = gateway_tests()
        .into_iter()
        .filter(|name| name.ends_with(&format!("_{suffix}")))
        .collect();
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests of the binding that answer {id}");
    assert!(
        read("VERIFIER.md").contains(id),
        "VERIFIER.md does not name {id}"
    );
}

/// The position of `needle` in `text` from `from`.
fn after(text: &str, from: usize, needle: &str) -> usize {
    from + text[from..]
        .find(needle)
        .unwrap_or_else(|| panic!("`{needle}` is not there"))
}

#[test]
fn the_program_speaks_the_protocol_and_leads_a_session_vp_01() {
    answered_by(
        "VP-01",
        &[
            "it_leads_a_session_and_prints_the_failed_verdict_of_a_bad_draft_vp_01",
            "a_line_that_is_not_a_job_is_a_failed_verdict_of_class_5_vp_01",
            "the_end_of_the_input_before_a_job_ends_the_process_without_a_verdict_vp_01",
            "a_cancel_ends_a_real_verification_and_leaves_no_process_in_its_session_vp_01",
        ],
    );
    let child = read(CHILD);
    for needed in ["setsid()", "\"cancel\"", "end_live_children", "\"started\""] {
        assert!(child.contains(needed), "child.rs lacks {needed}");
    }
    assert!(
        read(&format!("{BINDING}/Cargo.toml")).contains("name = \"gateway-verifier\""),
        "the program is a binary of the binding"
    );
    // The gateway job runs the ignored tests by count: the count in the workflow is the number of
    // ignored tests of the binding, and a new ignored test fails here until the workflow says so.
    let mut ignored = 0;
    for entry in std::fs::read_dir(root().join(format!("{BINDING}/tests"))).expect("tests") {
        let path = entry.expect("an entry").path();
        if path.extension().is_some_and(|extension| extension == "rs") {
            ignored += std::fs::read_to_string(&path)
                .expect("a test file")
                .matches("#[ignore")
                .count();
        }
    }
    let workflow = read(".github/workflows/lexlean.yml");
    assert!(
        workflow.contains(&format!("gateway_ignored_tests gateway-binding {ignored} ")),
        "the workflow runs the ignored tests of the binding by count, and the binding has {ignored}"
    );
}

#[test]
fn the_process_has_the_settings_and_not_the_key_vp_02() {
    answered_by(
        "VP-02",
        &[
            "the_process_gets_the_settings_and_nothing_of_the_key_vp_02",
            "a_program_that_is_not_a_file_is_verifier_unavailable_and_copies_nothing_vp_02",
        ],
    );
    let verifier = read(VERIFIER);
    for needed in [
        ".env_remove(SIGNING_KEY_VARIABLE)",
        "\"LEXLEAN_VERIFY_WIDTH\"",
        "\"LEXLEAN_VERIFY_MEMORY_KIB\"",
    ] {
        assert!(verifier.contains(needed), "verifier.rs lacks {needed}");
    }
    // The project is judged before the program, so a project that is not one is refused whatever the
    // service, and the copy a draft makes is dropped when the program is missing.
    let host = read(&format!("{BINDING}/src/lib.rs"));
    let start = host.find("fn verify_start(").expect("verify_start");
    let program = after(&host, start, "verifier_program");
    let copy = after(&host, start, "project::resolve(");
    assert!(copy < program, "the project is judged before the program");
}

#[test]
fn the_slots_and_the_record_of_a_verified_task_vp_03() {
    answered_by(
        "VP-03",
        &[
            "one_slot_runs_one_task_at_a_time_and_the_others_wait_vp_03",
            "two_slots_run_two_tasks_at_once_vp_03",
            "a_verified_task_is_signed_and_recorded_by_the_host_vp_03",
        ],
    );
    let verifier = read(VERIFIER);
    assert!(
        verifier.contains("while tasks.running < shared.concurrency"),
        "tasks start while a slot is free"
    );
    assert_eq!(
        verifier.matches("conclude(&task.job").count(),
        1,
        "a record is made in one place, in `settle`"
    );
}

#[test]
fn a_cancel_ends_one_task_vp_04() {
    answered_by(
        "VP-04",
        &[
            "a_queued_task_leaves_the_queue_and_its_process_is_never_started_vp_04",
            "a_running_task_is_cancelled_and_the_other_task_is_not_touched_vp_04",
            "a_process_that_verifies_while_the_cancel_is_pending_leaves_no_record_vp_04",
            "a_process_that_ignores_the_cancel_is_killed_after_the_grace_vp_04",
        ],
    );
    // The record is made under the lock: `settle` takes it, and the arm that concludes is the one
    // of a task that is `Running`, not of one that is `Cancelling`.
    let verifier = read(VERIFIER);
    let settle = verifier.find("fn settle(").expect("settle");
    let lock = after(&verifier, settle, "let mut tasks = lock(shared);");
    let conclude = after(&verifier, settle, "conclude(&task.job");
    assert!(lock < conclude, "settle takes the lock before it concludes");
    let running_arm = after(&verifier, settle, "(Phase::Running, Some(\"verified\")");
    assert!(running_arm < conclude);
    assert!(
        verifier[settle..conclude]
            .matches("Phase::Cancelling")
            .count()
            == 0,
        "a task that is being cancelled is not concluded"
    );
}

#[test]
fn a_lost_verifier_leaves_nothing_vp_05() {
    answered_by(
        "VP-05",
        &[
            "a_process_that_ends_without_a_verdict_is_a_lost_verifier_vp_05",
            "the_session_of_a_lost_verifier_is_swept_vp_05",
        ],
    );
    let verifier = read(VERIFIER);
    assert!(verifier.contains("verifier-lost"));
    let watch = verifier.find("fn watch(").expect("watch");
    let settle = after(&verifier, watch, "settle(shared, name, verdict");
    let sweep = after(&verifier, watch, "sweep::kill_session(session)");
    assert!(
        settle < sweep,
        "the task is settled, then the session is swept"
    );
    // The process is watched, not only its pipe: an orphan may hold the pipe.
    assert!(verifier.contains("child.try_wait()"));
    let sweep_source = read(&format!("{BINDING}/src/sweep.rs"));
    assert!(
        sweep_source.contains("/proc") && sweep_source.contains("Signal::KILL"),
        "the sweep reads /proc and kills"
    );
}

#[test]
fn each_gate_of_the_verifier_process_rejects_its_planted_defect_vp_06() {
    let document = read("VERIFIER.md");
    let section = document
        .split("## Planted defects")
        .nth(1)
        .expect("VERIFIER.md has a section Planted defects");
    for defect in [
        "the key left in the environment of the process",
        "a record made for a task that is being cancelled",
        "the session of a lost verifier not killed",
        "the slots ignored so that every task starts at once",
        "a process that ignores cancel left running",
    ] {
        let row = section
            .lines()
            .find(|line| line.starts_with('|') && line.contains(defect))
            .unwrap_or_else(|| panic!("VERIFIER.md records no planted defect: {defect}"));
        assert!(
            row.contains("cargo test") && row.contains("vp_0") && row.contains("git checkout"),
            "the row of `{defect}` lacks its command, its failing test or its restore: {row}"
        );
    }
}
