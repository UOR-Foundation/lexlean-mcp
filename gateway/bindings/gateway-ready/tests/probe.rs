//! DP-06: `gateway-ready`, the readiness command the deployment names for the gateway.
//!
//! The probe runs against loopback listeners that stand for the gateway, so these tests need no
//! served gateway; the image run starts the real one.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener};
use std::process::Command;
use std::thread;
use std::time::Duration;

use gateway_ready::{is_status_line, probe, NotReady};

/// A listener on an ephemeral loopback port that reads one request and answers `answer`.
fn answering(answer: &'static [u8]) -> SocketAddr {
    let listener = TcpListener::bind("127.0.0.1:0").expect("a listener");
    let address = listener.local_addr().expect("its address");
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener.accept() {
            let mut request = [0u8; 512];
            let _ = stream.read(&mut request);
            let _ = stream.write_all(answer);
        }
    });
    address
}

#[test]
fn a_status_line_is_ready_dp_06() {
    let address = answering(b"HTTP/1.1 405 Method Not Allowed\r\ncontent-length: 0\r\n\r\n");
    assert_eq!(probe(address, Duration::from_secs(5)), Ok(()));
}

#[test]
fn an_answer_that_is_not_http_is_not_ready_dp_06() {
    for answer in [&b"hello\n"[..], b"HTTP/1.0 200 OK\r\n\r\n", b""] {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a listener");
        let address = listener.local_addr().expect("its address");
        let owned = answer.to_vec();
        thread::spawn(move || {
            if let Ok((mut stream, _)) = listener.accept() {
                let mut request = [0u8; 512];
                let _ = stream.read(&mut request);
                let _ = stream.write_all(&owned);
            }
        });
        assert_eq!(
            probe(address, Duration::from_secs(5)),
            Err(NotReady::NotHttp),
            "{answer:?}"
        );
    }
}

#[test]
fn nothing_listening_is_unreachable_dp_06() {
    let address = {
        let listener = TcpListener::bind("127.0.0.1:0").expect("a listener");
        listener.local_addr().expect("its address")
    };
    match probe(address, Duration::from_secs(5)) {
        Err(NotReady::Unreachable(error)) => assert!(!error.is_empty()),
        other => panic!("not unreachable: {other:?}"),
    }
}

#[test]
fn a_status_line_is_the_version_a_space_and_three_digits_dp_06() {
    assert!(is_status_line(b"HTTP/1.1 200 OK\r\n"));
    assert!(is_status_line(b"HTTP/1.1 403\r\n"));
    assert!(is_status_line(b"HTTP/1.1 501"));
    assert!(!is_status_line(b"HTTP/1.1 20 OK\r\n"));
    assert!(!is_status_line(b"HTTP/1.1 2000 OK\r\n"));
    assert!(!is_status_line(b"HTTP/2 200\r\n"));
    assert!(!is_status_line(b" HTTP/1.1 200 OK\r\n"));
}

#[test]
fn any_argument_is_a_usage_error_dp_06() {
    let output = Command::new(env!("CARGO_BIN_EXE_gateway-ready"))
        .arg("now")
        .output()
        .expect("the program runs");
    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage: gateway-ready"));
}
