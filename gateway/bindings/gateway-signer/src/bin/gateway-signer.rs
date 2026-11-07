//! `gateway-signer`: the process that holds the signing key for the host of the gateway.
//!
//! It answers lines of JSON on a Unix socket; `gateway_signer::service` describes the protocol.
//! `gateway-signer ready` asks the running one whether it can load its key, and is the readiness
//! command of the deployment (DEPLOYMENT.md).

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    match arguments.as_slice() {
        [] => match gateway_signer::service::run_from_environment() {
            Ok(never) => match never {},
            Err(error) => {
                eprintln!("gateway-signer: {error}");
                ExitCode::from(2)
            }
        },
        [word] if word == "ready" => {
            match gateway_signer::client::ready_from_variables(|name| std::env::var_os(name)) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("gateway-signer ready: {error}");
                    ExitCode::from(1)
                }
            }
        }
        _ => {
            eprintln!("usage: gateway-signer [ready]");
            ExitCode::from(64)
        }
    }
}
