//! The verifier program: what `gateway-verifier` does.
//!
//! The host runs one of these for each task, so that cancelling a task ends only the child
//! process groups of that task (`end_live_children` ends every group of the calling process,
//! SPEC 24.6) and so that the key of the gateway is never in the process that runs Lean. The
//! two speak lines of JSON.
//!
//! The program makes a session of its own, and says so: `{"event":"started","sid":<its ID>}`. The
//! host kills what is left in that session if the program is lost. Then it reads one job line from
//! its standard input, `{"directory":<project>,"selection":{"kind":0|1|2,"files":[...]}}`, runs
//! `Engine::verify`, and prints one verdict and exits:
//!
//! - `{"event":"verified","attestation_id","build_id","source_id","semantic_id","modules","root"}`;
//! - `{"event":"failed","class":<ErrorClass>,"result":<the diagnostics of the engine>}`; a line that
//!   is no job is `failed` with class 5;
//! - `{"event":"cancelled","ended":<groups>,"failed":<groups>}` after a line `cancel` on its input,
//!   which ends every live child group of the process.
//!
//! The end of the input before the verdict means the host is gone: the program ends its groups and
//! exits with status 1, without a verdict.

use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{Duration, Instant};

use camino::Utf8Path;
use lexlean::verify::{end_live_children, GroupEnd};
use lexlean::{Engine, LexLeanError, VerifyRequest};
use serde_json::{json, Value};

use crate::engine::{class_number, verify_result};
use crate::project;

/// The job on the first line of the input.
struct Spec {
    directory: PathBuf,
    kind: u32,
    files: Vec<String>,
}

fn parse(line: &str) -> Result<Spec, String> {
    let value: Value =
        serde_json::from_str(line).map_err(|_| "the first line is not JSON".to_owned())?;
    let directory = value
        .get("directory")
        .and_then(Value::as_str)
        .ok_or("the job names no directory")?;
    let selection = value.get("selection").ok_or("the job names no selection")?;
    let kind = selection
        .get("kind")
        .and_then(Value::as_u64)
        .and_then(|kind| u32::try_from(kind).ok())
        .ok_or("the selection has no kind")?;
    let files = match selection.get("files") {
        None => Vec::new(),
        Some(list) => list
            .as_array()
            .and_then(|list| {
                list.iter()
                    .map(|file| file.as_str().map(str::to_owned))
                    .collect::<Option<Vec<_>>>()
            })
            .ok_or("the files of the selection are not a list of paths")?,
    };
    Ok(Spec {
        directory: PathBuf::from(directory),
        kind,
        files,
    })
}

fn failed(class: u32, message: &str) -> Value {
    json!({"event": "failed", "class": class, "result": {"diagnostics": [{"message": message}]}})
}

fn from_error(error: &LexLeanError) -> Value {
    let result = serde_json::from_slice(&verify_result(error)).unwrap_or(Value::Null);
    json!({"event": "failed", "class": class_number(error.class), "result": result})
}

/// Run `Engine::verify` on the job and make the verdict.
fn verdict(spec: &Spec) -> Value {
    let selection = match project::selection(spec.kind, &spec.files.join("\n")) {
        Ok(selection) => selection,
        Err(message) => return failed(5, &message),
    };
    let file = spec.directory.join("lexlean.toml");
    let Some(file) = Utf8Path::from_path(&file) else {
        return failed(5, "the project path is not UTF-8");
    };
    let engine = match Engine::load(file) {
        Ok(engine) => engine,
        Err(error) => return from_error(&error),
    };
    match engine.verify(VerifyRequest { selection }) {
        Ok(verified) => json!({
            "event": "verified",
            "attestation_id": verified.attestation_id.to_hex(),
            "build_id": verified.build_id.to_hex(),
            "source_id": verified.source_id.to_hex(),
            "semantic_id": verified.semantic_id.to_hex(),
            "modules": verified.units.keys().collect::<Vec<_>>(),
            "root": verified.root.as_str(),
        }),
        Err(error) => from_error(&error),
    }
}

/// One line on the standard output: the verdict and the first event are the only writers.
fn emit(value: &Value) {
    let mut out = io::stdout().lock();
    let _ = writeln!(out, "{value}").and_then(|()| out.flush());
}

/// End the child groups of this process and count what became of them.
fn end_groups() -> (u32, u32) {
    let (mut ended, mut failed) = (0, 0);
    for group in end_live_children() {
        match group.outcome {
            GroupEnd::Ended => ended += 1,
            GroupEnd::Failed(_) => failed += 1,
            GroupEnd::AlreadyGone => {}
        }
    }
    (ended, failed)
}

/// What the program does, as the exit status of its `main`.
pub fn run() -> ExitCode {
    // A session of its own: a session lives on after its leader, and everything the verification
    // starts belongs to it, however the children are started.
    let session = rustix::process::setsid().ok().map(|_| std::process::id());
    emit(&json!({"event": "started", "sid": session}));

    let mut first = String::new();
    if !matches!(io::stdin().read_line(&mut first), Ok(count) if count > 0) {
        return ExitCode::from(1);
    }
    let spec = match parse(&first) {
        Ok(spec) => spec,
        Err(message) => {
            emit(&failed(5, &message));
            return ExitCode::SUCCESS;
        }
    };

    let requested = Arc::new(AtomicBool::new(false));
    let report: Arc<Mutex<Option<(u32, u32)>>> = Arc::new(Mutex::new(None));
    {
        let requested = Arc::clone(&requested);
        let report = Arc::clone(&report);
        std::thread::spawn(move || {
            // Its own lock: the lock of the first read cannot move to another thread.
            for line in io::stdin().lines() {
                match line {
                    Ok(line) if line.trim() == "cancel" => {
                        requested.store(true, Ordering::SeqCst);
                        let counts = end_groups();
                        *report.lock().unwrap_or_else(PoisonError::into_inner) = Some(counts);
                    }
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            // The input ended while the verification runs: the host is gone.
            let _ = end_groups();
            std::process::exit(1);
        });
    }

    let verdict = verdict(&spec);
    if requested.load(Ordering::SeqCst) {
        // A cancel was asked for: whatever the engine concluded, the verdict is the cancel's. Its
        // groups are counted by the thread that ended them, which may be still at it.
        let until = Instant::now() + Duration::from_secs(10);
        let counts = loop {
            if let Some(counts) = *report.lock().unwrap_or_else(PoisonError::into_inner) {
                break counts;
            }
            if Instant::now() > until {
                break (0, 0);
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        emit(&json!({"event": "cancelled", "ended": counts.0, "failed": counts.1}));
    } else {
        emit(&verdict);
    }
    ExitCode::SUCCESS
}
