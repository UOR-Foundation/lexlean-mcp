//! EH-01 to EH-03: the engine host over a real socket, the program and the service on a thread.

use std::collections::BTreeSet;
use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use camino::Utf8PathBuf;
use gateway_engine_host::client;
use gateway_engine_host::engine::{self, Answer, Call, Unavailable, RESULT_MAXIMUM};
use gateway_engine_host::service::{
    call_name, call_named, from_variables, refusal_name, refusal_named, Refusal, Service,
    StartError, ENGINE_VARIABLE, MAX_REQUEST_BYTES, STORE_VARIABLE,
};
use lexlean::Selection;
use serde_json::{json, Value};

/// The fixture project.
const TINY: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../vectors/fixture/tiny");

const CALLS: [Call; 5] = [
    Call::Check,
    Call::Snapshot,
    Call::Format(false),
    Call::Format(true),
    Call::Build,
];

fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from)
        .expect("a fixture directory")
        .flatten()
    {
        let name = entry.file_name();
        if name == ".lexlean" {
            continue;
        }
        if entry.file_type().expect("a type").is_dir() {
            copy(&entry.path(), &to.join(&name));
        } else {
            std::fs::copy(entry.path(), to.join(&name)).expect("a copy");
        }
    }
}

/// A store below a new directory, a copy of the fixture in it, and a service on a thread.
fn served() -> (tempfile::TempDir, PathBuf, PathBuf) {
    let directory = tempfile::tempdir().expect("a directory");
    let store = directory.path().join("store");
    let project = store.join("drafts").join("tiny");
    copy(Path::new(TINY), &project);
    let socket = directory.path().join("engine.sock");
    Service::new(&store)
        .bind(&socket)
        .expect("a socket")
        .spawn();
    (directory, project, socket)
}

/// One raw line to the socket, and the answer.
fn raw(socket: &Path, line: &[u8]) -> Value {
    let stream = UnixStream::connect(socket).expect("a connection");
    let mut writer = &stream;
    writer.write_all(line).expect("a request");
    let mut answer = Vec::new();
    BufReader::new(&stream)
        .read_until(b'\n', &mut answer)
        .expect("an answer");
    serde_json::from_slice(&answer).expect("JSON")
}

fn refused(refusal: Refusal) -> Value {
    json!({"refused": refusal_name(refusal)})
}

/// The program, with these settings, once it says that it listens.
struct Program {
    child: Child,
}

impl Program {
    fn start(socket: &Path, store: &Path) -> Program {
        let mut child = Command::new(env!("CARGO_BIN_EXE_gateway-engine-host"))
            .env(ENGINE_VARIABLE, socket)
            .env(STORE_VARIABLE, store)
            .stdout(Stdio::piped())
            .spawn()
            .expect("the program");
        let out = child.stdout.take().expect("its output");
        let mut line = String::new();
        BufReader::new(out).read_line(&mut line).expect("a line");
        assert!(line.contains("listening on"), "{line}");
        Program { child }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn the_program_answers_on_a_socket_of_mode_0600_eh_01() {
    let directory = tempfile::tempdir().expect("a directory");
    let store = directory.path().join("store");
    let project = store.join("drafts").join("tiny");
    copy(Path::new(TINY), &project);
    let socket = directory.path().join("engine.sock");
    let _program = Program::start(&socket, &store);
    let found = std::fs::symlink_metadata(&socket).expect("the socket");
    assert!(found.file_type().is_socket());
    assert_eq!(found.permissions().mode() & 0o777, 0o600);
    let answer = client::call(&socket, &project, &Ok(Selection::Entrypoints), Call::Check)
        .expect("an answer");
    assert_eq!(
        answer.class,
        0,
        "{}",
        String::from_utf8_lossy(&answer.result)
    );
}

#[test]
fn the_program_replaces_the_socket_of_an_earlier_run_eh_01() {
    let directory = tempfile::tempdir().expect("a directory");
    let store = directory.path().join("store");
    std::fs::create_dir_all(&store).expect("a store");
    let socket = directory.path().join("engine.sock");
    let earlier = UnixListener::bind(&socket).expect("an earlier socket");
    let _program = Program::start(&socket, &store);
    drop(earlier);
    assert!(UnixStream::connect(&socket).is_ok());
}

#[test]
fn each_call_through_the_socket_is_the_answer_of_the_engine_eh_02() {
    let (directory, project, socket) = served();
    let beside = directory.path().join("store").join("drafts").join("beside");
    for call in CALLS {
        copy(Path::new(TINY), &beside);
        let here = engine::run(&beside, Ok(Selection::Entrypoints), call).expect("an answer");
        let there =
            client::call(&socket, &project, &Ok(Selection::Entrypoints), call).expect("an answer");
        assert_eq!(here, there, "{}", call_name(call));
        std::fs::remove_dir_all(&beside).expect("removed");
    }
}

#[test]
fn each_selection_travels_as_itself_eh_02() {
    let (directory, project, socket) = served();
    let beside = directory.path().join("store").join("drafts").join("beside");
    copy(Path::new(TINY), &beside);
    let file: BTreeSet<Utf8PathBuf> = [Utf8PathBuf::from("src/Tiny.lex.tex")].into();
    let selections: [Result<Selection, String>; 4] = [
        Ok(Selection::Entrypoints),
        Ok(Selection::All),
        Ok(Selection::Files(file)),
        Err("9 is not a selection: 0 entrypoints, 1 all, 2 files".to_string()),
    ];
    for selection in selections {
        let here = engine::run(&beside, selection.clone(), Call::Check).expect("an answer");
        let there = client::call(&socket, &project, &selection, Call::Check).expect("an answer");
        assert_eq!(here, there, "{selection:?}");
    }
}

#[test]
fn a_refusal_of_the_engine_is_an_answer_with_its_class_eh_02() {
    let (_directory, project, socket) = served();
    std::fs::write(project.join("lexlean.toml"), b"not a configuration").expect("spoiled");
    let answer = client::call(&socket, &project, &Ok(Selection::Entrypoints), Call::Check)
        .expect("an answer");
    assert_eq!(
        answer.class,
        2,
        "{}",
        String::from_utf8_lossy(&answer.result)
    );
    let result: Value = serde_json::from_slice(&answer.result).expect("a command result");
    assert_eq!(result["command"], "check");
    assert_eq!(result["exit_code"], 2);
}

#[test]
fn every_refusal_and_call_travels_by_its_name_eh_03() {
    for refusal in [Refusal::Request, Refusal::Directory, Refusal::Engine] {
        assert_eq!(refusal_named(refusal_name(refusal)), Some(refusal));
    }
    assert_eq!(refusal_named("no such refusal"), None);
    let names: BTreeSet<&str> = CALLS.iter().map(|call| call_name(*call)).collect();
    assert_eq!(names.len(), CALLS.len());
    for call in CALLS {
        let back = call_named(call_name(call)).expect("a call");
        assert_eq!(call_name(back), call_name(call));
    }
    assert!(call_named("verify").is_none());
}

#[test]
fn a_directory_outside_the_store_is_refused_eh_03() {
    let (directory, project, socket) = served();
    let outside = directory.path().join("outside");
    copy(Path::new(TINY), &outside);
    let linked = directory.path().join("store").join("drafts").join("linked");
    std::os::unix::fs::symlink(&outside, &linked).expect("a link");
    for place in [
        outside.clone(),
        linked,
        project.join("..").join("..").join("..").join("outside"),
        directory.path().join("store"),
        directory.path().join("absent"),
    ] {
        let request = json!({
            "directory": place,
            "call": "check",
            "selection": {"kind": 0},
        });
        let mut line = serde_json::to_vec(&request).expect("a line");
        line.push(b'\n');
        assert_eq!(
            raw(&socket, &line),
            refused(Refusal::Directory),
            "{}",
            place.display()
        );
    }
}

#[test]
fn a_request_that_is_not_one_is_refused_and_the_next_is_served_eh_03() {
    let (_directory, project, socket) = served();
    let place = project.to_str().expect("a path");
    for line in [
        b"not JSON\n".to_vec(),
        b"{}\n".to_vec(),
        json!({"directory": place, "call": "verify", "selection": {"kind": 0}})
            .to_string()
            .into_bytes(),
        json!({"directory": place, "call": "check", "selection": {"kind": 3}})
            .to_string()
            .into_bytes(),
        json!({"directory": place, "call": "check", "selection": {"kind": 2}})
            .to_string()
            .into_bytes(),
        json!({"directory": place, "call": "check", "selection": {"invalid": 1}})
            .to_string()
            .into_bytes(),
        json!({"directory": 1, "call": "check", "selection": {"kind": 0}})
            .to_string()
            .into_bytes(),
    ] {
        let mut line = line;
        line.push(b'\n');
        assert_eq!(raw(&socket, &line), refused(Refusal::Request));
    }
    assert!(client::call(&socket, &project, &Ok(Selection::Entrypoints), Call::Check).is_ok());
}

#[test]
fn a_request_over_the_bound_is_refused_and_the_next_is_served_eh_03() {
    let (_directory, project, socket) = served();
    // A request that would be answered, were it not too long.
    let mut line = serde_json::to_vec(&json!({
        "directory": project,
        "call": "check",
        "selection": {"kind": 0},
    }))
    .expect("a line");
    line.resize(
        usize::try_from(MAX_REQUEST_BYTES).expect("a size") + 1,
        b' ',
    );
    line.push(b'\n');
    assert_eq!(raw(&socket, &line), refused(Refusal::Request));
    assert!(client::call(&socket, &project, &Ok(Selection::Entrypoints), Call::Check).is_ok());
}

#[test]
fn no_answer_at_the_socket_is_unavailable_eh_03() {
    let directory = tempfile::tempdir().expect("a directory");
    let answer = client::call(
        &directory.path().join("absent.sock"),
        directory.path(),
        &Ok(Selection::Entrypoints),
        Call::Check,
    );
    assert_eq!(answer, Err(Unavailable));
}

/// A socket that answers one request with `line` and closes.
fn answering(directory: &Path, line: Vec<u8>) -> PathBuf {
    let socket = directory.join("answers.sock");
    let listener = UnixListener::bind(&socket).expect("a socket");
    std::thread::spawn(move || {
        let (stream, _) = listener.accept().expect("a connection");
        let mut request = Vec::new();
        BufReader::new(&stream)
            .read_until(b'\n', &mut request)
            .expect("a request");
        let mut writer = &stream;
        let _ = writer.write_all(&line);
    });
    socket
}

#[test]
fn an_answer_that_is_not_one_is_unavailable_eh_03() {
    let result = "A".repeat((RESULT_MAXIMUM + 3) / 3 * 4);
    for line in [
        b"not JSON\n".to_vec(),
        b"{}\n".to_vec(),
        b"".to_vec(),
        json!({"class": 0}).to_string().into_bytes(),
        json!({"class": 0, "result": "!"}).to_string().into_bytes(),
        json!({"class": 3, "result": ""}).to_string().into_bytes(),
        json!({"class": 5, "result": ""}).to_string().into_bytes(),
        json!({"class": 0, "result": result})
            .to_string()
            .into_bytes(),
        refused(Refusal::Request).to_string().into_bytes(),
        refused(Refusal::Directory).to_string().into_bytes(),
        refused(Refusal::Engine).to_string().into_bytes(),
    ] {
        let directory = tempfile::tempdir().expect("a directory");
        let mut line = line;
        line.push(b'\n');
        let socket = answering(directory.path(), line.clone());
        let answer: Result<Answer, Unavailable> = client::call(
            &socket,
            directory.path(),
            &Ok(Selection::Entrypoints),
            Call::Check,
        );
        assert_eq!(answer, Err(Unavailable), "{}", line.len());
    }
}

#[test]
fn a_service_lost_before_it_answers_is_unavailable_eh_03() {
    let directory = tempfile::tempdir().expect("a directory");
    let socket = directory.path().join("lost.sock");
    let listener = UnixListener::bind(&socket).expect("a socket");
    std::thread::spawn(move || {
        let (stream, _) = listener.accept().expect("a connection");
        let mut request = [0_u8; 16];
        let _ = (&stream).read(&mut request);
    });
    let answer = client::call(
        &socket,
        directory.path(),
        &Ok(Selection::Entrypoints),
        Call::Check,
    );
    assert_eq!(answer, Err(Unavailable));
}

#[test]
fn each_setting_the_program_lacks_stops_it_eh_03() {
    let settings = |socket: Option<&str>, store: Option<&str>| {
        from_variables(|name| {
            match name {
                ENGINE_VARIABLE => socket,
                STORE_VARIABLE => store,
                _ => None,
            }
            .map(Into::into)
        })
        .map(|_| ())
    };
    assert_eq!(settings(None, Some("/s")), Err(StartError::NoSocket));
    assert_eq!(settings(Some(""), Some("/s")), Err(StartError::NoSocket));
    assert_eq!(settings(Some("/e"), None), Err(StartError::NoStore));
    assert_eq!(settings(Some("/e"), Some("")), Err(StartError::NoStore));
    assert_eq!(settings(Some("/e"), Some("/s")), Ok(()));

    let directory = tempfile::tempdir().expect("a directory");
    let taken = directory.path().join("taken");
    std::fs::write(&taken, b"a file").expect("a file");
    assert_eq!(
        Service::new(directory.path())
            .bind(&taken)
            .map(|_| ())
            .unwrap_err(),
        StartError::PathTaken
    );
    assert_eq!(std::fs::read(&taken).expect("left there"), b"a file");
    let nowhere = directory.path().join("absent").join("engine.sock");
    assert!(matches!(
        Service::new(directory.path()).bind(&nowhere).map(|_| ()),
        Err(StartError::Bind(_))
    ));

    for (socket, store) in [(None, Some("/s")), (Some("/e"), None)] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_gateway-engine-host"));
        command
            .env_remove(ENGINE_VARIABLE)
            .env_remove(STORE_VARIABLE);
        if let Some(socket) = socket {
            command.env(ENGINE_VARIABLE, socket);
        }
        if let Some(store) = store {
            command.env(STORE_VARIABLE, store);
        }
        let status = command.output().expect("the program").status;
        assert_eq!(status.code(), Some(2));
    }
}
