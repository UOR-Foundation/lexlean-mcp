//! The verifier: tasks that run `Engine::verify` on a draft project, one at a time.
//!
//! The owner value `verify.concurrency` is 1: a task started while another runs waits, and
//! none fails for waiting. A task is a thread of this process. LexLean runs Lean in child
//! process groups of its own; cancelling a running task ends every live group the library
//! started (`end_live_children`), which is the running task's, because only one runs.
//! Tasks live as long as the process: a task of an earlier process is unknown.

use std::collections::{BTreeMap, VecDeque};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

use camino::Utf8Path;
use lexlean::verify::{allow_children, end_live_children, GroupEnd};
use lexlean::{Engine, Selection, VerifyRequest};
use serde_json::json;

use crate::attestation;
use crate::engine::{class_number, diagnostics};

/// The state of a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    /// Waiting for the running task to end.
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
}

/// What runs a job; replaced in tests.
pub type Runner = Arc<dyn Fn(&Job) -> Outcome + Send + Sync>;

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

struct Task {
    job: Job,
    state: State,
    cancelled: bool,
    outcome: Option<Outcome>,
}

#[derive(Default)]
struct Tasks {
    next: u64,
    tasks: BTreeMap<String, Task>,
    queue: VecDeque<String>,
    running: Option<String>,
}

struct Shared {
    tasks: Mutex<Tasks>,
    runner: Runner,
}

/// The verifier of a host.
pub struct Verifier {
    shared: Arc<Shared>,
}

fn lock(tasks: &Mutex<Tasks>) -> MutexGuard<'_, Tasks> {
    tasks.lock().unwrap_or_else(PoisonError::into_inner)
}

impl Default for Verifier {
    fn default() -> Verifier {
        Verifier::with_runner(Arc::new(verify))
    }
}

impl Verifier {
    /// A verifier that runs jobs with `runner`.
    pub fn with_runner(runner: Runner) -> Verifier {
        Verifier {
            shared: Arc::new(Shared {
                tasks: Mutex::new(Tasks::default()),
                runner,
            }),
        }
    }

    /// Queue a task; it runs when none is running. Returns its name and its state.
    pub fn start(&self, job: Job) -> (String, State) {
        let name = {
            let mut tasks = lock(&self.shared.tasks);
            tasks.next += 1;
            let name = format!("task-{}", tasks.next);
            tasks.tasks.insert(
                name.clone(),
                Task {
                    job,
                    state: State::Queued,
                    cancelled: false,
                    outcome: None,
                },
            );
            tasks.queue.push_back(name.clone());
            name
        };
        pump(&self.shared);
        let state = lock(&self.shared.tasks)
            .tasks
            .get(&name)
            .map_or(State::Queued, |task| task.state);
        (name, state)
    }

    /// The state of a task; `None` when there is no such task.
    pub fn status(&self, name: &str) -> Option<Report> {
        let tasks = lock(&self.shared.tasks);
        tasks.tasks.get(name).map(|task| report(task, 0, 0))
    }

    /// Cancel a task: a queued one leaves the queue, a running one has its process groups
    /// ended. A task that has ended stays as it ended. `None` when there is no such task.
    pub fn cancel(&self, name: &str) -> Option<Report> {
        let mut tasks = lock(&self.shared.tasks);
        let state = tasks.tasks.get(name)?.state;
        match state {
            State::Queued => {
                tasks.queue.retain(|queued| queued != name);
                let task = tasks.tasks.get_mut(name)?;
                task.state = State::Cancelled;
                task.cancelled = true;
                task.outcome = Some(cancelled());
                Some(report(task, 0, 0))
            }
            State::Running => {
                // Held under the lock of the tasks: the worker takes it to end the task, so it
                // cannot finish and re-enable children between the mark and the ending.
                let task = tasks.tasks.get_mut(name)?;
                task.cancelled = true;
                task.state = State::Cancelled;
                task.outcome = Some(cancelled());
                let mut ended = 0;
                let mut failed = 0;
                for group in end_live_children() {
                    match group.outcome {
                        GroupEnd::Ended => ended += 1,
                        GroupEnd::Failed(_) => failed += 1,
                        GroupEnd::AlreadyGone => {}
                    }
                }
                Some(report(tasks.tasks.get(name)?, ended, failed))
            }
            State::Verified | State::Failed | State::Cancelled => {
                Some(report(tasks.tasks.get(name)?, 0, 0))
            }
        }
    }
}

fn cancelled() -> Outcome {
    Outcome {
        class: 0,
        result: serde_json::to_vec(&json!({"cancelled": true})).unwrap_or_default(),
    }
}

fn report(task: &Task, groups_ended: u32, groups_failed: u32) -> Report {
    let (class, result) = task
        .outcome
        .as_ref()
        .map_or((0, Vec::new()), |outcome| (outcome.class, outcome.result.clone()));
    Report {
        state: task.state,
        class,
        result,
        groups_ended,
        groups_failed,
    }
}

/// Start the next queued task when none is running.
fn pump(shared: &Arc<Shared>) {
    let next = {
        let mut tasks = lock(&shared.tasks);
        if tasks.running.is_some() {
            return;
        }
        let Some(name) = tasks.queue.pop_front() else {
            return;
        };
        tasks.running = Some(name.clone());
        let Some(task) = tasks.tasks.get_mut(&name) else {
            tasks.running = None;
            return;
        };
        task.state = State::Running;
        (name, task.job.clone())
    };
    let shared = Arc::clone(shared);
    std::thread::spawn(move || {
        let (name, job) = next;
        let outcome = (shared.runner)(&job);
        {
            let mut tasks = lock(&shared.tasks);
            if let Some(task) = tasks.tasks.get_mut(&name) {
                if task.cancelled {
                    task.state = State::Cancelled;
                    task.outcome = Some(cancelled());
                    // The library refuses children after `end_live_children`.
                    allow_children();
                } else {
                    task.state = if outcome.class == 0 {
                        State::Verified
                    } else {
                        State::Failed
                    };
                    task.outcome = Some(outcome);
                }
            }
            tasks.running = None;
        }
        pump(&shared);
    });
}

/// Run `Engine::verify` on the project of a job and record the verification.
pub fn verify(job: &Job) -> Outcome {
    let file = job.directory.join("lexlean.toml");
    let Some(file) = Utf8Path::from_path(&file) else {
        return failure(5, "the project path is not UTF-8");
    };
    let engine = match Engine::load(file) {
        Ok(engine) => engine,
        Err(error) => {
            return Outcome {
                class: class_number(error.class),
                result: diagnostics(&error),
            }
        }
    };
    match engine.verify(VerifyRequest {
        selection: job.selection.clone(),
    }) {
        Ok(verified) => {
            let attestation = verified.attestation_id.to_hex();
            let record = json!({
                "attestation_id": attestation,
                "build_id": verified.build_id.to_hex(),
                "source_id": verified.source_id.to_hex(),
                "semantic_id": verified.semantic_id.to_hex(),
                "modules": verified.units.keys().collect::<Vec<_>>(),
            });
            match attestation::record(&job.store, &job.directory, &attestation, &record) {
                Ok(()) => Outcome {
                    class: 0,
                    result: serde_json::to_vec(&record).unwrap_or_default(),
                },
                Err(()) => failure(5, "the record of the verification could not be written"),
            }
        }
        Err(error) => Outcome {
            class: class_number(error.class),
            result: diagnostics(&error),
        },
    }
}

fn failure(class: u32, message: &str) -> Outcome {
    Outcome {
        class,
        result: serde_json::to_vec(&json!({"diagnostics": [{"message": message}]}))
            .unwrap_or_default(),
    }
}
