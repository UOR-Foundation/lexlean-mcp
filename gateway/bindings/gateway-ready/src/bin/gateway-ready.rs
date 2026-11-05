//! `gateway-ready`: exits 0 when the gateway answers HTTP at the address and port its
//! declaration serves, and 1 with the reason otherwise. It is the readiness command of the gateway
//! in the deployment (DEPLOYMENT.md).

use std::net::SocketAddr;
use std::process::ExitCode;

use gateway_host::surface::{ADDRESS, PORT};

fn main() -> ExitCode {
    if std::env::args_os().len() > 1 {
        eprintln!("usage: gateway-ready");
        return ExitCode::from(64);
    }
    match gateway_ready::probe(SocketAddr::from((ADDRESS, PORT)), gateway_ready::TIMEOUT) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("gateway-ready: {error}");
            ExitCode::from(1)
        }
    }
}
