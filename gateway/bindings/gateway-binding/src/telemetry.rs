//! The observability of the host: OpenTelemetry records for the logs, metrics and traces of
//! `src/Observability.lex.tex`, the alerts on the service level indicators, and the trace context
//! of a request.
//!
//! The model fixes what is emitted; this module emits it. The records are OTLP/JSON, one line each,
//! written to a sink: a file for the operator's collector (`otlpjsonfile` receiver), standard
//! error, or memory for a test. MCP Logging is deprecated, so nothing here is a JSON-RPC
//! notification and `notifications/message` is never emitted (OB-09: `EMITTED` is the whole set).
//!
//! **Redaction is a type.** No entry point takes the text of a module, an argument or a lexicon
//! search query and keeps it. The few places a caller has such text accept it only to turn it into
//! `Redacted`, which has one value and not even the length (OB-04). Profile labels are checked
//! against the grammar of a profile name, so a text that is not one is labelled `other`.

use std::collections::{BTreeMap, VecDeque};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

use rustix::rand::{getrandom, GetRandomFlags};
use serde_json::{json, Map, Value};

/// The notifications the server emits (OB-09): `notifications/progress` and no other. A log
/// message is never among them.
pub const EMITTED: &[&str] = &["notifications/progress"];

/// The environment variable that names the telemetry sink: `stderr`, or the file that OTLP/JSON
/// lines are appended to. Unset, nothing is exported (the alerts still count).
pub const TELEMETRY_VARIABLE: &str = "LEXLEAN_MCP_TELEMETRY";

/// `tracestate` longer than this is dropped whole (this product's choice, OB-08).
pub const TRACESTATE_LIMIT: usize = 512;

/// `baggage` longer than this is dropped whole: the minimum a platform must propagate (W3C
/// Baggage 3.3.2).
pub const BAGGAGE_LIMIT: usize = 8192;

/// The class of a failed tool, by the exit codes of LexLean's `ErrorClass` (SPEC 24.5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorClass {
    /// Exit code 1: a bad draft.
    Language,
    /// Exit code 2: a bad configuration or selection.
    CliOrConfiguration,
    /// Exit code 3: the environment.
    Environment,
    /// Exit code 4: a security or limit refusal.
    SecurityOrLimit,
    /// Exit code 70: an internal fault.
    Internal,
}

impl ErrorClass {
    /// The class of a number the host reports (`engine::class_number`: 1 to 4, and 5 for internal).
    pub fn from_number(number: u32) -> Option<ErrorClass> {
        match number {
            1 => Some(ErrorClass::Language),
            2 => Some(ErrorClass::CliOrConfiguration),
            3 => Some(ErrorClass::Environment),
            4 => Some(ErrorClass::SecurityOrLimit),
            5 | 70 => Some(ErrorClass::Internal),
            _ => None,
        }
    }

    /// The exit code of the class.
    pub fn exit_code(self) -> u32 {
        match self {
            ErrorClass::Language => 1,
            ErrorClass::CliOrConfiguration => 2,
            ErrorClass::Environment => 3,
            ErrorClass::SecurityOrLimit => 4,
            ErrorClass::Internal => 70,
        }
    }
}

/// How a verification ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerifyOutcome {
    /// Verified.
    Verified,
    /// Failed.
    Failed,
    /// Cancelled.
    Cancelled,
}

impl VerifyOutcome {
    fn label(self) -> &'static str {
        match self {
            VerifyOutcome::Verified => "verified",
            VerifyOutcome::Failed => "failed",
            VerifyOutcome::Cancelled => "cancelled",
        }
    }
}

/// How the end of a child group came out (LexLean SPEC 24.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupEnd {
    /// The group was ended.
    Ended,
    /// The group could not be ended.
    Failed,
}

impl GroupEnd {
    fn label(self) -> &'static str {
        match self {
            GroupEnd::Ended => "ended",
            GroupEnd::Failed => "failed",
        }
    }
}

/// The alerts of the model: classes 2, 3 and 70, a failed child group end, a drift and a signing
/// failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Alert {
    /// A tool ended in class 2.
    ClassTwo,
    /// A tool ended in class 3.
    ClassThree,
    /// A tool ended in class 70.
    ClassSeventy,
    /// A child group could not be ended.
    GroupEndFailed,
    /// A profile drifted.
    Drift,
    /// A verification could not be signed.
    SigningFailure,
}

impl Alert {
    /// Every alert.
    pub const ALL: [Alert; 6] = [
        Alert::ClassTwo,
        Alert::ClassThree,
        Alert::ClassSeventy,
        Alert::GroupEndFailed,
        Alert::Drift,
        Alert::SigningFailure,
    ];

    /// The name of the alert in a record.
    pub fn name(self) -> &'static str {
        match self {
            Alert::ClassTwo => "tool.class_2",
            Alert::ClassThree => "tool.class_3",
            Alert::ClassSeventy => "tool.class_70",
            Alert::GroupEndFailed => "child_group.end_failed",
            Alert::Drift => "profile.drift",
            Alert::SigningFailure => "signing.failure",
        }
    }

    /// The window of the objective that raises it, in milliseconds (`slo.S1`, `S6`, `S7`, `S8`).
    pub fn window_ms(self) -> u64 {
        match self {
            Alert::Drift => 3_600_000,
            _ => 300_000,
        }
    }
}

/// What an alert is evaluated on: the count of an event inside the window of its objective.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sample {
    /// Tools that ended in this class.
    ToolOutcome(ErrorClass, u64),
    /// Child group ends of this kind.
    ChildGroupEnds(GroupEnd, u64),
    /// Profiles that drifted.
    ProfileDrift(u64),
    /// Signing failures.
    SigningFailures(u64),
}

/// The zero tolerance of `slo.S1`, `S6`, `S7` and `S8`: any count above it raises the alert.
pub const THRESHOLD: u64 = 0;

/// Whether `alert` fires on `sample` (`firing` of the model).
pub fn firing(alert: Alert, sample: Sample) -> bool {
    match (alert, sample) {
        (Alert::ClassTwo, Sample::ToolOutcome(ErrorClass::CliOrConfiguration, count))
        | (Alert::ClassThree, Sample::ToolOutcome(ErrorClass::Environment, count))
        | (Alert::ClassSeventy, Sample::ToolOutcome(ErrorClass::Internal, count))
        | (Alert::GroupEndFailed, Sample::ChildGroupEnds(GroupEnd::Failed, count))
        | (Alert::Drift, Sample::ProfileDrift(count))
        | (Alert::SigningFailure, Sample::SigningFailures(count)) => count > THRESHOLD,
        _ => false,
    }
}

/// The text that a record would have held. It has one value: not even the length is kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Redacted {
    /// The text was withheld.
    Withheld,
}

/// Withhold `text`. Every free-text field of a record passes through here.
pub fn redact(_text: &str) -> Redacted {
    Redacted::Withheld
}

impl Redacted {
    fn value(self) -> &'static str {
        "[redacted]"
    }
}

/// What a request carried in `_meta`: the parent of its trace, and the state and baggage to pass on.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Extraction {
    /// The trace and span the request continues, when its `traceparent` parsed.
    pub parent: Option<Parent>,
    /// The `tracestate`, when it is within the limit and followed a valid `traceparent`.
    pub tracestate: Option<String>,
    /// The `baggage`, when it is within the limit.
    pub baggage: Option<String>,
}

/// A W3C `traceparent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Parent {
    /// The trace.
    pub trace_id: [u8; 16],
    /// The span of the caller.
    pub parent_id: [u8; 8],
    /// The trace flags.
    pub flags: u8,
}

fn hex_digits(text: &[u8]) -> Option<Vec<u8>> {
    // W3C Trace Context 3.2: lowercase hexadecimal only.
    let nibble = |c: u8| match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        _ => None,
    };
    text.chunks(2)
        .map(|pair| Some(nibble(pair[0])? << 4 | nibble(*pair.get(1)?)?))
        .collect()
}

/// The `traceparent` in `text`, or `None` when it is malformed (W3C Trace Context 3.2, 4.3).
pub fn parse_traceparent(text: &str) -> Option<Parent> {
    let bytes = text.as_bytes();
    if bytes.len() < 55 || bytes[2] != b'-' || bytes[35] != b'-' || bytes[52] != b'-' {
        return None;
    }
    let version = hex_digits(&bytes[0..2])?[0];
    // `ff` is forbidden; version 00 is exactly 55 bytes; a later version may only extend with `-`.
    if version == 0xff || (version == 0 && bytes.len() != 55) {
        return None;
    }
    if bytes.len() > 55 && bytes[55] != b'-' {
        return None;
    }
    let trace_id: [u8; 16] = hex_digits(&bytes[3..35])?.try_into().ok()?;
    let parent_id: [u8; 8] = hex_digits(&bytes[36..52])?.try_into().ok()?;
    let flags = hex_digits(&bytes[53..55])?[0];
    if trace_id == [0; 16] || parent_id == [0; 8] {
        return None;
    }
    Some(Parent {
        trace_id,
        parent_id,
        flags,
    })
}

/// The trace context of a request's `_meta`. A header that fails to parse is ignored and the trace
/// restarts; a failure never fails the request. `tracestate` is not read after a `traceparent`
/// that did not parse. A repeated `traceparent` cannot be one value, so it is not parsed (JSON
/// objects keep one value for a key, so a request with two arrives here with the last).
pub fn extract(meta: Option<&Value>) -> Extraction {
    let text = |key: &str| meta?.get(key)?.as_str();
    let parent = text("traceparent").and_then(parse_traceparent);
    let tracestate = parent
        .and(text("tracestate"))
        .filter(|state| state.len() <= TRACESTATE_LIMIT)
        .map(str::to_owned);
    let baggage = text("baggage")
        .filter(|baggage| baggage.len() <= BAGGAGE_LIMIT)
        .map(str::to_owned);
    Extraction {
        parent,
        tracestate,
        baggage,
    }
}

/// Where the records go.
pub trait Sink: Send + Sync {
    /// Take one line of OTLP/JSON.
    fn write(&self, line: &str);
}

/// A sink that appends the lines to a file.
pub struct FileSink(Mutex<std::fs::File>);

impl FileSink {
    /// A sink that appends to `path`, or `None` when it cannot be opened.
    pub fn open(path: &std::path::Path) -> Option<FileSink> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .ok()?;
        Some(FileSink(Mutex::new(file)))
    }
}

impl Sink for FileSink {
    fn write(&self, line: &str) {
        let mut file = self.0.lock().unwrap_or_else(PoisonError::into_inner);
        let _ = writeln!(file, "{line}");
    }
}

struct StderrSink;

impl Sink for StderrSink {
    fn write(&self, line: &str) {
        eprintln!("{line}");
    }
}

/// A sink that keeps the lines, for a test.
#[derive(Default)]
pub struct MemorySink(Mutex<Vec<String>>);

impl MemorySink {
    /// The lines written so far.
    pub fn lines(&self) -> Vec<String> {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }
}

impl Sink for MemorySink {
    fn write(&self, line: &str) {
        self.0
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .push(line.to_owned());
    }
}

/// The sink the environment names, or `None` when it names none.
fn sink_from(value: Option<&str>) -> Option<Arc<dyn Sink>> {
    match value? {
        "" => None,
        "stderr" => Some(Arc::new(StderrSink)),
        path => FileSink::open(std::path::Path::new(path)).map(|sink| Arc::new(sink) as _),
    }
}

/// A profile name is a key segment: lowercase letters, digits, `.`, `_` and `-`, at most 64.
fn profile_label(profile: &str) -> &str {
    let valid = !profile.is_empty()
        && profile.len() <= 64
        && profile
            .bytes()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || b".-_".contains(&c));
    if valid {
        profile
    } else {
        "other"
    }
}

/// Bounds of the duration histograms, in milliseconds.
const BUCKETS: [u64; 11] = [
    10, 50, 100, 500, 1_000, 5_000, 30_000, 120_000, 300_000, 600_000, 1_800_000,
];

#[derive(Default)]
struct Histogram {
    counts: [u64; BUCKETS.len() + 1],
    sum: u64,
    count: u64,
}

impl Histogram {
    fn record(&mut self, ms: u64) {
        let at = BUCKETS.iter().position(|bound| ms <= *bound);
        self.counts[at.unwrap_or(BUCKETS.len())] += 1;
        self.sum = self.sum.saturating_add(ms);
        self.count += 1;
    }
}

type Labels = Vec<(&'static str, String)>;

#[derive(Default)]
struct Metrics {
    counters: BTreeMap<(&'static str, Labels), u64>,
    histograms: BTreeMap<(&'static str, Labels), Histogram>,
    /// The instants of each alerting event, newest last, within the window of its objective.
    recent: BTreeMap<Alert, VecDeque<u64>>,
}

/// The SLI names of the model (`sliDefs`).
pub mod names {
    /// S1: tool outcomes by class.
    pub const TOOL_OUTCOMES: &str = "lexlean_mcp.tool.outcomes";
    /// S2: protocol errors by code.
    pub const PROTOCOL_ERRORS: &str = "lexlean_mcp.protocol.errors";
    /// The requests that S2 is a share of.
    pub const REQUESTS: &str = "lexlean_mcp.requests";
    /// S3: check latency.
    pub const CHECK_DURATION: &str = "lexlean_mcp.check.duration";
    /// S4: verify duration.
    pub const VERIFY_DURATION: &str = "lexlean_mcp.verify.duration";
    /// S5: verifier queue wait.
    pub const QUEUE_WAIT: &str = "lexlean_mcp.verify.queue.wait";
    /// S6: child group ends.
    pub const CHILD_GROUP_ENDS: &str = "lexlean_mcp.child_group.ends";
    /// S7: profile drift.
    pub const PROFILE_DRIFT: &str = "lexlean_mcp.profile.drift";
    /// S8: signing failures.
    pub const SIGNING_FAILURES: &str = "lexlean_mcp.signing.failures";
}

struct Inner {
    sink: Option<Arc<dyn Sink>>,
    now_ms: Box<dyn Fn() -> u64 + Send + Sync>,
    metrics: Mutex<Metrics>,
}

/// The telemetry of a host. Cloning gives another handle to the same counters and sink.
#[derive(Clone)]
pub struct Telemetry {
    inner: Arc<Inner>,
}

fn wall_ms() -> u64 {
    let since = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| since.as_millis());
    u64::try_from(since).unwrap_or(u64::MAX)
}

fn random<const N: usize>() -> [u8; N] {
    let mut bytes = [0u8; N];
    let _ = getrandom(&mut bytes, GetRandomFlags::empty());
    // An all-zero id is invalid (W3C Trace Context); a failed read must not produce one.
    if bytes == [0; N] {
        bytes[N - 1] = 1;
    }
    bytes
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn attribute(key: &str, value: &str) -> Value {
    json!({"key": key, "value": {"stringValue": value}})
}

fn resource() -> Value {
    json!({"attributes": [attribute("service.name", "lexlean-mcp-gateway")]})
}

fn nanos(ms: u64) -> String {
    ms.saturating_mul(1_000_000).to_string()
}

impl Telemetry {
    /// Telemetry that exports to `sink`, or nowhere.
    pub fn new(sink: Option<Arc<dyn Sink>>) -> Telemetry {
        Telemetry::with_clock(sink, wall_ms)
    }

    /// Telemetry with the clock `now_ms`, in Unix milliseconds. A test moves the windows with it.
    pub fn with_clock(
        sink: Option<Arc<dyn Sink>>,
        now_ms: impl Fn() -> u64 + Send + Sync + 'static,
    ) -> Telemetry {
        Telemetry {
            inner: Arc::new(Inner {
                sink,
                now_ms: Box::new(now_ms),
                metrics: Mutex::new(Metrics::default()),
            }),
        }
    }

    /// Telemetry for the sink `LEXLEAN_MCP_TELEMETRY` names.
    pub fn from_environment() -> Telemetry {
        let value = std::env::var(TELEMETRY_VARIABLE).ok();
        Telemetry::new(sink_from(value.as_deref()))
    }

    /// Telemetry that appends to `path`.
    pub fn to_file(path: impl Into<PathBuf>) -> Telemetry {
        let sink = FileSink::open(&path.into()).map(|sink| Arc::new(sink) as Arc<dyn Sink>);
        Telemetry::new(sink)
    }

    fn emit(&self, line: &Value) {
        if let Some(sink) = &self.inner.sink {
            sink.write(&line.to_string());
        }
    }

    fn metrics(&self) -> std::sync::MutexGuard<'_, Metrics> {
        self.inner
            .metrics
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
    }

    fn count(&self, name: &'static str, labels: Labels) {
        *self.metrics().counters.entry((name, labels)).or_default() += 1;
    }

    fn observe(&self, name: &'static str, labels: Labels, ms: u64) {
        self.metrics()
            .histograms
            .entry((name, labels))
            .or_default()
            .record(ms);
    }

    /// The value of a counter, for a test and for the operator's own check.
    pub fn counter(&self, name: &str, labels: &[(&str, &str)]) -> u64 {
        self.metrics()
            .counters
            .iter()
            .filter(|((counter, have), _)| {
                *counter == name
                    && labels
                        .iter()
                        .all(|(key, value)| have.iter().any(|(k, v)| k == key && v == value))
            })
            .map(|(_, count)| *count)
            .sum()
    }

    /// A log record. `body` is withheld: a record states which event happened, never its text.
    fn log(&self, severity: &str, event: &str, attributes: Vec<Value>, parent: Option<&Parent>) {
        let mut attributes = attributes;
        attributes.push(attribute("event", event));
        let mut record = json!({
            "timeUnixNano": nanos((self.inner.now_ms)()),
            "severityText": severity,
            "body": {"stringValue": Redacted::Withheld.value()},
            "attributes": attributes,
        });
        if let Some(parent) = parent {
            record["traceId"] = json!(hex(&parent.trace_id));
            record["spanId"] = json!(hex(&parent.parent_id));
        }
        self.emit(&json!({"resourceLogs": [{
            "resource": resource(),
            "scopeLogs": [{"scope": {"name": "lexlean-mcp"}, "logRecords": [record]}],
        }]}));
    }

    /// Count an alerting event and fire every alert that now holds inside its window.
    fn alerting(&self, alert: Alert, sample: impl Fn(u64) -> Sample) {
        let now = (self.inner.now_ms)();
        let count = {
            let mut metrics = self.metrics();
            let recent = metrics.recent.entry(alert).or_default();
            recent.push_back(now);
            while recent
                .front()
                .is_some_and(|at| now.saturating_sub(*at) > alert.window_ms())
            {
                recent.pop_front();
            }
            recent.len() as u64
        };
        if firing(alert, sample(count)) {
            self.log(
                "ERROR",
                "alert",
                vec![
                    attribute("alert", alert.name()),
                    attribute("count", &count.to_string()),
                ],
                None,
            );
        }
    }

    /// S1: a tool ended. `None` is a tool that succeeded.
    pub fn tool_outcome(&self, class: Option<ErrorClass>) {
        let label = class.map_or("ok".to_owned(), |class| class.exit_code().to_string());
        self.count(names::TOOL_OUTCOMES, vec![("class", label)]);
        let Some(class) = class else { return };
        let alert = match class {
            ErrorClass::CliOrConfiguration => Alert::ClassTwo,
            ErrorClass::Environment => Alert::ClassThree,
            ErrorClass::Internal => Alert::ClassSeventy,
            ErrorClass::Language | ErrorClass::SecurityOrLimit => return,
        };
        self.alerting(alert, |count| Sample::ToolOutcome(class, count));
    }

    /// S2: a request was answered, with a protocol error of `code` or none.
    pub fn request_answered(&self, protocol_error: Option<i64>) {
        self.count(names::REQUESTS, Vec::new());
        if let Some(code) = protocol_error {
            self.count(names::PROTOCOL_ERRORS, vec![("code", code.to_string())]);
        }
    }

    /// S3: a check took `ms` on `profile`.
    pub fn check_latency(&self, profile: &str, ms: u64) {
        let labels = vec![("profile", profile_label(profile).to_owned())];
        self.observe(names::CHECK_DURATION, labels, ms);
    }

    /// S4: a verification ended after `ms`.
    pub fn verify_duration(&self, profile: &str, outcome: VerifyOutcome, ms: u64) {
        let labels = vec![
            ("profile", profile_label(profile).to_owned()),
            ("outcome", outcome.label().to_owned()),
        ];
        self.observe(names::VERIFY_DURATION, labels, ms);
    }

    /// S5: a verification waited `ms` for a slot.
    pub fn queue_wait(&self, ms: u64) {
        self.observe(names::QUEUE_WAIT, Vec::new(), ms);
    }

    /// S6: `count` child groups ended as `end`.
    pub fn child_group_ends(&self, end: GroupEnd, count: u32) {
        for _ in 0..count {
            self.count(
                names::CHILD_GROUP_ENDS,
                vec![("outcome", end.label().to_owned())],
            );
            if end == GroupEnd::Failed {
                self.alerting(Alert::GroupEndFailed, |count| {
                    Sample::ChildGroupEnds(GroupEnd::Failed, count)
                });
            }
        }
    }

    /// S7: `profile` drifted: its record no longer names the profile of a draft.
    pub fn profile_drift(&self, profile: &str) {
        let labels = vec![("profile", profile_label(profile).to_owned())];
        self.count(names::PROFILE_DRIFT, labels);
        self.alerting(Alert::Drift, Sample::ProfileDrift);
    }

    /// S8: a verification could not be signed or recorded.
    pub fn signing_failure(&self) {
        self.count(names::SIGNING_FAILURES, Vec::new());
        self.alerting(Alert::SigningFailure, Sample::SigningFailures);
    }

    /// Begin the span of a request, continuing the trace its `_meta` carries.
    pub fn begin_request(&self, meta: Option<&Value>) -> RequestScope {
        RequestScope {
            telemetry: self.clone(),
            extraction: extract(meta),
            span_id: random::<8>(),
            fresh_trace: random::<16>(),
            started: (self.inner.now_ms)(),
            arguments: Redacted::Withheld,
            query: Redacted::Withheld,
        }
    }

    /// Export the metrics as one OTLP/JSON line: sums for the counters, histograms for the
    /// durations. The values are cumulative since the host began.
    pub fn flush(&self) {
        let now = nanos((self.inner.now_ms)());
        let attributes = |labels: &Labels| {
            labels
                .iter()
                .map(|(key, value)| attribute(key, value))
                .collect::<Vec<_>>()
        };
        let mut by_name: BTreeMap<&str, Value> = BTreeMap::new();
        let metrics = self.metrics();
        for ((name, labels), count) in &metrics.counters {
            let metric = by_name.entry(name).or_insert_with(|| {
                json!({"name": name, "sum": {"aggregationTemporality": 2, "isMonotonic": true, "dataPoints": []}})
            });
            metric["sum"]["dataPoints"]
                .as_array_mut()
                .into_iter()
                .for_each(|points| {
                    points.push(json!({
                        "attributes": attributes(labels),
                        "timeUnixNano": now,
                        "asInt": count.to_string(),
                    }));
                });
        }
        for ((name, labels), histogram) in &metrics.histograms {
            let metric = by_name.entry(name).or_insert_with(|| {
                json!({"name": name, "unit": "ms", "histogram": {"aggregationTemporality": 2, "dataPoints": []}})
            });
            metric["histogram"]["dataPoints"]
                .as_array_mut()
                .into_iter()
                .for_each(|points| {
                    points.push(json!({
                    "attributes": attributes(labels),
                    "timeUnixNano": now,
                    "count": histogram.count.to_string(),
                    "sum": histogram.sum,
                    "bucketCounts": histogram.counts.iter().map(u64::to_string).collect::<Vec<_>>(),
                    "explicitBounds": BUCKETS,
                }));
                });
        }
        let list: Vec<Value> = by_name.into_values().collect();
        drop(metrics);
        self.emit(&json!({"resourceMetrics": [{
            "resource": resource(),
            "scopeMetrics": [{"scope": {"name": "lexlean-mcp"}, "metrics": list}],
        }]}));
    }
}

/// The span of a request in flight.
pub struct RequestScope {
    telemetry: Telemetry,
    extraction: Extraction,
    span_id: [u8; 8],
    fresh_trace: [u8; 16],
    started: u64,
    arguments: Redacted,
    query: Redacted,
}

impl RequestScope {
    /// The trace context the request carried.
    pub fn extraction(&self) -> &Extraction {
        &self.extraction
    }

    /// The arguments of the call. They are withheld: the text is not kept.
    pub fn arguments(&mut self, text: &str) {
        self.arguments = redact(text);
    }

    /// A lexicon search query. It is withheld: the text is not kept.
    pub fn query(&mut self, text: &str) {
        self.query = redact(text);
    }

    /// End the span: the method, the failure class of the tool if any, and the protocol error code
    /// if any. Counts the request and its protocol error (S2). The tool outcome (S1) is counted
    /// where the host learns it, once for each tool, so it is not counted here again.
    pub fn end(self, method: &str, tool: Option<ErrorClass>, protocol_error: Option<i64>) {
        let telemetry = &self.telemetry;
        telemetry.request_answered(protocol_error);
        let now = (telemetry.inner.now_ms)();
        let trace_id = self
            .extraction
            .parent
            .map_or(self.fresh_trace, |p| p.trace_id);
        let failure = tool.map(|class| class.exit_code().to_string());
        let mut attributes = vec![
            attribute("rpc.method", method),
            attribute("arguments", self.arguments.value()),
            attribute("query", self.query.value()),
            attribute("status", Redacted::Withheld.value()),
        ];
        if let Some(code) = protocol_error {
            attributes.push(attribute("rpc.error_code", &code.to_string()));
        }
        if let Some(class) = failure {
            attributes.push(attribute("error.class", &class));
        }
        if let Some(baggage) = &self.extraction.baggage {
            attributes.push(attribute("baggage", baggage));
        }
        let mut span = Map::new();
        span.insert("traceId".into(), json!(hex(&trace_id)));
        span.insert("spanId".into(), json!(hex(&self.span_id)));
        span.insert("name".into(), json!(method));
        span.insert("kind".into(), json!(2));
        span.insert("startTimeUnixNano".into(), json!(nanos(self.started)));
        span.insert("endTimeUnixNano".into(), json!(nanos(now)));
        span.insert("attributes".into(), json!(attributes));
        if let Some(parent) = self.extraction.parent {
            span.insert("parentSpanId".into(), json!(hex(&parent.parent_id)));
            span.insert("flags".into(), json!(parent.flags));
        }
        if let Some(state) = &self.extraction.tracestate {
            span.insert("traceState".into(), json!(state));
        }
        if protocol_error.is_some() || tool.is_some() {
            span.insert("status".into(), json!({"code": 2}));
        }
        telemetry.emit(&json!({"resourceSpans": [{
            "resource": resource(),
            "scopeSpans": [{"scope": {"name": "lexlean-mcp"}, "spans": [Value::Object(span)]}],
        }]}));
    }
}
