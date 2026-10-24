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
| Emission through OpenTelemetry, the collector, alerts firing in a running service | none | not built: needs the verify task (#11) and the host (#17) |

The definition of done of #16, "every SLI is emitted, and every alert fires on
its planted condition", is therefore not met. What is met is that the shape of
what is emitted, the predicates that are evaluated and the context that is read
are fixed and checked before the service exists.

## Decisions the model makes

- The alerts on "classes 2, 3 and 70" are on the exit codes of LexLean's
  `ErrorClass` at `release/v0.4` at `9796067` (SPEC 24.5 and 23.6): 2 is
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
(`lexlean lock`, then `lexlean verify`), with LexLean `release/v0.4` at `9796067`.
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
