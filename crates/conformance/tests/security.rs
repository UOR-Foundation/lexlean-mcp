//! `SM-01`: the measurement of R5 (path escape) is executed. `SM-02`: the measurement of R2 (a
//! lexicon entry that misstates its denotation) is executed by the review records of provisioning.
//!
//! `src/Security.lex.tex` records the status of each measurement. A status of
//! `Executed` is a claim about rows of the register, and LexLean cannot see the
//! register, so the tie lives here: the rows the claim cites are read from the
//! statement of `SM-01` itself (one source, R1), and each must be registered at
//! the build level and have a test. Whether those tests pass is the verdict of
//! the workspace test run that includes this file; this file does not rerun them.

use std::collections::BTreeSet;
use std::fs;

use repo_model::{Level, Model};

mod common;
use common::{root, workspace_test_names};

/// The rows `SM-01` is expected to cite. The statement is the source; this list
/// makes a dropped citation a failure rather than a smaller claim.
const CITED: [&str; 7] = [
    "NF-09", "NF-12", "DL-01", "DL-02", "DL-03", "DL-04", "DR-08",
];

/// The start of the measurement of R5, the risk of index 4, in `Security`.
const R5_MEASUREMENT: &str = "record Measurement {risk := 4,";

/// The status constructor of a measurement that has been executed.
const EXECUTED: &str = "ctor MeasureStatus.Executed";

/// The registered IDs a statement names, other than `SM-01`.
fn cited_ids(statement: &str) -> BTreeSet<String> {
    let bytes = statement.as_bytes();
    let mut found = BTreeSet::new();
    for start in 0..bytes.len().saturating_sub(4) {
        let w = &bytes[start..start + 5];
        let shaped = w[0].is_ascii_uppercase()
            && w[1].is_ascii_uppercase()
            && w[2] == b'-'
            && w[3].is_ascii_digit()
            && w[4].is_ascii_digit();
        let word_start = start == 0 || !bytes[start - 1].is_ascii_alphanumeric();
        if shaped && word_start {
            found.insert(statement[start..start + 5].to_string());
        }
    }
    found.remove("SM-01");
    found
}

/// The text of the declaration `kind{ name: <name>, ...` up to its closing brace.
fn block<'a>(source: &'a str, kind: &str, name: &str) -> Option<&'a str> {
    let head = format!("{kind}{{\n  name: {name},");
    if let Some(start) = source.find(&head) {
        let end = source[start..].find("\n}\n")? + start;
        return Some(&source[start..end]);
    }
    // The formatter writes a declaration that fits on one line on one line.
    let head = format!("{kind}{{name: {name},");
    let start = source.find(&head)?;
    let end = source[start..].find("}\n")? + start;
    Some(&source[start..end])
}

/// Everything the claim of `SM-01` needs, as a list of violations; empty when
/// the claim holds. A function of its inputs so that a planted defect can be
/// given to it.
fn violations(
    model: &Model,
    tests: &BTreeSet<String>,
    security: &str,
    admission: &str,
) -> Vec<String> {
    let mut out = Vec::new();

    let Some(row) = model.ids.id.iter().find(|r| r.id == "SM-01") else {
        return vec!["SM-01 is not in model/ids.toml".to_string()];
    };
    if row.level != Level::Build || row.suite != "security" {
        out.push("SM-01 is not a build row of the security suite".to_string());
    }
    let cited = cited_ids(&row.statement);
    let expected: BTreeSet<String> = CITED.iter().map(|s| s.to_string()).collect();
    if cited != expected {
        out.push(format!(
            "SM-01 cites {cited:?}, and the expected evidence is {expected:?}"
        ));
    }
    if !row
        .statement
        .contains("symlink clause of K5 is executed by DR-08")
    {
        out.push("SM-01 does not state that DR-08 executes the symlink clause of K5".to_string());
    }

    for id in &expected {
        let Some(cited_row) = model.ids.id.iter().find(|r| &r.id == id) else {
            out.push(format!("{id} is cited and is not in model/ids.toml"));
            continue;
        };
        if cited_row.level != Level::Build {
            out.push(format!("{id} is cited and is not a build row"));
        }
        let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
        if !tests.iter().any(|t| t.ends_with(&suffix)) {
            out.push(format!("{id} is cited and no test name ends in `{suffix}`"));
        }
    }

    // The right files: a module of another name cannot satisfy the checks below.
    if !security.contains("\\begin{lexlean}{Security}") {
        out.push("src/Security.lex.tex is not the Security module".to_string());
    }
    if !admission.contains("\\begin{lexlean}{Admission}") {
        out.push("src/Admission.lex.tex is not the Admission module".to_string());
    }

    // R1, R2, R3, R4, R5, R7 and R8 are the Executed measurements (SM-03, SM-02, SM-04, SM-06,
    // SM-01, SM-07, SM-05), and the status of R5 is the Executed one.
    if security.matches(EXECUTED).count() != 7 {
        out.push("exactly seven measurements must be Executed".to_string());
    }
    match security.find(R5_MEASUREMENT) {
        None => out.push("Security has no measurement of R5".to_string()),
        Some(at) => {
            let record = &security[at..];
            let record = &record[..record.find(")})").unwrap_or(record.len())];
            if !record.contains(EXECUTED) {
                out.push("the measurement of R5 is not Executed".to_string());
            }
            if !record.contains("admittedPathHasNoDotDot") {
                out.push("the Executed status of R5 does not name its theorem".to_string());
            }
            if !record.contains("DR-08") {
                out.push("the Executed status of R5 does not name DR-08".to_string());
            }
            if record.contains("pendingClauses") {
                out.push("the Executed status of R5 points at pendingClauses".to_string());
            }
        }
    }

    // The theorem the status names exists, and no clause is pending.
    if block(admission, "theorem", "admittedPathHasNoDotDot").is_none() {
        out.push("Admission has no theorem admittedPathHasNoDotDot".to_string());
    }
    for name in [
        "executedMeasurementsAreR1ToR5R7AndR8",
        "noClauseIsPending",
        "measurementsPendingAreR6AndR9",
    ] {
        if block(security, "theorem", name).is_none() {
            out.push(format!("Security has no theorem {name}"));
        }
    }
    match block(security, "definition", "pendingClauses") {
        Some(clauses) if clauses.contains("record Named") => out.push(
            "pendingClauses records a clause, and DR-08 executes the symlink clause of K5"
                .to_string(),
        ),
        Some(_) => {}
        None => out.push("Security has no definition pendingClauses".to_string()),
    }
    out
}

fn inputs() -> (Model, BTreeSet<String>, String, String) {
    let root = root();
    let model = Model::load(&root.join("model")).expect("model loads");
    let tests = workspace_test_names(&root);
    let security = fs::read_to_string(root.join("src/Security.lex.tex")).expect("Security reads");
    let admission =
        fs::read_to_string(root.join("src/Admission.lex.tex")).expect("Admission reads");
    (model, tests, security, admission)
}

/// `SM-01`: the claim holds on the repository as it is.
#[test]
fn the_measurement_of_r5_is_executed_by_the_admission_rows_sm_01() {
    let (model, tests, security, admission) = inputs();
    let found = violations(&model, &tests, &security, &admission);
    assert!(
        found.is_empty(),
        "SM-01 does not hold:\n{}",
        found.join("\n")
    );
}

/// `SM-01`: the check can fail. Each planted defect is one `SECURITY.md`
/// records, and each must be reported.
#[test]
fn the_check_rejects_a_pending_r5_and_a_missing_row_sm_01() {
    let (model, tests, security, admission) = inputs();

    // R5 flipped back to Pending.
    let at = security.find(R5_MEASUREMENT).expect("R5 is stated");
    let flipped = format!(
        "{}{}",
        &security[..at],
        security[at..].replacen(EXECUTED, "ctor MeasureStatus.Pending", 1)
    );
    let found = violations(&model, &tests, &flipped, &admission);
    assert!(
        found
            .iter()
            .any(|v| v.contains("the measurement of R5 is not Executed")),
        "{found:?}"
    );

    // A cited row with no test.
    let mut without_test = tests.clone();
    without_test.retain(|t| !t.ends_with("_nf_12"));
    let found = violations(&model, &without_test, &security, &admission);
    assert!(
        found
            .iter()
            .any(|v| v.contains("NF-12") && v.contains("no test name")),
        "{found:?}"
    );

    // A cited row removed from the register.
    let mut without_row = model.clone();
    without_row.ids.id.retain(|r| r.id != "DL-03");
    let found = violations(&without_row, &tests, &security, &admission);
    assert!(
        found
            .iter()
            .any(|v| v.contains("DL-03") && v.contains("not in model/ids.toml")),
        "{found:?}"
    );

    // The symlink clause put back into pendingClauses.
    let with_clause = security.replace(
        "name: pendingClauses, parameters: [], result: (List Named), body: (nil Named)",
        "name: pendingClauses, parameters: [], result: (List Named), body: (cons (record Named {index := 4, name := \"K5 symlink refusal\", text := \"materialization refuses symlinks in a profile bundle\"}) (nil Named))",
    );
    assert_ne!(with_clause, security, "the plant applies");
    let found = violations(&model, &tests, &with_clause, &admission);
    assert!(
        found
            .iter()
            .any(|v| v.contains("pendingClauses records a clause")),
        "{found:?}"
    );

    // DR-08 removed from the register.
    let mut without_symlink_row = model.clone();
    without_symlink_row.ids.id.retain(|r| r.id != "DR-08");
    let found = violations(&without_symlink_row, &tests, &security, &admission);
    assert!(
        found
            .iter()
            .any(|v| v.contains("DR-08") && v.contains("not in model/ids.toml")),
        "{found:?}"
    );

    // The control: nothing planted, nothing reported.
    assert!(violations(&model, &tests, &security, &admission).is_empty());
}

/// The rows `SM-02` is expected to cite.
const CITED_R2: [&str; 3] = ["VO-06", "VO-07", "VO-15"];

/// The start of the measurement of R2, the risk of index 1, in `Security`.
const R2_MEASUREMENT: &str = "record Measurement {risk := 1,";

/// Everything the claim of `SM-02` needs, as a list of violations; empty when the claim holds.
fn violations_r2(model: &Model, tests: &BTreeSet<String>, security: &str) -> Vec<String> {
    let mut out = Vec::new();
    let Some(row) = model.ids.id.iter().find(|r| r.id == "SM-02") else {
        return vec!["SM-02 is not in model/ids.toml".to_string()];
    };
    if row.level != Level::Build || row.suite != "security" {
        out.push("SM-02 is not a build row of the security suite".to_string());
    }
    let mut cited = cited_ids(&row.statement);
    cited.remove("SM-02");
    let expected: BTreeSet<String> = CITED_R2.iter().map(|s| s.to_string()).collect();
    if cited != expected {
        out.push(format!(
            "SM-02 cites {cited:?}, and the expected evidence is {expected:?}"
        ));
    }
    for id in &expected {
        let Some(cited_row) = model.ids.id.iter().find(|r| &r.id == id) else {
            out.push(format!("{id} is cited and is not in model/ids.toml"));
            continue;
        };
        if cited_row.level != Level::Build {
            out.push(format!("{id} is cited and is not a build row"));
        }
        let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
        if !tests.iter().any(|t| t.ends_with(&suffix)) {
            out.push(format!("{id} is cited and no test name ends in `{suffix}`"));
        }
    }
    match security.find(R2_MEASUREMENT) {
        None => out.push("Security has no measurement of R2".to_string()),
        Some(at) => {
            let record = &security[at..];
            let record = &record[..record.find(")})").unwrap_or(record.len())];
            if !record.contains(EXECUTED) {
                out.push("the measurement of R2 is not Executed".to_string());
            }
            for word in ["ReviewMissing", "VO-06", "VO-07", "VO-15"] {
                if !record.contains(word) {
                    out.push(format!("the Executed status of R2 does not name {word}"));
                }
            }
        }
    }
    out
}

/// `SM-02`: the claim holds on the repository as it is.
#[test]
fn the_measurement_of_r2_is_executed_by_the_review_rows_sm_02() {
    let (model, tests, security, _) = inputs();
    let found = violations_r2(&model, &tests, &security);
    assert!(
        found.is_empty(),
        "SM-02 does not hold:\n{}",
        found.join("\n")
    );
}

/// `SM-02`: the check can fail, with the planted defects `SECURITY.md` records.
#[test]
fn the_check_rejects_a_pending_r2_and_a_missing_row_sm_02() {
    let (model, tests, security, _) = inputs();

    let at = security.find(R2_MEASUREMENT).expect("R2 is stated");
    let flipped = format!(
        "{}{}",
        &security[..at],
        security[at..].replacen(EXECUTED, "ctor MeasureStatus.Pending", 1)
    );
    let found = violations_r2(&model, &tests, &flipped);
    assert!(
        found
            .iter()
            .any(|v| v.contains("the measurement of R2 is not Executed")),
        "{found:?}"
    );

    let mut without_test = tests.clone();
    without_test.retain(|t| !t.ends_with("_vo_06"));
    let found = violations_r2(&model, &without_test, &security);
    assert!(
        found
            .iter()
            .any(|v| v.contains("VO-06") && v.contains("no test name")),
        "{found:?}"
    );

    let mut without_row = model.clone();
    without_row.ids.id.retain(|r| r.id != "VO-07");
    let found = violations_r2(&without_row, &tests, &security);
    assert!(
        found
            .iter()
            .any(|v| v.contains("VO-07") && v.contains("not in model/ids.toml")),
        "{found:?}"
    );

    assert!(violations_r2(&model, &tests, &security).is_empty());
}

/// One executed measurement that cites rows: the risk (its index in `Security`), the row that
/// registers the claim, the rows the claim cites, and the words its status must name.
struct Executed {
    risk: u32,
    label: &'static str,
    row: &'static str,
    cited: &'static [&'static str],
    words: &'static [&'static str],
}

/// `SM-03`, `SM-04`, `SM-05`, `SM-06` and `SM-07`: the risks measured by the signed record, the
/// adapter, the task host and the results of the tools.
const MEASURED: [Executed; 5] = [
    Executed {
        risk: 0,
        label: "R1",
        row: "SM-03",
        cited: &["SG-03", "SG-10", "SG-11"],
        words: &["SG-03", "SG-10", "SG-11", "SG-04"],
    },
    Executed {
        risk: 2,
        label: "R3",
        row: "SM-04",
        cited: &["GW-12", "GW-13"],
        words: &["GW-12", "GW-13", "dns-rebinding-protection"],
    },
    Executed {
        risk: 7,
        label: "R8",
        row: "SM-05",
        cited: &["SG-04", "SG-05", "SG-07"],
        words: &["KeyRevoked", "SG-04", "SG-05", "SG-07"],
    },
    Executed {
        risk: 3,
        label: "R4",
        row: "SM-06",
        cited: &["TH-12", "TH-13"],
        words: &["TH-12", "TH-13", "LLS8002", "429"],
    },
    Executed {
        risk: 6,
        label: "R7",
        row: "SM-07",
        cited: &["TL-16"],
        words: &["TL-16", "output schema"],
    },
];

/// Everything the claim of one of those rows needs, as a list of violations.
fn violations_measured(
    m: &Executed,
    model: &Model,
    tests: &BTreeSet<String>,
    security: &str,
) -> Vec<String> {
    let mut out = Vec::new();
    let Some(row) = model.ids.id.iter().find(|r| r.id == m.row) else {
        return vec![format!("{} is not in model/ids.toml", m.row)];
    };
    if row.level != Level::Build || row.suite != "security" {
        out.push(format!(
            "{} is not a build row of the security suite",
            m.row
        ));
    }
    let mut cited = cited_ids(&row.statement);
    cited.remove(m.row);
    let expected: BTreeSet<String> = m.cited.iter().map(|s| s.to_string()).collect();
    if cited != expected {
        out.push(format!(
            "{} cites {cited:?}, and the expected evidence is {expected:?}",
            m.row
        ));
    }
    for id in &expected {
        let Some(cited_row) = model.ids.id.iter().find(|r| &r.id == id) else {
            out.push(format!("{id} is cited and is not in model/ids.toml"));
            continue;
        };
        if cited_row.level != Level::Build {
            out.push(format!("{id} is cited and is not a build row"));
        }
        let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
        if !tests.iter().any(|t| t.ends_with(&suffix)) {
            out.push(format!("{id} is cited and no test name ends in `{suffix}`"));
        }
    }
    match security.find(&format!("record Measurement {{risk := {},", m.risk)) {
        None => out.push(format!("Security has no measurement of {}", m.label)),
        Some(at) => {
            let record = &security[at..];
            let record = &record[..record.find(")})").unwrap_or(record.len())];
            if !record.contains(EXECUTED) {
                out.push(format!("the measurement of {} is not Executed", m.label));
            }
            for word in m.words {
                if !record.contains(word) {
                    out.push(format!(
                        "the Executed status of {} does not name {word}",
                        m.label
                    ));
                }
            }
        }
    }
    out
}

/// The claim of one row holds on the repository as it is.
fn holds(m: &Executed) {
    let (model, tests, security, _) = inputs();
    let found = violations_measured(m, &model, &tests, &security);
    assert!(
        found.is_empty(),
        "{} does not hold:\n{}",
        m.row,
        found.join("\n")
    );
}

/// `SM-03`: the measurement of R1 is executed by the signed record.
#[test]
fn the_measurement_of_r1_is_executed_by_the_signed_record_rows_sm_03() {
    holds(&MEASURED[0]);
}

/// `SM-04`: the measurement of R3 is executed by the conformance suite and the adapter.
#[test]
fn the_measurement_of_r3_is_executed_by_the_conformance_and_origin_rows_sm_04() {
    holds(&MEASURED[1]);
}

/// `SM-05`: the measurement of R8 is executed by the key history.
#[test]
fn the_measurement_of_r8_is_executed_by_the_key_history_rows_sm_05() {
    holds(&MEASURED[2]);
}

/// `SM-06`: the measurement of R4 is executed by the task host.
#[test]
fn the_measurement_of_r4_is_executed_by_the_rate_and_limit_rows_sm_06() {
    holds(&MEASURED[3]);
}

/// `SM-07`: the measurement of R7 is executed by the validation of the results of the tools.
#[test]
fn the_measurement_of_r7_is_executed_by_the_output_schema_row_sm_07() {
    holds(&MEASURED[4]);
}

/// `SM-03` to `SM-07`: the checks can fail, with the planted defects `SECURITY.md` records.
#[test]
fn the_check_rejects_a_pending_measure_and_a_missing_row_sm_03() {
    let (model, tests, security, _) = inputs();
    for m in &MEASURED {
        let marker = format!("record Measurement {{risk := {},", m.risk);
        let at = security.find(&marker).expect("the risk is stated");
        let flipped = format!(
            "{}{}",
            &security[..at],
            security[at..].replacen(EXECUTED, "ctor MeasureStatus.Pending", 1)
        );
        let found = violations_measured(m, &model, &tests, &flipped);
        assert!(
            found
                .iter()
                .any(|v| v.contains(&format!("the measurement of {} is not Executed", m.label))),
            "{found:?}"
        );

        let first = m.cited[0];
        let mut without_row = model.clone();
        without_row.ids.id.retain(|r| r.id != first);
        let found = violations_measured(m, &without_row, &tests, &security);
        assert!(
            found
                .iter()
                .any(|v| v.contains(first) && v.contains("not in model/ids.toml")),
            "{found:?}"
        );

        let suffix = format!("_{}", first.to_lowercase().replace('-', "_"));
        let mut without_test = tests.clone();
        without_test.retain(|t| !t.ends_with(&suffix));
        let found = violations_measured(m, &model, &without_test, &security);
        assert!(
            found
                .iter()
                .any(|v| v.contains(first) && v.contains("no test name")),
            "{found:?}"
        );

        assert!(violations_measured(m, &model, &tests, &security).is_empty());
    }
}

/// `SM-05`: R9 stays Pending, naming the issue, while no tool issues a requestState.
#[test]
fn the_measurement_of_r9_stays_pending_and_names_its_issue_sm_05() {
    let (_, _, security, _) = inputs();
    let at = security
        .find("record Measurement {risk := 8,")
        .expect("R9 is stated");
    let record = &security[at..];
    let record = &record[..record.find(")})").unwrap_or(record.len())];
    assert!(record.contains("ctor MeasureStatus.Pending"), "{record}");
    assert!(record.contains("#45"), "{record}");
}
