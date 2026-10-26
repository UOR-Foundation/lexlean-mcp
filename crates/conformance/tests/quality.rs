//! `QM-07`: the measures of Q2 and Q8 are executed.
//!
//! `src/Quality.lex.tex` records the status of each measure. A status of
//! `Executed` is a claim about rows of the register, and LexLean cannot see the
//! register, so the tie lives here, as it does for `SM-01`: the rows each claim
//! cites are read from the statement of `QM-07` itself (one source, R1), and
//! each must be registered with a test. Whether those tests pass is the verdict
//! of the workspace test run that includes this file.

use std::collections::BTreeSet;
use std::fs;

use quality_map::{cardinality_of_model, load_files, Map, Registry, Sources};
use repo_model::{Level, Model};

mod common;
use common::{root, workspace_test_names};

/// The rows `QM-07` is expected to cite, by measure: Q2 (index 1) and Q8 (index 7). `TL-13` and
/// `TL-15` are the rows of the tools suite that run the clauses which wait for the tools: the
/// running `lexlean_explain` for Q8, and a tool that runs for each in-scope function for Q2.
/// Neither clause is in `pendingClauses` any more.
const CITED: [(u32, &[&str]); 2] = [
    (1, &["QM-01", "QM-02", "QM-03", "TL-15"]),
    (7, &["QM-04", "QM-05", "QM-06", "TL-13"]),
];

/// The measures that stay pending, each naming what executes it.
const PENDING: [u32; 6] = [0, 2, 3, 4, 5, 6];

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
        let suite = if id.starts_with("TL-") {
            "tools"
        } else {
            "quality"
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
    if quality.matches(EXECUTED).count() != 2 {
        out.push("exactly two measures must be Executed".to_string());
    }
    if quality.matches(PENDING_CTOR).count() != PENDING.len() {
        out.push(format!(
            "exactly {} measures must be Pending",
            PENDING.len()
        ));
    }

    // Q2 and Q8 are Executed, each naming its rows; neither points at a pending clause.
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
                if record.contains("pendingClauses") {
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
        "onlyApiMapAndExplainAreExecuted",
        "measuresPendingExceptQ2AndQ8",
        "noQualityClauseIsPending",
    ] {
        if block(quality, "theorem", name).is_none() {
            out.push(format!("Quality has no theorem {name}"));
        }
    }
    match block(quality, "definition", "pendingClauses") {
        Some(clauses) => {
            if clauses.contains("index := 1") {
                out.push(
                    "pendingClauses records a clause of Q2, whose running clause TL-15 executes"
                        .to_string(),
                );
            }
            if clauses.contains("index := 7") {
                out.push(
                    "pendingClauses records a clause of Q8, whose running clause TL-13 executes"
                        .to_string(),
                );
            }
        }
        None => out.push("pendingClauses is missing".to_string()),
    }

    // The counts the model states are the counts the pinned files give.
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
    }
}

/// `QM-07`: the claim holds on the repository as it is.
#[test]
fn the_measures_of_q2_and_q8_are_executed_by_the_quality_rows_qm_07() {
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
fn the_check_rejects_a_pending_q2_a_missing_row_and_a_wrong_count_qm_07() {
    let base = inputs();
    let reported = |i: &Inputs, needle: &str| violations(i).iter().any(|v| v.contains(needle));

    // Q2 flipped back to Pending.
    let at = base
        .quality
        .find("record MeasureEvidence {measure := 1,")
        .expect("Q2");
    let flipped = format!(
        "{}{}",
        &base.quality[..at],
        base.quality[at..].replacen(EXECUTED, PENDING_CTOR, 1)
    );
    let planted = Inputs {
        quality: flipped,
        ..clone(&base)
    };
    assert!(reported(&planted, "the measure 1 is not Executed"));

    // Q8 flipped back to Pending.
    let at = base
        .quality
        .find("record MeasureEvidence {measure := 7,")
        .expect("Q8");
    let flipped = format!(
        "{}{}",
        &base.quality[..at],
        base.quality[at..].replacen(EXECUTED, PENDING_CTOR, 1)
    );
    let planted = Inputs {
        quality: flipped,
        ..clone(&base)
    };
    assert!(reported(&planted, "the measure 7 is not Executed"));

    // The running clause of a measure left in pendingClauses once the row that runs it executes it.
    for (index, name, row) in [
        (7, "Q8 running explain", "TL-13"),
        (1, "Q2 tools exist", "TL-15"),
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
    for (index, last) in [(7, "QM-06"), (1, "QM-03")] {
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

    // The row that runs a clause dropped from the statement of QM-07: the clause loses its row.
    for (measure, row) in [("Q8", "TL-13"), ("Q2", "TL-15")] {
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

    // A measure the register does not execute flipped to Executed.
    let at = base
        .quality
        .find("record MeasureEvidence {measure := 0,")
        .expect("Q1");
    let flipped = format!(
        "{}{}",
        &base.quality[..at],
        base.quality[at..].replacen(PENDING_CTOR, EXECUTED, 1)
    );
    let planted = Inputs {
        quality: flipped,
        ..clone(&base)
    };
    assert!(reported(&planted, "exactly two measures must be Executed"));

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
    }
}
