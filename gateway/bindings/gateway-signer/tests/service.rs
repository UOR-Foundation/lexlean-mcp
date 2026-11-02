//! SG-16 to SG-18: the program `gateway-signer`, over its socket.
//!
//! Each test runs the real program unless it says otherwise, so the key file, the socket and the
//! history are the ones an operator has. What the host does with the service is in the tests of the
//! binding (`records.rs`).

use std::io::{BufRead, BufReader, Read, Write};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

use gateway_signer::client::SignerClient;
use gateway_signer::service::{
    refusal_name, refusal_named, MAX_REQUEST_BYTES, SIGNER_VARIABLE, SIGNING_KEY_VARIABLE,
    STORE_VARIABLE,
};
use gateway_signer::signer::{
    self, envelope_keyid, HistoryError, KeyError, RecordSigner, SignError, Signer,
};
use serde_json::{json, Value};

const SEED_A: [u8; 32] = [0x66; 32];
const SEED_B: [u8; 32] = [0x77; 32];
const SUBJECT: &[u8] = br#"{"attestation_id":"00ff","status":"verified"}"#;

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn predicate() -> Value {
    json!({"status": "verified", "modules": ["Tiny"], "files": ["00"]})
}

fn write_key(path: &Path, seed: [u8; 32]) {
    std::fs::write(path, format!("{}\n", hex(&seed))).expect("a key file");
}

/// The program, ended with the test.
struct Program {
    child: Child,
    socket: PathBuf,
    key: PathBuf,
    history: PathBuf,
}

impl Drop for Program {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// The program with the key file of `dir`, started and listening.
fn program(dir: &Path) -> Program {
    let key = dir.join("signing.key");
    let socket = dir.join("signer.sock");
    let store = dir.join("store");
    let mut child = Command::new(env!("CARGO_BIN_EXE_gateway-signer"))
        .env_clear()
        .env(SIGNING_KEY_VARIABLE, &key)
        .env(SIGNER_VARIABLE, &socket)
        .env(STORE_VARIABLE, &store)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("the program starts");
    let mut line = String::new();
    BufReader::new(child.stdout.take().expect("its output"))
        .read_line(&mut line)
        .expect("a line");
    assert!(line.contains("listening"), "{line}");
    Program {
        child,
        socket,
        key,
        history: store.join("keys").join("history"),
    }
}

/// A request as a line, and the answer as JSON.
fn raw(socket: &Path, line: &[u8]) -> Value {
    let mut stream = UnixStream::connect(socket).expect("a connection");
    stream.write_all(line).expect("the request");
    stream.write_all(b"\n").expect("the end of the line");
    let mut answer = String::new();
    BufReader::new(&stream)
        .read_line(&mut answer)
        .expect("an answer");
    serde_json::from_str(&answer).expect("JSON")
}

// SG-16

#[test]
fn the_program_answers_on_a_socket_of_mode_0600_sg_16() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let program = program(dir.path());
    let mode = std::fs::metadata(&program.socket)
        .expect("the socket")
        .permissions()
        .mode();
    assert_eq!(mode & 0o777, 0o600, "only this user reaches the signer");
    let answer = raw(&program.socket, br#"{"op":"key"}"#);
    let expected = Signer::from_seed(SEED_A, program.history.clone());
    assert_eq!(answer["keyid"], expected.keyid());
}

#[test]
fn the_program_replaces_the_socket_of_an_earlier_run_sg_16() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let socket = dir.path().join("signer.sock");
    drop(UnixListener::bind(&socket).expect("what an earlier run left"));
    assert!(socket.exists(), "the socket outlives its listener");
    let program = program(dir.path());
    assert!(SignerClient::ready(&program.socket).is_ok());
}

// SG-17

#[test]
fn an_envelope_through_the_socket_is_the_envelope_of_the_signer_sg_17() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let program = program(dir.path());
    let client = SignerClient::ready(&program.socket).expect("the signer is ready");
    let envelope = client
        .sign_record(SUBJECT, &predicate())
        .expect("an envelope");

    let other = tempfile::tempdir().expect("a directory");
    let beside = Signer::from_seed(SEED_A, other.path().join("history"))
        .sign(SUBJECT, &predicate(), 1)
        .expect("the same envelope");
    assert_eq!(envelope, beside, "the time of a signature is not in it");

    let history = signer::read_history(&program.history).expect("the program wrote the history");
    assert_eq!(history.len(), 1);
    signer::validate(&envelope, &history, SUBJECT).expect("the envelope validates");
    assert!(history[0].not_before > 0, "the clock of the program");
}

#[test]
fn a_key_file_put_right_or_rotated_is_used_at_the_next_request_sg_17() {
    let dir = tempfile::tempdir().expect("a directory");
    let program = program(dir.path());
    assert_eq!(
        SignerClient::ready(&program.socket).map(|_| ()),
        Err(SignError::Key(KeyError::Missing)),
        "the program starts before its key exists"
    );

    write_key(&program.key, SEED_A);
    let client = SignerClient::ready(&program.socket).expect("no restart");
    let first = client.sign_record(SUBJECT, &predicate()).expect("first");

    write_key(&program.key, SEED_B);
    let second = client.sign_record(SUBJECT, &predicate()).expect("second");

    let keyid = |seed| {
        Signer::from_seed(seed, dir.path().join("unused"))
            .keyid()
            .to_owned()
    };
    assert_eq!(envelope_keyid(&first), Some(keyid(SEED_A)));
    assert_eq!(envelope_keyid(&second), Some(keyid(SEED_B)), "the new key");
    let history = signer::read_history(&program.history).expect("the history");
    assert_eq!(history.len(), 2, "a rotation adds an entry");
    signer::validate(&first, &history, SUBJECT).expect("the first stays valid");
    signer::validate(&second, &history, SUBJECT).expect("the second validates");
}

// SG-18

#[test]
fn every_refusal_travels_by_its_name_sg_18() {
    let every = [
        SignError::History(HistoryError::Unreadable),
        SignError::History(HistoryError::Unwritable),
        SignError::History(HistoryError::Malformed),
        SignError::History(HistoryError::UnknownKey),
        SignError::KeyRevoked,
        SignError::Key(KeyError::Missing),
        SignError::Key(KeyError::Unreadable),
        SignError::Key(KeyError::Malformed),
        SignError::RequestRefused,
        SignError::Unavailable,
    ];
    let mut names = Vec::new();
    for error in every {
        let name = refusal_name(error);
        assert_eq!(refusal_named(name), Some(error), "{name}");
        names.push(name);
    }
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), every.len(), "each refusal has its own name");
    assert_eq!(refusal_named("who"), None);
}

#[test]
fn each_fault_of_the_key_file_comes_back_as_itself_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    let program = program(dir.path());
    let ready = || SignerClient::ready(&program.socket).map(|_| ());
    assert_eq!(ready(), Err(SignError::Key(KeyError::Missing)));
    std::fs::write(&program.key, "not a key").expect("a file");
    assert_eq!(ready(), Err(SignError::Key(KeyError::Malformed)));
    std::fs::remove_file(&program.key).expect("removed");
    std::fs::create_dir(&program.key).expect("a directory in its place");
    assert_eq!(ready(), Err(SignError::Key(KeyError::Unreadable)));
}

#[test]
fn a_revoked_key_is_refused_through_the_socket_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let program = program(dir.path());
    let client = SignerClient::ready(&program.socket).expect("ready");
    client.sign_record(SUBJECT, &predicate()).expect("signed");
    let keyid = Signer::from_seed(SEED_A, program.history.clone())
        .keyid()
        .to_owned();
    signer::revoke(&program.history, &keyid, 2).expect("revoked");
    assert_eq!(
        client.sign_record(SUBJECT, &predicate()),
        Err(SignError::KeyRevoked)
    );
}

#[test]
fn a_history_that_cannot_be_written_is_refused_through_the_socket_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let program = program(dir.path());
    // A file where the directory of the history belongs.
    let keys = program.history.parent().expect("the keys area");
    std::fs::create_dir_all(keys.parent().expect("the store")).expect("the store");
    std::fs::write(keys, b"in the way").expect("a file");
    let client = SignerClient::ready(&program.socket).expect("the key loads");
    assert!(matches!(
        client.sign_record(SUBJECT, &predicate()),
        Err(SignError::History(_))
    ));
}

#[test]
fn a_request_that_is_not_one_is_refused_and_the_next_is_served_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let program = program(dir.path());
    let refused = [
        &b"nonsense"[..],
        br#"{"op":"other"}"#,
        br#"{"op":"sign","predicate":{}}"#,
        br#"{"op":"sign","subject":"*not base64*","predicate":{}}"#,
        br#"{"op":"sign","subject":"AA=="}"#,
    ];
    for line in refused {
        assert_eq!(
            raw(&program.socket, line),
            json!({"refused": "request-refused"}),
            "{}",
            String::from_utf8_lossy(line)
        );
    }
    assert!(!program.history.exists(), "nothing was signed");
    assert!(SignerClient::ready(&program.socket).is_ok());
}

#[test]
fn a_request_over_the_bound_is_refused_and_the_next_is_served_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    write_key(&dir.path().join("signing.key"), SEED_A);
    let program = program(dir.path());
    let client = SignerClient::ready(&program.socket).expect("ready");
    let subject = vec![b'x'; usize::try_from(MAX_REQUEST_BYTES).expect("a size")];
    assert_eq!(
        client.sign_record(&subject, &predicate()),
        Err(SignError::RequestRefused)
    );
    assert!(!program.history.exists(), "nothing was signed");
    client
        .sign_record(SUBJECT, &predicate())
        .expect("the next request");
}

#[test]
fn no_answer_at_the_socket_is_unavailable_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    assert_eq!(
        SignerClient::ready(dir.path().join("absent")).map(|_| ()),
        Err(SignError::Unavailable)
    );
}

#[test]
fn an_answer_that_is_not_one_is_unavailable_sg_18() {
    for answer in [
        &b""[..],
        b"nonsense\n",
        b"{\"keyid\":3}\n",
        b"{\"refused\":\"who\"}\n",
        b"{\"refused\":3}\n",
    ] {
        let dir = tempfile::tempdir().expect("a directory");
        let socket = dir.path().join("not-a-signer");
        let listener = UnixListener::bind(&socket).expect("a socket");
        let replies = std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().expect("a connection");
            let mut line = String::new();
            BufReader::new(&stream)
                .read_line(&mut line)
                .expect("a line");
            stream.write_all(answer).expect("the answer");
        });
        assert_eq!(
            SignerClient::ready(&socket).map(|_| ()),
            Err(SignError::Unavailable),
            "{}",
            String::from_utf8_lossy(answer)
        );
        replies.join().expect("the thread");
    }
}

#[test]
fn an_envelope_that_is_not_one_is_unavailable_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    let socket = dir.path().join("not-a-signer");
    let listener = UnixListener::bind(&socket).expect("a socket");
    let replies = std::thread::spawn(move || {
        for answer in [&b"{\"keyid\":\"00\"}\n"[..], b"{\"envelope\":\"*\"}\n"] {
            let (mut stream, _) = listener.accept().expect("a connection");
            let mut line = String::new();
            BufReader::new(&stream)
                .read_line(&mut line)
                .expect("a line");
            stream.write_all(answer).expect("the answer");
        }
    });
    let client = SignerClient::ready(&socket).expect("it says it is ready");
    assert_eq!(
        client.sign_record(SUBJECT, &predicate()),
        Err(SignError::Unavailable)
    );
    replies.join().expect("the thread");
}

/// A way to start the program wrongly: its name, its variables, and what it must say.
type StartCase<'a> = (&'a str, Vec<(&'a str, &'a Path)>, &'a str);

/// The program started with `variables`: its exit status and what it said.
fn refused_start(variables: &[(&str, &Path)]) -> (Option<i32>, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_gateway-signer"))
        .env_clear()
        .envs(variables.iter().copied())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the program starts");
    let mut said = String::new();
    child
        .stderr
        .take()
        .expect("its errors")
        .read_to_string(&mut said)
        .expect("text");
    (child.wait().expect("it ends").code(), said)
}

#[test]
fn each_start_error_stops_the_program_with_status_2_sg_18() {
    let dir = tempfile::tempdir().expect("a directory");
    let key = dir.path().join("signing.key");
    write_key(&key, SEED_A);
    let socket = dir.path().join("signer.sock");
    let store = dir.path().join("store");
    let taken = dir.path().join("taken");
    std::fs::write(&taken, b"a file").expect("a file");
    let nowhere = dir.path().join("no-such-directory").join("signer.sock");
    let empty = Path::new("");

    let cases: [StartCase; 6] = [
        (
            "NoKey",
            vec![(SIGNER_VARIABLE, &socket), (STORE_VARIABLE, &store)],
            SIGNING_KEY_VARIABLE,
        ),
        (
            "NoKey, empty",
            vec![
                (SIGNING_KEY_VARIABLE, empty),
                (SIGNER_VARIABLE, &socket),
                (STORE_VARIABLE, &store),
            ],
            SIGNING_KEY_VARIABLE,
        ),
        (
            "NoSocket",
            vec![(SIGNING_KEY_VARIABLE, &key), (STORE_VARIABLE, &store)],
            SIGNER_VARIABLE,
        ),
        (
            "NoStore",
            vec![(SIGNING_KEY_VARIABLE, &key), (SIGNER_VARIABLE, &socket)],
            STORE_VARIABLE,
        ),
        (
            "PathTaken",
            vec![
                (SIGNING_KEY_VARIABLE, &key),
                (SIGNER_VARIABLE, &taken),
                (STORE_VARIABLE, &store),
            ],
            "not a socket",
        ),
        (
            "Bind",
            vec![
                (SIGNING_KEY_VARIABLE, &key),
                (SIGNER_VARIABLE, &nowhere),
                (STORE_VARIABLE, &store),
            ],
            "cannot be made",
        ),
    ];
    for (label, variables, expected) in cases {
        let (status, said) = refused_start(&variables);
        assert_eq!(status, Some(2), "{label}: {said}");
        assert!(said.contains(expected), "{label}: {said}");
        assert!(!said.contains(&hex(&SEED_A)), "{label}: the seed");
    }
    assert_eq!(
        std::fs::read(&taken).expect("the file"),
        b"a file",
        "what is at the path is left there"
    );
}
