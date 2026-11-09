//! OB-11 to OB-17: the observability runtime of issue #16.
//!
//! The behavior is tested where the host runs: `gateway/bindings/gateway-binding/tests/
//! telemetry.rs`. These tests read it as text and hold the rows against the tests that answer
//! them, and the indicators against the places the host emits them.

use std::collections::BTreeSet;

use serde_json::json;

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

#[test]
fn served_requests_end_their_own_spans_ob_15() {
    answered_by(
        "OB-15",
        &[
            "each_request_ends_its_own_span_with_its_method_and_error_ob_15",
            "a_failed_tool_is_the_class_of_its_own_request_ob_15",
        ],
    );
    // The host implements the generated host's per-request methods, keyed by the request's number.
    let host = read(&format!("{BINDING}/src/lib.rs"));
    assert!(host.contains("fn request_begin(&mut self, id: u64, request: &[u8])"));
    assert!(host.contains("fn request_end(&mut self, id: u64, request: &[u8], response: &[u8])"));
}

#[test]
fn gaps_are_recorded_as_fingerprints_ob_16() {
    answered_by(
        "OB-16",
        &[
            "the_gaps_of_a_request_are_recorded_as_fingerprints_ob_16",
            "a_gap_that_is_not_a_fingerprint_is_dropped_ob_16",
            "a_served_request_records_the_gaps_its_meta_carries_ob_16",
        ],
    );
    // The gaps are read where every request's _meta is: when its span begins.
    let module = read(&format!("{BINDING}/src/telemetry.rs"));
    assert!(module.contains("self.gaps(meta, extraction.parent.as_ref());"));
}

#[test]
fn outcomes_carry_profile_and_code_ob_17() {
    answered_by(
        "OB-17",
        &[
            "a_tool_outcome_and_a_verification_carry_profile_and_code_ob_17",
            "the_first_error_code_is_read_from_the_result_ob_17",
            "a_failed_verification_reports_its_code_and_profile_ob_17",
        ],
    );
    // Both emission sites of S1, and the one of S4, pass the code of the first error.
    let host =
        read(&format!("{BINDING}/src/lib.rs")) + &read(&format!("{BINDING}/src/verifier.rs"));
    assert_eq!(
        host.matches("first_error_code(").count(),
        2,
        "the Engine tools and the verifier"
    );
    assert!(host.contains(".tool_outcome(class, profile, code.as_deref())"));
    assert!(host.contains(".tool_outcome(class, &task.profile, code.as_deref())"));
}

// OB-18: the demand report

/// One gap record as the gateway writes it.
fn gap_line(kind: &str, construct: &str, code: &str, install: &str, count: u64) -> String {
    let attribute = |key: &str, value: &str| json!({"key": key, "value": {"stringValue": value}});
    json!({"resourceLogs": [{"scopeLogs": [{"logRecords": [{
        "severityText": "INFO",
        "body": {"stringValue": "[redacted]"},
        "attributes": [
            attribute("gap.kind", kind),
            attribute("gap.construct", construct),
            attribute("gap.code", code),
            attribute("gap.version", "0.6.0"),
            attribute("gap.count", &count.to_string()),
            attribute("gap.install", install),
            attribute("event", "gap"),
        ],
    }]}]}]})
    .to_string()
}

/// One export of S1 with a failed series of `count`.
fn outcome_line(count: u64) -> String {
    let attribute = |key: &str, value: &str| json!({"key": key, "value": {"stringValue": value}});
    json!({"resourceMetrics": [{"scopeMetrics": [{"metrics": [{
        "name": "lexlean_mcp.tool.outcomes",
        "sum": {"dataPoints": [
            {"attributes": [attribute("class", "1"), attribute("profile", "prism-app"), attribute("code", "LLV7002")], "asInt": count.to_string()},
            {"attributes": [attribute("class", "ok"), attribute("profile", "prism-app"), attribute("code", "none")], "asInt": "40"},
        ]},
    }]}]}]})
    .to_string()
}

/// Run `deploy/demand.py` with a `gh` first on the PATH that only records that it ran, in `bin`.
fn demand(bin: &std::path::Path, args: &[&str]) -> String {
    use std::os::unix::fs::PermissionsExt;
    let gh = bin.join("gh");
    std::fs::write(
        &gh,
        format!("#!/bin/sh\ntouch '{}/gh-ran'\nexit 1\n", bin.display()),
    )
    .expect("a stand-in gh");
    std::fs::set_permissions(&gh, std::fs::Permissions::from_mode(0o755)).expect("executable");
    let path = format!(
        "{}:{}",
        bin.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let output = std::process::Command::new("python3")
        .arg("-I")
        .arg(repo_model::repo_root().join("deploy/demand.py"))
        .args(args)
        .env("PATH", path)
        .output()
        .expect("python3 runs deploy/demand.py");
    assert!(
        output.status.success(),
        "demand.py {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8")
}

#[test]
fn the_demand_report_ranks_gaps_by_installs_times_and_blocking_ob_18() {
    let a = "a".repeat(32);
    let b = "b".repeat(32);
    let bin = std::env::temp_dir().join(format!("demand-{}", std::process::id()));
    std::fs::create_dir_all(&bin).expect("a directory");
    let file = bin.join("otlp.jsonl");
    let lines = [
        gap_line("refusal", "fields.too-many", "none", &a, 2),
        gap_line("refusal", "fields.too-many", "none", &b, 1),
        gap_line("unproved", "always:choice-is", "LLV7002", &a, 2),
        gap_line("refusal", "outcome.unknown", "none", &a, 1),
        // Forged lines: text where a fingerprint has a value. None of them is counted.
        gap_line("refusal", "fields.amount987654", "none", &a, 1),
        gap_line("refusal", "fields.none", "none", "secretinstall", 1),
        gap_line("refusal", "fields.none", "secret", &a, 1),
        outcome_line(1),
        outcome_line(3),
        "not json".to_owned(),
    ];
    std::fs::write(&file, lines.join("\n")).expect("a fixture");
    let path = file.to_str().expect("a path");
    let report: serde_json::Value =
        serde_json::from_str(&demand(&bin, &["report", "--json", path])).expect("JSON");
    let ranked: Vec<(String, u64, u64, bool, u64)> = report["gaps"]
        .as_array()
        .expect("gaps")
        .iter()
        .map(|row| {
            (
                row["construct"].as_str().unwrap().to_owned(),
                row["users"].as_u64().unwrap(),
                row["count"].as_u64().unwrap(),
                row["blocks"].as_bool().unwrap(),
                row["score"].as_u64().unwrap(),
            )
        })
        .collect();
    assert_eq!(
        ranked,
        [
            ("fields.too-many".to_owned(), 2, 3, false, 6),
            ("always:choice-is".to_owned(), 1, 2, true, 4),
            ("outcome.unknown".to_owned(), 1, 1, false, 1),
        ]
    );
    assert_eq!(
        report["failures"],
        json!([{"class": 1, "profile": "prism-app", "code": "LLV7002", "count": 3}]),
        "a cumulative series counts its largest value"
    );
    let markdown = demand(&bin, &["report", path]);
    for text in [&markdown, &report.to_string()] {
        assert!(
            !text.contains("987654") && !text.contains("secret"),
            "{text}"
        );
    }
    // Without --apply the issues are only planned, and gh never runs.
    let plan = demand(&bin, &["issues", "--repo", "example/repo", path]);
    assert_eq!(
        plan.lines().next(),
        Some("would open or update: demand: refusal fields.too-many (score 6)")
    );
    assert!(!bin.join("gh-ran").exists(), "gh ran without --apply");
    std::fs::remove_dir_all(&bin).ok();
    assert!(read("OBSERVABILITY.md").contains("OB-18"));
}
