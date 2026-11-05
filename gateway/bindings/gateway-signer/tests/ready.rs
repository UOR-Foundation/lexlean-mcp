//! DP-05: `gateway-signer ready`, the readiness command the deployment names for the signer.
//!
//! Each test runs the real program twice: once as the service, once as the question.

use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};

use gateway_signer::service::{SIGNER_VARIABLE, SIGNING_KEY_VARIABLE, STORE_VARIABLE};

const PROGRAM: &str = env!("CARGO_BIN_EXE_gateway-signer");

struct Service(Child);

impl Drop for Service {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// The signer of `dir`, listening, with its key at `dir/signing.key`.
fn service(dir: &Path) -> Service {
    let mut child = Command::new(PROGRAM)
        .env_clear()
        .env(SIGNING_KEY_VARIABLE, dir.join("signing.key"))
        .env(SIGNER_VARIABLE, dir.join("signer.sock"))
        .env(STORE_VARIABLE, dir.join("store"))
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

fn write_key(dir: &Path) {
    std::fs::write(dir.join("signing.key"), format!("{}\n", "66".repeat(32))).expect("a key");
}

/// The program run with `arguments` and the socket variable `socket`, if any.
fn ask(socket: Option<&Path>, arguments: &[&str]) -> Output {
    let mut command = Command::new(PROGRAM);
    command.env_clear().args(arguments);
    if let Some(socket) = socket {
        command.env(SIGNER_VARIABLE, socket);
    }
    command.output().expect("the program runs")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn a_signer_that_can_load_its_key_is_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(dir.path());
    let _service = service(dir.path());
    let output = ask(Some(&dir.path().join("signer.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stdout.is_empty() && output.stderr.is_empty());
}

#[test]
fn a_signer_whose_key_is_gone_is_not_ready_and_says_why_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(dir.path());
    let _service = service(dir.path());
    std::fs::remove_file(dir.path().join("signing.key")).expect("the key removed");
    let output = ask(Some(&dir.path().join("signer.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("the signer is not ready: key-missing"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn no_signer_at_the_socket_is_not_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let output = ask(Some(&dir.path().join("signer.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("the signer is not ready: unavailable"),
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
            stderr(&output).contains(&format!("{SIGNER_VARIABLE} is not set")),
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
        assert!(stderr(&output).contains("usage: gateway-signer [ready]"));
    }
}
