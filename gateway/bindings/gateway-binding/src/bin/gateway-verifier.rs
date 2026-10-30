//! `gateway-verifier`: the process that runs one verification for the host of the gateway.
//!
//! It is started by the host, one for each task, and speaks lines of JSON on its standard input
//! and output; `gateway_binding::child` describes the protocol.

fn main() -> std::process::ExitCode {
    gateway_binding::child::run()
}
