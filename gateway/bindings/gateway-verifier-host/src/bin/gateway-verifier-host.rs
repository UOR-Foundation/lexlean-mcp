//! `gateway-verifier-host`: the process that starts one verifier process for each task of the host.
//!
//! It answers lines of JSON on a Unix socket; `gateway_verifier_host::service` describes the protocol.
//! `gateway-verifier-host ready` asks the running one whether it has its verifier program, and is the readiness
//! command of the deployment (DEPLOYMENT.md).

use std::process::ExitCode;

fn main() -> ExitCode {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    match arguments.as_slice() {
        [] => match gateway_verifier_host::service::run_from_environment() {
            Ok(never) => match never {},
            Err(error) => {
                eprintln!("gateway-verifier-host: {error}");
                ExitCode::from(2)
            }
        },
        [word] if word == "ready" => {
            match gateway_verifier_host::client::ready_from_variables(|name| std::env::var_os(name))
            {
                Ok(()) => ExitCode::SUCCESS,
                Err(error) => {
                    eprintln!("gateway-verifier-host ready: {error}");
                    ExitCode::from(1)
                }
            }
        }
        _ => {
            eprintln!("usage: gateway-verifier-host [ready]");
            ExitCode::from(64)
        }
    }
}
