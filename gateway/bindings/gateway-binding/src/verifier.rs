//! The verifier of the host: tasks that each run `Engine::verify` in a process of their own, and
//! the signing of the verifications that end `verified`.
//!
//! A task is a `gateway-verifier` process (see `child`), which `gateway-verifier-host` starts where
//! the verifier runs, one connection for each task. The host starts at most
//! `verify.concurrency` of them at once and holds the others in a queue, and none fails for waiting.
//! One process for each task is the only isolation the library offers: `end_live_children` ends
//! every child group of the process that calls it, so a process that runs two tasks cannot cancel
//! one. The key of the gateway is not in these processes: they print the facts of a verification,
//! and the host signs and records them (`conclude`) under the lock that a cancel takes, so a task
//! is either cancelled or recorded and never both.
//!
//! A process that ends without a verdict is a lost verifier: its task fails, and whatever its
//! session still holds is killed (`sweep`). Tasks live as long as the host: a task of an earlier
//! process is unknown, and the processes of a host that has gone end themselves when their input
//! does.

use std::collections::{BTreeMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Condvar, Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant};

use gateway_verifier_host::client::{self, Events, Input};
use lexlean::Selection;
use serde_json::{json, Value};

use crate::attestation;
use crate::config::Config;
use crate::evidence;
use crate::signer::{envelope_keyid, RecordSigner};
use crate::telemetry::{self, ErrorClass, GroupEnd, Telemetry, VerifyOutcome};

/// The state of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Waiting for a slot.
    Queued = 0,
    /// Running.
    Running = 1,
    /// Verified: the result holds the attestation.
    Verified = 2,
    /// Failed: the class and the result say how.
    Failed = 3,
    /// Cancelled by `cancel`.
    Cancelled = 4,
}

/// What a finished verification came to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outcome {
    /// 0 when verified, else the class of the error.
    pub class: u32,
    /// The result document.
    pub result: Vec<u8>,
}

/// One verification to run.
#[derive(Debug, Clone)]
pub struct Job {
    /// The directory of the project.
    pub directory: PathBuf,
    /// The modules to verify.
    pub selection: Selection,
    /// The root of the store, where the record of a verification is written.
    pub store: PathBuf,
    /// The signer of the record: a verification is not started without one.
    pub signer: Arc<dyn RecordSigner>,
}

/// A status or a cancellation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Report {
    /// The state of the task.
    pub state: State,
    /// The class of its outcome; 0 until it fails.
    pub class: u32,
    /// The result; empty until it ends.
    pub result: Vec<u8>,
    /// Groups a cancellation ended.
    pub groups_ended: u32,
    /// Groups a cancellation could not end.
    pub groups_failed: u32,
}

/// Where a task is. A cancel in progress is `Cancelling`, which a status reports as running.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Queued,
    Running,
    Cancelling,
    Verified,
    Failed,
    Cancelled,
}

impl Phase {
    fn state(self) -> State {
        match self {
            Phase::Queued => State::Queued,
            Phase::Running | Phase::Cancelling => State::Running,
            Phase::Verified => State::Verified,
            Phase::Failed => State::Failed,
            Phase::Cancelled => State::Cancelled,
        }
    }

    fn ended(self) -> bool {
        matches!(self, Phase::Verified | Phase::Failed | Phase::Cancelled)
    }
}

struct Task {
    job: Job,
    phase: Phase,
    outcome: Option<Outcome>,
    /// The input of the process, held while it runs; letting go of it is the end of the input.
    input: Option<Input>,
    /// The processes the verifier host killed in its session.
    swept: u32,
    /// The groups a cancel ended and could not end, as the process reported them.
    groups: (u32, u32),
    /// The profile of the draft, for the labels of the indicators.
    profile: String,
    queued: Instant,
    started: Option<Instant>,
}

#[derive(Default)]
struct Tasks {
    next: u64,
    tasks: BTreeMap<String, Task>,
    queue: VecDeque<String>,
    /// The processes that run: tasks that are `Running` or `Cancelling`, until the process is gone.
    running: usize,
}

struct Shared {
    tasks: Mutex<Tasks>,
    changed: Condvar,
    socket: Option<PathBuf>,
    concurrency: usize,
    width: u32,
    memory_kib: u64,
    grace: Duration,
    telemetry: Telemetry,
}

/// The verifier of a host.
pub struct Verifier {
    shared: Arc<Shared>,
}

fn lock(shared: &Shared) -> MutexGuard<'_, Tasks> {
    shared.tasks.lock().unwrap_or_else(PoisonError::into_inner)
}

fn millis(elapsed: Duration) -> u64 {
    u64::try_from(elapsed.as_millis()).unwrap_or(u64::MAX)
}

fn cancelled() -> Outcome {
    Outcome {
        class: 0,
        result: serde_json::to_vec(&json!({"cancelled": true})).unwrap_or_default(),
    }
}

fn report(task: &Task, groups_ended: u32, groups_failed: u32) -> Report {
    let (class, result) = task.outcome.as_ref().map_or((0, Vec::new()), |outcome| {
        (outcome.class, outcome.result.clone())
    });
    Report {
        state: task.phase.state(),
        class,
        result,
        groups_ended,
        groups_failed,
    }
}

impl Verifier {
    /// A verifier that runs the program of `config`, `verify.concurrency` tasks at a time.
    pub fn new(config: &Config) -> Verifier {
        Verifier {
            shared: Arc::new(Shared {
                tasks: Mutex::new(Tasks::default()),
                changed: Condvar::new(),
                socket: config.verifier.clone(),
                concurrency: config.verify_concurrency.max(1),
                width: config.verify_width,
                memory_kib: config.verify_memory_kib,
                grace: config.cancel_grace,
                telemetry: Telemetry::new(None),
            }),
        }
    }

    /// This verifier reporting to `telemetry`.
    pub fn with_telemetry(mut self, telemetry: Telemetry) -> Verifier {
        if let Some(shared) = Arc::get_mut(&mut self.shared) {
            shared.telemetry = telemetry;
        }
        self
    }

    /// Queue a task; it runs when a slot is free. Returns its name and its state.
    pub fn start(&self, job: Job) -> (String, State) {
        self.start_as(job, "other")
    }

    /// Queue a task of a draft of `profile`, which labels its indicators.
    pub fn start_as(&self, job: Job, profile: &str) -> (String, State) {
        let name = {
            let mut tasks = lock(&self.shared);
            tasks.next += 1;
            let name = format!("task-{}", tasks.next);
            tasks.tasks.insert(
                name.clone(),
                Task {
                    job,
                    phase: Phase::Queued,
                    outcome: None,
                    input: None,
                    swept: 0,
                    groups: (0, 0),
                    profile: profile.to_owned(),
                    queued: Instant::now(),
                    started: None,
                },
            );
            tasks.queue.push_back(name.clone());
            name
        };
        pump(&self.shared);
        let state = lock(&self.shared)
            .tasks
            .get(&name)
            .map_or(State::Queued, |task| task.phase.state());
        (name, state)
    }

    /// The state of a task; `None` when there is no such task.
    pub fn status(&self, name: &str) -> Option<Report> {
        let tasks = lock(&self.shared);
        tasks.tasks.get(name).map(|task| report(task, 0, 0))
    }

    /// Cancel a task. A queued one leaves the queue and is never given to a process. A running one
    /// is sent `cancel`; the answer carries the groups its process ended, and when the process does
    /// not answer within the grace it is killed with its session. A task that has ended stays as it
    /// ended. `None` when there is no such task.
    pub fn cancel(&self, name: &str) -> Option<Report> {
        let shared = &self.shared;
        let mut tasks = lock(shared);
        match tasks.tasks.get(name)?.phase {
            Phase::Queued => {
                tasks.queue.retain(|queued| queued != name);
                let task = tasks.tasks.get_mut(name)?;
                task.phase = Phase::Cancelled;
                task.outcome = Some(cancelled());
                shared.changed.notify_all();
                return Some(report(task, 0, 0));
            }
            Phase::Running => {
                let task = tasks.tasks.get_mut(name)?;
                task.phase = Phase::Cancelling;
                if let Some(input) = task.input.as_mut() {
                    input.cancel();
                }
            }
            Phase::Cancelling => {}
            Phase::Verified | Phase::Failed | Phase::Cancelled => {
                return Some(report(tasks.tasks.get(name)?, 0, 0));
            }
        }

        // The watcher of the process ends the task when the process answers or is gone.
        let (mut tasks, mut over) = wait_for_end(shared, tasks, name, shared.grace);
        if !over {
            if let Some(input) = tasks.tasks.get_mut(name)?.input.as_mut() {
                input.kill();
            }
            let (again, ended) = wait_for_end(shared, tasks, name, Duration::from_secs(5));
            tasks = again;
            over = ended;
        }
        let task = tasks.tasks.get_mut(name)?;
        if !over {
            // The watcher is stuck; the task is cancelled all the same, and the process is not its business.
            task.phase = Phase::Cancelled;
            task.outcome = Some(cancelled());
            shared.changed.notify_all();
        }
        let (ended, failed) = task.groups;
        let swept = task.swept;
        Some(report(task, ended + swept, failed))
    }
}

impl Drop for Verifier {
    fn drop(&mut self) {
        // Letting go of the inputs ends the processes: each ends its groups when its input does.
        let mut tasks = lock(&self.shared);
        tasks.queue.clear();
        for task in tasks.tasks.values_mut() {
            task.input = None;
        }
    }
}

/// Wait until the task has ended, for at most `limit`; whether it has.
fn wait_for_end<'a>(
    shared: &'a Shared,
    mut tasks: MutexGuard<'a, Tasks>,
    name: &str,
    limit: Duration,
) -> (MutexGuard<'a, Tasks>, bool) {
    let deadline = Instant::now() + limit;
    loop {
        if tasks.tasks.get(name).is_none_or(|task| task.phase.ended()) {
            return (tasks, true);
        }
        let remaining = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return (tasks, false);
        }
        tasks = shared
            .changed
            .wait_timeout(tasks, remaining)
            .unwrap_or_else(PoisonError::into_inner)
            .0;
    }
}

/// The kind and the files of a selection, as the process reads them.
fn selection_spec(selection: &Selection) -> (u32, Vec<String>) {
    match selection {
        Selection::Entrypoints => (0, Vec::new()),
        Selection::All => (1, Vec::new()),
        Selection::Files(files) => (2, files.iter().map(ToString::to_string).collect()),
    }
}

/// Start the queued tasks while a slot is free.
fn pump(shared: &Arc<Shared>) {
    let mut tasks = lock(shared);
    while tasks.running < shared.concurrency {
        let Some(name) = tasks.queue.pop_front() else {
            break;
        };
        let Some(task) = tasks.tasks.get_mut(&name) else {
            continue;
        };
        match launch(shared, &name, task) {
            Ok(()) => tasks.running += 1,
            Err(message) => {
                task.phase = Phase::Failed;
                task.outcome = Some(failure(5, &message));
                shared.changed.notify_all();
            }
        }
    }
}

/// Start the process of a task at the verifier host, and the thread that watches it.
fn launch(shared: &Arc<Shared>, name: &str, task: &mut Task) -> Result<(), String> {
    let socket = shared
        .socket
        .as_deref()
        .ok_or("no verifier host is configured")?;
    let directory = task
        .job
        .directory
        .to_str()
        .ok_or("the project path is not UTF-8")?;
    let (kind, files) = selection_spec(&task.job.selection);
    let (input, events) = client::run(
        socket,
        &client::Run {
            width: shared.width,
            memory_kib: shared.memory_kib,
            grace: shared.grace,
            job: json!({"directory": directory, "selection": {"kind": kind, "files": files}}),
        },
    )
    .map_err(|error| format!("the verifier could not be started: {error:?}"))?;
    task.input = Some(input);
    task.phase = Phase::Running;
    task.started = Some(Instant::now());
    shared.telemetry.queue_wait(millis(task.queued.elapsed()));

    let shared = Arc::clone(shared);
    let name = name.to_owned();
    std::thread::spawn(move || watch(&shared, &name, events));
    Ok(())
}

/// Read the events of a task until its process has gone, settle the task, and free its slot. The
/// verdict settles the task as soon as it comes; the slot is free when the verifier host says that
/// the process and its session have gone, or when the verifier host is lost.
fn watch(shared: &Arc<Shared>, name: &str, events: Events) {
    let mut settled = false;
    let mut log = String::new();
    for event in events {
        match event.get("event").and_then(Value::as_str) {
            Some("verified" | "failed" | "cancelled") if !settled => {
                settled = true;
                settle(shared, name, Some(event), "");
            }
            Some("ended") => {
                log = event
                    .get("log")
                    .and_then(Value::as_str)
                    .unwrap_or_default()
                    .to_owned();
                let swept = event
                    .get("swept")
                    .and_then(Value::as_u64)
                    .and_then(|swept| u32::try_from(swept).ok())
                    .unwrap_or(0);
                if let Some(task) = lock(shared).tasks.get_mut(name) {
                    task.swept = swept;
                }
            }
            _ => {}
        }
    }
    if !settled {
        settle(shared, name, None, &log);
    }
    {
        let mut tasks = lock(shared);
        tasks.running = tasks.running.saturating_sub(1);
        if let Some(task) = tasks.tasks.get_mut(name) {
            task.input = None;
        }
    }
    shared.changed.notify_all();
    pump(shared);
}

/// The facts of a `verified` verdict, or `None` when it is not one.
fn facts_of(event: &Value) -> Option<Facts> {
    let text = |field: &str| event.get(field).and_then(Value::as_str).map(str::to_owned);
    let modules = event
        .get("modules")?
        .as_array()?
        .iter()
        .map(|module| module.as_str().map(str::to_owned))
        .collect::<Option<Vec<_>>>()?;
    Some(Facts {
        attestation_id: text("attestation_id")?,
        build_id: text("build_id")?,
        source_id: text("source_id")?,
        semantic_id: text("semantic_id")?,
        modules,
        root: PathBuf::from(text("root")?),
    })
}

/// The end of a task, given the verdict of its process or none. This is the one place a verdict
/// becomes a state, and the one place a record is made, under the lock that a cancel takes.
fn settle(shared: &Shared, name: &str, verdict: Option<Value>, log: &str) {
    let mut tasks = lock(shared);
    let Some(task) = tasks.tasks.get_mut(name) else {
        return;
    };
    let was_over = task.phase.ended();
    let event = verdict
        .as_ref()
        .and_then(|verdict| verdict.get("event"))
        .and_then(Value::as_str);
    match (task.phase, event, &verdict) {
        (Phase::Running, Some("verified"), Some(verdict)) => match facts_of(verdict) {
            Some(facts) => {
                let outcome = conclude(&task.job, &facts);
                // A verified task that cannot be recorded is a signing failure (`slo.S8`).
                if outcome.class != 0 {
                    shared.telemetry.signing_failure();
                }
                task.phase = if outcome.class == 0 {
                    Phase::Verified
                } else {
                    Phase::Failed
                };
                task.outcome = Some(outcome);
            }
            None => lose(task, log),
        },
        (Phase::Running, Some("failed"), Some(verdict)) => {
            let class = verdict
                .get("class")
                .and_then(Value::as_u64)
                .and_then(|class| u32::try_from(class).ok())
                .unwrap_or(5);
            task.phase = Phase::Failed;
            task.outcome = Some(Outcome {
                class,
                result: serde_json::to_vec(verdict.get("result").unwrap_or(&Value::Null))
                    .unwrap_or_default(),
            });
        }
        (Phase::Running, _, _) => lose(task, log),
        (Phase::Cancelling, event, verdict) => {
            // Whatever the process concluded, the task was cancelled: nothing is recorded.
            if let (Some("cancelled"), Some(verdict)) = (event, verdict) {
                let count = |field: &str| {
                    verdict
                        .get(field)
                        .and_then(Value::as_u64)
                        .and_then(|count| u32::try_from(count).ok())
                        .unwrap_or(0)
                };
                task.groups = (count("ended"), count("failed"));
                shared
                    .telemetry
                    .child_group_ends(GroupEnd::Ended, task.groups.0);
                shared
                    .telemetry
                    .child_group_ends(GroupEnd::Failed, task.groups.1);
            }
            task.phase = Phase::Cancelled;
            task.outcome = Some(cancelled());
        }
        _ => {}
    }
    if !was_over && task.phase.ended() {
        end_telemetry(shared, task);
    }
    shared.changed.notify_all();
}

/// The indicators of a task that has just ended: its duration and outcome (S4), and the class of a
/// failure (S1), each with the profile and the code of the first error (OB-17). A task that failed
/// because of its process is class 5, an internal fault.
fn end_telemetry(shared: &Shared, task: &Task) {
    let outcome = match task.phase {
        Phase::Verified => VerifyOutcome::Verified,
        Phase::Cancelled => VerifyOutcome::Cancelled,
        _ => VerifyOutcome::Failed,
    };
    let code = task
        .outcome
        .as_ref()
        .and_then(|outcome| telemetry::first_error_code(&outcome.result));
    if let Some(started) = task.started {
        shared.telemetry.verify_duration(
            &task.profile,
            outcome,
            code.as_deref(),
            millis(started.elapsed()),
        );
    }
    let class = match outcome {
        VerifyOutcome::Failed => task
            .outcome
            .as_ref()
            .and_then(|outcome| ErrorClass::from_number(outcome.class)),
        _ => None,
    };
    shared
        .telemetry
        .tool_outcome(class, &task.profile, code.as_deref());
}

/// The task of a process that ended without a verdict.
fn lose(task: &mut Task, log: &str) {
    task.phase = Phase::Failed;
    task.outcome = Some(Outcome {
        class: 5,
        result: serde_json::to_vec(&json!({"failure": "verifier-lost", "log": log}))
            .unwrap_or_default(),
    });
}

/// What a verification that ended `verified` established, and where its verified directory is.
#[derive(Debug, Clone)]
pub struct Facts {
    /// The attestation ID, in lowercase hexadecimal.
    pub attestation_id: String,
    /// The build ID.
    pub build_id: String,
    /// The source ID.
    pub source_id: String,
    /// The semantic ID.
    pub semantic_id: String,
    /// The modules that were verified.
    pub modules: Vec<String>,
    /// The verified directory, which holds `attestation.json`.
    pub root: PathBuf,
}

/// Sign and record a verification that ended `verified`, or leave the store as it was.
///
/// The subject is the `attestation.json` of the verified directory, byte for byte; the predicate
/// carries the IDs, the modules and the digest of each source file, so the envelope binds the
/// record to the files it covers. The evidence that the subject binds (the build manifest, the
/// renderings and the sources, `evidence`) is gathered and checked first: a verification whose
/// evidence cannot be kept whole has no record. The envelope is written last, after the history
/// that names its key (the signer's), the subject and the evidence, so a verification that could
/// not be signed has no record.
pub fn conclude(job: &Job, facts: &Facts) -> Outcome {
    let Ok(subject) = std::fs::read(facts.root.join("attestation.json")) else {
        return failure(
            5,
            "the attestation.json of the verification could not be read",
        );
    };
    let evidence = match evidence::gather(&facts.root, &job.directory, &subject) {
        Ok(evidence) => evidence,
        Err(reason) => {
            return failure(
                5,
                &format!("the evidence of the verification could not be kept: {reason}"),
            );
        }
    };
    let digests = attestation::source_digests(&job.directory);
    let predicate = json!({
        "status": "verified",
        "attestation_id": facts.attestation_id,
        "build_id": facts.build_id,
        "source_id": facts.source_id,
        "semantic_id": facts.semantic_id,
        "modules": facts.modules,
        "files": digests,
    });
    let envelope = match job.signer.sign_record(&subject, &predicate) {
        Ok(envelope) => envelope,
        Err(error) => {
            return failure(5, &format!("the verification could not be signed: {error}"));
        }
    };
    if attestation::record(
        &job.store,
        &facts.attestation_id,
        &digests,
        &subject,
        &evidence,
        &envelope,
    )
    .is_err()
    {
        return failure(5, "the record of the verification could not be written");
    }
    let result = json!({
        "attestation_id": facts.attestation_id,
        "build_id": facts.build_id,
        "source_id": facts.source_id,
        "semantic_id": facts.semantic_id,
        "modules": facts.modules,
        "keyid": envelope_keyid(&envelope),
    });
    Outcome {
        class: 0,
        result: serde_json::to_vec(&result).unwrap_or_default(),
    }
}

fn failure(class: u32, message: &str) -> Outcome {
    Outcome {
        class,
        result: serde_json::to_vec(&json!({"diagnostics": [{"message": message}]}))
            .unwrap_or_default(),
    }
}
