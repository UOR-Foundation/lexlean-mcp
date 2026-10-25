//! `SM-01`: the measurement of R5 (path escape) is executed.
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

    // R5 is the only Executed measurement, and its status is the Executed one.
    if security.matches(EXECUTED).count() != 1 {
        out.push("exactly one measurement must be Executed".to_string());
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
        "onlyPathEscapeIsExecuted",
        "noClauseIsPending",
        "measurementsPendingExceptR5",
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
