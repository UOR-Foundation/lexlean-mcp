//! DP-05: `gateway-engine-host ready`, the readiness command the deployment names for the engine
//! host.
//!
//! Each test runs the real program twice: once as the service, once as the question.

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};

use gateway_engine_host::service::{ENGINE_VARIABLE, STORE_VARIABLE};

const PROGRAM: &str = env!("CARGO_BIN_EXE_gateway-engine-host");

struct Service(Child);

impl Drop for Service {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

/// The engine host of `dir`, listening, with its store at `dir/store`.
fn service(dir: &Path) -> Service {
    let mut child = Command::new(PROGRAM)
        .env_clear()
        .env(ENGINE_VARIABLE, dir.join("engine.sock"))
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

/// The program run with `arguments` and the socket variable `socket`, if any.
fn ask(socket: Option<&Path>, arguments: &[&str]) -> Output {
    let mut command = Command::new(PROGRAM);
    command.env_clear().args(arguments);
    if let Some(socket) = socket {
        command.env(ENGINE_VARIABLE, socket);
    }
    command.output().expect("the program runs")
}

fn stderr(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn an_engine_host_with_its_store_is_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    std::fs::create_dir(dir.path().join("store")).expect("a store");
    let _service = service(dir.path());
    let output = ask(Some(&dir.path().join("engine.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(0), "{}", stderr(&output));
    assert!(output.stdout.is_empty() && output.stderr.is_empty());
}

#[test]
fn the_service_answers_the_ready_request_on_its_socket_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    std::fs::create_dir(dir.path().join("store")).expect("a store");
    let _service = service(dir.path());
    let mut stream = UnixStream::connect(dir.path().join("engine.sock")).expect("a connection");
    stream
        .write_all(b"{\"op\":\"ready\"}\n")
        .expect("the request");
    let mut answer = String::new();
    stream.read_to_string(&mut answer).expect("an answer");
    assert_eq!(answer, "{\"ready\":true}\n");
}

#[test]
fn an_engine_host_without_its_store_is_not_ready_and_says_why_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let _service = service(dir.path());
    let output = ask(Some(&dir.path().join("engine.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("the engine host is not ready: directory-refused"),
        "{}",
        stderr(&output)
    );
}

#[test]
fn no_engine_host_at_the_socket_is_not_ready_dp_05() {
    let dir = tempfile::tempdir().expect("a directory");
    let output = ask(Some(&dir.path().join("engine.sock")), &["ready"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(
        stderr(&output).contains("the engine host is not ready: unavailable"),
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
            stderr(&output).contains(&format!("{ENGINE_VARIABLE} is not set")),
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
        assert!(stderr(&output).contains("usage: gateway-engine-host [ready]"));
    }
}
