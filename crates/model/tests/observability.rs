//! OB-01 to OB-10: the observability model of issue #16.
//!
//! The model is LexLean source (`src/Observability.lex.tex`) and `lexlean verify`
//! checks the kernel side. These tests read the same source as text and hold it to
//! what its authorities say, to `src/OwnerValues.lex.tex` for the thresholds, and to
//! `OBSERVABILITY.md` for the planted defects. The traceparent oracle below is a
//! second reading of the W3C grammar, written as slicing and not as the model's
//! sequential parse, so a disagreement names the header it disagrees on.

use std::collections::BTreeSet;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The source with runs of white space collapsed and braces tightened, so the
/// formatter's one-line and multi-line spellings of a declaration read the same.
fn tight(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("{ ", "{")
        .replace(" }", "}")
}

fn module() -> String {
    tight(&read("src/Observability.lex.tex"))
}

/// Index of the brace that closes the one opened just before `text`.
fn closing_brace(text: &str) -> usize {
    let mut depth = 1usize;
    let mut in_string = false;
    for (index, c) in text.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
            }
            _ => {}
        }
    }
    panic!("a declaration is not closed");
}

/// The text between the braces of `kind{name: <name>,` ... `}`.
fn declaration(source: &str, kind: &str, name: &str) -> String {
    let opening = format!("{kind}{{name: {name},");
    let at = source
        .find(&opening)
        .unwrap_or_else(|| panic!("the module has no {kind} named {name}"));
    let chunk = &source[at + kind.len() + 1..];
    chunk[..closing_brace(chunk)].to_string()
}

fn has(source: &str, kind: &str, name: &str) -> bool {
    source.contains(&format!("{kind}{{name: {name},"))
}

fn theorem_names(source: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut rest = source;
    while let Some(at) = rest.find("theorem{name: ") {
        let after = &rest[at + "theorem{name: ".len()..];
        let end = after.find(',').expect("a theorem name ends with a comma");
        found.insert(after[..end].to_string());
        rest = &after[end..];
    }
    found
}

fn require_theorems(names: &[&str]) {
    let have = theorem_names(&module());
    for name in names {
        assert!(have.contains(*name), "the module has no theorem {name}");
    }
}

/// The `fields: [...]` of a structure as `(name, type)` pairs.
fn structure_fields(source: &str, name: &str) -> Vec<(String, String)> {
    let inner = declaration(source, "structure", name);
    let at = inner.find("fields: [").expect("a structure has fields") + "fields: [".len();
    let list = &inner[at..inner.rfind(']').expect("fields close")];
    let mut found = Vec::new();
    let mut rest = list;
    while let Some(start) = rest.find("{name: ") {
        let after = &rest[start + "{name: ".len()..];
        let (field, after) = after.split_once(", type: ").expect("a field has a type");
        let end = {
            let mut depth = 0usize;
            let mut cut = after.len();
            for (index, c) in after.char_indices() {
                match c {
                    '(' => depth += 1,
                    ')' => depth -= 1,
                    '}' if depth == 0 => {
                        cut = index;
                        break;
                    }
                    _ => {}
                }
            }
            cut
        };
        found.push((field.to_string(), after[..end].to_string()));
        rest = &after[end..];
    }
    found
}

/// The constructors of an inductive as `(name, field types)`.
fn constructors(source: &str, name: &str) -> Vec<(String, String)> {
    let inner = declaration(source, "inductive", name);
    let at = inner
        .find("constructors: [")
        .expect("an inductive has constructors")
        + "constructors: [".len();
    let list = &inner[at..inner.rfind(']').expect("constructors close")];
    let mut found = Vec::new();
    let mut rest = list;
    while let Some(start) = rest.find("{name: ") {
        let after = &rest[start + "{name: ".len()..];
        let (ctor, after) = after
            .split_once(", fields: [")
            .expect("a constructor has fields");
        let end = after.find(']').expect("fields close");
        found.push((ctor.to_string(), after[..end].to_string()));
        rest = &after[end..];
    }
    found
}

/// The numerals of a `(cons n (cons n ... (nil Nat)))` chain that starts at `from`.
fn cons_chain(text: &str, from: usize) -> Vec<u128> {
    let mut rest = &text[from..];
    let mut found = Vec::new();
    while let Some(after) = rest.strip_prefix("(cons ") {
        let end = after.find(' ').expect("a numeral ends with a space");
        found.push(
            after[..end]
                .parse::<u128>()
                .unwrap_or_else(|_| panic!("`{}` is not a numeral", &after[..end])),
        );
        rest = &after[end + 1..];
    }
    assert!(
        rest.starts_with("(nil Nat)"),
        "a cons chain ends in (nil Nat)"
    );
    found
}

fn text_of(codes: &[u128]) -> String {
    codes
        .iter()
        .map(|code| char::from_u32(u32::try_from(*code).expect("a code fits")).expect("a char"))
        .collect()
}

/// The body of the `parseTraceparent` application and the right-hand side of a
/// vector theorem: the header text and the fields or fault code it expects.
fn vectors() -> Vec<(String, String, Vec<u128>)> {
    let source = module();
    let mut found = Vec::new();
    for name in theorem_names(&source) {
        if !name.starts_with("vector") {
            continue;
        }
        let inner = declaration(&source, "theorem", &name);
        let at = inner
            .find("(parseTraceparent ")
            .expect("a vector parses a header")
            + "(parseTraceparent ".len();
        let header = text_of(&cons_chain(&inner, at));
        let eq = inner.find(")) = ").expect("a vector states its fields") + ")) = ".len();
        found.push((name, header, cons_chain(&inner, eq)));
    }
    found
}

/// W3C Trace Context Level 1, 3.2.2 to 3.2.4, read as slices. The result is the
/// fields `[version, trace-id, parent-id, flags]`, or `[100 + fault]` with the
/// fault numbered as `faultCode` numbers it.
fn oracle(header: &str) -> Vec<u128> {
    fn hex(part: &str) -> Option<bool> {
        // None: a character is not lowercase hexadecimal.
        if part.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f')) {
            Some(true)
        } else {
            None
        }
    }
    const LENGTH: u128 = 101;
    const SEPARATOR: u128 = 102;
    const DIGIT: u128 = 103;
    let text = header.as_bytes();
    let mut at = 0usize;
    let mut value = |width: usize, at: &mut usize| -> Result<u128, u128> {
        let end = (*at + width).min(text.len());
        let taken = &header[*at..end];
        if hex(taken).is_none() {
            return Err(DIGIT);
        }
        if taken.len() < width {
            return Err(LENGTH);
        }
        *at = end;
        Ok(u128::from_str_radix(taken, 16).expect("hexadecimal digits"))
    };
    let dash = |at: &mut usize| -> Result<(), u128> {
        match text.get(*at) {
            None => Err(LENGTH),
            Some(b'-') => {
                *at += 1;
                Ok(())
            }
            Some(_) => Err(SEPARATOR),
        }
    };
    let parsed = (|| -> Result<Vec<u128>, u128> {
        let version = value(2, &mut at)?;
        dash(&mut at)?;
        let trace = value(32, &mut at)?;
        dash(&mut at)?;
        let parent = value(16, &mut at)?;
        dash(&mut at)?;
        let flags = value(2, &mut at)?;
        if version == 0xff {
            return Err(104);
        }
        if trace == 0 {
            return Err(105);
        }
        if parent == 0 {
            return Err(106);
        }
        if at < text.len() && (version == 0 || text[at] != b'-') {
            return Err(if version == 0 { LENGTH } else { SEPARATOR });
        }
        Ok(vec![version, trace, parent, flags])
    })();
    parsed.unwrap_or_else(|fault| vec![fault])
}

fn tracked_authority(id: &str, realizes: &str) {
    let text = read("model/authorities.toml");
    let line = text
        .lines()
        .find(|line| line.contains(&format!("id = \"{id}\"")))
        .unwrap_or_else(|| panic!("model/authorities.toml has no authority {id}"));
    assert!(
        line.contains(&format!("\"{realizes}\"")),
        "{id} does not list {realizes} in realized_by"
    );
}

/// OB-01: the ErrorClass table of LexLean.
#[test]
fn error_classes_map_to_the_exit_codes_of_lexlean_ob_01() {
    let source = module();
    let exit = declaration(&source, "definition", "exitCode");
    assert!(
        exit.contains(
            "| ErrorClass.Language => 1 | ErrorClass.CliOrConfiguration => 2 | ErrorClass.Environment => 3 | ErrorClass.SecurityOrLimit => 4 | ErrorClass.Internal => 70)"
        ),
        "exitCode is not the table of SPEC 23.6 and 24.5: {exit}"
    );
    let alerting = declaration(&source, "definition", "alerting");
    assert!(
        alerting.contains(
            "| ErrorClass.Language => false | ErrorClass.CliOrConfiguration => true | ErrorClass.Environment => true | ErrorClass.SecurityOrLimit => false | ErrorClass.Internal => true)"
        ),
        "the classes 2, 3 and 70 are not the ones that alert: {alerting}"
    );
    let names: Vec<String> = constructors(&source, "ErrorClass")
        .into_iter()
        .map(|(name, _)| name)
        .collect();
    assert_eq!(
        names,
        [
            "Language",
            "CliOrConfiguration",
            "Environment",
            "SecurityOrLimit",
            "Internal"
        ]
    );
    require_theorems(&[
        "errorClassesMapToTheExitCodes",
        "alertsRaiseOnClasses2And3And70",
    ]);
    tracked_authority("A-LEXLEAN-SPEC", "OB-01");
}

/// OB-02: the eight methods of ServerNotification.
#[test]
fn server_notification_methods_are_the_eight_of_the_schema_ob_02() {
    let source = module();
    let methods = declaration(&source, "definition", "notificationMethod");
    let mut found = Vec::new();
    let mut rest = methods.as_str();
    while let Some(at) = rest.find("=> \"") {
        let after = &rest[at + 4..];
        let end = after.find('"').expect("a string closes");
        found.push(after[..end].to_string());
        rest = &after[end..];
    }
    assert_eq!(
        found,
        [
            "notifications/cancelled",
            "notifications/progress",
            "notifications/resources/list_changed",
            "notifications/subscriptions/acknowledged",
            "notifications/resources/updated",
            "notifications/prompts/list_changed",
            "notifications/tools/list_changed",
            "notifications/message",
        ]
    );
    require_theorems(&["schemaServerNotificationsAreEight"]);
    tracked_authority("A-MCP-SCHEMA-2026-07-28", "OB-02");
}

/// OB-03: the W3C rules the model reproduces.
#[test]
fn the_w3c_rules_are_the_ones_the_model_applies_ob_03() {
    let source = module();
    let parse = declaration(&source, "definition", "parseTraceparent");
    for (needle, rule) in [
        ("readHex 2 0 bytes", "a two-digit version"),
        ("readHex 32 0 second", "a 32-digit trace-id"),
        ("readHex 16 0 fourth", "a 16-digit parent-id"),
        ("readHex 2 0 sixth", "a two-digit trace-flags"),
    ] {
        assert!(parse.contains(needle), "traceparent lacks {rule}");
    }
    let finish = declaration(&source, "definition", "finish");
    for (needle, rule) in [
        ("(version =? 255)", "version ff is forbidden"),
        ("(traceId =? 0)", "an all-zero trace-id is invalid"),
        ("(parentId =? 0)", "an all-zero parent-id is invalid"),
        ("(version =? 0)", "version 00 ends after trace-flags"),
        ("(head =? 45)", "a higher version continues after a hyphen"),
    ] {
        assert!(finish.contains(needle), "finish lacks: {rule}");
    }
    let hexvalue = declaration(&source, "definition", "hexValue");
    assert!(
        hexvalue.contains("(47 <? code) && (code <? 58)")
            && hexvalue.contains("(96 <? code) && (code <? 103)"),
        "the digits are not 0-9 and a-f only: {hexvalue}"
    );
    let extract = declaration(&source, "definition", "extract");
    assert!(
        extract.contains("carried 8192"),
        "baggage is not carried up to the 8192 bytes of W3C Baggage 3.3.2"
    );
    let tracestate = declaration(&source, "definition", "tracestateOf");
    assert!(
        tracestate.contains("Parent.Invalid fault => (ctor Option.none")
            && tracestate.contains("Parent.Absent => (ctor Option.none"),
        "tracestate is read after a traceparent that did not parse"
    );
    require_theorems(&["tracestateNeedsAValidTraceparent"]);
    tracked_authority("A-W3C-TRACE-CONTEXT", "OB-03");
    tracked_authority("A-W3C-BAGGAGE", "OB-03");
}

/// OB-04: no field of a telemetry record can carry free text.
#[test]
fn a_telemetry_record_cannot_hold_free_text_ob_04() {
    let source = module();
    let redacted = constructors(&source, "Redacted");
    assert_eq!(
        redacted,
        [("Withheld".to_string(), String::new())],
        "Redacted has more than one value, or its value carries something"
    );
    let free_text = ["body", "arguments", "query", "status"];
    let allowed = [
        "Severity",
        "EventKind",
        "(Option TraceParent)",
        "(Option ErrorClass)",
        "Redacted",
        "Nat",
    ];
    for record in ["LogRecord", "SpanRecord"] {
        for (field, kind) in structure_fields(&source, record) {
            assert!(
                allowed.contains(&kind.as_str()),
                "{record}.{field} has the type {kind}, which can hold text"
            );
            if free_text.contains(&field.as_str()) {
                assert_eq!(kind, "Redacted", "{record}.{field} is free text");
            }
        }
    }
    // Every text the inputs carry has a redacted field of the same name.
    for (input, record) in [("LogInput", "LogRecord"), ("SpanInput", "SpanRecord")] {
        let redacted_fields: BTreeSet<String> = structure_fields(&source, record)
            .into_iter()
            .filter(|(_, kind)| kind == "Redacted")
            .map(|(field, _)| field)
            .collect();
        let text_fields: BTreeSet<String> = structure_fields(&source, input)
            .into_iter()
            .filter(|(_, kind)| kind == "(List Nat)")
            .map(|(field, _)| field)
            .collect();
        assert_eq!(
            text_fields, redacted_fields,
            "{input} carries text that {record} does not redact, or the reverse"
        );
    }
    for (name, fields) in [("TraceParent", 4usize)] {
        let kinds: Vec<String> = structure_fields(&source, name)
            .into_iter()
            .map(|(_, kind)| kind)
            .collect();
        assert_eq!(kinds, vec!["Nat"; fields], "{name} holds more than numbers");
    }
    let sample = declaration(&source, "inductive", "Sample");
    for text in ["String", "List", "Bytes"] {
        assert!(!sample.contains(text), "a metric sample can carry {text}");
    }
    let redact = declaration(&source, "definition", "redact");
    assert!(
        redact.contains("body: (ctor Redacted.Withheld)"),
        "redact does not return Withheld without reading its text"
    );
    require_theorems(&[
        "redactedHasOneValue",
        "redactionIgnoresText",
        "noTelemetryRecordDependsOnText",
    ]);
}

/// OB-05: one indicator for each service level objective.
#[test]
fn the_indicators_follow_the_objectives_ob_05() {
    let source = module();
    let defs = declaration(&source, "definition", "sliDefs");
    let mut rows = Vec::new();
    let mut rest = defs.as_str();
    while let Some(at) = rest.find("(record SliDef {index := ") {
        let after = &rest[at + "(record SliDef {index := ".len()..];
        let (index, after) = after.split_once(", name := \"").expect("a name");
        let (name, after) = after.split_once("\", unit := (ctor Unit.").expect("a unit");
        let (unit, after) = after.split_once("), labels := ").expect("labels");
        rows.push((
            index.parse::<usize>().expect("an index"),
            name.to_string(),
            unit.to_string(),
        ));
        rest = after;
    }
    assert_eq!(rows.len(), 8, "S1 to S8 are eight objectives");
    let objectives = tight(&read("src/OwnerValues.lex.tex"));
    let slos = declaration(&objectives, "definition", "sloRecords");
    let unit_names = [
        ("Count", "count"),
        ("Percent", "percent"),
        ("Milliseconds", "milliseconds"),
    ];
    let mut names = BTreeSet::new();
    for (position, (index, name, unit)) in rows.iter().enumerate() {
        assert_eq!(
            *index, position,
            "the indicators are not in the order of S1 to S8"
        );
        assert!(names.insert(name.clone()), "{name} is used twice");
        assert!(
            name.starts_with("lexlean_mcp."),
            "{name} is not in the product namespace"
        );
        let slo = format!("{{index := {index}, sli := ");
        let at = slos
            .find(&slo)
            .unwrap_or_else(|| panic!("S{} is not in OwnerValues", index + 1));
        let row = &slos[at..];
        let unit_text = unit_names
            .iter()
            .find(|(constructor, _)| constructor == unit)
            .map(|(_, text)| *text)
            .expect("a known unit");
        let expected = format!("unit := \"{unit_text}\"");
        let row = &row[..row.find('}').expect("the row closes")];
        assert!(
            row.contains(&expected),
            "indicator {index} has the unit {unit_text}, S{} says otherwise: {row}",
            index + 1
        );
    }
    let sample = constructors(&source, "Sample");
    assert_eq!(sample.len(), 8, "one sample constructor for each indicator");
    // The indicators of issue #16, in its order, and the signing counter last.
    for (needle, which) in [
        ("tool.outcomes", "tool outcome by ErrorClass"),
        ("protocol.errors", "protocol errors by code"),
        ("check.duration", "check latency"),
        ("verify.duration", "verify duration and outcome"),
        ("queue.wait", "verifier queue"),
        ("child_group.ends", "child group ends"),
        ("profile.drift", "profile drift"),
        ("signing.failures", "signing failures"),
    ] {
        assert!(defs.contains(needle), "no indicator for {which}");
    }
    require_theorems(&[
        "slisAreS1ToS8",
        "sliUnitsAreTheUnitsOfTheSlos",
        "sliNamesAreDistinct",
        "toolOutcomeIsLabelledByErrorClassOnly",
        "everySampleHasADefinition",
    ]);
}

/// OB-06: every alert has a planted and a healthy instance, from the S-values.
#[test]
fn every_alert_fires_on_its_planted_condition_ob_06() {
    let source = module();
    let firing = declaration(&source, "definition", "firing");
    let class = declaration(&source, "definition", "classFires");
    assert!(
        class.contains("((threshold 0) <? count)"),
        "the class alerts do not read the threshold of S1"
    );
    for (index, what) in [(5, "S6"), (6, "S7"), (7, "S8")] {
        assert!(
            firing.contains(&format!("((threshold {index}) <? count)")),
            "the alert of {what} does not read its threshold"
        );
    }
    assert!(
        !firing.contains("+ 1") && !class.contains("+ 1"),
        "a threshold is shifted by a constant"
    );
    let threshold = declaration(&source, "definition", "threshold");
    assert!(
        threshold.contains("sloThreshold index (OwnerValues::sloRecords)"),
        "the thresholds are not read from OwnerValues"
    );
    // The S-values the alerts depend on are zero-tolerance counts.
    let objectives = tight(&read("src/OwnerValues.lex.tex"));
    let slos = declaration(&objectives, "definition", "sloRecords");
    for index in [0, 5, 6, 7] {
        let at = slos
            .find(&format!("{{index := {index}, sli := "))
            .expect("the objective");
        let row = &slos[at..at + slos[at..].find('}').expect("closes")];
        assert!(
            row.contains("threshold := 0, unit := \"count\""),
            "S{} is not a zero-tolerance count: {row}",
            index + 1
        );
    }
    let have = theorem_names(&source);
    for alert in [
        "classTwo",
        "classThree",
        "classSeventy",
        "failedGroupEnd",
        "drift",
        "signingFailure",
    ] {
        for suffix in ["FiresOnItsPlantedCondition", "IsSilentWhenHealthy"] {
            let name = format!("{alert}{suffix}");
            assert!(have.contains(&name), "the module has no theorem {name}");
        }
    }
    require_theorems(&[
        "thresholdsAreTheZeroToleranceOfTheSlos",
        "endedAndGoneGroupsDoNotAlert",
        "languageAndLimitClassesNeverAlert",
    ]);
}

/// OB-07: the parser against the oracle, on the W3C examples and the rules.
#[test]
fn the_traceparent_parser_agrees_with_the_grammar_ob_07() {
    let found = vectors();
    assert!(found.len() >= 12, "the module has {} vectors", found.len());
    let headers: BTreeSet<&str> = found.iter().map(|(_, header, _)| header.as_str()).collect();
    for example in [
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01",
        "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-00",
    ] {
        assert!(
            headers.contains(example),
            "the W3C example {example} is not a vector"
        );
    }
    for (name, header, expected) in &found {
        assert_eq!(
            &oracle(header),
            expected,
            "{name}: the oracle and the theorem disagree on `{header}`"
        );
    }
    // Each rule is exercised by at least one vector.
    let faults: BTreeSet<u128> = found
        .iter()
        .filter(|(_, _, expected)| expected.len() == 1)
        .map(|(_, _, expected)| expected[0])
        .collect();
    assert_eq!(faults, BTreeSet::from([101, 102, 103, 104, 105, 106]));
    assert!(
        found
            .iter()
            .any(|(_, _, expected)| expected.len() == 4 && expected[0] == 1),
        "no vector accepts a higher version with extra fields"
    );
}

/// OB-08: the extraction against its stated limits and drops.
#[test]
fn trace_context_extraction_follows_its_rules_ob_08() {
    let source = module();
    let extract = declaration(&source, "definition", "extract");
    assert!(
        extract.contains("(lookup \"traceparent\" meta)")
            && extract.contains("(lookup \"tracestate\" meta)")
            && extract.contains("(lookup \"baggage\" meta)"),
        "extract does not read the three keys"
    );
    assert!(
        extract.contains("carried 8192 (lookup \"baggage\" meta)"),
        "baggage is not limited to 8192 bytes"
    );
    let tracestate = declaration(&source, "definition", "tracestateOf");
    assert!(
        tracestate.contains("carried 512 found"),
        "tracestate is not limited to 512"
    );
    let parent = declaration(&source, "definition", "parentOf");
    assert!(
        parent.contains("Lookup.Many => (ctor Parent.Invalid (ctor Fault.Duplicate))")
            && parent.contains("MetaValue.Other => (ctor Parent.Invalid (ctor Fault.NotText))"),
        "a repeated or non-text traceparent is not refused"
    );
    let limit = declaration(&source, "definition", "carried");
    assert!(
        limit.contains("(size bytes) <? (limit + 1)"),
        "the limit is not inclusive"
    );
    let have = theorem_names(&source);
    for name in [
        "extractAbsent",
        "extractPresent",
        "extractInvalidDropsTracestate",
        "extractDuplicateTraceparent",
        "extractNonTextTraceparent",
        "extractBaggageWithoutTraceparent",
        "extractTracestateAtTheLimit",
        "extractTracestatePastTheLimit",
        "extractBaggageAtTheLimit",
        "extractBaggagePastTheLimit",
        "extractOtherKeysAreIgnored",
    ] {
        assert!(have.contains(name), "the module has no theorem {name}");
    }
    // At the limit the length is kept, past it nothing is.
    let at_limit = declaration(&source, "theorem", "extractTracestateAtTheLimit");
    assert!(at_limit.contains("replicate 512 97") && at_limit.contains("(cons 1 (cons 512 "));
    let past = declaration(&source, "theorem", "extractTracestatePastTheLimit");
    assert!(past.contains("replicate 513 97") && !past.contains("(cons 513 "));
    let bag = declaration(&source, "theorem", "extractBaggageAtTheLimit");
    assert!(bag.contains("replicate 8192 97") && bag.contains("(cons 1 (cons 8192 (nil Nat))"));
    let bag_past = declaration(&source, "theorem", "extractBaggagePastTheLimit");
    assert!(bag_past.contains("replicate 8193 97") && !bag_past.contains("(cons 8193 "));
}

/// OB-09: the emit set is closed and holds no log message.
#[test]
fn the_emitted_notifications_are_closed_and_exclude_the_log_message_ob_09() {
    let source = module();
    assert_eq!(
        constructors(&source, "Emitted"),
        [("Progress".to_string(), String::new())],
        "the emitted set is not notifications/progress alone"
    );
    let all = declaration(&source, "definition", "allEmitted");
    assert!(all.contains("(cons (ctor Emitted.Progress) (nil Emitted))"));
    let mapped = declaration(&source, "definition", "emittedNotification");
    assert!(
        !mapped.contains("Message"),
        "an emitted notification maps to the log message"
    );
    require_theorems(&[
        "noEmittedNotificationIsALogMessage",
        "emittedMethodsAreProgressOnly",
    ]);
    // No other source can emit it either.
    for relative in [
        "src/Architecture.lex.tex",
        "src/Quality.lex.tex",
        "src/Security.lex.tex",
    ] {
        assert!(
            !read(relative).contains("notifications/message"),
            "{relative} names notifications/message"
        );
    }
}

/// OB-10: every planted defect is recorded in OBSERVABILITY.md.
#[test]
fn every_claim_has_a_recorded_planted_defect_ob_10() {
    let document = read("OBSERVABILITY.md");
    let mut rows: Vec<Vec<String>> = Vec::new();
    for line in document.lines() {
        if !line.starts_with("| `OB-") {
            continue;
        }
        let cells: Vec<String> = line
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim().to_string())
            .collect();
        assert_eq!(cells.len(), 6, "a row has six cells: {line}");
        for cell in &cells {
            assert!(!cell.is_empty(), "a cell is empty: {line}");
        }
        rows.push(cells);
    }
    let source = module();
    let have = theorem_names(&source);
    let covered: BTreeSet<String> = rows
        .iter()
        .map(|row| row[0].trim_matches('`').to_string())
        .collect();
    for id in ["OB-04", "OB-06", "OB-07", "OB-08", "OB-09"] {
        assert!(
            covered.contains(id),
            "OBSERVABILITY.md has no planted defect for {id}"
        );
    }
    assert!(rows.len() >= 5, "five planted defects are recorded");
    for row in &rows {
        for theorem in row[2].split(',').map(|name| name.trim().trim_matches('`')) {
            assert!(
                have.contains(theorem),
                "{} names {theorem}, which is not a theorem",
                row[0]
            );
        }
        assert!(
            row[3].contains("lexlean verify"),
            "{}: the command is lexlean verify",
            row[0]
        );
    }
    // The module on disk is the restored one.
    for planted in [
        "Redacted.Text",
        "Emitted.Message",
        "(threshold 0) + 1) <? count",
        "(version =? 256)",
    ] {
        assert!(
            !source.contains(planted),
            "the module still carries the planted `{planted}`"
        );
    }
}
