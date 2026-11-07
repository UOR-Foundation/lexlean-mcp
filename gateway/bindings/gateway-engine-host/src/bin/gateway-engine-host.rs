//! `gateway-engine-host`: the process that makes the Engine calls for the host of the gateway.
//!
//! It answers lines of JSON on a Unix socket; `gateway_engine_host::service` describes the protocol.
//! `gateway-engine-host ready` asks the running one whether it has its store, and is the readiness
//! command of the deployment (DEPLOYMENT.md).

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    match arguments.as_slice() {
        [] => match gateway_engine_host::service::run_from_environment() {
            Ok(never) => match never {},
            Err(error) => {
                eprintln!("gateway-engine-host: {error}");
                ExitCode::from(2)
            }
        },
        [word] if word == "ready" => {
            match gateway_engine_host::client::ready_from_variables(|name| std::env::var_os(name)) {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("gateway-engine-host ready: {error}");
                    ExitCode::from(1)
                }
            }
        }
        _ => {
            eprintln!("usage: gateway-engine-host [ready]");
            ExitCode::from(64)
        }
    }
}
