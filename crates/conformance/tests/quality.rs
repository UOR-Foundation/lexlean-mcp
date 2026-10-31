//! `QM-07`: the measures of Q1 to Q4 and Q6 to Q8 are executed.
//!
//! `src/Quality.lex.tex` records the status of each measure. A status of
//! `Executed` is a claim about rows of the register, and LexLean cannot see the
//! register, so the tie lives here, as it does for `SM-01`: the rows each claim
//! cites are read from the statement of `QM-07` itself (one source, R1), and
//! each must be registered with a test. Whether those tests pass is the verdict
//! of the workspace test run that includes this file.

use std::collections::BTreeSet;
use std::fs;

use negative_fixtures::Register;
use quality_map::{cardinality_of_model, load_files, Map, Registry, Sources};
use repo_model::{Level, Model};

mod common;
use common::{root, workspace_test_names};

/// The rows `QM-07` is expected to cite, by measure: Q1 (index 0), Q2 (1), Q3 (2), Q4 (3), Q6 (5),
/// Q7 (6) and Q8 (7).
/// The last row of each is the row of the tools suite that runs its clause where the tools are
/// built: `TL-06` the build of every example beside the CLI for Q1, `TL-15` a tool that runs for
/// each in-scope function for Q2, `TL-08` the host process accounting across each call for Q4,
/// and `TL-13` the running `lexlean_explain` for Q8. Q3 is observed by the fault cases of the
/// verifier (`VP-04`, `VP-05`) and of the signed record (`SG-07`, `SG-12`). Q6 is observed by the
/// search of the host adapter source for a tool or method name (`GW-06`, and `EF-06` for the
/// effect names), Q7 by the build of a fixed project in every run of `just vv` (`QM-09`).
const CITED: [(u32, &[&str]); 7] = [
    (0, &["TL-03", "TL-06"]),
    (1, &["QM-01", "QM-02", "QM-03", "TL-15"]),
    (2, &["VP-04", "VP-05", "SG-07", "SG-12"]),
    (3, &["TL-08"]),
    (5, &["EF-06", "GW-06"]),
    (6, &["QM-09"]),
    (7, &["QM-04", "QM-05", "QM-06", "TL-13"]),
];

/// The one executed measure that keeps a clause in `pendingClauses`: Q1, whose clause for
/// `lexlean_verify` and the verify fixtures waits for verification as tasks (#11).
const KEEPS_A_CLAUSE: u32 = 0;

/// The measures that stay pending, each naming what executes it.
const PENDING: [u32; 1] = [4];

const EXECUTED: &str = "ctor EvidenceStatus.Executed";
const PENDING_CTOR: &str = "ctor EvidenceStatus.Pending";

/// The registered IDs a statement names, other than `QM-07`.
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
    found.remove("QM-07");
    found
}

/// The text of the declaration `kind{ name: <name>, ...` up to its closing brace.
fn block<'a>(source: &'a str, kind: &str, name: &str) -> Option<&'a str> {
    // The formatter writes a short declaration on one line and a long one over several.
    let long = format!("{kind}{{\n  name: {name},");
    if let Some(start) = source.find(&long) {
        let end = source[start..].find("\n}\n")? + start;
        return Some(&source[start..end]);
    }
    let short = format!("{kind}{{name: {name},");
    let start = source.find(&short)?;
    let end = source[start..].find("}\n")? + start;
    Some(&source[start..end])
}

/// The evidence record of one measure.
fn evidence_of(quality: &str, measure: u32) -> Option<&str> {
    let head = format!("record MeasureEvidence {{measure := {measure}, status := ");
    let at = quality.find(&head)?;
    let record = &quality[at..];
    Some(&record[..record.find("\")})").map_or(record.len(), |e| e + 4)])
}

struct Inputs {
    model: Model,
    tests: BTreeSet<String>,
    quality: String,
    map: Map,
    registry: Registry,
    fixtures: Register,
}

/// Everything the claim of `QM-07` needs, as a list of violations; empty when
/// the claim holds. A function of its inputs so that a planted defect can be
/// given to it.
fn violations(i: &Inputs) -> Vec<String> {
    let mut out = Vec::new();
    let Inputs {
        model,
        tests,
        quality,
        map,
        registry,
        fixtures,
    } = i;

    let Some(row) = model.ids.id.iter().find(|r| r.id == "QM-07") else {
        return vec!["QM-07 is not in model/ids.toml".to_string()];
    };
    if row.level != Level::Build || row.suite != "quality" {
        out.push("QM-07 is not a build row of the quality suite".to_string());
    }
    let expected: BTreeSet<String> = CITED
        .iter()
        .flat_map(|(_, ids)| ids.iter().map(|s| s.to_string()))
        .collect();
    let cited = cited_ids(&row.statement);
    if cited != expected {
        out.push(format!(
            "QM-07 cites {cited:?}, and the expected evidence is {expected:?}"
        ));
    }
    for id in &expected {
        let Some(cited_row) = model.ids.id.iter().find(|r| &r.id == id) else {
            out.push(format!("{id} is cited and is not in model/ids.toml"));
            continue;
        };
        let suite = match &id[..2] {
            "TL" => "tools",
            "EF" => "effects",
            "GW" => "gateway",
            "VP" => "verifier",
            "SG" => "signing",
            _ => "quality",
        };
        if cited_row.suite != suite {
            out.push(format!(
                "{id} is cited and is not a row of the {suite} suite"
            ));
        }
        let suffix = format!("_{}", id.to_lowercase().replace('-', "_"));
        if !tests.iter().any(|t| t.ends_with(&suffix)) {
            out.push(format!("{id} is cited and no test name ends in `{suffix}`"));
        }
    }

    if !quality.contains("\\begin{lexlean}{Quality}") {
        out.push("src/Quality.lex.tex is not the Quality module".to_string());
    }
    if quality.matches(EXECUTED).count() != CITED.len() {
        out.push(format!("exactly {} measures must be Executed", CITED.len()));
    }
    if quality.matches(PENDING_CTOR).count() != PENDING.len() {
        out.push(format!(
            "exactly {} measures must be Pending",
            PENDING.len()
        ));
    }

    // Q1 to Q4 and Q6 to Q8 are Executed, each naming its rows; only Q1 points at a pending clause.
    for (measure, ids) in CITED {
        match evidence_of(quality, measure) {
            None => out.push(format!("Quality has no evidence for measure {measure}")),
            Some(record) => {
                if !record.contains(EXECUTED) {
                    out.push(format!("the measure {measure} is not Executed"));
                }
                for id in ids {
                    if !record.contains(id) {
                        out.push(format!(
                            "the Executed status of measure {measure} does not name {id}"
                        ));
                    }
                }
                if measure == KEEPS_A_CLAUSE {
                    if !record.contains("pendingClauses") || !record.contains("#11") {
                        out.push(format!(
                            "the Executed status of measure {measure} does not point at its clause for lexlean_verify in pendingClauses, which waits for #11"
                        ));
                    }
                } else if record.contains("pendingClauses") {
                    out.push(format!(
                        "the Executed status of measure {measure} points at pendingClauses, and its running clause is executed"
                    ));
                }
            }
        }
    }
    // Every other measure is Pending and names what executes it.
    for measure in PENDING {
        match evidence_of(quality, measure) {
            None => out.push(format!("Quality has no evidence for measure {measure}")),
            Some(record) => {
                if !record.contains(PENDING_CTOR) {
                    out.push(format!("the measure {measure} is not Pending"));
                }
                if !record.contains('#') {
                    out.push(format!(
                        "the pending measure {measure} names no issue that executes it"
                    ));
                }
            }
        }
    }

    for name in [
        "executedMeasuresAreQ1ToQ4AndQ6ToQ8",
        "measuresPendingAreQ5",
        "onlyTheVerifyClauseOfQ1IsPending",
    ] {
        if block(quality, "theorem", name).is_none() {
            out.push(format!("Quality has no theorem {name}"));
        }
    }
    match block(quality, "definition", "pendingClauses") {
        Some(clauses) => {
            for (measure, ids) in CITED {
                if measure != KEEPS_A_CLAUSE && clauses.contains(&format!("index := {measure}")) {
                    out.push(format!(
                        "pendingClauses records a clause of Q{}, whose running clause {} executes",
                        measure + 1,
                        ids[ids.len() - 1]
                    ));
                }
            }
            if !clauses.contains(&format!("index := {KEEPS_A_CLAUSE}")) || !clauses.contains("#11")
            {
                out.push(
                    "pendingClauses does not record the clause of Q1 for lexlean_verify, which waits for #11"
                        .to_string(),
                );
            }
            let count = clauses.matches("record Named {").count();
            if count != 1 {
                out.push(format!(
                    "pendingClauses records {count} clauses, and only the clause of Q1 for lexlean_verify waits"
                ));
            }
        }
        None => out.push("pendingClauses is missing".to_string()),
    }

    // The counts the model states are the counts the pinned files give.
    let in_scope = fixtures.counts().in_scope() as u64;
    if cardinality_of_model(quality, 0) != Some(in_scope) {
        out.push(format!(
            "the model counts {:?} negative fixtures and the register holds {in_scope} in scope",
            cardinality_of_model(quality, 0)
        ));
    }
    let exposed = map.exposed().len() as u64;
    if cardinality_of_model(quality, 1) != Some(exposed) {
        out.push(format!(
            "the model counts {:?} in-scope functions and the map exposes {exposed}",
            cardinality_of_model(quality, 1)
        ));
    }
    let codes = registry.error.len() as u64;
    if cardinality_of_model(quality, 7) != Some(codes) {
        out.push(format!(
            "the model counts {:?} diagnostic codes and the registry holds {codes}",
            cardinality_of_model(quality, 7)
        ));
    }
    out
}

fn inputs() -> Inputs {
    let root = root();
    let dir = root.join("crates/quality-map");
    let sources = Sources::load(&dir.join("sources.toml")).expect("sources.toml loads");
    let files = load_files(&dir.join("lexlean"), &sources).expect("lexlean/ loads");
    let registry = Registry::parse(
        &String::from_utf8(files.get("errors.toml").expect("vendored").clone()).expect("UTF-8"),
    )
    .expect("the registry parses");
    Inputs {
        model: Model::load(&root.join("model")).expect("model loads"),
        tests: workspace_test_names(&root),
        quality: fs::read_to_string(root.join("src/Quality.lex.tex")).expect("Quality reads"),
        map: Map::load(&dir.join("map.toml")).expect("map.toml loads"),
        registry,
        fixtures: Register::load(&root.join("crates/negative-fixtures/register.toml"))
            .expect("register.toml loads"),
    }
}

/// `QM-07`: the claim holds on the repository as it is.
#[test]
fn the_measures_of_q1_to_q4_and_q6_to_q8_are_executed_by_their_rows_qm_07() {
    let found = violations(&inputs());
    assert!(
        found.is_empty(),
        "QM-07 does not hold:\n{}",
        found.join("\n")
    );
}

/// `QM-07`: the check can fail. Each planted defect is one `QUALITY.md`
/// records, and each must be reported.
#[test]
fn the_check_rejects_a_pending_measure_a_missing_row_and_a_wrong_count_qm_07() {
    let base = inputs();
    let reported = |i: &Inputs, needle: &str| violations(i).iter().any(|v| v.contains(needle));

    // Each Executed measure flipped back to Pending.
    for (measure, _) in CITED {
        let at = base
            .quality
            .find(&format!("record MeasureEvidence {{measure := {measure},"))
            .expect("the evidence");
        let flipped = format!(
            "{}{}",
            &base.quality[..at],
            base.quality[at..].replacen(EXECUTED, PENDING_CTOR, 1)
        );
        let planted = Inputs {
            quality: flipped,
            ..clone(&base)
        };
        assert!(reported(
            &planted,
            &format!("the measure {measure} is not Executed")
        ));
    }

    // The running clause of a measure left in pendingClauses once the row that runs it executes it.
    for (index, name, row) in [
        (7, "Q8 running explain", "TL-13"),
        (1, "Q2 tools exist", "TL-15"),
        (3, "Q4 no child process", "TL-08"),
        (2, "Q3 no record on a fault", "SG-12"),
    ] {
        let at = base
            .quality
            .find("name: pendingClauses,")
            .expect("pendingClauses");
        let end = at
            + base.quality[at..]
                .find("(nil Named)")
                .expect("the end of the list");
        let clause = format!(
            "(cons (record Named {{index := {index}, name := \"{name}\", text := \"left pending\"}}) (nil Named))"
        );
        let left_pending = format!(
            "{}{clause}{}",
            &base.quality[..end],
            &base.quality[end + "(nil Named)".len()..]
        );
        let planted = Inputs {
            quality: left_pending,
            ..clone(&base)
        };
        assert!(reported(
            &planted,
            &format!(
                "pendingClauses records a clause of Q{}, whose running clause {row} executes",
                index + 1
            )
        ));
    }

    // A measure pointing at pendingClauses again, with the clause gone.
    for (index, last) in [(7, "QM-06"), (1, "QM-03"), (3, "TL-08"), (2, "SG-12")] {
        let at = base
            .quality
            .find(&format!("record MeasureEvidence {{measure := {index},"))
            .expect("the evidence");
        let pointing = format!(
            "{}{}",
            &base.quality[..at],
            base.quality[at..].replacen(
                last,
                &format!("{last}; the running clause is in pendingClauses"),
                1
            )
        );
        let planted = Inputs {
            quality: pointing,
            ..clone(&base)
        };
        assert!(reported(
            &planted,
            &format!("the Executed status of measure {index} points at pendingClauses")
        ));
    }

    // Q1 no longer pointing at its clause for lexlean_verify.
    let at = base
        .quality
        .find("record MeasureEvidence {measure := 0,")
        .expect("Q1");
    let silent = format!(
        "{}{}",
        &base.quality[..at],
        base.quality[at..].replacen("is in pendingClauses", "is done", 1)
    );
    let planted = Inputs {
        quality: silent,
        ..clone(&base)
    };
    assert!(reported(
        &planted,
        "the Executed status of measure 0 does not point at its clause for lexlean_verify"
    ));

    // The clause of Q1 for lexlean_verify dropped while verification is not a task.
    let at = base
        .quality
        .find("name: pendingClauses,")
        .expect("pendingClauses");
    let start = at
        + base.quality[at..]
            .find("body: ")
            .expect("the body of pendingClauses")
        + "body: ".len();
    let end = at
        + base.quality[at..]
            .find("\n}\n")
            .expect("the end of pendingClauses");
    let emptied = format!(
        "{}(nil Named){}",
        &base.quality[..start],
        &base.quality[end..]
    );
    let planted = Inputs {
        quality: emptied,
        ..clone(&base)
    };
    assert!(reported(
        &planted,
        "pendingClauses does not record the clause of Q1 for lexlean_verify"
    ));

    // The row that runs a clause dropped from the statement of QM-07: the clause loses its row.
    for (measure, row) in [("Q8", "TL-13"), ("Q2", "TL-15"), ("Q4", "TL-08")] {
        let mut model = base.model.clone();
        for item in &mut model.ids.id {
            if item.id == "QM-07" {
                item.statement = item.statement.replace(
                    &format!(" ({measure}, whose running clause {row} executes)"),
                    &format!(" ({measure})"),
                );
            }
        }
        let planted = Inputs {
            model,
            ..clone(&base)
        };
        assert!(reported(&planted, "QM-07 cites"), "{measure} without {row}");
    }
    let mut model = base.model.clone();
    for item in &mut model.ids.id {
        if item.id == "QM-07" {
            item.statement = item.statement.replace("by TL-03 and TL-06,", "by TL-03,");
        }
    }
    let planted = Inputs {
        model,
        ..clone(&base)
    };
    assert!(reported(&planted, "QM-07 cites"), "Q1 without TL-06");
    let mut model = base.model.clone();
    for item in &mut model.ids.id {
        if item.id == "QM-07" {
            item.statement = item.statement.replace("SG-07 and SG-12", "SG-07");
        }
    }
    let planted = Inputs {
        model,
        ..clone(&base)
    };
    assert!(reported(&planted, "QM-07 cites"), "Q3 without SG-12");

    // A measure the register does not execute flipped to Executed.
    let at = base
        .quality
        .find("record MeasureEvidence {measure := 4,")
        .expect("Q5");
    let flipped = format!(
        "{}{}",
        &base.quality[..at],
        base.quality[at..].replacen(PENDING_CTOR, EXECUTED, 1)
    );
    let planted = Inputs {
        quality: flipped,
        ..clone(&base)
    };
    assert!(reported(
        &planted,
        &format!("exactly {} measures must be Executed", CITED.len())
    ));

    // A cited row removed from the register.
    let mut model = base.model.clone();
    model.ids.id.retain(|r| r.id != "QM-03");
    let planted = Inputs {
        model,
        ..clone(&base)
    };
    assert!(reported(
        &planted,
        "QM-03 is cited and is not in model/ids.toml"
    ));

    // A cited row with no test.
    let mut tests = base.tests.clone();
    tests.retain(|t| !t.ends_with("_qm_05"));
    let planted = Inputs {
        tests,
        ..clone(&base)
    };
    assert!(reported(&planted, "QM-05 is cited and no test name"));

    // The pending clauses dropped from the model.
    let dropped = base
        .quality
        .replace("name: pendingClauses,", "name: droppedClauses,");
    let planted = Inputs {
        quality: dropped,
        ..clone(&base)
    };
    assert!(reported(&planted, "pendingClauses is missing"));

    // A count that disagrees with the pinned files.
    let wrong = base.quality.replace(
        "record Cardinality {measure := 7, count := 86,",
        "record Cardinality {measure := 7, count := 85,",
    );
    let planted = Inputs {
        quality: wrong,
        ..clone(&base)
    };
    assert!(reported(
        &planted,
        "the model counts Some(85) diagnostic codes"
    ));
    let stale = base.quality.replace(
        "record Cardinality {measure := 0, count := 285,",
        "record Cardinality {measure := 0, count := 273,",
    );
    let planted = Inputs {
        quality: stale,
        ..clone(&base)
    };
    assert!(reported(
        &planted,
        "the model counts Some(273) negative fixtures and the register holds 285"
    ));

    // The control: nothing planted, nothing reported.
    assert!(violations(&base).is_empty());
}

fn clone(i: &Inputs) -> Inputs {
    Inputs {
        model: i.model.clone(),
        tests: i.tests.clone(),
        quality: i.quality.clone(),
        map: i.map.clone(),
        registry: i.registry.clone(),
        fixtures: i.fixtures.clone(),
    }
}
