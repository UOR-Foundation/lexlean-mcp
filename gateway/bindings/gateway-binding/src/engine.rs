//! The four Engine calls of LexLean on a draft project: check, snapshot, format and build.
//!
//! A call answers with a class and a result. The result is what the CLI prints for the same
//! command with `--diagnostic-format json` (SPEC 23.7): the `lexlean/command-result/1` object,
//! made by the function the CLI calls (`lexlean::api::command_result_json`) from the same
//! `Engine` result, in the same bytes. The one exception is the snapshot of a project that
//! checks: the CLI has no `snapshot` command, and its result is the canonical bytes of the
//! semantic snapshot of the Engine. Class 0 is success. Classes 1, 2 and 4 are the answers of a
//! bad draft, a bad configuration or selection, and an over-limit one (`ErrorClass`): the class
//! is the exit code of the CLI, and the result lists the diagnostics. They are not failures of
//! the host. A failure of the host is an environment or internal error of the Engine, which the
//! effect declares as `engine-unavailable`.
//!
//! A snapshot that is refused gives the command result of `check` under the command name
//! `snapshot`: SPEC 24.1 has `snapshot` run the pipeline of `check`, so its refusals are
//! `check`'s.

use std::collections::BTreeSet;
use std::path::Path;

use camino::Utf8Path;
use lexlean::api::{command_result_json, CommandIds};
use lexlean::{
    code, BuildRequest, CheckRequest, Diagnostic, Engine, ErrorClass, FormatRequest, LexLeanError,
    Selection,
};

/// The class and result of a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    /// 0 success; 1 language, 2 configuration or selection, 4 limit: the exit code of the CLI.
    pub class: u32,
    /// The command result of the call, or the canonical snapshot.
    pub result: Vec<u8>,
}

/// The call to make.
#[derive(Debug, Clone, Copy)]
pub enum Call {
    /// `Engine::check`.
    Check,
    /// `Engine::snapshot`.
    Snapshot,
    /// `Engine::format`, with its `check_only`.
    Format(bool),
    /// `Engine::build`.
    Build,
}

impl Call {
    /// The command the result names: the CLI's own (`fmt` for format), and `snapshot`.
    fn command(self) -> &'static str {
        match self {
            Call::Check => "check",
            Call::Snapshot => "snapshot",
            Call::Format(_) => "fmt",
            Call::Build => "build",
        }
    }
}

/// The engine could not run at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unavailable;

/// The number of an error class. The classes of an answer of the four Engine calls, 1, 2 and 4,
/// are the exit codes of the CLI (SPEC 24.5); an environment or internal error is not an answer
/// of those calls (it is the failure `engine-unavailable`), but it is one of the verifier.
pub fn class_number(class: ErrorClass) -> u32 {
    match class {
        ErrorClass::Language => 1,
        ErrorClass::CliOrConfiguration => 2,
        ErrorClass::Environment => 3,
        ErrorClass::SecurityOrLimit => 4,
        ErrorClass::Internal => 5,
    }
}

/// The result of an error of the verifier: its diagnostics, in canonical JSON.
pub fn diagnostics(error: &LexLeanError) -> Vec<u8> {
    let rendered: Vec<serde_json::Value> = error
        .diagnostics
        .iter()
        .map(|diagnostic| {
            serde_json::from_str(&diagnostic.to_json().to_canonical_string())
                .unwrap_or_else(|_| serde_json::json!({"message": diagnostic.message}))
        })
        .collect();
    serde_json::to_vec(&serde_json::json!({"diagnostics": rendered})).unwrap_or_default()
}

/// What a command reports besides its diagnostics (the `Outcome` of the CLI).
#[derive(Default)]
struct Outcome {
    modules: BTreeSet<String>,
    artifacts: Vec<String>,
    ids: CommandIds,
}

/// The command result of `call`: the bytes the CLI prints with `--diagnostic-format json`.
fn command_result(
    call: Call,
    exit_code: i32,
    outcome: &Outcome,
    diagnostics: &[Diagnostic],
) -> Vec<u8> {
    command_result_json(
        call.command(),
        exit_code,
        &outcome.modules,
        &outcome.artifacts,
        diagnostics,
        &outcome.ids,
        None,
    )
    .to_file_bytes()
}

fn host_failure(error: &LexLeanError) -> bool {
    matches!(error.class, ErrorClass::Environment | ErrorClass::Internal)
}

fn answered(call: Call, error: &LexLeanError) -> Result<Answer, Unavailable> {
    if host_failure(error) {
        return Err(Unavailable);
    }
    Ok(Answer {
        class: class_number(error.class),
        result: command_result(
            call,
            error.class.exit_code(),
            &Outcome::default(),
            &error.diagnostics,
        ),
    })
}

fn succeeded(call: Call, outcome: &Outcome) -> Answer {
    Answer {
        class: 0,
        result: command_result(call, 0, outcome, &[]),
    }
}

/// The largest result of a call, in bytes: the declaration's `resultMaximum` of the four
/// Engine effects leaves room for it and the three words around it.
pub const RESULT_MAXIMUM: usize = 4_194_304;

/// Make `call` on the project in `directory`. A result over `RESULT_MAXIMUM` is the answer
/// of class 4 (limit) that says so, so it never exceeds what the effect declares.
pub fn run(
    directory: &Path,
    selection: Result<Selection, String>,
    call: Call,
) -> Result<Answer, Unavailable> {
    let answer = make(directory, selection, call)?;
    if answer.result.len() <= RESULT_MAXIMUM {
        return Ok(answer);
    }
    let error = LexLeanError::from_diagnostic(Diagnostic::new(
        code!("LLS8002"),
        format!(
            "the result is {} bytes, over the limit of {RESULT_MAXIMUM}",
            answer.result.len()
        ),
    ));
    answered(call, &error)
}

fn make(
    directory: &Path,
    selection: Result<Selection, String>,
    call: Call,
) -> Result<Answer, Unavailable> {
    let selection = match selection {
        Ok(selection) => selection,
        Err(message) => {
            let error = LexLeanError::from_diagnostic(Diagnostic::new(code!("LLC0002"), message));
            return answered(call, &error);
        }
    };
    let file = directory.join("lexlean.toml");
    let file = Utf8Path::from_path(&file).ok_or(Unavailable)?;
    let engine = match Engine::load(file) {
        Ok(engine) => engine,
        Err(error) => return answered(call, &error),
    };
    match call {
        Call::Check => match engine.check(CheckRequest { selection }) {
            Ok(set) => Ok(succeeded(
                call,
                &Outcome {
                    modules: set.units.keys().cloned().collect(),
                    ids: CommandIds {
                        source_id: Some(set.source_id),
                        semantic_id: Some(set.semantic_id),
                        ..CommandIds::default()
                    },
                    ..Outcome::default()
                },
            )),
            Err(error) => answered(call, &error),
        },
        Call::Snapshot => match engine.snapshot(CheckRequest { selection }) {
            Ok(snapshot) => Ok(Answer {
                class: 0,
                result: snapshot.canonical_bytes(),
            }),
            Err(error) => answered(call, &error),
        },
        Call::Format(check_only) => match engine.format(FormatRequest {
            selection,
            check_only,
        }) {
            Ok(set) => Ok(succeeded(
                call,
                &Outcome {
                    modules: set.units.keys().cloned().collect(),
                    ..Outcome::default()
                },
            )),
            Err(error) => answered(call, &error),
        },
        Call::Build => match engine.build(BuildRequest { selection }) {
            Ok(set) => {
                let mut outcome = Outcome {
                    modules: set.units.keys().cloned().collect(),
                    ids: CommandIds {
                        source_id: Some(set.source_id),
                        semantic_id: Some(set.semantic_id),
                        build_id: set.build_id,
                        ..CommandIds::default()
                    },
                    ..Outcome::default()
                };
                if let Some(id) = set.build_id {
                    outcome.artifacts.push(format!(
                        "{}/build/{}",
                        build_root(directory)?,
                        id.to_hex()
                    ));
                }
                Ok(succeeded(call, &outcome))
            }
            Err(error) => answered(call, &error),
        },
    }
}

/// The `build_root` of the project, as its configuration writes it: the CLI names the build
/// by it, and the `Engine` does not hand it out.
fn build_root(directory: &Path) -> Result<String, Unavailable> {
    let bytes = std::fs::read(directory.join("lexlean.toml")).map_err(|_| Unavailable)?;
    lexlean::config::parse_project("lexlean.toml", &bytes)
        .map(|config| config.build_root)
        .map_err(|_| Unavailable)
}
