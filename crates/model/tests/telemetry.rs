//! OB-11 to OB-14: the observability runtime of issue #16.
//!
//! The behavior is tested where the host runs: `gateway/bindings/gateway-binding/tests/
//! telemetry.rs`. These tests read it as text and hold the rows against the tests that answer
//! them, and the indicators against the places the host emits them.

use std::collections::BTreeSet;

const BINDING: &str = "gateway/bindings/gateway-binding";

fn read(relative: &str) -> String {
    std::fs::read_to_string(repo_model::repo_root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The tests of the binding that answer the row `id` are exactly `expected`.
fn answered_by(id: &str, expected: &[&str]) {
    let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
    let mut found = BTreeSet::new();
    let mut in_test = false;
    for line in read(&format!("{BINDING}/tests/telemetry.rs")).lines() {
        let line = line.trim();
        if line == "#[test]" {
            in_test = true;
        } else if in_test && line.starts_with("fn ") {
            let name = line[3..].split('(').next().expect("a name").to_string();
            if name.ends_with(&suffix) {
                found.insert(name);
            }
            in_test = false;
        }
    }
    let expected: BTreeSet<String> = expected.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(found, expected, "the tests of the binding that answer {id}");
    assert!(
        read("OBSERVABILITY.md").contains(id),
        "OBSERVABILITY.md does not name {id}"
    );
}

#[test]
fn every_indicator_is_emitted_ob_11() {
    answered_by(
        "OB-11",
        &[
            "every_indicator_is_emitted_ob_11",
            "the_indicator_names_are_the_names_of_the_model_ob_11",
            "a_verification_reports_its_indicators_ob_11",
        ],
    );
    // Each indicator has an emission site in the host, not only a method.
    let host = read(&format!("{BINDING}/src/lib.rs"))
        + &read(&format!("{BINDING}/src/verifier.rs"))
        + &read(&format!("{BINDING}/src/project.rs"));
    for site in [
        "tool_outcome(",
        "check_latency(",
        "verify_duration(",
        "queue_wait(",
        "child_group_ends(",
        "profile_drift(",
        "signing_failure(",
    ] {
        assert!(host.contains(site), "no emission site for {site}");
    }
}

#[test]
fn every_alert_fires_on_its_planted_condition_ob_12() {
    answered_by(
        "OB-12",
        &[
            "each_alert_fires_on_its_planted_condition_and_no_other_ob_12",
            "the_predicates_are_zero_tolerance_ob_12",
            "healthy_conditions_raise_no_alert_ob_12",
            "an_alert_counts_the_events_of_its_window_ob_12",
            "a_lost_verifier_raises_the_internal_class_alert_ob_12",
        ],
    );
}

#[test]
fn no_record_holds_module_text_ob_13() {
    answered_by(
        "OB-13",
        &[
            "no_telemetry_record_contains_module_text_ob_13",
            "a_span_carries_the_context_of_the_request_ob_13",
        ],
    );
    // The module takes no String it could keep: the text goes to `redact` and is dropped.
    let module = read(&format!("{BINDING}/src/telemetry.rs"));
    assert!(module.contains("pub fn redact(_text: &str) -> Redacted"));
}

#[test]
fn trace_context_and_no_notification_ob_14() {
    answered_by(
        "OB-14",
        &[
            "trace_context_is_read_from_meta_ob_14",
            "a_malformed_traceparent_restarts_the_trace_and_drops_tracestate_ob_14",
            "state_and_baggage_over_their_limits_are_dropped_whole_ob_14",
            "no_log_message_notification_is_emitted_ob_14",
        ],
    );
}
