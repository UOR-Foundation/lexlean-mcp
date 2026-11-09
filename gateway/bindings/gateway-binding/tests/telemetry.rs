//! OB-11 to OB-17: the observability runtime of issue #16. The model (`src/Observability.lex.tex`)
//! fixes the indicators, the alerts, the redaction and the trace context; these tests run what
//! the host emits against it, and the gaps and codes a request and a result carry. The verifier
//! is the fake of `common/fake-verifier.sh`.

#[allow(dead_code)]
mod common;

use std::sync::Arc;

use common::{config, fake_verifier, install_draft, install_named, start_named, wait_for};
use gateway_binding::telemetry::{
    extract, firing, first_error_code, gaps_of, names, redact, Alert, ErrorClass, GapKind,
    GroupEnd, MemorySink, Redacted, Sample, Telemetry, VerifyOutcome, BAGGAGE_LIMIT, GAPS_LIMIT,
    GAPS_META_KEY, TRACESTATE_LIMIT,
};
use gateway_binding::{Native, State};
use serde_json::{json, Value};

const TRACEPARENT: &str = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
/// A marker that stands for the text of a module, an argument or a search query.
const SECRET: &str = "theorem secretModuleText";

fn memory() -> (Telemetry, Arc<MemorySink>) {
    let sink = Arc::new(MemorySink::default());
    (Telemetry::new(Some(sink.clone())), sink)
}

fn parsed(sink: &MemorySink) -> Vec<Value> {
    sink.lines()
        .iter()
        .map(|line| serde_json::from_str(line).expect("a line of OTLP/JSON"))
        .collect()
}

/// The names of the alerts that fired, from the log records of the sink.
fn alerts(sink: &MemorySink) -> Vec<String> {
    parsed(sink)
        .iter()
        .filter_map(|line| line["resourceLogs"][0]["scopeLogs"][0]["logRecords"].get(0))
        .flat_map(|record| record["attributes"].as_array().cloned().unwrap_or_default())
        .filter(|attribute| attribute["key"] == "alert")
        .map(|attribute| {
            attribute["value"]["stringValue"]
                .as_str()
                .unwrap()
                .to_owned()
        })
        .collect()
}

fn metric_names(sink: &MemorySink) -> Vec<String> {
    let mut found: Vec<String> = parsed(sink)
        .iter()
        .flat_map(|line| {
            line["resourceMetrics"][0]["scopeMetrics"][0]["metrics"]
                .as_array()
                .cloned()
                .unwrap_or_default()
        })
        .map(|metric| metric["name"].as_str().unwrap().to_owned())
        .collect();
    found.sort();
    found
}

// OB-11: every indicator is emitted

#[test]
fn every_indicator_is_emitted_ob_11() {
    let (telemetry, sink) = memory();
    telemetry.tool_outcome(None, "vocab", None);
    telemetry.request_answered(Some(-32602));
    telemetry.check_latency("vocab", 120);
    telemetry.verify_duration("vocab", VerifyOutcome::Verified, None, 9_000);
    telemetry.queue_wait(40);
    telemetry.child_group_ends(GroupEnd::Ended, 1);
    telemetry.profile_drift("vocab");
    telemetry.signing_failure();
    telemetry.flush();

    let mut expected = vec![
        names::TOOL_OUTCOMES,
        names::PROTOCOL_ERRORS,
        names::REQUESTS,
        names::CHECK_DURATION,
        names::VERIFY_DURATION,
        names::QUEUE_WAIT,
        names::CHILD_GROUP_ENDS,
        names::PROFILE_DRIFT,
        names::SIGNING_FAILURES,
    ];
    expected.sort();
    assert_eq!(metric_names(&sink), expected);
    assert_eq!(
        telemetry.counter(names::PROTOCOL_ERRORS, &[("code", "-32602")]),
        1
    );
    assert_eq!(
        telemetry.counter(names::TOOL_OUTCOMES, &[("class", "ok")]),
        1
    );
}

#[test]
fn the_indicator_names_are_the_names_of_the_model_ob_11() {
    let model = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../src/Observability.lex.tex"),
    )
    .expect("the model");
    for name in [
        names::TOOL_OUTCOMES,
        names::PROTOCOL_ERRORS,
        names::CHECK_DURATION,
        names::VERIFY_DURATION,
        names::QUEUE_WAIT,
        names::CHILD_GROUP_ENDS,
        names::PROFILE_DRIFT,
        names::SIGNING_FAILURES,
    ] {
        assert!(model.contains(&format!("\"{name}\"")), "{name}");
    }
    let exits: Vec<u32> = [
        ErrorClass::Language,
        ErrorClass::CliOrConfiguration,
        ErrorClass::Environment,
        ErrorClass::SecurityOrLimit,
        ErrorClass::Internal,
    ]
    .map(ErrorClass::exit_code)
    .to_vec();
    assert_eq!(exits, [1, 2, 3, 4, 70]);
}

// OB-12: every alert fires on its planted condition, and on nothing else

fn plant(telemetry: &Telemetry, alert: Alert) {
    match alert {
        Alert::ClassTwo => {
            telemetry.tool_outcome(Some(ErrorClass::CliOrConfiguration), "vocab", None)
        }
        Alert::ClassThree => telemetry.tool_outcome(Some(ErrorClass::Environment), "vocab", None),
        Alert::ClassSeventy => telemetry.tool_outcome(Some(ErrorClass::Internal), "vocab", None),
        Alert::GroupEndFailed => telemetry.child_group_ends(GroupEnd::Failed, 1),
        Alert::Drift => telemetry.profile_drift("vocab"),
        Alert::SigningFailure => telemetry.signing_failure(),
    }
}

#[test]
fn each_alert_fires_on_its_planted_condition_and_no_other_ob_12() {
    for alert in Alert::ALL {
        let (telemetry, sink) = memory();
        plant(&telemetry, alert);
        assert_eq!(alerts(&sink), [alert.name()], "{alert:?}");
    }
}

#[test]
fn the_predicates_are_zero_tolerance_ob_12() {
    let samples = |count| {
        [
            (
                Alert::ClassTwo,
                Sample::ToolOutcome(ErrorClass::CliOrConfiguration, count),
            ),
            (
                Alert::ClassThree,
                Sample::ToolOutcome(ErrorClass::Environment, count),
            ),
            (
                Alert::ClassSeventy,
                Sample::ToolOutcome(ErrorClass::Internal, count),
            ),
            (
                Alert::GroupEndFailed,
                Sample::ChildGroupEnds(GroupEnd::Failed, count),
            ),
            (Alert::Drift, Sample::ProfileDrift(count)),
            (Alert::SigningFailure, Sample::SigningFailures(count)),
        ]
    };
    for (alert, sample) in samples(0) {
        assert!(
            !firing(alert, sample),
            "{alert:?} is silent at the threshold"
        );
    }
    for (alert, sample) in samples(1) {
        assert!(firing(alert, sample), "{alert:?} fires above it");
    }
    assert!(!firing(
        Alert::ClassTwo,
        Sample::ToolOutcome(ErrorClass::Language, 9)
    ));
    assert!(!firing(
        Alert::ClassSeventy,
        Sample::ToolOutcome(ErrorClass::SecurityOrLimit, 9)
    ));
    assert!(!firing(
        Alert::GroupEndFailed,
        Sample::ChildGroupEnds(GroupEnd::Ended, 9)
    ));
}

#[test]
fn healthy_conditions_raise_no_alert_ob_12() {
    let (telemetry, sink) = memory();
    telemetry.tool_outcome(None, "vocab", None);
    telemetry.tool_outcome(Some(ErrorClass::Language), "vocab", None);
    telemetry.tool_outcome(Some(ErrorClass::SecurityOrLimit), "vocab", None);
    telemetry.child_group_ends(GroupEnd::Ended, 3);
    telemetry.request_answered(None);
    telemetry.check_latency("vocab", 1);
    telemetry.verify_duration("vocab", VerifyOutcome::Failed, None, 1);
    telemetry.queue_wait(1);
    assert_eq!(alerts(&sink), Vec::<String>::new());
}

#[test]
fn an_alert_counts_the_events_of_its_window_ob_12() {
    let now = Arc::new(std::sync::atomic::AtomicU64::new(1_000));
    let clock = Arc::clone(&now);
    let sink = Arc::new(MemorySink::default());
    let telemetry = Telemetry::with_clock(Some(sink.clone()), move || {
        clock.load(std::sync::atomic::Ordering::SeqCst)
    });
    telemetry.signing_failure();
    telemetry.signing_failure();
    now.store(
        1_000 + Alert::SigningFailure.window_ms() + 1,
        std::sync::atomic::Ordering::SeqCst,
    );
    telemetry.signing_failure();
    let counts: Vec<String> = parsed(&sink)
        .iter()
        .map(|line| {
            line["resourceLogs"][0]["scopeLogs"][0]["logRecords"][0]["attributes"]
                .as_array()
                .unwrap()
                .iter()
                .find(|attribute| attribute["key"] == "count")
                .unwrap()["value"]["stringValue"]
                .as_str()
                .unwrap()
                .to_owned()
        })
        .collect();
    assert_eq!(counts, ["1", "2", "1"]);
}

// OB-13: no record holds the text of a module, an argument or a query

#[test]
fn no_telemetry_record_contains_module_text_ob_13() {
    let (telemetry, sink) = memory();
    assert_eq!(redact(SECRET), redact(""));
    assert_eq!(redact(SECRET), Redacted::Withheld);
    let meta = json!({"traceparent": TRACEPARENT, "baggage": "user=alice"});
    let mut scope = telemetry.begin_request(Some(&meta));
    scope.arguments(SECRET);
    scope.query(SECRET);
    scope.end("request", Some(ErrorClass::Language), None);
    // Free text that reaches a label is not a profile name.
    telemetry.check_latency(SECRET, 5);
    telemetry.verify_duration(SECRET, VerifyOutcome::Failed, None, 5);
    telemetry.profile_drift(SECRET);
    telemetry.tool_outcome(Some(ErrorClass::Language), SECRET, Some(SECRET));
    telemetry.flush();
    let lines = sink.lines();
    assert!(lines.len() >= 3);
    for line in lines {
        assert!(!line.contains("secretModuleText"), "{line}");
        assert!(!line.contains(SECRET), "{line}");
    }
}

#[test]
fn a_span_carries_the_context_of_the_request_ob_13() {
    let (telemetry, sink) = memory();
    let meta = json!({"traceparent": TRACEPARENT, "tracestate": "congo=t61rcWkgMzE"});
    telemetry
        .begin_request(Some(&meta))
        .end("request", None, Some(-32601));
    let span = &parsed(&sink)[0]["resourceSpans"][0]["scopeSpans"][0]["spans"][0];
    assert_eq!(span["traceId"], "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(span["parentSpanId"], "00f067aa0ba902b7");
    assert_eq!(span["traceState"], "congo=t61rcWkgMzE");
    assert_eq!(span["status"]["code"], 2);
}

// OB-14: the trace context of `_meta`, and the notifications

#[test]
fn trace_context_is_read_from_meta_ob_14() {
    let found = extract(Some(&json!({
        "traceparent": TRACEPARENT,
        "tracestate": "congo=t61rcWkgMzE",
        "baggage": "userId=alice",
    })));
    assert!(found.parent.is_some());
    assert_eq!(found.tracestate.as_deref(), Some("congo=t61rcWkgMzE"));
    assert_eq!(found.baggage.as_deref(), Some("userId=alice"));
    assert_eq!(extract(None), Default::default());
    assert_eq!(extract(Some(&json!({}))), Default::default());
}

#[test]
fn a_malformed_traceparent_restarts_the_trace_and_drops_tracestate_ob_14() {
    let malformed = [
        "ff-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        "00-4BF92F3577B34DA6A3CE929D0E0E4736-00f067aa0ba902b7-01",
        "00-00000000000000000000000000000000-00f067aa0ba902b7-01",
        "00-4bf92f3577b34da6a3ce929d0e0e4736-0000000000000000-01",
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01-extra",
        "01-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01x",
        "00_4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-0",
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902g7-01",
        "",
    ];
    for header in malformed {
        let found = extract(Some(&json!({"traceparent": header, "tracestate": "a=b"})));
        assert!(found.parent.is_none(), "{header:?}");
        assert!(found.tracestate.is_none(), "{header:?}");
    }
    let future = extract(Some(&json!({
        "traceparent": "01-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01-extra"
    })));
    assert!(future.parent.is_some(), "a later version may extend");
    assert!(extract(Some(&json!({"traceparent": 7}))).parent.is_none());
}

#[test]
fn state_and_baggage_over_their_limits_are_dropped_whole_ob_14() {
    let meta = |state: usize, baggage: usize| {
        extract(Some(&json!({
            "traceparent": TRACEPARENT,
            "tracestate": "k".repeat(state),
            "baggage": "b".repeat(baggage),
        })))
    };
    let within = meta(TRACESTATE_LIMIT, BAGGAGE_LIMIT);
    assert!(within.tracestate.is_some() && within.baggage.is_some());
    let over = meta(TRACESTATE_LIMIT + 1, BAGGAGE_LIMIT + 1);
    assert!(over.tracestate.is_none() && over.baggage.is_none());
    assert!(
        over.parent.is_some(),
        "a long tracestate never fails the request"
    );
}

#[test]
fn no_log_message_notification_is_emitted_ob_14() {
    let (telemetry, sink) = memory();
    for alert in Alert::ALL {
        plant(&telemetry, alert);
    }
    telemetry.begin_request(None).end("request", None, None);
    telemetry.flush();
    for line in sink.lines() {
        assert!(!line.contains("notifications/"), "{line}");
        assert!(!line.contains("jsonrpc"), "{line}");
    }
}

// The host: the indicators come from what it does.

#[test]
fn a_verification_reports_its_indicators_ob_11() {
    let dir = tempfile::tempdir().expect("a directory");
    let (telemetry, sink) = memory();
    let mut host = Native::with_telemetry(
        config(dir.path(), fake_verifier(dir.path())),
        telemetry.clone(),
    );
    install_draft(&host, None);
    install_named(&host, "t1", "verified");
    let (task, _) = start_named(&mut host, "t1").expect("a task");
    wait_for(&mut host, &task, State::Verified);
    telemetry.flush();
    let found = metric_names(&sink);
    for name in [
        names::VERIFY_DURATION,
        names::QUEUE_WAIT,
        names::TOOL_OUTCOMES,
    ] {
        assert!(found.iter().any(|found| found == name), "{name}: {found:?}");
    }
    assert_eq!(alerts(&sink), Vec::<String>::new());
}

#[test]
fn a_lost_verifier_raises_the_internal_class_alert_ob_12() {
    let dir = tempfile::tempdir().expect("a directory");
    let (telemetry, sink) = memory();
    let mut host = Native::with_telemetry(config(dir.path(), fake_verifier(dir.path())), telemetry);
    install_draft(&host, None);
    install_named(&host, "t1", "die");
    let (task, _) = start_named(&mut host, "t1").expect("a task");
    wait_for(&mut host, &task, State::Failed);
    assert_eq!(alerts(&sink), [Alert::ClassSeventy.name()]);
}

// OB-15: the host implements the generated host's per-request methods

/// A JSON-RPC request with this method and `_meta`.
fn request(method: &str, meta: Value) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": method,
        "params": {"_meta": meta, "arguments": {"module": SECRET}},
    }))
    .expect("a request")
}

fn spans(sink: &MemorySink) -> Vec<Value> {
    parsed(sink)
        .iter()
        .flat_map(|line| {
            line["resourceSpans"][0]["scopeSpans"][0]["spans"]
                .as_array()
                .cloned()
                .unwrap_or_default()
        })
        .collect()
}

fn span_attribute(span: &Value, key: &str) -> Option<String> {
    span["attributes"]
        .as_array()?
        .iter()
        .find(|attribute| attribute["key"] == key)?["value"]["stringValue"]
        .as_str()
        .map(str::to_owned)
}

#[test]
fn each_request_ends_its_own_span_with_its_method_and_error_ob_15() {
    use gateway_host_api::protocol::Host;
    let directory = tempfile::tempdir().expect("a temporary directory");
    let (telemetry, sink) = memory();
    let probe = telemetry.clone();
    let mut host = Native::with_telemetry(
        gateway_binding::Config::at(directory.path().join("store")),
        telemetry,
    );
    let call = request("example/first", json!({"traceparent": TRACEPARENT}));
    let list = request("example/second", json!({}));
    // The calls of two requests interleave, as on two connections.
    host.request_begin(0, &call);
    host.request_begin(1, &list);
    host.request_end(
        1,
        &list,
        br#"{"jsonrpc":"2.0","id":1,"result":{"tools":[]}}"#,
    );
    host.request_end(
        0,
        &call,
        br#"{"jsonrpc":"2.0","id":1,"error":{"code":-32602,"message":"bad"}}"#,
    );
    let spans = spans(&sink);
    assert_eq!(spans.len(), 2, "{spans:?}");
    assert_eq!(spans[0]["name"], "example/second");
    assert_eq!(span_attribute(&spans[0], "rpc.error_code"), None);
    assert_ne!(spans[0]["traceId"], "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(spans[1]["name"], "example/first");
    assert_eq!(spans[1]["traceId"], "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(spans[1]["parentSpanId"], "00f067aa0ba902b7");
    assert_eq!(
        span_attribute(&spans[1], "rpc.error_code").as_deref(),
        Some("-32602")
    );
    // S2: both requests are counted, and the one protocol error by its code.
    assert_eq!(probe.counter(names::REQUESTS, &[]), 2);
    assert_eq!(
        probe.counter(names::PROTOCOL_ERRORS, &[("code", "-32602")]),
        1
    );
    // The arguments of the call are withheld like every other free text.
    for line in sink.lines() {
        assert!(!line.contains(SECRET), "{line}");
    }
}

#[test]
fn a_failed_tool_is_the_class_of_its_own_request_ob_15() {
    use gateway_host_api::protocol::{engine_check, Host};
    let directory = tempfile::tempdir().expect("a temporary directory");
    let (telemetry, sink) = memory();
    // No engine host: the check fails for the environment (class 3).
    let mut host = Native::with_telemetry(
        gateway_binding::Config::at(directory.path().join("store")),
        telemetry,
    );
    install_draft(&host, None);
    let call = request("example/first", json!({}));
    let list = request("example/second", json!({}));
    host.request_begin(0, &call);
    // Another connection's request is in flight on its own thread while the tool runs here.
    std::thread::scope(|scope| {
        scope
            .spawn(|| host.request_begin(1, &list))
            .join()
            .expect("a thread");
    });
    let outcome = host.engine_check(engine_check::Request {
        project: "tiny".to_string(),
        selection: 0,
        files: String::new(),
    });
    assert!(outcome.is_err(), "no engine host answers");
    std::thread::scope(|scope| {
        scope
            .spawn(|| host.request_end(1, &list, br#"{"jsonrpc":"2.0","id":1,"result":{}}"#))
            .join()
            .expect("a thread");
    });
    host.request_end(
        0,
        &call,
        br#"{"jsonrpc":"2.0","id":1,"result":{"isError":true}}"#,
    );
    let spans = spans(&sink);
    let named = |name: &str| {
        spans
            .iter()
            .find(|span| span["name"] == name)
            .cloned()
            .expect(name)
    };
    assert_eq!(
        span_attribute(&named("example/first"), "error.class").as_deref(),
        Some("3")
    );
    assert_eq!(
        span_attribute(&named("example/second"), "error.class"),
        None
    );
}

// OB-16: the gaps prism-code sends in `_meta` are recorded as fingerprints, and nothing else of them

const INSTALL: &str = "0123456789abcdef0123456789abcdef";

/// A batch of `prism/gaps/1` with these gaps.
fn batch(gaps: Value) -> Value {
    json!({GAPS_META_KEY: {"schema": "prism/gaps/1", "install": INSTALL, "gaps": gaps}})
}

/// The attributes of each gap record of the sink, by key.
fn gap_records(sink: &MemorySink) -> Vec<std::collections::BTreeMap<String, String>> {
    let is_gap = |attribute: &Value| {
        attribute["key"] == "event" && attribute["value"]["stringValue"] == "gap"
    };
    parsed(sink)
        .iter()
        .filter_map(|line| line["resourceLogs"][0]["scopeLogs"][0]["logRecords"].get(0))
        .filter(|record| {
            record["attributes"]
                .as_array()
                .is_some_and(|all| all.iter().any(is_gap))
        })
        .map(|record| {
            record["attributes"]
                .as_array()
                .unwrap()
                .iter()
                .map(|attribute| {
                    (
                        attribute["key"].as_str().unwrap().to_owned(),
                        attribute["value"]["stringValue"]
                            .as_str()
                            .unwrap()
                            .to_owned(),
                    )
                })
                .collect()
        })
        .collect()
}

#[test]
fn the_gaps_of_a_request_are_recorded_as_fingerprints_ob_16() {
    let (telemetry, sink) = memory();
    let mut meta = batch(json!([
        {"kind": "refusal", "construct": "fields.too-many", "code": null, "version": "0.6.0", "count": 3},
        {"kind": "roundtrip", "construct": "rules:choice.value", "code": null, "version": "0.6.1-rc.1", "count": 1},
        {"kind": "unproved", "construct": "always:choice-is+number-le-field", "code": "LLV7002", "version": "0.6.0", "count": 2},
        {"kind": "model-refused", "construct": "check", "code": "PP2014", "version": "0.6.0", "count": 1},
    ]));
    meta["traceparent"] = json!(TRACEPARENT);
    telemetry
        .begin_request(Some(&meta))
        .end("tools/call", None, None);
    let records = gap_records(&sink);
    assert_eq!(records.len(), 4);
    let kinds: Vec<&str> = records.iter().map(|r| r["gap.kind"].as_str()).collect();
    assert_eq!(kinds, ["refusal", "roundtrip", "unproved", "model-refused"]);
    assert_eq!(records[0]["gap.construct"], "fields.too-many");
    assert_eq!(records[0]["gap.code"], "none");
    assert_eq!(records[0]["gap.count"], "3");
    assert_eq!(records[0]["gap.install"], INSTALL);
    assert_eq!(records[1]["gap.version"], "0.6.1-rc.1");
    assert_eq!(records[2]["gap.code"], "LLV7002");
    // Each record is in the trace of its request, and its body is withheld like every other.
    let first = parsed(&sink)[0]["resourceLogs"][0]["scopeLogs"][0]["logRecords"][0].clone();
    assert_eq!(first["traceId"], "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(first["severityText"], "INFO");
    assert_eq!(first["body"]["stringValue"], "[redacted]");
    assert_eq!(
        gaps_of(Some(&meta)).expect("a batch").gaps[2].kind,
        GapKind::Unproved
    );
    assert_eq!(alerts(&sink), Vec::<String>::new());
    // A request without a batch records no gap.
    let (quiet, quiet_sink) = memory();
    quiet
        .begin_request(Some(&json!({"traceparent": TRACEPARENT})))
        .end("tools/call", None, None);
    assert!(gap_records(&quiet_sink).is_empty());
}

#[test]
fn a_gap_that_is_not_a_fingerprint_is_dropped_ob_16() {
    let (telemetry, sink) = memory();
    let good = json!({"kind": "refusal", "construct": "fields.none", "code": null, "version": "0.6.0", "count": 1});
    let mut gaps = vec![good.clone()];
    gaps.extend([
        json!({"kind": "refusal", "construct": SECRET, "code": null, "version": "0.6.0", "count": 1}),
        json!({"kind": "refusal", "construct": "fields.amount987654", "code": null, "version": "0.6.0", "count": 1}),
        json!({"kind": "refusal", "construct": "fields.too-many", "code": SECRET, "version": "0.6.0", "count": 1}),
        json!({"kind": "refusal", "construct": "fields.too-many", "code": "ZQX9876", "version": "0.6.0", "count": 1}),
        json!({"kind": "refusal", "construct": "fields.too-many", "code": null, "version": SECRET, "count": 1}),
        json!({"kind": "refusal", "construct": "fields.too-many", "code": null, "version": "0.6.0", "count": 0}),
        json!({"kind": "refusal", "construct": "fields.too-many", "code": null, "version": "0.6.0", "count": 987_654}),
        json!({"kind": SECRET, "construct": "fields.too-many", "code": null, "version": "0.6.0", "count": 1}),
        json!({"kind": "roundtrip", "construct": "secret:fields.none", "code": null, "version": "0.6.0", "count": 1}),
        json!({"kind": "unproved", "construct": "always:choice-is+amount", "code": null, "version": "0.6.0", "count": 1}),
        json!({"kind": "model-refused", "construct": "my-app", "code": null, "version": "0.6.0", "count": 1}),
        json!(SECRET),
    ]);
    // Text beside a valid fingerprint is not kept either.
    let mut beside = good.clone();
    beside["purpose"] = json!(SECRET);
    gaps.push(beside);
    telemetry
        .begin_request(Some(&batch(Value::Array(gaps))))
        .end("tools/call", None, None);
    assert_eq!(
        gap_records(&sink).len(),
        2,
        "the valid gap and the one with text beside it"
    );
    for line in sink.lines() {
        assert!(!line.contains("secretModuleText"), "{line}");
        assert!(!line.contains("987654"), "{line}");
        assert!(!line.contains("ZQX"), "{line}");
    }
    // A batch whose schema or install ID is not one is dropped whole: text of the ID's length, and
    // hexadecimal of another length, each alone.
    let mut wrong = batch(json!([good]));
    for install in [
        SECRET.to_owned(),
        "secretmoduletextsecretmoduletext".to_owned(),
        format!("{INSTALL}0"),
    ] {
        wrong[GAPS_META_KEY]["install"] = json!(install);
        assert_eq!(gaps_of(Some(&wrong)), None, "{install}");
    }
    wrong[GAPS_META_KEY]["install"] = json!(INSTALL);
    wrong[GAPS_META_KEY]["schema"] = json!("prism/gaps/2");
    assert_eq!(gaps_of(Some(&wrong)), None);
    // At most GAPS_LIMIT gaps of one request are read.
    let many = batch(Value::Array(vec![good; GAPS_LIMIT + 5]));
    assert_eq!(
        gaps_of(Some(&many)).expect("a batch").gaps.len(),
        GAPS_LIMIT
    );
}

#[test]
fn a_served_request_records_the_gaps_its_meta_carries_ob_16() {
    use gateway_host_api::protocol::Host;
    let directory = tempfile::tempdir().expect("a temporary directory");
    let (telemetry, sink) = memory();
    let mut host = Native::with_telemetry(
        gateway_binding::Config::at(directory.path().join("store")),
        telemetry,
    );
    let call = request(
        "tools/call",
        batch(
            json!([{"kind": "refusal", "construct": "outcome.unknown", "code": null, "version": "0.6.0", "count": 1}]),
        ),
    );
    host.request_begin(0, &call);
    host.request_end(0, &call, br#"{"jsonrpc":"2.0","id":1,"result":{}}"#);
    let records = gap_records(&sink);
    assert_eq!(records.len(), 1);
    assert_eq!(records[0]["gap.construct"], "outcome.unknown");
}

// OB-17: a tool outcome (S1) and a verification (S4) carry the profile and the code of the first
// error

#[test]
fn a_tool_outcome_and_a_verification_carry_profile_and_code_ob_17() {
    let (telemetry, sink) = memory();
    telemetry.tool_outcome(Some(ErrorClass::Language), "prism-app", Some("LLV7002"));
    telemetry.tool_outcome(None, "prism-app", None);
    telemetry.tool_outcome(Some(ErrorClass::Language), SECRET, Some(SECRET));
    telemetry.verify_duration("prism-app", VerifyOutcome::Failed, Some("LLV7005"), 10);
    telemetry.flush();
    let count = |labels: &[(&str, &str)]| telemetry.counter(names::TOOL_OUTCOMES, labels);
    assert_eq!(
        count(&[
            ("class", "1"),
            ("profile", "prism-app"),
            ("code", "LLV7002")
        ]),
        1
    );
    assert_eq!(
        count(&[("class", "ok"), ("profile", "prism-app"), ("code", "none")]),
        1
    );
    // Text that is not a profile name or a code is not kept.
    assert_eq!(
        count(&[("class", "1"), ("profile", "other"), ("code", "other")]),
        1
    );
    let verify = parsed(&sink)
        .iter()
        .flat_map(|line| {
            line["resourceMetrics"][0]["scopeMetrics"][0]["metrics"]
                .as_array()
                .cloned()
                .unwrap_or_default()
        })
        .find(|metric| metric["name"] == names::VERIFY_DURATION)
        .expect("S4");
    let attributes = verify["histogram"]["dataPoints"][0]["attributes"].to_string();
    for expected in ["prism-app", "failed", "LLV7005"] {
        assert!(attributes.contains(expected), "{attributes}");
    }
    for line in sink.lines() {
        assert!(!line.contains("secretModuleText"), "{line}");
    }
}

#[test]
fn the_first_error_code_is_read_from_the_result_ob_17() {
    let result =
        |diagnostics: Value| serde_json::to_vec(&json!({"diagnostics": diagnostics})).unwrap();
    assert_eq!(
        first_error_code(&result(json!([
            {"code": "LLV7002", "severity": "warning", "message": SECRET},
            {"code": "LLV7005", "severity": "error", "message": SECRET},
        ])))
        .as_deref(),
        Some("LLV7005"),
        "a warning is not the error"
    );
    assert_eq!(
        first_error_code(&result(json!([{"code": SECRET, "severity": "error"}]))),
        None
    );
    assert_eq!(first_error_code(&result(json!([]))), None);
    assert_eq!(first_error_code(b"not json"), None);
    assert_eq!(
        first_error_code(br#"{"failure":"verifier-lost","log":"boom"}"#),
        None
    );
}

#[test]
fn a_failed_verification_reports_its_code_and_profile_ob_17() {
    let dir = tempfile::tempdir().expect("a directory");
    let (telemetry, sink) = memory();
    let mut host = Native::with_telemetry(
        config(dir.path(), fake_verifier(dir.path())),
        telemetry.clone(),
    );
    install_draft(&host, None);
    install_named(&host, "t1", "failed-code");
    let (task, _) = start_named(&mut host, "t1").expect("a task");
    wait_for(&mut host, &task, State::Failed);
    assert_eq!(
        telemetry.counter(names::TOOL_OUTCOMES, &[("class", "1"), ("code", "LLV7002")]),
        1
    );
    telemetry.flush();
    for line in sink.lines() {
        assert!(!line.contains("secretModuleText"), "{line}");
    }
}
