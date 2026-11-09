# Observability model

`src/Observability.lex.tex` is the model-side slice of issue #16: the telemetry
records with redaction as a type, the service level indicators, the alert
predicates, trace context extraction and the closed set of notifications the
server emits. This file is project-owned evidence; the register rows are OB-01
to OB-10 in `model/ids.toml`, and `cargo test -p repo-model --test observability`
holds the module, this file and the authorities to one statement.

## What is modeled, and what is not

| Part | Where | Status |
| --- | --- | --- |
| Telemetry records whose free-text fields are `Redacted` | `Redacted`, `LogRecord`, `SpanRecord`, `toTelemetry` | modeled, kernel-checked (OB-04) |
| The indicators S1 to S8 | `sliDefs`, `Sample` | modeled (OB-05) |
| Alert predicates on classes 2, 3 and 70, a failed child group end, a drift and a signing failure | `firing` | modeled, planted and healthy instances (OB-06) |
| `traceparent`, `tracestate`, `baggage` from a request's `_meta` | `parseTraceparent`, `extract` | modeled (OB-07, OB-08) |
| The server notifications that can be emitted | `Emitted` | modeled (OB-09) |
| Emission as OpenTelemetry logs, metrics and spans, the alerts, redaction and the trace context in the host | `gateway/bindings/gateway-binding/src/telemetry.rs` | built, tested without Lean (OB-11 to OB-14, below) |
| The gaps prism-code reports in `_meta`, and the profile and diagnostic code on S1 and S4 | `gaps_of`, `Telemetry::gaps`, `first_error_code` | built, tested without Lean (OB-16, OB-17, below); not in the model, which they leave as it is: they add no free-text field, only labels from closed grammars |
| Spans of the requests, and the protocol errors by code | `Telemetry::begin_request`, `RequestScope::end` | built; the driver of the gateway binary calls them (below) |
| A collector receiving the file | `deploy/images/collector.Containerfile` | built; an alert reaches it in the run of the images (IM-04, `IMAGES.md`) |
| Alert routing | the operator's | not built here |

## The runtime

`telemetry.rs` in the gateway binding emits what the model fixes. The records are
OTLP/JSON, one line each, appended to the file that `LEXLEAN_MCP_TELEMETRY` names
(`stderr` writes them to standard error; unset exports nothing and the alerts still
count). A collector reads the file with its `otlpjsonfile` receiver. The model's
names are the metric names; `lexlean_mcp.requests` is the denominator of S2.

| Indicator | Emitted where |
| --- | --- |
| S1 tool outcome by class, profile and code | the four Engine tools in `engine_call`, and a verification when it ends (`verifier::end_telemetry`); class 5 of the host is exit code 70 |
| S2 protocol errors by code | `RequestScope::end` |
| S3 check latency | `engine_call` for `check`, labelled by the profile of the draft handle |
| S4 verify duration and outcome, by profile and code | `verifier::end_telemetry` |
| S5 queue wait | `verifier::launch`, from the queueing to the start of the process |
| S6 child group ends | `verifier::settle`, from the groups a cancelled process reports |
| S7 profile drift | `materialize_noting`: the profile record no longer names the identity a draft was made for |
| S8 signing failures | `verify_start` when the key does not load, and `verifier::conclude_with` when a verified task cannot be signed or recorded |

An alert is an `ERROR` log record whose `alert` attribute names it and whose `count`
is the number of its events inside the window of its objective (300 s; 3600 s for
drift). The thresholds are zero, so one event fires it. The only other log record
is a gap (below). The body of every record is `[redacted]`, and the arguments, query
and status of a span are the same. No entry point keeps the text of a module, an
argument or a lexicon search query. A profile label that is not a profile name is
`other`.
`notifications/message` is never emitted: nothing in the module is a JSON-RPC
message, which the tests check (`no_log_message_notification_is_emitted_ob_14`); the
model's `Emitted` is the set of notifications the server may send.

The served driver of the gateway (PrismPM's generated host, at the pin of
`.github/prismpm.rev`) calls two methods of the host for each request it answers
(UOR-Foundation/PrismPM#429): `request_begin` with the request's number and body
before the core runs, and `request_end` with the same number, the body and the
answer after the answer is written. `Native` begins the span with the request's
`_meta` (`begin_request`), keeps it by the number, and ends it with the JSON-RPC
method, the class of a tool of that request that failed and the protocol error
code of the answer (a JSON message, or the last answering message of an event
stream); `end` counts the request and its error (S2), and `flush` exports the
metrics after each request. Connections are served on threads that share the host,
so the calls of two requests interleave: the span is found by the number, and a
tool's failure (`engine_call`) is the request's that its thread serves (OB-15).
Every other indicator is emitted by the host as it acts.

### What users needed: gaps and codes

Two records say which LexLean and Prism capabilities users need, ranked by demand,
with no work for a user or an agent and no endpoint of their own.

**Gaps (OB-16).** prism-code records what its users asked for that Prism or Lean
could not do, as fingerprints, and sends the unsent ones in the `_meta` of its next
call to this server, under `foundation.uor/prism-gaps`:

```json
{"schema": "prism/gaps/1", "install": "<32 lowercase hex>", "gaps": [
  {"kind": "refusal", "construct": "fields.too-many", "code": null, "version": "0.6.0", "count": 2},
  {"kind": "unproved", "construct": "always:choice-is+number-le-field", "code": "LLV7002", "version": "0.6.0", "count": 1}
]}
```

`begin_request` reads the batch with the trace context, and each gap becomes one
`INFO` log record in the request's trace, `event` `gap`, with the attributes
`gap.kind`, `gap.construct`, `gap.code` (`none` when null), `gap.version`,
`gap.count` and `gap.install`. Every field has a closed grammar, and a gap with any
field outside it is dropped whole: the kind is one of `refusal`, `roundtrip`,
`unproved`, `model-refused`; a refusal rule is two words of lowercase letters and
hyphens joined by a dot; a round trip names a part of a spec, and may add a refusal
rule after a colon; an unproved property is `always` or `never` and its comparisons
from a fixed set; a refused model names an SDK step; a code is a diagnostic code;
a version is `x.y.z` with an optional `-alpha.n`, `-beta.n` or `-rc.n`; the count is
1 to 10000; the install ID is 32 lowercase hexadecimal digits, drawn by the client
and derived from nothing. Outside a code and a version no grammar admits a digit, so
no number a user wrote fits. At most 64 gaps of one request are read. prism-code
holds its side to the same fingerprints (`sdk/npm/prism-code/src/gaps.mjs` in
PrismPM), and a user turns it off there with `PRISM_TELEMETRY=0` or `DO_NOT_TRACK=1`.

**Codes (OB-17).** S1 and S4 carry the profile of the draft and the code of the
first error of the result (`first_error_code`: the first diagnostic that is not a
warning), so a failure says which part of LexLean a draft needed, not only its
class. A code is `LL` and a letter, or `PP`, and four digits; a result with no error
is `none`, and a text that is not a code is `other`.

**The report (OB-18).** `deploy/demand.py` reads the OTLP/JSON file, the gateway's
or a collector's, and ranks the gaps by fingerprint: distinct installs, times
recorded, and whether it blocks a build (an unproved property, a refused model, or a
round trip a refusal stopped), scored as installs x times x 2 when it blocks. Beside
them it lists the failed tools of S1 by profile and code; S1 is cumulative, so each
series counts its largest value, and a restart between two files undercounts. It
holds every value it reads to the gateway's grammar, so a hand-edited file cannot
put text in a report.

```sh
python3 -I deploy/demand.py report /telemetry/otlp.jsonl
python3 -I deploy/demand.py issues --repo UOR-Foundation/lexlean-mcp /telemetry/otlp.jsonl          # the plan
python3 -I deploy/demand.py issues --repo UOR-Foundation/lexlean-mcp --apply /telemetry/otlp.jsonl  # gh opens or updates them
```

`issues` keeps one issue per fingerprint, labelled `demand` and found by its title,
and changes nothing on GitHub without `--apply`. Whether a collector reads the
production file is the operator's: the images run one (IM-04), which writes what it
received to a file and forwards nothing.

### Planted defects of the runtime

Each defect was applied to `telemetry.rs` and the tests of
`gateway/bindings/gateway-binding/tests/telemetry.rs` that do not need Lean were
run (the rest run in the gateway job). Restoring the file passes all of them.

| Row | Plant | Test that rejects it |
| --- | --- | --- |
| OB-11 | the counter of S8 is not incremented | `every_indicator_is_emitted_ob_11` |
| OB-12 | `firing` compares with `>=` the threshold | `the_predicates_are_zero_tolerance_ob_12` |
| OB-13 | a profile label is kept as given | `no_telemetry_record_contains_module_text_ob_13` |
| OB-14 | the forbidden version `ff` is accepted | `a_malformed_traceparent_restarts_the_trace_and_drops_tracestate_ob_14` |
| OB-14 | `tracestate` is kept after a refused `traceparent` | `a_malformed_traceparent_restarts_the_trace_and_drops_tracestate_ob_14` |
| OB-15 | `request_end` ends the most recent span instead of the request's own | `each_request_ends_its_own_span_with_its_method_and_error_ob_15` |
| OB-15 | a tool's failure is recorded on the latest request instead of its thread's | `a_failed_tool_is_the_class_of_its_own_request_ob_15` |
| OB-16 | a word of a refusal rule admits digits, so `fields.amount987654` is kept | `a_gap_that_is_not_a_fingerprint_is_dropped_ob_16`: 3 gap records where 2 are expected |
| OB-16 | `begin_request` does not read the gaps of `_meta` | `the_gaps_of_a_request_are_recorded_as_fingerprints_ob_16`: 0 gap records where 4 are expected |
| OB-16 | the install ID's length is not checked | `a_gap_that_is_not_a_fingerprint_is_dropped_ob_16`: a 33-digit ID is kept |
| OB-16 | the install ID is not held to hexadecimal | `a_gap_that_is_not_a_fingerprint_is_dropped_ob_16`: `secretmoduletextsecretmoduletext` is kept |
| OB-17 | a code label is kept as given | `a_tool_outcome_and_a_verification_carry_profile_and_code_ob_17`: no outcome labelled `other` |
| OB-17 | a warning is taken for the first error | `the_first_error_code_is_read_from_the_result_ob_17`: `LLV7002` where `LLV7005` is expected |
| OB-17 | S1 drops the profile label | `a_tool_outcome_and_a_verification_carry_profile_and_code_ob_17`: no outcome labelled `prism-app` |

The rows of OB-16 and OB-17 were planted in `telemetry.rs` compiled on its own, with
the tests of `tests/telemetry.rs` that need neither the generated host nor the fake
verifier, at Rust 1.97.1; the gateway job runs all of them, and the host's emission
sites are held by `cargo test -p repo-model --test telemetry`
(`outcomes_carry_profile_and_code_ob_17`, `gaps_are_recorded_as_fingerprints_ob_16`).

## Decisions the model makes

- The alerts on "classes 2, 3 and 70" are on the exit codes of LexLean's
  `ErrorClass` at v0.4.0 at `e0b74b8` (SPEC 24.5 and 23.6): 2 is
  `CliOrConfiguration`, 3 is `Environment`, 70 is `Internal`. Classes 1
  (`Language`) and 4 (`SecurityOrLimit`) are answers to a bad or over-limit
  draft and never alert.
- The issue lists seven indicators and an alert on signing failures that none of
  them feeds, so the model has eight definitions, one for each of S1 to S8 of
  `OwnerValues`.
- `Redacted` has one value, `Withheld`. Not even the length of a text is kept.
- The trace context of a request that fails to parse is ignored and the trace
  restarts (W3C Trace Context 4.3); a failure never fails the request.
  `tracestate` is not read after a `traceparent` that did not parse (3.3).
- `tracestate` longer than 512 bytes and `baggage` longer than 8192 bytes are
  dropped whole. 8192 is the minimum a platform must propagate (W3C Baggage
  3.3.2); the 512 is this product's choice. Members are not validated here.
- The emitted set is `notifications/progress` alone. Adding a method means
  adding a constructor to `Emitted`, and then OB-09 states whether it is allowed.

## Planted defects

A gate is evidence only after a planted defect shows it can fail (R2 of
`AGENTS.md`). Each defect below was applied to a copy of `src/Observability.lex.tex`
in a project of the three modules `Product`, `OwnerValues` and `Observability`
(`lexlean lock`, then `lexlean verify`), with LexLean v0.4.0 at `e0b74b8`.
The pull request's `lexlean / verify` job runs the same command on every module
of the repository. The diagnostic is the first one `lexlean verify` prints; the
restore is the unedited module, which verifies.

| ID | Plant | Theorems | Command | Diagnostic | Restore |
| --- | --- | --- | --- | --- | --- |
| `OB-01` | in `exitCode`, the class `Internal` maps to 71 | errorClassesMapToTheExitCodes, alertsRaiseOnClasses2And3And70 | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic decide proved that the proposition exitCodes allClasses = [1, 2, 3, 4, 70] is false` | revert the edit; `verified 3 modules` |
| `OB-02` | in `notificationMethod`, the method of `Message` is `notifications/log` | schemaServerNotificationsAreEight | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic decide proved that the proposition methodsOf allNotifications = [...] is false` | revert the edit; `verified 3 modules` |
| `OB-04` | a free-text field kept without redaction: `Redacted` gains a constructor `Text` that holds the text, and `redact` returns it | redactedHasOneValue, redactionIgnoresText | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic rfl failed: The left-hand side Redacted.Text kept is not definitionally equal to the right-hand side Redacted.Withheld` | revert the three edits; `verified 3 modules` |
| `OB-05` | the unit of the tool outcome indicator is `Percent`, where S1 says count | sliUnitsAreTheUnitsOfTheSlos, observabilityIsDefined | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic decide proved that the proposition unitsAgree sliDefs OwnerValues.sloRecords = true is false` | revert the edit; `verified 3 modules` |
| `OB-06` | an alert threshold off by one: the class alerts compare with the threshold plus one | classTwoFiresOnItsPlantedCondition, classThreeFiresOnItsPlantedCondition, classSeventyFiresOnItsPlantedCondition | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic decide proved that the proposition firing Alert.ClassTwo (Sample.ToolOutcome ErrorClass.CliOrConfiguration (threshold 0 + 1)) = true is false` | revert the edit; `verified 3 modules` |
| `OB-07` | a malformed traceparent accepted: the forbidden version in `finish` is 256 and not 255 (`ff`), so `ff` passes | vectorForbiddenVersion | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic decide proved that the proposition headerFields (parseTraceparent [102, 102, 45, ...]) = [104] is false` | revert the edit; `verified 3 modules` |
| `OB-08` | tracestate kept after a refused traceparent: `tracestateOf` carries it for `Invalid` | tracestateNeedsAValidTraceparent, extractInvalidDropsTracestate | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic rfl failed: The left-hand side restarts (Parent.Invalid fault) && isCarried (tracestateOf (Parent.Invalid fault) found) is not definitionally equal to the right-hand side false` | revert the edit; `verified 3 modules` |
| `OB-09` | `notifications/message` added to the emit set: `Emitted` gains `Message`, mapped to `ServerNotification.Message` and listed in `allEmitted` | emittedMethodsAreProgressOnly, noEmittedNotificationIsALogMessage | `lexlean verify` | `error[LLV7002]: Lean rejected LexLeanMcp.Observability: Tactic decide proved that the proposition emittedMethods allEmitted = [notifications/progress] is false` | revert the edits; `verified 3 modules` |

The restored module verifies as three modules with the attestation
`2809aae6bccc2e982b5777033497a2f6f913e0ece0d0990a7e2c889edde5d90c`. The test of
OB-10 reads this table, checks that every theorem named in it is in the module,
and fails if the planted code is still in the module.

The Rust side has its own check: `cargo test -p repo-model --test observability`
reads the field types of the log and span records and fails on any field that
could hold text, so the type-level claim of OB-04 is held by the kernel and
by the test.
