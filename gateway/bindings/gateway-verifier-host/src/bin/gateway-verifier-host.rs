//! `gateway-verifier-host`: the process that starts one verifier process for each task of the host.
//!
//! It answers lines of JSON on a Unix socket; `gateway_verifier_host::service` describes the protocol.

use std::process::ExitCode;

fn main() -> ExitCode {
    match gateway_verifier_host::service::run_from_environment() {
        Ok(never) => match never {},
        Err(error) => {
            eprintln!("gateway-verifier-host: {error}");
            ExitCode::from(2)
        }
    }
}
