//! `gateway-signer`: the process that holds the signing key for the host of the gateway.
//!
//! It answers lines of JSON on a Unix socket; `gateway_signer::service` describes the protocol.

use std::process::ExitCode;

fn main() -> ExitCode {
    match gateway_signer::service::run_from_environment() {
        Ok(never) => match never {},
        Err(error) => {
            eprintln!("gateway-signer: {error}");
            ExitCode::from(2)
        }
    }
}
