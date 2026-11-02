//! `gateway-engine-host`: the process that makes the Engine calls for the host of the gateway.
//!
//! It answers lines of JSON on a Unix socket; `gateway_engine_host::service` describes the protocol.

use std::process::ExitCode;

fn main() -> ExitCode {
    match gateway_engine_host::service::run_from_environment() {
        Ok(never) => match never {},
        Err(error) => {
            eprintln!("gateway-engine-host: {error}");
            ExitCode::from(2)
        }
    }
}
