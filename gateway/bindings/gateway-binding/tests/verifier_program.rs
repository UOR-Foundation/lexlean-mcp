//! VP-01: the program `gateway-verifier`, run as the host runs it: a line of JSON in, lines of
//! JSON out. The cases that need no Lean are run here; the one that cancels a real verification
//! needs the toolchain and is ignored, as the real verification of `effects.rs` is.

#[allow(dead_code)]
mod common;

use std::io::{BufRead, BufReader, Lines, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, ChildStdout, Command, ExitStatus, Stdio};
use std::time::Duration;

use serde_json::{json, Value};

/// The program, started, with its input and its lines of output.
struct Program {
    child: Child,
    input: Option<ChildStdin>,
    output: Lines<BufReader<ChildStdout>>,
}

impl Program {
    fn start() -> Program {
        let mut child = Command::new(common::real_verifier())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .expect("the program starts");
        let input = child.stdin.take();
        let output = BufReader::new(child.stdout.take().expect("an output")).lines();
        Program {
            child,
            input,
            output,
        }
    }

    fn send(&mut self, line: &str) {
        let input = self.input.as_mut().expect("an open input");
        writeln!(input, "{line}").expect("a write");
        input.flush().expect("a flush");
    }

    /// The next event, or `None` when the output ended.
    fn event(&mut self) -> Option<Value> {
        let line = self.output.next()?.expect("a line");
        Some(serde_json::from_str(&line).expect("a line of JSON"))
    }

    fn finish(mut self) -> ExitStatus {
        drop(self.input.take());
        self.child.wait().expect("the program ends")
    }
}

fn job(directory: &Path, kind: u32) -> String {
    json!({"directory": directory, "selection": {"kind": kind, "files": []}}).to_string()
}

/// A copy of the fixture project whose only module is nonsense.
fn bad_draft(dir: &Path) -> std::path::PathBuf {
    let project = dir.join("project");
    common::copy(Path::new(common::TINY), &project);
    std::fs::write(
        project.join("src/Tiny.lex.tex"),
        b"\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n",
    )
    .expect("a source");
    project
}

#[test]
fn it_leads_a_session_and_prints_the_failed_verdict_of_a_bad_draft_vp_01() {
    let dir = tempfile::tempdir().expect("a directory");
    let project = bad_draft(dir.path());
    let mut program = Program::start();
    let started = program.event().expect("a started event");
    assert_eq!(started["event"], "started");
    assert_eq!(
        started["sid"],
        program.child.id(),
        "the session it leads is its own process ID"
    );
    program.send(&job(&project, 0));
    let verdict = program.event().expect("a verdict");
    assert_eq!(verdict["event"], "failed", "{verdict}");
    assert_eq!(verdict["class"], 1, "a language error: {verdict}");
    assert!(
        verdict["result"]["diagnostics"].is_array(),
        "the diagnostics of the engine: {verdict}"
    );
    assert_eq!(program.event(), None, "one verdict and nothing after it");
    assert!(program.finish().success());
}

#[test]
fn a_line_that_is_not_a_job_is_a_failed_verdict_of_class_5_vp_01() {
    let dir = tempfile::tempdir().expect("a directory");
    let project = bad_draft(dir.path());
    let lines = [
        "not json".to_owned(),
        "{}".to_owned(),
        json!({"directory": project}).to_string(),
        json!({"directory": project, "selection": {"kind": "all"}}).to_string(),
        job(&project, 7),
    ];
    for line in lines {
        let mut program = Program::start();
        assert_eq!(program.event().expect("started")["event"], "started");
        program.send(&line);
        let verdict = program.event().expect("a verdict");
        assert_eq!(verdict["event"], "failed", "{line}: {verdict}");
        assert_eq!(verdict["class"], 5, "{line}: {verdict}");
        assert!(program.finish().success(), "{line}");
    }
}

#[test]
fn the_end_of_the_input_before_a_job_ends_the_process_without_a_verdict_vp_01() {
    let mut program = Program::start();
    assert_eq!(program.event().expect("started")["event"], "started");
    drop(program.input.take());
    assert_eq!(program.event(), None, "no verdict");
    let status = program.child.wait().expect("the program ends");
    assert_eq!(status.code(), Some(1), "an end that no verdict explains");
}

#[test]
#[ignore = "runs a real verification, which needs the Lean toolchain: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn a_cancel_ends_a_real_verification_and_leaves_no_process_in_its_session_vp_01() {
    let dir = tempfile::tempdir().expect("a directory");
    let project = dir.path().join("project");
    common::copy(Path::new(common::TINY), &project);
    let mut program = Program::start();
    let session = program.child.id();
    assert_eq!(program.event().expect("started")["event"], "started");
    program.send(&job(&project, 0));
    // The verification is under way: it takes seconds, and the cancel comes after one.
    std::thread::sleep(Duration::from_secs(1));
    program.send("cancel");
    let verdict = program.event().expect("a verdict");
    assert_eq!(verdict["event"], "cancelled", "{verdict}");
    assert!(
        verdict["ended"].is_u64() && verdict["failed"].is_u64(),
        "{verdict}"
    );
    assert!(program.finish().success());
    assert_eq!(
        common::members_of_session(session),
        Vec::<u32>::new(),
        "nothing of the verification is left in its session"
    );
}
