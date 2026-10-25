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
mod config;
mod engine;
mod lexicon;
pub mod materialize;
mod project;
mod rate;
mod store;
mod verifier;

pub use config::{Config, DRAFT_BLOB_HEADROOM, DRAFT_MAX_BYTES, LOG_TAIL_BYTES, RATE_WINDOW};
pub use engine::{Answer, Call, Unavailable};
pub use verifier::{Job, Outcome, Report, Runner, State, Verifier};

use std::time::Instant;

use gateway_host_api::protocol::{
    attestation_lookup, engine_build, engine_check, engine_format, engine_snapshot, lexicon_read,
    rate_calls, store_get, store_put, verify_control, verify_start, Host, HostError,
};

/// The host: a store, a verifier and a rate counter.
pub struct Native {
    config: Config,
    verifier: Verifier,
    counter: rate::Counter,
}

impl Default for Native {
    fn default() -> Native {
        Native::new(Config::from_environment())
    }
}

impl Native {
    /// A host with this configuration.
    pub fn new(config: Config) -> Native {
        Native {
            config,
            verifier: Verifier::default(),
            counter: rate::Counter::default(),
        }
    }

    /// A host whose verifications are run by `runner`.
    pub fn with_runner(config: Config, runner: Runner) -> Native {
        Native {
            config,
            verifier: Verifier::with_runner(runner),
            counter: rate::Counter::default(),
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
        let found = store::read(
            &self.config.store,
            request.area,
            &request.key,
            self.config.blob_max_bytes,
        );
        match found {
            Ok(content) => Ok(store_get::Completed {
                found: content.is_some(),
                content: content.unwrap_or_default(),
            }),
            Err(store::StoreError::Area) => Err(HostError::Failed(store_get::Failure::AreaUnknown)),
            Err(store::StoreError::Key) => Err(HostError::Failed(store_get::Failure::KeyRefused)),
            Err(store::StoreError::Io) => Err(HostError::Failed(store_get::Failure::Unreadable)),
        }
    }

    fn store_put(
        &mut self,
        request: store_put::Request,
    ) -> Result<store_put::Completed, HostError<store_put::Failure>> {
        let length = store::write(
            &self.config.store,
            request.area,
            &request.key,
            &request.content,
            self.config.blob_max_bytes,
        );
        match length {
            Ok(length) => Ok(store_put::Completed { length }),
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
        let Some(resolved) = project::resolve(
            &self.config.store,
            &request.project,
            self.config.blob_max_bytes,
        ) else {
            return Err(HostError::Failed(verify_start::Failure::ProjectRefused));
        };
        // The verification outlives this call: its directory is kept, and the `builds` area is
        // a cache that the lifecycle of the store drops.
        let directory = resolved.keep();
        let selection = match project::selection(request.selection, &request.files) {
            Ok(selection) => selection,
            Err(_) => return Err(HostError::Failed(verify_start::Failure::ProjectRefused)),
        };
        let (task, state) = self.verifier.start(Job {
            directory,
            selection,
            store: self.config.store.clone(),
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
}

/// The last `LOG_TAIL_BYTES` bytes of a result.
fn tail(mut result: Vec<u8>) -> Vec<u8> {
    if result.len() > LOG_TAIL_BYTES {
        result.drain(..result.len() - LOG_TAIL_BYTES);
    }
    result
}
