//! TH-05, TH-07 to TH-09: the production core serves the gateway at the pin of MCP with the new
//! frame, and the adapter asks the host for the three frame effects of the time, the identifier
//! and the table. A client that declares the tasks extension is given a task for
//! `lexlean_verify` (TH-07), polls it (TH-08) and cancels it (TH-09); the verifier behind the
//! host is the oracle of what became of the task.
//!
//! The adapter is the generated one, served over a real socket (as in `origin.rs`), with the
//! real host behind a recorder that notes which effect of the host was asked for, in order,
//! for each request. Observed here: an exchange that touches no task asks the core for no
//! `task.store`, and every request that reaches the core is given a time, an identifier and a
//! table.

use std::io::{Read, Write};
use std::net::{Ipv4Addr, TcpStream};
use std::sync::{Arc, Mutex};

use gateway_binding::{Config, Native};
use gateway_host::serve;
use gateway_host_api::protocol::{
    attestation_lookup, clock_now, engine_build, engine_check, engine_format, engine_snapshot,
    lexicon_read, rate_calls, store_get, store_put, task_id, task_store, task_table,
    verify_control, verify_start, Host, HostError,
};
use gateway_tool_tests::{list_http_request, tiny, Gateway, Reply};
use serde_json::{json, Value};

/// A host that notes the name of every effect that is asked of it.
struct Recording {
    inner: Native,
    asked: Arc<Mutex<Vec<&'static str>>>,
}

macro_rules! recorded {
    ($method:ident, $module:ident) => {
        fn $method(
            &mut self,
            request: $module::Request,
        ) -> Result<$module::Completed, HostError<$module::Failure>> {
            self.asked
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner())
                .push(stringify!($method));
            self.inner.$method(request)
        }
    };
}

impl Host for Recording {
    recorded!(store_get, store_get);
    recorded!(store_put, store_put);
    recorded!(engine_check, engine_check);
    recorded!(engine_snapshot, engine_snapshot);
    recorded!(engine_format, engine_format);
    recorded!(engine_build, engine_build);
    recorded!(verify_start, verify_start);
    recorded!(verify_control, verify_control);
    recorded!(lexicon_read, lexicon_read);
    recorded!(attestation_lookup, attestation_lookup);
    recorded!(rate_calls, rate_calls);
    recorded!(clock_now, clock_now);
    recorded!(task_id, task_id);
    recorded!(task_table, task_table);
    recorded!(task_store, task_store);
}

/// Serve `requests` in order, one connection each, and return the status of each answer and the
/// effects of the host that were asked for it.
fn served(requests: &[Vec<u8>]) -> Vec<(u16, Vec<&'static str>)> {
    let directory = tempfile::tempdir().expect("a directory");
    let asked = Arc::new(Mutex::new(Vec::new()));
    let host = Recording {
        inner: Native::new(Config::at(directory.path().join("store"))),
        asked: Arc::clone(&asked),
    };
    let listener = serve::listen(Ipv4Addr::LOCALHOST, 0).expect("a loopback listener");
    let port = listener.local_addr().expect("an address").port();
    let count = requests.len();
    let server = std::thread::spawn(move || {
        serve::run(listener, host, gateway_host::exchange::core, Some(count))
    });
    let mut out = Vec::new();
    for request in requests {
        asked
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clear();
        let mut stream = TcpStream::connect((Ipv4Addr::LOCALHOST, port)).expect("a connection");
        stream.write_all(request).expect("the request");
        let mut bytes = Vec::new();
        let _ = stream.read_to_end(&mut bytes);
        let text = String::from_utf8_lossy(&bytes);
        let status: u16 = text
            .strip_prefix("HTTP/1.1 ")
            .and_then(|rest| rest.get(..3))
            .and_then(|code| code.parse().ok())
            .unwrap_or_else(|| panic!("an HTTP/1.1 status line in {text:?}"));
        let names = asked
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone();
        out.push((status, names));
    }
    server
        .join()
        .expect("the server thread")
        .expect("the server ran");
    out
}

#[test]
fn a_listing_is_given_the_time_an_identifier_and_the_table_and_stores_nothing_th_05() {
    let requests = [list_http_request(1, &[]), list_http_request(2, &[])];
    for (status, names) in served(&requests) {
        assert_eq!(status, 200, "{names:?}");
        for needed in ["clock_now", "task_id", "task_table"] {
            assert_eq!(
                names.iter().filter(|name| **name == needed).count(),
                1,
                "{needed} once for the request: {names:?}"
            );
        }
        assert!(
            !names.contains(&"task_store"),
            "no task was touched, so nothing is stored: {names:?}"
        );
    }
}

#[test]
fn a_tool_call_that_creates_no_task_asks_the_core_for_no_store_th_05() {
    let mut gateway = Gateway::new();
    for (tool, arguments) in [
        ("lexlean_explain", serde_json::json!({"code": "LLC0001"})),
        ("lexlean_profiles", serde_json::json!({})),
    ] {
        let reply = gateway.call(tool, arguments);
        assert!(
            !reply.effects.iter().any(|name| name == "task.store"),
            "{tool} asked for {:?}",
            reply.effects
        );
    }
}

// ---- TH-07 to TH-09: the tasks of lexlean_verify ------------------------------------------------

/// The time to live and the poll interval of a task of the gateway (TASKHOST.md).
const TTL_MS: u64 = 86_400_000;
const POLL_MS: u64 = 5_000;

const UNKNOWN_TASK: i64 = -32602;
const INTERNAL_ERROR: i64 = -32603;

/// The states of the verifier (`verify.control`): queued, running, verified, failed, cancelled.
const VERIFIED: u32 = 2;
const FAILED: u32 = 3;
const CANCELLED: u32 = 4;

/// A gateway whose store holds the project `dir`, the directory of the tiny project.
fn with_project() -> Gateway {
    let gateway = Gateway::new();
    gateway.install("dir", &tiny());
    gateway
}

fn verification() -> Value {
    json!({"project": "dir", "files": []})
}

/// The handle of the task that a declaring client is given for a verification.
fn created(gateway: &mut Gateway) -> String {
    let reply = gateway.task_call("lexlean_verify", verification());
    reply.result()["taskId"]
        .as_str()
        .expect("a task handle")
        .to_string()
}

/// The table of tasks the host keeps, as JSON.
fn table_of(gateway: &mut Gateway) -> Value {
    let kept = gateway
        .host()
        .task_table(task_table::Request {})
        .expect("the table")
        .table;
    if kept.is_empty() {
        return json!([]);
    }
    serde_json::from_slice(&kept).expect("a table of JSON")
}

/// What the verifier says of the task `task`.
fn verifier_state(gateway: &mut Gateway, task: &str) -> u32 {
    gateway
        .host()
        .verify_control(verify_control::Request {
            task: task.to_string(),
            action: 0,
        })
        .expect("the verifier knows the task")
        .state
}

fn effects(reply: &Reply) -> Vec<&str> {
    reply.effects.iter().map(String::as_str).collect()
}

/// The real host, except that `verify.control` answers what the test says: the verifier has
/// reached `state` with `class` and `result`.
struct Verdict {
    inner: Native,
    state: u32,
    class: u32,
    result: Vec<u8>,
    groups_ended: u32,
    groups_failed: u32,
}

macro_rules! delegated {
    ($method:ident, $module:ident) => {
        fn $method(
            &mut self,
            request: $module::Request,
        ) -> Result<$module::Completed, HostError<$module::Failure>> {
            self.inner.$method(request)
        }
    };
}

impl Host for Verdict {
    delegated!(store_get, store_get);
    delegated!(store_put, store_put);
    delegated!(engine_check, engine_check);
    delegated!(engine_snapshot, engine_snapshot);
    delegated!(engine_format, engine_format);
    delegated!(engine_build, engine_build);
    delegated!(verify_start, verify_start);
    delegated!(lexicon_read, lexicon_read);
    delegated!(attestation_lookup, attestation_lookup);
    delegated!(rate_calls, rate_calls);
    delegated!(clock_now, clock_now);
    delegated!(task_id, task_id);
    delegated!(task_table, task_table);
    delegated!(task_store, task_store);

    fn verify_control(
        &mut self,
        _request: verify_control::Request,
    ) -> Result<verify_control::Completed, HostError<verify_control::Failure>> {
        Ok(verify_control::Completed {
            state: self.state,
            class: self.class,
            result: self.result.clone(),
            groups_ended: self.groups_ended,
            groups_failed: self.groups_failed,
        })
    }
}

/// Poll `task` with a verifier that has reached `state`.
fn polled_when(gateway: &mut Gateway, task: &str, state: u32, class: u32, result: &[u8]) -> Reply {
    let mut verdict = Verdict {
        inner: gateway.take_host(),
        state,
        class,
        result: result.to_vec(),
        groups_ended: 1,
        groups_failed: u32::from(class != 0),
    };
    let reply = gateway.task_rpc_with(&mut verdict, "tasks/get", task);
    gateway.put_host(verdict.inner);
    reply
}

/// Cancel `task` with a verifier that ended `ended` child groups and failed to end `failed`.
fn cancelled_when(gateway: &mut Gateway, task: &str, ended: u32, failed: u32) -> Reply {
    let mut verdict = Verdict {
        inner: gateway.take_host(),
        state: CANCELLED,
        class: 0,
        result: Vec::new(),
        groups_ended: ended,
        groups_failed: failed,
    };
    let reply = gateway.task_rpc_with(&mut verdict, "tasks/cancel", task);
    gateway.put_host(verdict.inner);
    reply
}

#[test]
fn a_client_that_declares_tasks_is_given_a_task_for_a_verification_th_07() {
    let mut gateway = with_project();
    let reply = gateway.task_call("lexlean_verify", verification());
    assert_eq!(effects(&reply), ["verify.start", "task.store"]);
    let result = reply.result();
    assert_eq!(result["resultType"], "task", "{}", reply.body);
    assert_eq!(result["status"], "working");
    assert_eq!(result["ttlMs"], TTL_MS);
    assert_eq!(result["pollIntervalMs"], POLL_MS);
    let handle = result["taskId"].as_str().expect("a task handle");
    // The handle is the verifier's own name for the verification: the verifier knows it.
    assert_ne!(verifier_state(&mut gateway, handle), CANCELLED);
    // The table the host keeps holds the task, working, under that handle.
    let table = table_of(&mut gateway);
    assert_eq!(table.as_array().map(Vec::len), Some(1), "{table}");
    assert_eq!(table[0]["task"]["taskId"], handle);
    assert_eq!(table[0]["task"]["status"], "working");
}

#[test]
fn a_client_that_does_not_declare_tasks_is_answered_as_before_th_07() {
    let mut gateway = with_project();
    let reply = gateway.call("lexlean_verify", verification());
    assert_eq!(effects(&reply), ["verify.start"]);
    assert!(!reply.is_error(), "{}", reply.body);
    let task = reply.structured()["task"].as_str().expect("a task name");
    assert_eq!(reply.text(), task);
    assert_eq!(table_of(&mut gateway), json!([]), "no task was made");
}

#[test]
fn a_verification_that_is_refused_creates_no_task_th_07() {
    let mut gateway = with_project();
    let reply = gateway.task_call("lexlean_verify", json!({"project": "nope", "files": []}));
    assert_eq!(effects(&reply), ["verify.start"], "{}", reply.body);
    assert!(reply.is_error(), "{}", reply.body);
    assert_eq!(reply.structured()["failure"], "project-refused");
    assert_eq!(table_of(&mut gateway), json!([]), "no task was made");
}

#[test]
fn a_working_task_stays_working_while_the_verifier_runs_th_08() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let reply = gateway.task_rpc("tasks/get", &handle);
    assert_eq!(effects(&reply), ["verify.control", "task.store"]);
    let result = reply.result();
    assert_eq!(result["taskId"], handle.as_str());
    assert_eq!(result["status"], "working");
    assert_eq!(result["ttlMs"], TTL_MS);
    assert_eq!(result["pollIntervalMs"], POLL_MS);
    assert!(
        reply.notifications.is_empty(),
        "nothing changed: {:?}",
        reply.notifications
    );
    assert_eq!(table_of(&mut gateway)[0]["task"]["status"], "working");
}

#[test]
fn a_verified_task_is_completed_with_the_result_of_the_tool_th_08() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let reply = polled_when(&mut gateway, &handle, VERIFIED, 0, b"verified");
    assert_eq!(effects(&reply), ["verify.control", "task.store"]);
    let result = reply.result();
    assert_eq!(result["status"], "completed", "{}", reply.body);
    assert_eq!(result["result"]["isError"], Value::Null, "{}", reply.body);
    assert_eq!(result["result"]["structuredContent"]["state"], "verified");
    assert_eq!(result["result"]["content"][0]["text"], "verified");
    assert_eq!(reply.notifications.len(), 1, "{:?}", reply.notifications);
    assert_eq!(reply.notifications[0]["params"]["status"], "completed");
    assert_eq!(table_of(&mut gateway)[0]["task"]["status"], "completed");
}

#[test]
fn a_failed_verification_completes_the_task_with_an_error_result_th_08() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let reply = polled_when(&mut gateway, &handle, FAILED, 1, b"failed");
    assert_eq!(effects(&reply), ["verify.control", "task.store"]);
    let result = reply.result();
    assert_eq!(result["status"], "completed", "{}", reply.body);
    assert_eq!(result["result"]["isError"], true, "{}", reply.body);
    assert_eq!(result["result"]["structuredContent"]["state"], "failed");
}

#[test]
fn a_task_that_the_verifier_lost_is_failed_th_08() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    // The verifier is a new one, as after a restart: it has never heard of the task.
    let table = gateway
        .host()
        .task_table(task_table::Request {})
        .expect("the table")
        .table;
    let mut other = Gateway::new();
    other
        .host()
        .task_store(task_store::Request { table })
        .expect("the table is kept");
    let reply = other.task_rpc("tasks/get", &handle);
    assert_eq!(effects(&reply), ["verify.control", "task.store"]);
    let result = reply.result();
    assert_eq!(result["status"], "failed", "{}", reply.body);
    assert_eq!(result["error"]["code"], INTERNAL_ERROR);
    assert_eq!(
        result["error"]["message"],
        "the verifier no longer knows the task"
    );
    assert_eq!(reply.notifications.len(), 1, "{:?}", reply.notifications);
}

#[test]
fn a_terminal_task_is_answered_from_the_table_th_08() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let first = polled_when(&mut gateway, &handle, VERIFIED, 0, b"verified");
    assert_eq!(first.result()["status"], "completed");
    let again = gateway.task_rpc("tasks/get", &handle);
    assert!(
        !again.effects.iter().any(|name| name == "verify.control"),
        "the verifier is not asked again: {:?}",
        again.effects
    );
    assert_eq!(again.result()["status"], "completed", "{}", again.body);
    assert_eq!(again.result()["result"], first.result()["result"]);
    assert!(again.notifications.is_empty());
}

#[test]
fn a_task_that_the_table_does_not_hold_is_unknown_th_08() {
    let mut gateway = with_project();
    let reply = gateway.task_rpc("tasks/get", "no-such-task");
    assert!(reply.effects.is_empty(), "{:?}", reply.effects);
    assert_eq!(reply.error()["code"], UNKNOWN_TASK);
    assert_eq!(reply.status, 400);
}

#[test]
fn a_cancelled_task_is_stored_as_cancelled_th_09() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let reply = gateway.task_rpc("tasks/cancel", &handle);
    assert_eq!(effects(&reply), ["verify.control", "task.store"]);
    assert_eq!(reply.result()["resultType"], "complete", "{}", reply.body);
    assert!(reply.result().get("status").is_none(), "an acknowledgement");
    assert_eq!(reply.notifications.len(), 1, "{:?}", reply.notifications);
    assert_eq!(reply.notifications[0]["params"]["status"], "cancelled");
    assert_eq!(reply.notifications[0]["params"]["taskId"], handle.as_str());
    // The verifier agrees, and so does the table.
    assert_eq!(verifier_state(&mut gateway, &handle), CANCELLED);
    assert_eq!(table_of(&mut gateway)[0]["task"]["status"], "cancelled");
    let polled = gateway.task_rpc("tasks/get", &handle);
    assert_eq!(polled.result()["status"], "cancelled", "{}", polled.body);
}

#[test]
fn a_second_cancel_is_answered_from_the_table_th_09() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let first = gateway.task_rpc("tasks/cancel", &handle);
    assert_eq!(first.notifications.len(), 1);
    let second = gateway.task_rpc("tasks/cancel", &handle);
    assert!(
        !second.effects.iter().any(|name| name == "verify.control"),
        "the verifier is not asked again: {:?}",
        second.effects
    );
    assert!(second.notifications.is_empty());
    assert_eq!(second.result()["resultType"], "complete", "{}", second.body);
}

#[test]
fn cancelling_a_task_that_the_table_does_not_hold_is_refused_th_09() {
    let mut gateway = with_project();
    let reply = gateway.task_rpc("tasks/cancel", "no-such-task");
    assert!(reply.effects.is_empty(), "{:?}", reply.effects);
    assert_eq!(reply.error()["code"], UNKNOWN_TASK);
}

// ---- TH-10: a cancel says what became of the child groups (AC-12) --------------------------------

#[test]
fn a_cancel_reports_the_child_groups_that_ended_th_10() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let reply = cancelled_when(&mut gateway, &handle, 2, 0);
    assert_eq!(effects(&reply), ["verify.control", "task.store"]);
    assert_eq!(reply.notifications.len(), 1, "{:?}", reply.notifications);
    let notified = &reply.notifications[0]["params"];
    assert_eq!(notified["status"], "cancelled");
    assert_eq!(notified["statusMessage"], "child groups ended: 2 failed: 0");
    // The poll that follows and the table say the same.
    let polled = gateway.task_rpc("tasks/get", &handle);
    assert_eq!(polled.result()["status"], "cancelled");
    assert_eq!(
        polled.result()["statusMessage"],
        "child groups ended: 2 failed: 0"
    );
    assert_eq!(
        table_of(&mut gateway)[0]["task"]["statusMessage"],
        "child groups ended: 2 failed: 0"
    );
}

#[test]
fn a_cancel_reports_a_child_group_that_failed_to_end_th_10() {
    let mut gateway = with_project();
    let handle = created(&mut gateway);
    let reply = cancelled_when(&mut gateway, &handle, 1, 1);
    assert_eq!(
        reply.notifications[0]["params"]["statusMessage"], "child groups ended: 1 failed: 1",
        "{:?}",
        reply.notifications
    );
}

// ---- TH-11: the task path with the real verifier -------------------------------------------------

#[test]
#[ignore = "runs a real verification, which needs the Lean toolchain: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn a_real_verification_runs_as_a_task_to_completed_th_11() {
    let mut gateway = Gateway::with_real_verifier();
    gateway.install("dir", &tiny());
    let handle = created(&mut gateway);
    let until = std::time::Instant::now() + std::time::Duration::from_secs(900);
    let done = loop {
        let reply = gateway.task_rpc("tasks/get", &handle);
        if reply.result()["status"] != "working" {
            break reply;
        }
        assert!(
            std::time::Instant::now() < until,
            "the verification did not end"
        );
        std::thread::sleep(std::time::Duration::from_millis(500));
    };
    assert_eq!(done.result()["status"], "completed", "{}", done.body);
    let result = &done.result()["result"];
    assert_eq!(result["structuredContent"]["state"], "verified", "{result}");
    assert_eq!(done.notifications[0]["params"]["status"], "completed");
    // The result of the tool names the record, which is signed and found by its ID.
    let text = result["content"][0]["text"].as_str().expect("a text block");
    let record: Value = serde_json::from_str(text).expect("JSON");
    let attestation = record["attestation_id"].as_str().expect("an ID");
    let found = gateway.call(
        "lexlean_attestation",
        json!({"by": "id", "value": attestation}),
    );
    assert_eq!(found.structured()["found"], true, "{}", found.body);
    let subject = std::fs::read(
        gateway
            .store()
            .join("verifications")
            .join(attestation)
            .join("attestation.json"),
    )
    .expect("the subject");
    let history = gateway_binding::signer::read_history(&gateway.config().history_path())
        .expect("the history");
    gateway_binding::signer::validate(found.text().as_bytes(), &history, &subject)
        .expect("a valid envelope");
    // The same record is presented and exported (issue 12): the resource is the canonical LaTeX the
    // verification wrote, and the bundle holds it beside the files that let a reader check it.
    let presented = gateway.call("lexlean_present", json!({"by": "id", "value": attestation}));
    assert!(!presented.is_error(), "{}", presented.body);
    let module = presented.structured()["module"].as_str().expect("a path");
    let resource = &presented.result()["content"][0]["resource"];
    let rendering = std::fs::read(
        gateway
            .store()
            .join("verifications")
            .join(attestation)
            .join(module),
    )
    .expect("the rendering");
    assert_eq!(
        resource["text"].as_str().expect("text").as_bytes(),
        rendering
    );
    let exported = gateway.call("lexlean_export", json!({"by": "id", "value": attestation}));
    assert!(!exported.is_error(), "{}", exported.body);
    let bundle = gateway.store().join("exports").join(attestation);
    assert_eq!(
        std::fs::read(bundle.join(module)).expect("in the bundle"),
        rendering
    );
    assert!(bundle.join("envelope.json").is_file() && bundle.join("keys.json").is_file());
}

/// Whether a program of the Lean toolchain is running in the store of this gateway: the child
/// group that a cancel has to end. Its working directory is the copy of the project.
fn lean_is_running(store: &std::path::Path) -> bool {
    let Ok(store) = store.canonicalize() else {
        return false;
    };
    let Ok(processes) = std::fs::read_dir("/proc") else {
        return false;
    };
    processes.flatten().any(|process| {
        let path = process.path();
        let program = std::fs::read_to_string(path.join("comm")).unwrap_or_default();
        (program.starts_with("lean") || program.starts_with("lake"))
            && std::fs::read_link(path.join("cwd")).is_ok_and(|cwd| cwd.starts_with(&store))
    })
}

#[test]
#[ignore = "runs a real verification, which needs the Lean toolchain: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn a_real_verification_is_cancelled_through_the_task_th_11() {
    // A child lives for moments, and a cancel between two children has nothing to end (it reports
    // 0, which is true): each attempt cancels when one is seen, until a cancel ends one.
    let mut seen = Vec::new();
    for _ in 0..8 {
        let mut gateway = Gateway::with_real_verifier();
        gateway.install("dir", &tiny());
        let handle = created(&mut gateway);
        let until = std::time::Instant::now() + std::time::Duration::from_secs(60);
        while !lean_is_running(gateway.store()) && std::time::Instant::now() < until {
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let reply = gateway.task_rpc("tasks/cancel", &handle);
        let notified = &reply.notifications[0]["params"];
        if notified["status"] != "cancelled" {
            seen.push(format!("{}", notified["status"]));
            continue;
        }
        assert_eq!(effects(&reply), ["verify.control", "task.store"]);
        let records = std::fs::read_dir(gateway.store().join("verifications"))
            .map(|entries| entries.count())
            .unwrap_or(0);
        assert_eq!(records, 0, "a cancelled verification leaves no record");
        let message = notified["statusMessage"].as_str().expect("a message");
        assert!(
            message.ends_with(" failed: 0"),
            "AC-12: no child group failed to end: {message}"
        );
        if message != "child groups ended: 0 failed: 0" {
            return;
        }
        seen.push(message.to_owned());
    }
    panic!("AC-12: no cancel ended a child group: {seen:?}");
}
