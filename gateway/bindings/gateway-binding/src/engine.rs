//! The four Engine calls of LexLean on a draft project: check, snapshot, format and build.
//!
//! A call answers with a class and a result. Class 0 is success and the result is the
//! document of the call. Classes 1, 2 and 4 are the answers of a bad draft, a bad
//! configuration or selection, and an over-limit one (`ErrorClass`), and the result lists the
//! diagnostics; they are not failures of the host. A failure of the host is an environment or
//! internal error of the Engine, which the effect declares as `engine-unavailable`.

use std::path::Path;

use camino::Utf8Path;
use lexlean::{
    BuildRequest, CheckRequest, Engine, ErrorClass, FormatRequest, LexLeanError, Selection,
};
use serde_json::{json, Value};

/// The class and result of a call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    /// 0 success; 1 language, 2 configuration, 3 environment, 4 limit, 5 internal.
    pub class: u32,
    /// The document of the call, or the diagnostics.
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

/// The engine could not run at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unavailable;

/// The number of an error class.
pub fn class_number(class: ErrorClass) -> u32 {
    match class {
        ErrorClass::Language => 1,
        ErrorClass::CliOrConfiguration => 2,
        ErrorClass::Environment => 3,
        ErrorClass::SecurityOrLimit => 4,
        ErrorClass::Internal => 5,
    }
}

/// The result of an error: its diagnostics, in canonical JSON.
pub fn diagnostics(error: &LexLeanError) -> Vec<u8> {
    let rendered: Vec<Value> = error
        .diagnostics
        .iter()
        .map(|diagnostic| {
            serde_json::from_str(&diagnostic.to_json().to_canonical_string())
                .unwrap_or_else(|_| json!({"message": diagnostic.message}))
        })
        .collect();
    document(&json!({"diagnostics": rendered}))
}

fn document(value: &Value) -> Vec<u8> {
    serde_json::to_vec(value).unwrap_or_default()
}

fn host_failure(error: &LexLeanError) -> bool {
    matches!(error.class, ErrorClass::Environment | ErrorClass::Internal)
}

fn answered(error: LexLeanError) -> Result<Answer, Unavailable> {
    if host_failure(&error) {
        return Err(Unavailable);
    }
    Ok(Answer {
        class: class_number(error.class),
        result: diagnostics(&error),
    })
}

/// Make `call` on the project in `directory`.
pub fn run(
    directory: &Path,
    selection: Result<Selection, String>,
    call: Call,
) -> Result<Answer, Unavailable> {
    let selection = match selection {
        Ok(selection) => selection,
        Err(message) => {
            return Ok(Answer {
                class: 2,
                result: document(&json!({"diagnostics": [{"message": message}]})),
            })
        }
    };
    let file = directory.join("lexlean.toml");
    let file = Utf8Path::from_path(&file).ok_or(Unavailable)?;
    let engine = match Engine::load(file) {
        Ok(engine) => engine,
        Err(error) => return answered(error),
    };
    let done = |value: Value| Answer {
        class: 0,
        result: document(&value),
    };
    match call {
        Call::Check => match engine.check(CheckRequest { selection }) {
            Ok(set) => Ok(done(json!({
                "source_id": set.source_id.to_hex(),
                "semantic_id": set.semantic_id.to_hex(),
                "modules": set.units.iter().map(|(name, unit)| (name.clone(), json!({
                    "lean_module": unit.summary.lean_module,
                    "declarations": unit.summary.declarations,
                }))).collect::<serde_json::Map<String, Value>>(),
            }))),
            Err(error) => answered(error),
        },
        Call::Snapshot => match engine.snapshot(CheckRequest { selection }) {
            Ok(snapshot) => Ok(Answer {
                class: 0,
                result: serde_json::to_vec(&snapshot).map_err(|_| Unavailable)?,
            }),
            Err(error) => answered(error),
        },
        Call::Format(check_only) => match engine.format(FormatRequest {
            selection,
            check_only,
        }) {
            Ok(set) => Ok(done(json!({
                "check_only": check_only,
                "canonical": set.units.iter().map(|(name, same)| (name.clone(), json!(same)))
                    .collect::<serde_json::Map<String, Value>>(),
            }))),
            Err(error) => answered(error),
        },
        Call::Build => match engine.build(BuildRequest { selection }) {
            Ok(set) => Ok(done(json!({
                "source_id": set.source_id.to_hex(),
                "semantic_id": set.semantic_id.to_hex(),
                "build_id": set.build_id.map(|id| id.to_hex()),
                "modules": set.units.iter().map(|(name, unit)| (name.clone(), json!(unit.artifacts.paths)))
                    .collect::<serde_json::Map<String, Value>>(),
            }))),
            Err(error) => answered(error),
        },
    }
}
