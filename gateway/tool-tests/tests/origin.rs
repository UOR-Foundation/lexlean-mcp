//! GW-13: the served adapter refuses a foreign `Origin` by itself.
//!
//! The declaration's `origins` (the allow-list of `gateway/vectors/generate.py`) are enforced by
//! the generated adapter before a request is framed, and MCP's core refuses a foreign `Origin`
//! once more (`Forbidden: invalid Origin`). The two checks are defence in depth, and MCP's
//! server suite cannot tell them apart: the core refuses first when it is asked, so a planted
//! extra origin in the adapter's list went unseen (GATEWAY-CONFORMANCE.md). A served vector
//! cannot hold the refusal either: a vector names a frame the core is given, and the adapter
//! refuses before there is one. So these tests run the adapter itself, over a real socket, with
//! the real core behind a counter, and observe what only the adapter does: its own word for the
//! refusal, `OriginForbidden`, and a core that was never called.

use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

use gateway_binding::{Config, Native};
use gateway_host::failure::Failure;
use gateway_host::serve;
use gateway_host::surface;
use gateway_tool_tests::{list_frame_with_origin, list_http_request, Gateway};
use serde_json::Value;

/// The calls of the core since the test began: the counter is the observation that the adapter,
/// and not the core, refused.
static CALLS: AtomicUsize = AtomicUsize::new(0);

/// Tests of this binary share `CALLS`, so they take turns.
static ONE_AT_A_TIME: Mutex<()> = Mutex::new(());

fn counted_core(input: Vec<u8>) -> Result<Vec<u8>, Failure> {
    CALLS.fetch_add(1, Ordering::SeqCst);
    gateway_host::exchange::core(input)
}

/// What a client read from the adapter.
struct Answer {
    status: u16,
    body: Vec<u8>,
}

fn answer_of(bytes: &[u8]) -> Answer {
    let text = String::from_utf8_lossy(bytes);
    let status = text
        .strip_prefix("HTTP/1.1 ")
        .and_then(|rest| rest.get(..3))
        .and_then(|code| code.parse().ok())
        .unwrap_or_else(|| panic!("an HTTP/1.1 status line in {text:?}"));
    let at = bytes
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .expect("the end of the head");
    Answer {
        status,
        body: bytes[at + 4..].to_vec(),
    }
}

/// Serve `requests` in order, one connection each, with the real host and the counted core,
/// and return what each was answered and how many times the core was called for each.
fn put_to_the_adapter(requests: &[Vec<u8>]) -> Vec<(Answer, usize)> {
    let directory = tempfile::tempdir().expect("a directory");
    let host = Native::new(Config::at(directory.path().join("store")));
    let listener = serve::listen(Ipv4Addr::LOCALHOST, 0).expect("a loopback listener");
    let port = listener.local_addr().expect("an address").port();
    let count = requests.len();
    let server = std::thread::spawn(move || serve::run(listener, host, counted_core, Some(count)));
    let mut out = Vec::new();
    for request in requests {
        let before = CALLS.load(Ordering::SeqCst);
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).expect("a connection");
        stream.write_all(request).expect("the request");
        let mut bytes = Vec::new();
        let _ = stream.read_to_end(&mut bytes);
        out.push((answer_of(&bytes), CALLS.load(Ordering::SeqCst) - before));
    }
    server
        .join()
        .expect("the server thread")
        .expect("the server ran");
    out
}

/// The origins no browser of this machine's user has a reason to send: none is the declaration's.
const FOREIGN: &[&str] = &[
    "http://evil.example.com",
    "https://evil.example.com",
    "http://evil.example",
    "http://evil.example.com:80",
    "http://localhost.evil.example.com",
    "http://127.0.0.1.evil.example.com",
    "http://evil-localhost",
    "http://notlocalhost",
    "http://localhost.",
    "http://[::2]",
    "http://[::1].evil.example.com",
    "http://10.0.0.1",
    "http://0.0.0.0",
    "http://192.168.0.1:5395",
    "http://user@localhost",
    "http://localhost/path",
    "http://localhost?query",
    "http://localhost:99999",
    "http://localhost:80a",
    "ftp://localhost",
    "ws://localhost",
    "file://localhost",
    "null",
    "localhost",
    "",
];

/// The six origins the declaration allows, with the port a browser would add.
const ALLOWED: &[&str] = &[
    "http://localhost",
    "https://localhost",
    "http://127.0.0.1",
    "https://127.0.0.1",
    "http://[::1]",
    "https://[::1]",
    "http://localhost:3000",
    "HTTP://LOCALHOST:8080",
    "https://127.0.0.1:8443",
    "http://[::1]:5395",
];

#[test]
fn a_foreign_origin_is_refused_by_the_adapter_and_never_reaches_the_core_gw_13() {
    let _turn = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let requests: Vec<Vec<u8>> = FOREIGN
        .iter()
        .enumerate()
        .map(|(index, origin)| list_http_request(index as u64 + 1, &[origin]))
        .collect();
    for ((answer, calls), origin) in put_to_the_adapter(&requests).iter().zip(FOREIGN) {
        assert_eq!(answer.status, 403, "Origin {origin:?}");
        assert_eq!(
            answer.body,
            b"OriginForbidden\n",
            "Origin {origin:?}: the adapter's word, not the core's: {:?}",
            String::from_utf8_lossy(&answer.body)
        );
        assert_eq!(*calls, 0, "Origin {origin:?}: the core was called");
    }
}

#[test]
fn two_origin_fields_are_refused_by_the_adapter_gw_13() {
    let _turn = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    // A browser sends one `Origin`; two are refused even when both are allowed.
    let request = list_http_request(1, &["http://localhost", "http://localhost"]);
    let answered = put_to_the_adapter(&[request]);
    let (answer, calls) = &answered[0];
    assert_eq!(answer.status, 403);
    assert_eq!(answer.body, b"OriginForbidden\n");
    assert_eq!(*calls, 0);
}

#[test]
fn an_allowed_origin_and_no_origin_reach_the_core_gw_13() {
    let _turn = ONE_AT_A_TIME
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    let mut requests: Vec<Vec<u8>> = ALLOWED
        .iter()
        .enumerate()
        .map(|(index, origin)| list_http_request(index as u64 + 1, &[origin]))
        .collect();
    // A request with no `Origin` is not a browser's and is not judged.
    requests.push(list_http_request(99, &[]));
    for (index, (answer, calls)) in put_to_the_adapter(&requests).iter().enumerate() {
        let label = ALLOWED.get(index).copied().unwrap_or("no Origin");
        assert_eq!(*calls, 1, "{label}: the core was called once");
        assert_eq!(answer.status, 200, "{label}");
        let body: Value = serde_json::from_slice(&answer.body).expect("a JSON-RPC response");
        assert!(
            body["result"]["tools"].is_array(),
            "{label}: the core listed its tools: {body}"
        );
    }
}

#[test]
fn the_core_refuses_a_foreign_origin_in_its_own_words_gw_13() {
    // The same foreign origins, handed to the core as a frame, are refused by the core too,
    // with a JSON-RPC error and not the adapter's word: two checks, and the test above is the
    // one that sees the adapter's.
    let mut gateway = Gateway::new();
    let reply = gateway.answer_frame(&list_frame_with_origin(1, "http://evil.example.com"));
    assert_eq!(reply.status, 403, "{}", reply.body);
    assert_eq!(reply.error()["message"], "Forbidden: invalid Origin");
    assert!(reply.effects.is_empty());
}

#[test]
fn the_declared_origins_are_the_six_loopback_origins_with_any_port_gw_13() {
    let mut declared: Vec<(&str, &str, u16)> = surface::ORIGINS.to_vec();
    declared.sort_unstable();
    let mut expected: Vec<(&str, &str, u16)> = Vec::new();
    for scheme in ["http", "https"] {
        for host in ["localhost", "127.0.0.1", "[::1]"] {
            expected.push((scheme, host, 0));
        }
    }
    expected.sort_unstable();
    assert_eq!(
        declared, expected,
        "the allow-list of the declaration is widened or narrowed"
    );
}
