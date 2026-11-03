//! VS-01 to VS-03: the verifier host over a real socket, the program and the service on a thread.
//! The verifier is a script that speaks the protocol of `gateway-verifier` and does what its job
//! says, so that these tests need no compiler.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::fs::{FileTypeExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use gateway_verifier_host::client::{self, Events, Input, Run, Unavailable};
use gateway_verifier_host::service::{
    from_variables, refusal_name, refusal_named, Refusal, Service, StartError, MAX_REQUEST_BYTES,
    PROGRAM_VARIABLE, SIGNING_KEY_VARIABLE, STORE_VARIABLE, VERIFIER_HOST_VARIABLE,
};
use serde_json::{json, Value};

/// A stand-in for `gateway-verifier`: it leads a session, says so and writes what it saw into the
/// directory of the job, then does what the file `behavior` of that directory says.
const FAKE: &str = r#"#!/bin/sh
[ -n "$FAKE_SESSION" ] || exec env FAKE_SESSION=1 setsid "$0" "$@"
printf '{"event":"started","sid":%s}\n' "$$"
IFS= read -r job || exit 1
dir=$(printf '%s' "$job" | sed 's/.*"directory":"\([^"]*\)".*/\1/')
{ echo "job: $job"; env; } > "$dir/seen"
echo "$$" > "$dir/pid"
case "$(cat "$dir/behavior" 2>/dev/null)" in
  verified) printf '{"event":"verified","attestation_id":"a"}\n' ;;
  linger) printf '{"event":"failed","class":1}\n'; while :; do sleep 0.05; done ;;
  cancel) IFS= read -r line; printf '{"event":"cancelled","ended":2,"failed":1,"heard":"%s"}\n' "$line" ;;
  hang) trap '' TERM; while :; do sleep 0.05; done ;;
  until-input-ends) cat > /dev/null; exit 1 ;;
  die) echo boom >&2; exit 3 ;;
  orphan) sleep 600 & echo "$!" > "$dir/orphan"; exit 3 ;;
esac
"#;

const GRACE: Duration = Duration::from_millis(300);

/// A store, a project below it and the fake, all below a new directory.
struct Place {
    directory: tempfile::TempDir,
    store: PathBuf,
    project: PathBuf,
    program: PathBuf,
}

fn place() -> Place {
    let directory = tempfile::tempdir().expect("a directory");
    let store = directory.path().join("store");
    let project = store.join("drafts").join("project");
    std::fs::create_dir_all(&project).expect("a project");
    let program = directory.path().join("fake-verifier");
    std::fs::write(&program, FAKE).expect("a script");
    std::fs::set_permissions(&program, std::fs::Permissions::from_mode(0o755)).expect("a mode");
    Place {
        directory,
        store,
        project,
        program,
    }
}

impl Place {
    /// The service on a thread of this test, with the fake as its program.
    fn served(&self) -> PathBuf {
        let socket = self.directory.path().join("verifier.sock");
        Service::new(&self.store, &self.program)
            .bind(&socket)
            .expect("a socket")
            .spawn();
        socket
    }

    fn run(&self, socket: &Path, behavior: &str) -> Result<(Input, Events), Unavailable> {
        std::fs::write(self.project.join("behavior"), behavior).expect("a behavior");
        client::run(socket, &self.request())
    }

    fn request(&self) -> Run {
        Run {
            width: 3,
            memory_kib: 4096,
            grace: GRACE,
            job: json!({"directory": self.project, "selection": {"kind": 0, "files": []}}),
        }
    }

    fn pid(&self) -> u32 {
        let file = self.project.join("pid");
        let until = Instant::now() + Duration::from_secs(10);
        loop {
            if let Some(pid) = std::fs::read_to_string(&file)
                .ok()
                .and_then(|text| text.trim().parse().ok())
            {
                return pid;
            }
            assert!(Instant::now() < until, "the process did not begin");
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

fn alive(pid: u32) -> bool {
    std::fs::read_to_string(format!("/proc/{pid}/stat"))
        .ok()
        .and_then(|stat| {
            let rest = stat[stat.rfind(')')? + 1..].trim_start().to_string();
            rest.chars().next()
        })
        .is_some_and(|state| state != 'Z')
}

fn gone_within(pid: u32, seconds: u64) -> bool {
    let until = Instant::now() + Duration::from_secs(seconds);
    while Instant::now() < until {
        if !alive(pid) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
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

/// The program, ended with the test, whichever way the test ends.
struct Program(Child);

impl Drop for Program {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn program_line(socket: &Path, store: &Path, program: &Path) -> Program {
    let mut child = Command::new(env!("CARGO_BIN_EXE_gateway-verifier-host"))
        .env(VERIFIER_HOST_VARIABLE, socket)
        .env(STORE_VARIABLE, store)
        .env(PROGRAM_VARIABLE, program)
        .stdout(Stdio::piped())
        .spawn()
        .expect("the program");
    let out = child.stdout.take().expect("its output");
    let mut line = String::new();
    BufReader::new(out).read_line(&mut line).expect("a line");
    assert!(line.contains("listening on"), "{line}");
    Program(child)
}

// VS-01

#[test]
fn the_program_answers_on_a_socket_of_mode_0600_vs_01() {
    let place = place();
    let socket = place.directory.path().join("verifier.sock");
    let _program = program_line(&socket, &place.store, &place.program);
    let found = std::fs::symlink_metadata(&socket).expect("the socket");
    assert!(found.file_type().is_socket());
    assert_eq!(found.permissions().mode() & 0o777, 0o600);
    assert!(client::ready(&socket), "it has a program to start");
    let (input, events) = place.run(&socket, "verified").expect("a task");
    drop(input);
    let last = events.last().expect("an event");
    assert_eq!(last["event"], "ended");
}

#[test]
fn the_program_replaces_the_socket_of_an_earlier_run_vs_01() {
    let place = place();
    let socket = place.directory.path().join("verifier.sock");
    let earlier = UnixListener::bind(&socket).expect("an earlier socket");
    let _program = program_line(&socket, &place.store, &place.program);
    drop(earlier);
    assert!(UnixStream::connect(&socket).is_ok());
}

#[test]
fn ready_says_whether_there_is_a_program_to_start_vs_01() {
    let place = place();
    let socket = place.served();
    assert!(client::ready(&socket));
    for (label, program) in [
        ("a missing file", place.directory.path().join("absent")),
        ("a directory", place.directory.path().to_path_buf()),
    ] {
        let other = place.directory.path().join("other.sock");
        Service::new(&place.store, program)
            .bind(&other)
            .expect("a socket")
            .spawn();
        assert!(!client::ready(&other), "{label}");
    }
    assert!(!client::ready(&place.directory.path().join("absent.sock")));
}

// VS-02

#[test]
fn the_process_gets_the_job_and_the_settings_and_nothing_of_the_key_vs_02() {
    std::env::set_var(SIGNING_KEY_VARIABLE, "/run/secrets/signing.key");
    let place = place();
    let socket = place.served();
    let (input, events) = place.run(&socket, "verified").expect("a task");
    let all: Vec<Value> = events.collect();
    drop(input);
    std::env::remove_var(SIGNING_KEY_VARIABLE);
    let seen = std::fs::read_to_string(place.project.join("seen")).expect("what it saw");
    assert!(seen.contains("LEXLEAN_VERIFY_WIDTH=3\n"), "{seen}");
    assert!(seen.contains("LEXLEAN_VERIFY_MEMORY_KIB=4096\n"), "{seen}");
    assert!(seen.contains("\"selection\""), "the job: {seen}");
    assert!(!seen.contains("signing.key"), "{seen}");
    assert!(!seen.contains(SIGNING_KEY_VARIABLE), "{seen}");
    let events: Vec<&str> = all.iter().filter_map(|e| e["event"].as_str()).collect();
    assert_eq!(events, ["started", "verified", "ended"], "{all:?}");
    assert_eq!(
        all[1]["attestation_id"], "a",
        "the line is passed as it was written"
    );
}

#[test]
fn ended_is_last_and_carries_the_tail_of_a_lost_process_vs_02() {
    let place = place();
    let socket = place.served();
    let (input, events) = place.run(&socket, "die").expect("a task");
    let all: Vec<Value> = events.collect();
    drop(input);
    let last = all.last().expect("an event");
    assert_eq!(last["event"], "ended", "{all:?}");
    assert!(
        last["log"].as_str().is_some_and(|log| log.contains("boom")),
        "{last}"
    );
    assert!(!all.iter().any(|e| e["event"] == "verified"));
}

#[test]
fn two_tasks_run_in_two_processes_at_once_vs_02() {
    let one = place();
    let two = place();
    let socket = one.directory.path().join("shared.sock");
    // One store above both: the service runs below one store.
    let store = one.directory.path().join("shared-store");
    std::fs::create_dir_all(&store).expect("a store");
    let first = store.join("first");
    let second = store.join("second");
    for project in [&first, &second] {
        std::fs::create_dir_all(project).expect("a project");
        std::fs::write(project.join("behavior"), "cancel").expect("a behavior");
    }
    Service::new(&store, &two.program)
        .bind(&socket)
        .expect("a socket")
        .spawn();
    let run = |project: &Path| {
        client::run(
            &socket,
            &Run {
                width: 1,
                memory_kib: 1,
                grace: GRACE,
                job: json!({"directory": project, "selection": {"kind": 0}}),
            },
        )
        .expect("a task")
    };
    let (mut input_one, events_one) = run(&first);
    let (mut input_two, events_two) = run(&second);
    let pid = |project: &Path| {
        let until = Instant::now() + Duration::from_secs(10);
        loop {
            if let Some(pid) = std::fs::read_to_string(project.join("pid"))
                .ok()
                .and_then(|text| text.trim().parse::<u32>().ok())
            {
                return pid;
            }
            assert!(Instant::now() < until);
            std::thread::sleep(Duration::from_millis(10));
        }
    };
    let (pid_one, pid_two) = (pid(&first), pid(&second));
    assert_ne!(pid_one, pid_two, "a process for each task");
    // Cancelling one ends that one, and the other runs on.
    input_one.cancel();
    assert!(gone_within(pid_one, 5));
    assert!(alive(pid_two), "the other task runs on");
    input_two.cancel();
    assert!(gone_within(pid_two, 5));
    for events in [events_one, events_two] {
        let all: Vec<Value> = events.collect();
        assert!(all.iter().any(|e| e["event"] == "cancelled"), "{all:?}");
    }
}

// VS-03

#[test]
fn a_cancel_reaches_the_process_and_its_answer_comes_back_vs_03() {
    let place = place();
    let socket = place.served();
    let (mut input, events) = place.run(&socket, "cancel").expect("a task");
    place.pid();
    input.cancel();
    let all: Vec<Value> = events.collect();
    let answer = all
        .iter()
        .find(|e| e["event"] == "cancelled")
        .expect("an answer");
    assert_eq!(answer["heard"], "cancel");
    assert_eq!(answer["ended"], 2);
    assert_eq!(all.last().expect("an event")["event"], "ended");
}

#[test]
fn kill_ends_a_process_that_ignores_everything_vs_03() {
    let place = place();
    let socket = place.served();
    let (mut input, events) = place.run(&socket, "hang").expect("a task");
    let pid = place.pid();
    assert!(alive(pid));
    input.kill();
    assert!(gone_within(pid, 5), "the process is gone");
    let last = events.last().expect("an event");
    assert_eq!(last["event"], "ended", "{last}");
}

#[test]
fn the_end_of_the_input_ends_the_process_when_the_host_has_gone_vs_03() {
    let place = place();
    let socket = place.served();
    let (input, events) = place.run(&socket, "until-input-ends").expect("a task");
    let pid = place.pid();
    assert!(alive(pid));
    drop(input);
    assert!(gone_within(pid, 5), "its input ended, so it ends");
    drop(events);
}

#[test]
fn a_process_that_ignores_the_end_of_the_input_is_killed_after_the_grace_vs_03() {
    let place = place();
    let socket = place.served();
    let (input, events) = place.run(&socket, "hang").expect("a task");
    let pid = place.pid();
    drop(input);
    drop(events);
    assert!(alive(pid), "it has the grace");
    assert!(gone_within(pid, 5), "and then it is killed");
}

#[test]
fn a_process_that_does_not_end_after_its_verdict_is_killed_after_the_grace_vs_03() {
    let place = place();
    let socket = place.served();
    let (input, events) = place.run(&socket, "linger").expect("a task");
    let pid = place.pid();
    // A service that waits for the process for ever is a failure here, and not a test that hangs.
    let (send, receive) = std::sync::mpsc::channel();
    std::thread::spawn(move || send.send(events.collect::<Vec<Value>>()));
    let all = receive
        .recv_timeout(Duration::from_secs(20))
        .expect("the service ends the task after the grace");
    drop(input);
    assert!(all.iter().any(|e| e["event"] == "failed"), "{all:?}");
    assert_eq!(all.last().expect("an event")["event"], "ended");
    assert!(!alive(pid), "the process is gone before `ended`");
}

#[test]
fn what_a_lost_process_leaves_in_its_session_is_swept_vs_03() {
    let place = place();
    let socket = place.served();
    let (input, events) = place.run(&socket, "orphan").expect("a task");
    let last = events.last().expect("an event");
    drop(input);
    let orphan: u32 = std::fs::read_to_string(place.project.join("orphan"))
        .expect("the orphan")
        .trim()
        .parse()
        .expect("a process ID");
    assert_eq!(last["event"], "ended", "{last}");
    assert!(
        last["swept"].as_u64().is_some_and(|swept| swept >= 1),
        "{last}"
    );
    assert!(gone_within(orphan, 5), "the child that outlived it is gone");
}

#[test]
fn a_child_that_holds_the_pipe_does_not_hold_the_end_of_the_task_vs_03() {
    let place = place();
    let socket = place.served();
    let begun = Instant::now();
    let (input, events) = place.run(&socket, "orphan").expect("a task");
    let all: Vec<Value> = events.collect();
    drop(input);
    assert!(
        begun.elapsed() < Duration::from_secs(30),
        "the end was found by asking the process, in {:?}",
        begun.elapsed()
    );
    assert_eq!(all.last().expect("an event")["event"], "ended");
}

// VS-04

#[test]
fn every_refusal_travels_by_its_name_vs_04() {
    for refusal in [Refusal::Request, Refusal::Directory, Refusal::Program] {
        assert_eq!(refusal_named(refusal_name(refusal)), Some(refusal));
    }
    assert_eq!(refusal_named("no such refusal"), None);
}

#[test]
fn a_directory_outside_the_store_starts_no_process_vs_04() {
    let place = place();
    let socket = place.served();
    let outside = place.directory.path().join("outside");
    std::fs::create_dir_all(&outside).expect("a directory");
    let linked = place.store.join("drafts").join("linked");
    std::os::unix::fs::symlink(&outside, &linked).expect("a link");
    for directory in [
        outside.clone(),
        linked,
        place.store.clone(),
        place.directory.path().join("absent"),
    ] {
        let answer = client::run(
            &socket,
            &Run {
                job: json!({"directory": directory, "selection": {"kind": 0}}),
                ..place.request()
            },
        );
        assert_eq!(
            answer.err(),
            Some(Unavailable::Refused(Refusal::Directory)),
            "{}",
            directory.display()
        );
        assert!(!outside.join("seen").exists());
    }
}

#[test]
fn a_program_that_cannot_start_is_refused_vs_04() {
    let place = place();
    for program in [
        place.directory.path().join("absent"),
        place.directory.path().to_path_buf(),
    ] {
        let socket = place.directory.path().join("no-program.sock");
        Service::new(&place.store, program)
            .bind(&socket)
            .expect("a socket")
            .spawn();
        assert_eq!(
            client::run(&socket, &place.request()).err(),
            Some(Unavailable::Refused(Refusal::Program))
        );
    }
    // A file that the system will not start.
    let socket = place.directory.path().join("not-executable.sock");
    let text = place.directory.path().join("not-executable");
    std::fs::write(&text, "not a program").expect("a file");
    Service::new(&place.store, &text)
        .bind(&socket)
        .expect("a socket")
        .spawn();
    assert_eq!(
        client::run(&socket, &place.request()).err(),
        Some(Unavailable::Refused(Refusal::Program))
    );
}

#[test]
fn a_request_that_is_not_one_is_refused_and_the_next_is_served_vs_04() {
    let place = place();
    let socket = place.served();
    let job = json!({"directory": place.project, "selection": {"kind": 0}});
    for line in [
        b"not JSON\n".to_vec(),
        b"{}\n".to_vec(),
        json!({"op": "verify"}).to_string().into_bytes(),
        json!({"op": "run", "job": job}).to_string().into_bytes(),
        json!({"op": "run", "width": 1, "memory_kib": 1, "grace_ms": 1})
            .to_string()
            .into_bytes(),
        json!({"op": "run", "width": 1, "memory_kib": 1, "grace_ms": 1, "job": {"selection": 0}})
            .to_string()
            .into_bytes(),
    ] {
        let mut line = line;
        line.push(b'\n');
        assert_eq!(raw(&socket, &line), refused(Refusal::Request));
    }
    assert!(client::ready(&socket));
}

#[test]
fn a_request_over_the_bound_is_refused_vs_04() {
    let place = place();
    let socket = place.served();
    // A request that would be answered, were it not too long.
    let mut line = json!({"op": "ready"}).to_string().into_bytes();
    line.resize(
        usize::try_from(MAX_REQUEST_BYTES).expect("a size") + 1,
        b' ',
    );
    line.push(b'\n');
    let stream = UnixStream::connect(&socket).expect("a connection");
    let mut writer = &stream;
    // The service stops reading at the bound and answers, so the end of the line may not be taken.
    let _ = writer.write_all(&line);
    let mut answer = String::new();
    BufReader::new(&stream)
        .read_line(&mut answer)
        .expect("an answer");
    assert_eq!(
        serde_json::from_str::<Value>(&answer).expect("JSON"),
        refused(Refusal::Request)
    );
    assert!(client::ready(&socket));
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
fn an_answer_that_is_not_one_is_lost_vs_04() {
    for line in [
        b"not JSON\n".to_vec(),
        b"{}\n".to_vec(),
        b"".to_vec(),
        b"{\"event\":\"started\"}\n".to_vec(),
        b"{\"refused\":\"no such refusal\"}\n".to_vec(),
        b"{\"refused\":1}\n".to_vec(),
    ] {
        let place = place();
        let socket = answering(place.directory.path(), line.clone());
        assert_eq!(
            client::run(&socket, &place.request()).err(),
            Some(Unavailable::Lost),
            "{}",
            String::from_utf8_lossy(&line)
        );
    }
    let place = place();
    let socket = answering(place.directory.path(), b"{\"ready\":false}\n".to_vec());
    assert!(!client::ready(&socket));
}

#[test]
fn no_answer_at_the_socket_is_lost_vs_04() {
    let place = place();
    assert_eq!(
        client::run(
            &place.directory.path().join("absent.sock"),
            &place.request()
        )
        .err(),
        Some(Unavailable::Lost)
    );
}

#[test]
fn a_service_lost_during_a_task_ends_the_events_without_ended_vs_04() {
    let place = place();
    let socket = answering(
        place.directory.path(),
        b"{\"event\":\"accepted\"}\n{\"event\":\"started\",\"sid\":1}\n".to_vec(),
    );
    let (input, events) = client::run(&socket, &place.request()).expect("a task");
    let all: Vec<Value> = events.collect();
    drop(input);
    assert_eq!(all.len(), 1, "{all:?}");
    assert_eq!(all[0]["event"], "started");
}

#[test]
fn each_setting_the_program_lacks_stops_it_vs_04() {
    let settings = |socket: Option<&str>, store: Option<&str>| {
        from_variables(|name| {
            match name {
                VERIFIER_HOST_VARIABLE => socket,
                STORE_VARIABLE => store,
                _ => None,
            }
            .map(Into::into)
        })
        .map(|_| ())
    };
    assert_eq!(settings(None, Some("/s")), Err(StartError::NoSocket));
    assert_eq!(settings(Some(""), Some("/s")), Err(StartError::NoSocket));
    assert_eq!(settings(Some("/v"), None), Err(StartError::NoStore));
    assert_eq!(settings(Some("/v"), Some("")), Err(StartError::NoStore));
    assert_eq!(settings(Some("/v"), Some("/s")), Ok(()));

    let directory = tempfile::tempdir().expect("a directory");
    let taken = directory.path().join("taken");
    std::fs::write(&taken, b"a file").expect("a file");
    assert_eq!(
        Service::new(directory.path(), directory.path())
            .bind(&taken)
            .map(|_| ())
            .unwrap_err(),
        StartError::PathTaken
    );
    assert_eq!(std::fs::read(&taken).expect("left there"), b"a file");
    let nowhere = directory.path().join("absent").join("verifier.sock");
    assert!(matches!(
        Service::new(directory.path(), directory.path())
            .bind(&nowhere)
            .map(|_| ()),
        Err(StartError::Bind(_))
    ));

    for (socket, store) in [(None, Some("/s")), (Some("/v"), None)] {
        let mut command = Command::new(env!("CARGO_BIN_EXE_gateway-verifier-host"));
        command
            .env_remove(VERIFIER_HOST_VARIABLE)
            .env_remove(STORE_VARIABLE);
        if let Some(socket) = socket {
            command.env(VERIFIER_HOST_VARIABLE, socket);
        }
        if let Some(store) = store {
            command.env(STORE_VARIABLE, store);
        }
        let status = command.output().expect("the program").status;
        assert_eq!(status.code(), Some(2));
    }
}

#[test]
fn a_program_named_by_its_variable_is_the_one_that_starts_vs_01() {
    let place = place();
    let socket = place.directory.path().join("verifier.sock");
    let _program = program_line(&socket, &place.store, &place.program);
    let (input, events) = place.run(&socket, "verified").expect("a task");
    let all: Vec<Value> = events.collect();
    drop(input);
    assert!(all.iter().any(|e| e["event"] == "verified"), "{all:?}");
}
