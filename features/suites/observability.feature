Feature: Observability

  Issue #16 asks for logs, metrics and traces whose records hold no module text, service level indicators with alerts on their planted conditions, trace context taken from each request, and no log notification. This suite holds the model: the record types with redaction as a type, the indicators and alert predicates, the trace context extraction and the closed set of emitted notifications. Emitting through OpenTelemetry and firing alerts in a running service need the verify task (#11) and the host (#17), and are not claimed here.

  @OB-01 @some-true
  Scenario: LexLean release/v0.4 at 9796067 maps the five variants of ErrorClass, Language, CliOrConfiguration, Environment, SecurityOrLimit and Internal, to the CLI exit codes 1, 2, 3, 4 and 70 (SPEC 24.5 and 23.6), so that the classes 2, 3 and 70 of issue #16 are CliOrConfiguration, Environment and Internal.
    Given the cited authority A-LEXLEAN-SPEC and the definitions exitCode and alerting of src/Observability.lex.tex
    When the five variants of ErrorClass are mapped to their exit codes
    Then the codes are 1, 2, 3, 4 and 70 in the order of SPEC 24.5, and the codes that raise an alert are 2, 3 and 70

  @OB-02 @some-true
  Scenario: The ServerNotification union of the MCP schema 2026-07-28 has eight methods: notifications/cancelled, notifications/progress, notifications/resources/list_changed, notifications/subscriptions/acknowledged, notifications/resources/updated, notifications/prompts/list_changed, notifications/tools/list_changed and notifications/message, the last being the log message a client opts into through the io.modelcontextprotocol/logLevel key of a request's _meta.
    Given the cited authority A-MCP-SCHEMA-2026-07-28 and the definition notificationMethod
    When the eight variants of ServerNotification are read with their methods
    Then the methods are the eight of the schema union, notifications/message among them

  @OB-03 @some-true
  Scenario: W3C Trace Context Level 1 defines traceparent as a two-digit version, a 32-digit trace-id, a 16-digit parent-id and a two-digit trace-flags in lowercase hexadecimal joined by hyphens, forbids version ff and an all-zero trace-id or parent-id, tells a vendor that fails to parse traceparent to ignore it and not to attempt tracestate, and lets a vendor parse a higher version from its first 55 characters; W3C Baggage requires a platform to propagate a baggage-string of at most 64 list-members and 8192 bytes.
    Given the cited authorities A-W3C-TRACE-CONTEXT and A-W3C-BAGGAGE and the constants of the parser and of the extraction
    When the grammar, the forbidden values, the limits and the processing rules are read from the model
    Then each equals the rule the authority states

  @OB-04 @build
  Scenario: A telemetry record holds no module text and no lexicon search query, by its types: the free-text fields of the log record and the span record, the log body, the tool arguments, the search query and the status text, have the type Redacted, whose only value is Withheld, and the kernel checks that Redacted has one value, that redaction ignores its text, and that for every input the record built from it is the same record whatever text its free-text fields carry.
    Given the types of src/Observability.lex.tex and its theorems redactedHasOneValue, redactionIgnoresText and noTelemetryRecordDependsOnText
    When the field types of the log record and the span record are read
    Then every field that can carry free text has the type Redacted, no record has a String or a list field, and the three theorems are in the module

  @OB-05 @build
  Scenario: The service level indicators are data: one definition for each of S1 to S8 of src/OwnerValues.lex.tex, in the same order and with the same unit, with a distinct metric name and a label set drawn from the ErrorClass, the protocol error code, the outcome and the profile; they are the seven indicators of issue #16 and the signing failure count its alert on signing failures needs.
    Given the eight definitions of sliDefs and the service level objectives of src/OwnerValues.lex.tex
    When the indexes, units and names of the definitions are held against the objectives
    Then the definitions are S1 to S8 in order with the units of the objectives, and no name is used twice

  @OB-06 @build
  Scenario: The alert predicates are total functions of a metric sample whose thresholds are read from the S-values of src/OwnerValues.lex.tex: the alerts on class 2, class 3 and class 70 tool outcomes, on a failed child group end, on a profile drift and on a signing failure each fire on a sample one above the threshold and stay silent on a healthy sample, and the classes 1 and 4 and the ended and already-gone group ends never alert.
    Given the alert predicates firing and classFires, the thresholds of the objectives and a planted and a healthy sample for each alert
    When each predicate is applied to the planted sample and to the healthy one
    Then each alert fires on the planted sample, stays silent on the healthy one, and classes 1 and 4 and the ended and already-gone group ends never alert

  @OB-07 @build
  Scenario: The traceparent parser gives, for the examples of W3C Trace Context and for each rule of OB-03, the fields of the header or the typed fault that refuses it: length, separator, digit, forbidden version, zero trace-id and zero parent-id, and it accepts a higher version whose extra fields follow a hyphen and refuses a version 00 header that is longer than 55 characters.
    Given parseTraceparent and the W3C examples with the invalid headers that each rule gives
    When each header is parsed
    Then the valid ones give their fields and each invalid one gives the typed fault of its rule

  @OB-08 @build
  Scenario: Trace context extraction is a total function of the _meta entries of a request: it gives the parsed traceparent or its typed fault, drops tracestate when the traceparent is absent or refused, carries baggage independently of the traceparent, carries tracestate up to 512 and baggage up to 8192 bytes and drops what is longer, refuses a traceparent that occurs twice or is not text, and ignores every other key.
    Given extract and the _meta entries of a request with and without each of the three keys
    When the entries are extracted
    Then the parent, tracestate and baggage are carried or dropped as the statement gives

  @OB-09 @build
  Scenario: The server notifications this product can emit are a closed set, the type Emitted with the one constructor Progress, and the kernel checks that no emitted notification is notifications/message and that the emitted methods are notifications/progress alone.
    Given the type Emitted, the set of ServerNotification and the theorems noEmittedNotificationIsALogMessage and emittedMethodsAreProgressOnly
    When the emitted methods are read
    Then they are notifications/progress alone and none is notifications/message

  @OB-10 @build
  Scenario: Each theorem of src/Observability.lex.tex that carries a claim rejects its planted defect under lexlean verify: a free-text field kept without redaction, an alert threshold off by one, a malformed traceparent accepted, a tracestate kept after a refused traceparent, and notifications/message added to the emitted set; OBSERVABILITY.md records the command, the diagnostic and the restore of each.
    Given OBSERVABILITY.md and the theorems of src/Observability.lex.tex
    When the table of planted defects is read against the theorems
    Then every claim theorem has a planted defect with its command, diagnostic and restore, and none of the planted code is in the source

  @OB-11 @build
  Scenario: Every service level indicator S1 to S8 is emitted by the host as OpenTelemetry metrics (OTLP/JSON) under the names of sliDefs: tool outcomes by class from the Engine tools and the verifier, protocol errors by code, check latency, verify duration and outcome, queue wait, child group ends, profile drift and signing failures.
    Given a host with a telemetry sink and a verification of the fake verifier
    When the host acts and the metrics are flushed
    Then a metric exists for each indicator under the name of the model

  @OB-12 @build
  Scenario: Each alert of the model (classes 2, 3 and 70, a failed child group end, a profile drift and a signing failure) fires as an error log record on its planted condition inside the window of its objective, fires on no healthy condition, and a lost verifier raises the class 70 alert.
    Given the predicates of the host and each planted condition
    When the condition is applied and then its healthy form
    Then the alert fires on the first and on nothing in the second

  @OB-13 @build
  Scenario: No telemetry record the host emits contains module text, call arguments or a lexicon search query: the text passes through redact, which keeps nothing, and a profile label that is not a profile name is other; a span carries the trace of the request's _meta.
    Given a marker standing for module text in the arguments, the query and the labels of a request
    When every record is emitted and read
    Then no line holds the marker

  @OB-14 @build
  Scenario: The host reads traceparent, tracestate and baggage from _meta as the model's extract does, restarting the trace on a malformed traceparent and dropping tracestate over 512 and baggage over 8192 bytes, and it emits no notification: the set it emits is notifications/progress alone.
    Given _meta entries with valid and malformed headers
    When the trace context is extracted and every record is read
    Then the context is carried or dropped as the model gives and no record is a notification
