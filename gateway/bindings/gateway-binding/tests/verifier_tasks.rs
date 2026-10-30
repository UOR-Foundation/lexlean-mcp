//! VP-02 to VP-05: the tasks of the host and the processes they run in. The verifier is the
//! script of `common/fake-verifier.sh`, which speaks the protocol of `gateway-verifier` and
//! does what the project says, so that the queue, the cancel and the loss of a process are
//! tested without Lean.

#[allow(dead_code)]
mod common;

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use common::{
    cancel, config, fake_verifier, files_below, found_in_drafts, gone_within, hex, install_draft,
    install_named, number_in, start, start_named, status, wait_for, SEED,
};
use gateway_binding::signer;
use gateway_binding::{Config, Native, State};
use gateway_host_api::protocol::{verify_control, verify_start};
use serde_json::Value;

/// How a test chooses the configured program from its directory.
type Choose = fn(&Path) -> Option<PathBuf>;

/// A host whose verifier is the fake, with the draft project installed.
fn host_with(dir: &Path, tweak: impl FnOnce(Config) -> Config) -> Native {
    let host = Native::new(tweak(config(dir, fake_verifier(dir))));
    install_draft(&host, None);
    host
}

/// Give the task a project of its own, with `behavior`, and start it.
fn task(host: &mut Native, name: &str, behavior: &str) -> (String, u32) {
    install_named(host, name, behavior);
    start_named(host, name).expect("a task")
}

fn gate(dir: &Path, number: u32) -> PathBuf {
    dir.join(format!("gate-{number}"))
}

fn open(gate: &Path) {
    std::fs::write(gate, b"").expect("a gate");
}

/// The envelopes in the verifications area: the records.
fn envelopes(host: &Native) -> Vec<String> {
    files_below(&host.config().store.join("verifications"))
        .into_iter()
        .filter(|file| file.ends_with(".json") && !file.contains('/'))
        .collect()
}

fn result_of(report: &verify_control::Completed) -> Value {
    serde_json::from_slice(&report.result).expect("a result in JSON")
}

// VP-02

#[test]
fn the_process_gets_the_settings_and_nothing_of_the_key_vp_02() {
    let dir = tempfile::tempdir().expect("a directory");
    let key_path = dir.path().join("signing.key");
    // The variable is set for the host, as an operator sets it, so that its removal is observed.
    std::env::set_var("LEXLEAN_MCP_SIGNING_KEY", &key_path);
    let mut host = host_with(dir.path(), |config| config);
    let (task, _) = task(&mut host, "t1", "verified");
    wait_for(&mut host, &task, State::Verified);
    std::env::remove_var("LEXLEAN_MCP_SIGNING_KEY");

    let seen = found_in_drafts(&host, "seen");
    assert_eq!(seen.len(), 1, "one process saw one task");
    let text = std::fs::read_to_string(&seen[0]).expect("what the process saw");
    assert!(
        text.contains("LEXLEAN_VERIFY_WIDTH=1\n"),
        "the width: {text}"
    );
    assert!(
        text.contains("LEXLEAN_VERIFY_MEMORY_KIB=12582912\n"),
        "the memory: {text}"
    );
    assert!(!text.contains("LEXLEAN_MCP_SIGNING_KEY"), "the variable");
    assert!(!text.contains(&hex(&SEED)), "the seed");
    assert!(
        !text.contains(key_path.to_str().expect("a path")),
        "the path of the key file"
    );
}

#[test]
fn a_program_that_is_not_a_file_is_verifier_unavailable_and_copies_nothing_vp_02() {
    let cases: [(&str, Choose); 3] = [
        ("no program", |_| None),
        ("a missing file", |dir| Some(dir.join("absent"))),
        ("a directory", |dir| Some(dir.to_path_buf())),
    ];
    for (label, program) in cases {
        let dir = tempfile::tempdir().expect("a directory");
        let mut config = config(dir.path(), fake_verifier(dir.path()));
        config.verifier_program = program(dir.path());
        let mut host = Native::new(config);
        install_draft(&host, None);
        assert_eq!(
            start(&mut host),
            Err(verify_start::Failure::VerifierUnavailable),
            "{label}"
        );
        assert_eq!(
            status(&mut host, "task-1").map(|_| ()),
            Err(verify_control::Failure::UnknownTask),
            "{label}: no task was made"
        );
        assert_eq!(
            files_below(&host.config().store.join("builds")),
            Vec::<String>::new(),
            "{label}: the project was not copied"
        );
    }
}

// VP-03

#[test]
fn one_slot_runs_one_task_at_a_time_and_the_others_wait_vp_03() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config);
    let gates: Vec<PathBuf> = (1..=3).map(|number| gate(dir.path(), number)).collect();
    let mut started = Vec::new();
    for (index, gate) in gates.iter().enumerate() {
        started.push(task(
            &mut host,
            &format!("t{index}"),
            &format!("verified {}", gate.display()),
        ));
    }
    assert_eq!(
        started,
        vec![
            ("task-1".to_owned(), State::Running as u32),
            ("task-2".to_owned(), State::Queued as u32),
            ("task-3".to_owned(), State::Queued as u32),
        ],
        "the first runs and the others wait, in the order they were started"
    );
    let running = |host: &mut Native| {
        (1..=3)
            .filter(|number| {
                status(host, &format!("task-{number}"))
                    .expect("known")
                    .state
                    == State::Running as u32
            })
            .count()
    };
    for (index, gate) in gates.iter().enumerate() {
        wait_for(&mut host, &format!("task-{}", index + 1), State::Running);
        assert_eq!(running(&mut host), 1, "one at a time, before gate {index}");
        open(gate);
        wait_for(&mut host, &format!("task-{}", index + 1), State::Verified);
        if index == 0 {
            wait_for(&mut host, "task-2", State::Running);
            assert_eq!(
                status(&mut host, "task-3").expect("known").state,
                State::Queued as u32,
                "the second goes before the third"
            );
        }
    }
    assert_eq!(envelopes(&host).len(), 3, "none failed for waiting");
}

#[test]
fn two_slots_run_two_tasks_at_once_vp_03() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config.with_verify_concurrency(2));
    let gates: Vec<PathBuf> = (1..=3).map(|number| gate(dir.path(), number)).collect();
    let mut states = Vec::new();
    for (index, gate) in gates.iter().enumerate() {
        states.push(
            task(
                &mut host,
                &format!("t{index}"),
                &format!("verified {}", gate.display()),
            )
            .1,
        );
    }
    assert_eq!(
        states,
        [
            State::Running as u32,
            State::Running as u32,
            State::Queued as u32
        ]
    );
    open(&gates[0]);
    wait_for(&mut host, "task-1", State::Verified);
    wait_for(&mut host, "task-3", State::Running);
    open(&gates[1]);
    open(&gates[2]);
    for task in ["task-2", "task-3"] {
        wait_for(&mut host, task, State::Verified);
    }
}

#[test]
fn a_verified_task_is_signed_and_recorded_by_the_host_vp_03() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config);
    let (task, _) = task(&mut host, "t1", "verified");
    let done = wait_for(&mut host, &task, State::Verified);
    assert_eq!(done.class, 0);
    let result = result_of(&done);
    let id = result["attestation_id"].as_str().expect("an id").to_owned();
    let store = host.config().store.clone();
    let envelope = std::fs::read(store.join("verifications").join(format!("{id}.json")))
        .expect("the envelope");
    let subject = std::fs::read(
        store
            .join("verifications")
            .join(&id)
            .join("attestation.json"),
    )
    .expect("the subject");
    let history = signer::read_history(&host.config().history_path()).expect("the history");
    let statement = signer::validate(&envelope, &history, &subject).expect("a valid envelope");
    assert_eq!(statement["predicate"]["attestation_id"], id.as_str());
    assert_eq!(result["keyid"], history[0].keyid.as_str());
}

// VP-04

#[test]
fn a_queued_task_leaves_the_queue_and_its_process_is_never_started_vp_04() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config);
    let (first, _) = task(&mut host, "t1", "wait-cancel");
    let (second, state) = task(&mut host, "t2", "verified");
    assert_eq!(state, State::Queued as u32);
    let cancelled = cancel(&mut host, &second).expect("a cancel");
    assert_eq!(cancelled.state, State::Cancelled as u32);
    assert_eq!(
        cancel(&mut host, &first).expect("a cancel").state,
        State::Cancelled as u32
    );
    // Nothing becomes of the queue afterwards: the cancelled task does not start.
    std::thread::sleep(Duration::from_millis(300));
    assert_eq!(
        found_in_drafts(&host, "seen").len(),
        1,
        "only the first task was ever given to a process"
    );
    assert_eq!(
        status(&mut host, &second).expect("known").state,
        State::Cancelled as u32
    );
}

#[test]
fn a_running_task_is_cancelled_and_the_other_task_is_not_touched_vp_04() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config.with_verify_concurrency(2));
    let (cancelled, _) = task(&mut host, "t1", "wait-cancel");
    let release = gate(dir.path(), 1);
    let (other, state) = task(&mut host, "t2", &format!("verified {}", release.display()));
    assert_eq!(state, State::Running as u32);

    let report = cancel(&mut host, &cancelled).expect("a cancel");
    assert_eq!(report.state, State::Cancelled as u32);
    assert_eq!(
        (report.groups_ended, report.groups_failed),
        (2, 1),
        "the groups the process reported"
    );
    assert_eq!(envelopes(&host), Vec::<String>::new(), "no record");
    assert_eq!(
        status(&mut host, &other).expect("known").state,
        State::Running as u32,
        "the other task is still running"
    );
    open(&release);
    wait_for(&mut host, &other, State::Verified);
    assert_eq!(
        envelopes(&host).len(),
        1,
        "the record of the other task only"
    );
    assert_eq!(
        status(&mut host, &cancelled).expect("known").state,
        State::Cancelled as u32
    );
}

#[test]
fn a_process_that_verifies_while_the_cancel_is_pending_leaves_no_record_vp_04() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config);
    let (task, _) = task(&mut host, "t1", "verified-on-cancel");
    let report = cancel(&mut host, &task).expect("a cancel");
    assert_eq!(report.state, State::Cancelled as u32);
    assert_eq!(envelopes(&host), Vec::<String>::new(), "no record");
    assert_eq!(
        files_below(&host.config().store.join("index")),
        Vec::<String>::new()
    );
    assert_eq!(
        status(&mut host, &task).expect("known").state,
        State::Cancelled as u32,
        "it stays cancelled"
    );
}

#[test]
fn a_process_that_ignores_the_cancel_is_killed_after_the_grace_vp_04() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| {
        config.with_cancel_grace(Duration::from_millis(300))
    });
    let (task, _) = task(&mut host, "t1", "hang");
    // The process has begun when it has written its process ID.
    let until = Instant::now() + Duration::from_secs(10);
    let pid = loop {
        if let Some(file) = found_in_drafts(&host, "pid").first() {
            if let Ok(text) = std::fs::read_to_string(file) {
                if let Ok(pid) = text.trim().parse::<u32>() {
                    break pid;
                }
            }
        }
        assert!(Instant::now() < until, "the process did not begin");
        std::thread::sleep(Duration::from_millis(20));
    };
    assert!(common::alive(pid));
    let report = cancel(&mut host, &task).expect("a cancel");
    assert_eq!(report.state, State::Cancelled as u32);
    assert!(
        gone_within(pid, 5),
        "the process that ignored the cancel is gone"
    );
    assert_eq!(envelopes(&host), Vec::<String>::new());
}

// VP-05

#[test]
fn a_process_that_ends_without_a_verdict_is_a_lost_verifier_vp_05() {
    for (mode, log) in [("die", Some("boom")), ("kill-self", None)] {
        let dir = tempfile::tempdir().expect("a directory");
        let mut host = host_with(dir.path(), |config| config.with_verify_concurrency(2));
        let (lost, _) = task(&mut host, "t1", mode);
        let (other, _) = task(&mut host, "t2", "verified");

        let report = wait_for(&mut host, &lost, State::Failed);
        assert_eq!(report.class, 5, "{mode}");
        let result = result_of(&report);
        assert_eq!(result["failure"], "verifier-lost", "{mode}: {result}");
        if let Some(log) = log {
            assert!(
                result["log"]
                    .as_str()
                    .is_some_and(|tail| tail.contains(log)),
                "{mode}: the tail of the standard error is in the result: {result}"
            );
        }
        wait_for(&mut host, &other, State::Verified);
        assert_eq!(
            envelopes(&host).len(),
            1,
            "{mode}: the record of the other task and none for the lost one"
        );
    }
}

#[test]
fn the_session_of_a_lost_verifier_is_swept_vp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let mut host = host_with(dir.path(), |config| config);
    let (task, _) = task(&mut host, "t1", "orphan");
    wait_for(&mut host, &task, State::Failed);
    let files = found_in_drafts(&host, "orphan.pid");
    assert_eq!(files.len(), 1, "the process left a child behind");
    let orphan = number_in(&files[0]);
    assert!(
        gone_within(orphan, 5),
        "the child that outlived its verifier was killed with the session"
    );
    assert_eq!(envelopes(&host), Vec::<String>::new());
}
