//! DP-05: `gateway-verifier-host ready`, the readiness command the deployment names for the
//! verifier host.
//!
//! Each test runs the real program twice: once as the service, once as the question.

use std::io::{BufRead, BufReader};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};

use gateway_verifier_host::service::{PROGRAM_VARIABLE, STORE_VARIABLE, VERIFIER_HOST_VARIABLE};

const PROGRAM: &str = env!("CARGO_BIN_EXE_gateway-verifier-host");

struct Service(Child);

impl Drop for Service {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// The verifier host of `dir`, listening, whose verifier program is `dir/verifier`.
fn service(dir: &Path) -> Service {
    std::fs::create_dir_all(dir.join("store")).expect("a store");
    let mut child = Command::new(PROGRAM)
        .env_clear()
        .env(VERIFIER_HOST_VARIABLE, dir.join("verifier.sock"))
        .env(STORE_VARIABLE, dir.join("store"))
        .env(PROGRAM_VARIABLE, dir.join("verifier"))
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("the service starts");
    let mut line = String::new();
    BufReader::new(child.stdout.take().expect("its output"))
        .read_line(&mut line)
        .expect("a line");
    assert!(line.contains("listening"), "{line}");
    Service(child)
}

fn write_program(dir: &Path) {
    let program = dir.join("verifier");
    std::fs::write(&program, "#!/bin/sh\nexit 0\n").expect("a program");
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("a mode");
}

/// The program run with `arguments` and the socket variable `socket`, if any.
fn ask(socket: Option<&Path>, arguments: &[&str]) -> Output {
    let mut command = Command::new(PROGRAM);
    command.env_clear().args(arguments);
    if let Some(socket) = socket {
        command.env(VERIFIER_HOST_VARIABLE, socket);
    }
    command.output().expect("the program runs")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn a_verifier_host_with_its_program_is_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    write_program(dir.path());
    let _service = service(dir.path());
    let output = ask(Some(&dir.path().join("verifier.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stdout.is_empty() && output.stderr.is_empty());
}

#[test]
fn a_verifier_host_without_its_program_is_not_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let _service = service(dir.path());
    let output = ask(Some(&dir.path().join("verifier.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("the verifier host is not ready"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn no_verifier_host_at_the_socket_is_not_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let output = ask(Some(&dir.path().join("verifier.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("the verifier host is not ready"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn ready_without_the_socket_variable_says_so_dp_05() {
    for socket in [None, Some(Path::new(""))] {
        let output = ask(socket, &["ready"]);
        assert_eq!(output.status.code(), Some(1));
        assert!(
            stderr(&output).contains(&format!("{VERIFIER_HOST_VARIABLE} is not set")),
            "{}",
            stderr(&output)
        );
    }
}

#[test]
fn any_other_argument_is_a_usage_error_dp_05() {
    for arguments in [&["readyy"][..], &["ready", "now"][..], &["--ready"][..]] {
        let output = ask(None, arguments);
        assert_eq!(output.status.code(), Some(64), "{arguments:?}");
        assert!(stderr(&output).contains("usage: gateway-verifier-host [ready]"));
    }
}
