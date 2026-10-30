//! The host of the LexLean MCP gateway.
//!
//! The core is a pure function that asks for the effects of `src/Effects.lex.tex`: a store,
//! the four calls of the LexLean Engine, a verifier, the lexicon of a profile and the
//! lookup of an attestation. This crate implements each of them for the generated `Host`
//! trait, and a counter for the one effect the adapter asks for itself, the rate of the
//! requests of the client. It holds implementations only: which effect the core asked for,
//! and what to do with the answer, is decided by the generated driver and the core.
//!
//! The state of the host is a directory (the store, see `Config`) and the verifications
//! that run in this process.

mod attestation;
pub mod child;
mod config;
mod engine;
mod evidence;
pub mod identity;
mod lexicon;
pub mod materialize;
pub mod modules;
mod project;
pub mod provision;
mod rate;
pub mod signer;
mod store;
mod sweep;
mod taskhost;
mod verifier;

pub use config::{
    Config, CANCEL_GRACE, DRAFT_BLOB_HEADROOM, DRAFT_MAX_BYTES, LOG_TAIL_BYTES, RATE_WINDOW,
    SIGNING_KEY_VARIABLE, VERIFY_CONCURRENCY, VERIFY_MEMORY_KIB, VERIFY_WIDTH,
};
pub use engine::{Answer, Call, Unavailable};
pub use taskhost::{iso, Clock};
pub use verifier::{conclude, Facts, Job, Outcome, Report, State, Verifier};

use std::sync::Arc;
use std::time::Instant;

use crate::signer::Signer;

use gateway_host_api::protocol::{
    attestation_lookup, clock_now, engine_build, engine_check, engine_format, engine_snapshot,
    lexicon_read, rate_calls, store_get, store_put, task_id, task_store, task_table,
    verify_control, verify_start, Host, HostError,
};

/// The host: a store, a verifier, a rate counter, a clock and the table of tasks.
pub struct Native {
    config: Config,
    verifier: Verifier,
    counter: rate::Counter,
    clock: Clock,
    tasks: Vec<u8>,
}

impl Default for Native {
    fn default() -> Native {
        Native::new(Config::from_environment())
    }
}

impl Native {
    /// A host with this configuration.
    pub fn new(config: Config) -> Native {
        let verifier = Verifier::new(&config);
        Native {
            config,
            verifier,
            counter: rate::Counter::default(),
            clock: Clock::default(),
            tasks: Vec::new(),
        }
    }

    /// The configuration of the host.
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// The rate counter, answered at `now`.
    pub fn count(&mut self, counted: bool, now: Instant) -> u32 {
        self.counter.answer(counted, now, self.config.rate_window)
    }

    /// The counter of milliseconds and the ISO 8601 text of the instant `at`.
    pub fn clock(&self, at: Instant) -> (u64, String) {
        self.clock.at(at)
    }

    fn engine_call(
        &self,
        project: &str,
        selection: u32,
        files: &str,
        call: Call,
    ) -> Result<engine::Answer, EngineRefusal> {
        let resolved = project::resolve(&self.config.store, project, self.config.blob_max_bytes)
            .ok_or(EngineRefusal::Project)?;
        engine::run(
            resolved.directory(),
            project::selection(selection, files),
            call,
        )
        .map_err(|Unavailable| EngineRefusal::Engine)
    }
}

/// The bytes of the file `path` of the draft at `key`, `None` when the draft has no such file,
/// and an error when `key` is not a draft of the drafts area or its bytes are not the draft their
/// name says (the identity is checked as a read of the core checks it).
fn file_of_draft(area: u32, key: &str, blob: &[u8], path: &str) -> Result<Option<Vec<u8>>, ()> {
    use sha2::{Digest, Sha256};
    if area != 1 {
        return Err(());
    }
    let (_, id) = materialize::split_handle(key).ok_or(())?;
    let digest: String = Sha256::digest(blob)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    if digest != id {
        return Err(());
    }
    let (_, rows) = materialize::decode(blob).ok_or(())?;
    Ok(rows
        .into_iter()
        .find(|(name, _)| *name == path.as_bytes())
        .map(|(_, content)| content.to_vec()))
}

/// Why an Engine call did not answer.
enum EngineRefusal {
    Project,
    Engine,
}

impl Host for Native {
    fn store_get(
        &mut self,
        request: store_get::Request,
    ) -> Result<store_get::Completed, HostError<store_get::Failure>> {
        // The view is a closed field: a number outside the model's is refused before the store
        // is read, and the sdk is reached for a view of the model only.
        let Some(view) = modules::View::from_number(request.view) else {
            return Err(HostError::Failed(store_get::Failure::ViewUnknown));
        };
        let found = store::read(
            &self.config.store,
            request.area,
            &request.key,
            self.config.blob_max_bytes,
        );
        let blob = match found {
            Ok(Some(blob)) => blob,
            Ok(None) => {
                return Ok(store_get::Completed {
                    found: false,
                    content: Vec::new(),
                    class: modules::ACCEPTED,
                })
            }
            Err(store::StoreError::Area) => {
                return Err(HostError::Failed(store_get::Failure::AreaUnknown))
            }
            Err(store::StoreError::Key) => {
                return Err(HostError::Failed(store_get::Failure::KeyRefused))
            }
            Err(store::StoreError::Io) => {
                return Err(HostError::Failed(store_get::Failure::Unreadable))
            }
        };
        let selected = if request.path.is_empty() {
            blob
        } else {
            match file_of_draft(request.area, &request.key, &blob, &request.path) {
                Ok(Some(file)) => file,
                Ok(None) => {
                    return Ok(store_get::Completed {
                        found: true,
                        content: Vec::new(),
                        class: modules::ABSENT,
                    })
                }
                Err(()) => return Err(HostError::Failed(store_get::Failure::Unreadable)),
            }
        };
        if view == modules::View::Stored {
            return Ok(store_get::Completed {
                found: true,
                content: selected,
                class: modules::ACCEPTED,
            });
        }
        Ok(
            match modules::view(view, &selected, self.config.blob_max_bytes) {
                Ok(content) => store_get::Completed {
                    found: true,
                    content,
                    class: modules::ACCEPTED,
                },
                Err(message) => store_get::Completed {
                    found: true,
                    content: message.into_bytes(),
                    class: modules::REFUSED,
                },
            },
        )
    }

    fn store_put(
        &mut self,
        request: store_put::Request,
    ) -> Result<store_put::Completed, HostError<store_put::Failure>> {
        // The form is a closed field, checked first for the same reason as the view.
        let Some(form) = modules::Form::from_number(request.form) else {
            return Err(HostError::Failed(store_put::Failure::FormUnknown));
        };
        let limit = self.config.blob_max_bytes;
        let rendered = if form == modules::Form::Plain {
            None
        } else {
            // A key the store would refuse is refused as such, whatever the content says.
            match store::area_directory(&self.config.store, request.area) {
                Err(_) => return Err(HostError::Failed(store_put::Failure::AreaUnknown)),
                Ok(_) if !store::key_is_valid(&request.key) => {
                    return Err(HostError::Failed(store_put::Failure::KeyRefused))
                }
                Ok(_) => {}
            }
            match modules::render(form, &request.content, limit) {
                Ok(text) => Some(text.into_bytes()),
                Err(message) => {
                    return Ok(store_put::Completed {
                        length: 0,
                        class: modules::REFUSED,
                        rendered: message.into_bytes(),
                    })
                }
            }
        };
        let content = rendered.as_deref().unwrap_or(&request.content);
        let length = store::write(
            &self.config.store,
            request.area,
            &request.key,
            content,
            limit,
        );
        match length {
            Ok(length) => Ok(store_put::Completed {
                length,
                class: modules::ACCEPTED,
                rendered: rendered.unwrap_or_default(),
            }),
            Err(store::StoreError::Area) => Err(HostError::Failed(store_put::Failure::AreaUnknown)),
            Err(store::StoreError::Key) => Err(HostError::Failed(store_put::Failure::KeyRefused)),
            Err(store::StoreError::Io) => Err(HostError::Failed(store_put::Failure::Unwritable)),
        }
    }

    fn engine_check(
        &mut self,
        request: engine_check::Request,
    ) -> Result<engine_check::Completed, HostError<engine_check::Failure>> {
        match self.engine_call(
            &request.project,
            request.selection,
            &request.files,
            Call::Check,
        ) {
            Ok(answer) => Ok(engine_check::Completed {
                class: answer.class,
                result: answer.result,
            }),
            Err(EngineRefusal::Project) => {
                Err(HostError::Failed(engine_check::Failure::ProjectRefused))
            }
            Err(EngineRefusal::Engine) => {
                Err(HostError::Failed(engine_check::Failure::EngineUnavailable))
            }
        }
    }

    fn engine_snapshot(
        &mut self,
        request: engine_snapshot::Request,
    ) -> Result<engine_snapshot::Completed, HostError<engine_snapshot::Failure>> {
        match self.engine_call(
            &request.project,
            request.selection,
            &request.files,
            Call::Snapshot,
        ) {
            Ok(answer) => Ok(engine_snapshot::Completed {
                class: answer.class,
                result: answer.result,
            }),
            Err(EngineRefusal::Project) => {
                Err(HostError::Failed(engine_snapshot::Failure::ProjectRefused))
            }
            Err(EngineRefusal::Engine) => Err(HostError::Failed(
                engine_snapshot::Failure::EngineUnavailable,
            )),
        }
    }

    fn engine_format(
        &mut self,
        request: engine_format::Request,
    ) -> Result<engine_format::Completed, HostError<engine_format::Failure>> {
        match self.engine_call(
            &request.project,
            request.selection,
            &request.files,
            Call::Format(request.check_only),
        ) {
            Ok(answer) => Ok(engine_format::Completed {
                class: answer.class,
                result: answer.result,
            }),
            Err(EngineRefusal::Project) => {
                Err(HostError::Failed(engine_format::Failure::ProjectRefused))
            }
            Err(EngineRefusal::Engine) => {
                Err(HostError::Failed(engine_format::Failure::EngineUnavailable))
            }
        }
    }

    fn engine_build(
        &mut self,
        request: engine_build::Request,
    ) -> Result<engine_build::Completed, HostError<engine_build::Failure>> {
        match self.engine_call(
            &request.project,
            request.selection,
            &request.files,
            Call::Build,
        ) {
            Ok(answer) => Ok(engine_build::Completed {
                class: answer.class,
                result: answer.result,
            }),
            Err(EngineRefusal::Project) => {
                Err(HostError::Failed(engine_build::Failure::ProjectRefused))
            }
            Err(EngineRefusal::Engine) => {
                Err(HostError::Failed(engine_build::Failure::EngineUnavailable))
            }
        }
    }

    fn verify_start(
        &mut self,
        request: verify_start::Request,
    ) -> Result<verify_start::Completed, HostError<verify_start::Failure>> {
        // The request is judged before the service: a project that is not one is refused whatever
        // the state of the key (the recorded exchanges of the protocol run with none), and the
        // copy that a draft makes is removed with `resolved` when the verifier is unavailable.
        let Some(resolved) = project::resolve(
            &self.config.store,
            &request.project,
            self.config.blob_max_bytes,
        ) else {
            return Err(HostError::Failed(verify_start::Failure::ProjectRefused));
        };
        let selection = match project::selection(request.selection, &request.files) {
            Ok(selection) => selection,
            Err(_) => return Err(HostError::Failed(verify_start::Failure::ProjectRefused)),
        };
        // A verification that could not be signed is not begun.
        let unavailable = || HostError::Failed(verify_start::Failure::VerifierUnavailable);
        let key = self.config.signing_key.as_deref().ok_or_else(unavailable)?;
        let signer = Signer::load(key, self.config.history_path()).map_err(|_| unavailable())?;
        if !self
            .config
            .verifier_program
            .as_deref()
            .is_some_and(std::path::Path::is_file)
        {
            return Err(unavailable());
        }
        // The verification outlives this call: its directory is kept, and the `builds` area is
        // a cache that the lifecycle of the store drops.
        let directory = resolved.keep();
        let (task, state) = self.verifier.start(Job {
            directory,
            selection,
            store: self.config.store.clone(),
            signer: Arc::new(signer),
        });
        Ok(verify_start::Completed {
            task,
            state: state as u32,
        })
    }

    fn verify_control(
        &mut self,
        request: verify_control::Request,
    ) -> Result<verify_control::Completed, HostError<verify_control::Failure>> {
        let report = if request.action == 1 {
            self.verifier.cancel(&request.task)
        } else {
            self.verifier.status(&request.task)
        };
        match report {
            Some(report) => Ok(verify_control::Completed {
                state: report.state as u32,
                class: report.class,
                result: tail(report.result),
                groups_ended: report.groups_ended,
                groups_failed: report.groups_failed,
            }),
            None => Err(HostError::Failed(verify_control::Failure::UnknownTask)),
        }
    }

    fn lexicon_read(
        &mut self,
        request: lexicon_read::Request,
    ) -> Result<lexicon_read::Completed, HostError<lexicon_read::Failure>> {
        match lexicon::read(&self.config.store, &request.profile, &request.entry) {
            Ok(content) => Ok(lexicon_read::Completed {
                found: content.is_some(),
                content: content.unwrap_or_default(),
            }),
            Err(lexicon::Refused::Profile) => {
                Err(HostError::Failed(lexicon_read::Failure::ProfileUnknown))
            }
            Err(lexicon::Refused::Key) => Err(HostError::Failed(lexicon_read::Failure::KeyRefused)),
        }
    }

    fn attestation_lookup(
        &mut self,
        request: attestation_lookup::Request,
    ) -> Result<attestation_lookup::Completed, HostError<attestation_lookup::Failure>> {
        match attestation::lookup(&self.config.store, request.by, &request.value) {
            Ok(envelope) => Ok(attestation_lookup::Completed {
                found: envelope.is_some(),
                envelope: envelope.unwrap_or_default(),
            }),
            Err(attestation::Refused::Value) => {
                Err(HostError::Failed(attestation_lookup::Failure::ValueRefused))
            }
            Err(attestation::Refused::Store) => Err(HostError::Fault),
        }
    }

    fn rate_calls(
        &mut self,
        request: rate_calls::Request,
    ) -> Result<rate_calls::Completed, HostError<rate_calls::Failure>> {
        Ok(rate_calls::Completed {
            calls: self.count(request.counted, Instant::now()),
        })
    }

    fn clock_now(
        &mut self,
        _request: clock_now::Request,
    ) -> Result<clock_now::Completed, HostError<clock_now::Failure>> {
        let (ms, iso) = self.clock(Instant::now());
        Ok(clock_now::Completed { ms, iso })
    }

    fn task_id(
        &mut self,
        _request: task_id::Request,
    ) -> Result<task_id::Completed, HostError<task_id::Failure>> {
        taskhost::fresh_id()
            .map(|id| task_id::Completed { id })
            .ok_or(HostError::Fault)
    }

    fn task_table(
        &mut self,
        _request: task_table::Request,
    ) -> Result<task_table::Completed, HostError<task_table::Failure>> {
        Ok(task_table::Completed {
            table: self.tasks.clone(),
        })
    }

    fn task_store(
        &mut self,
        request: task_store::Request,
    ) -> Result<task_store::Completed, HostError<task_store::Failure>> {
        self.tasks = request.table;
        Ok(task_store::Completed {})
    }
}

/// The last `LOG_TAIL_BYTES` bytes of a result.
fn tail(mut result: Vec<u8>) -> Vec<u8> {
    if result.len() > LOG_TAIL_BYTES {
        result.drain(..result.len() - LOG_TAIL_BYTES);
    }
    result
}
