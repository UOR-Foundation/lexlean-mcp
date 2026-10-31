//! OB-11 to OB-14: the observability runtime of issue #16. The model (`src/Observability.lex.tex`)
//! fixes the indicators, the alerts, the redaction and the trace context; these tests run what
//! the host emits against it. The verifier is the fake of `common/fake-verifier.sh`.

#[allow(dead_code)]
mod common;

use std::sync::Arc;

use common::{config, fake_verifier, install_draft, install_named, start_named, wait_for};
use gateway_binding::telemetry::{
    extract, firing, names, redact, Alert, ErrorClass, GroupEnd, MemorySink, Redacted, Sample,
    Telemetry, VerifyOutcome, BAGGAGE_LIMIT, TRACESTATE_LIMIT,
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
    telemetry.tool_outcome(None);
    telemetry.request_answered(Some(-32602));
    telemetry.check_latency("vocab", 120);
    telemetry.verify_duration("vocab", VerifyOutcome::Verified, 9_000);
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
        Alert::ClassTwo => telemetry.tool_outcome(Some(ErrorClass::CliOrConfiguration)),
        Alert::ClassThree => telemetry.tool_outcome(Some(ErrorClass::Environment)),
        Alert::ClassSeventy => telemetry.tool_outcome(Some(ErrorClass::Internal)),
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
    telemetry.tool_outcome(None);
    telemetry.tool_outcome(Some(ErrorClass::Language));
    telemetry.tool_outcome(Some(ErrorClass::SecurityOrLimit));
    telemetry.child_group_ends(GroupEnd::Ended, 3);
    telemetry.request_answered(None);
    telemetry.check_latency("vocab", 1);
    telemetry.verify_duration("vocab", VerifyOutcome::Failed, 1);
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
    telemetry.verify_duration(SECRET, VerifyOutcome::Failed, 5);
    telemetry.profile_drift(SECRET);
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
