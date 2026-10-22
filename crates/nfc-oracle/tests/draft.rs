//! DL-01 and DL-03: draft admission through the exported LexLean root
//! `admitDraftRun`, on the full tables.
//!
//! The decision and the order are LexLean source; `lexlean verify` checks the
//! instances of `src/Admission.lex.tex`. Here the package LexLean exports for
//! `admitDraftRun` judges the drafts of those instances, so what the module
//! states is established by running the exported decision, and the export is
//! shown to be the recorded one that declares no `recursion` effect.

mod common;

use std::path::Path;

use common::{admission_source, block, hex, refusal_names, root, sha256, verdict_text};
use nfc_oracle::admit_draft;
use sha2::{Digest, Sha256};

const PACKAGE: &str = "lexlean_mcp_draft_admission";

fn generated() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("generated")
        .join(PACKAGE)
}

fn provenance() -> serde_json::Value {
    serde_json::from_slice(
        &std::fs::read(generated().join("provenance.json")).expect("provenance.json is committed"),
    )
    .expect("provenance.json is JSON")
}

#[test]
fn the_draft_root_declares_no_recursion_and_is_the_recorded_export_dl_01() {
    let provenance = provenance();
    assert_eq!(provenance["name"], PACKAGE);
    assert_eq!(provenance["profile"], "rust-std");
    assert_eq!(provenance["spec"], "lexlean/rust-provenance/1");
    let exports = provenance["exports"].as_array().expect("exports is a list");
    assert_eq!(exports.len(), 1, "one root is exported");
    assert_eq!(exports[0]["name"], "run");
    assert_eq!(
        provenance["effects"],
        serde_json::json!(["allocation", "overflow"]),
        "a root that recurses to a depth bounded by its input states `recursion`"
    );
    // The oracle compiled exactly the recorded files, not an edited copy.
    let source = std::fs::read(generated().join("src").join("lib.rs")).expect("src/lib.rs");
    let recorded = provenance["files"]["src/lib.rs"]
        .as_str()
        .expect("the export records src/lib.rs");
    assert_eq!(
        recorded,
        hex(&Sha256::digest(&source)),
        "src/lib.rs is not the file the export recorded: regenerate it, never edit it"
    );
    assert_eq!(
        recorded,
        sha256(nfc_oracle::DRAFT_SOURCE.as_bytes()),
        "the oracle compiled a different file from the one recorded"
    );
    let manifest = std::fs::read(generated().join("Cargo.toml")).expect("Cargo.toml");
    assert_eq!(
        provenance["files"]["Cargo.toml"]
            .as_str()
            .expect("the export records Cargo.toml"),
        sha256(&manifest)
    );
    let lock = std::fs::read_to_string(root().join("lexlean.lock")).expect("lexlean.lock reads");
    let line = lock
        .lines()
        .find(|l| l.starts_with("compiler_semantics = "))
        .expect("the lock records the compiler semantics ID");
    let id = line
        .trim_start_matches("compiler_semantics = ")
        .trim_matches('"');
    assert_eq!(
        provenance["compiler_semantics"].as_str(),
        Some(id),
        "the package was exported by another LexLean than the lock names"
    );
}

#[test]
fn the_stages_over_the_rows_are_executable_definitions_dl_01() {
    let source = admission_source();
    for name in [
        "countRowsInto",
        "countRows",
        "totalBytesInto",
        "totalBytes",
        "rowsStage",
        "pathsInto",
        "occurs",
        "anyRepeat",
        "duplicatesStage",
        "admitDraft",
    ] {
        let definition = block(&source, "definition", name);
        assert!(
            definition.contains("executable: true"),
            "{name} is not executable, so it cannot be in the closure of a production root:\n{definition}"
        );
    }
    let root = block(&source, "definition", "admitDraftRun");
    assert!(root.contains("production: {effects: [allocation, overflow], targets: [rust-std]}"));
    assert!(
        !root.contains("recursion"),
        "the root declares no recursion effect:\n{root}"
    );
}

/// `0x737263` as octets; `0x` is empty.
fn hex_octets(token: &str) -> Vec<u8> {
    let digits = token.strip_prefix("0x").expect("a hexadecimal literal");
    (0..digits.len() / 2)
        .map(|i| u8::from_str_radix(&digits[2 * i..2 * i + 2], 16).expect("hex digits"))
        .collect()
}

/// A draft instance: the limits, the roots, the rows and the stated verdict.
struct Instance {
    name: String,
    max_files: u64,
    max_bytes: u64,
    roots: Vec<Vec<u8>>,
    rows: Vec<(Vec<u8>, Vec<u8>)>,
    verdict: String,
}

fn parse_instance(name: &str, statement: &str) -> Option<Instance> {
    let rest = statement.strip_prefix("((admitDraft ")?;
    let (max_files, rest) = rest.split_once(' ')?;
    let (max_bytes, rest) = rest.split_once(' ')?;
    let (head, tail) = rest.split_once(" = ")?;
    // The roots are the literals after `cons` before the first row; a row is `(0x.., 0x..)`.
    let first_row = head.find("(0x").map_or(head.len(), |at| at);
    let (roots_text, rows_text) = head.split_at(first_row);
    let roots = roots_text
        .split("cons ")
        .filter(|part| part.starts_with("0x"))
        .map(|part| hex_octets(part.split([' ', ')']).next().expect("a literal")))
        .collect();
    let mut rows = Vec::new();
    let mut rest = rows_text;
    while let Some(at) = rest.find("(0x") {
        let after = &rest[at + 1..];
        let (path, after) = after.split_once(", ")?;
        let (content, after) = after.split_once(')')?;
        rows.push((hex_octets(path), hex_octets(content)));
        rest = after;
    }
    let tail = tail.trim().trim_end_matches(',');
    let verdict = if tail.contains("Verdict.Admitted") {
        "Admitted".to_owned()
    } else {
        let refusal = tail
            .split("Refusal.")
            .nth(1)?
            .trim_end_matches(')')
            .trim()
            .to_owned();
        format!("Refused {refusal}")
    };
    Some(Instance {
        name: name.to_owned(),
        max_files: max_files.parse().ok()?,
        max_bytes: max_bytes.parse().ok()?,
        roots,
        rows,
        verdict,
    })
}

fn instances() -> Vec<Instance> {
    let source = admission_source();
    let mut out = Vec::new();
    for part in source.split("theorem{").skip(1) {
        let name = part
            .lines()
            .find_map(|l| l.trim().strip_prefix("name: "))
            .expect("a theorem has a name")
            .trim_end_matches(',');
        let Some(statement) = part
            .lines()
            .find_map(|l| l.trim().strip_prefix("statement: "))
        else {
            continue;
        };
        if let Some(instance) = parse_instance(name, statement.trim_end_matches(',')) {
            out.push(instance);
        }
    }
    out
}

fn judge(instance: &Instance) -> String {
    let roots: Vec<&[u8]> = instance.roots.iter().map(Vec::as_slice).collect();
    verdict_text(&admit_draft(
        instance.max_files,
        instance.max_bytes,
        &roots,
        &instance.rows,
    ))
}

#[test]
fn every_draft_instance_holds_through_the_exported_root_dl_03() {
    let all = instances();
    let named: Vec<&str> = all.iter().map(|i| i.name.as_str()).collect();
    for name in [
        "draftWithinLimits",
        "draftTooManyFiles",
        "draftTooManyBytes",
        "draftPathRefusalFirst",
        "maxBytesCountsContentOnly",
        "emptyDraftAdmitted",
        "draftDuplicatePath",
        "draftPathRefusalBeforeTheLimits",
        "draftEarlierRowRefusedFirst",
        "draftTooManyFilesBeforeTooManyBytes",
        "draftTooManyFilesBeforeDuplicate",
        "draftTooManyBytesBeforeDuplicate",
    ] {
        assert!(named.contains(&name), "{name} is not parsed: {named:?}");
    }
    let mut failures = Vec::new();
    for instance in &all {
        let got = judge(instance);
        if got != instance.verdict {
            failures.push(format!(
                "{}: the module states {}, the exported root returns {got}",
                instance.name, instance.verdict
            ));
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn a_draft_failing_several_stages_is_refused_by_the_earliest_dl_03() {
    let names = refusal_names();
    assert_eq!(
        names.last().map(String::as_str),
        Some("DuplicatePath"),
        "DuplicatePath follows the refusals that were already numbered: {names:?}"
    );
    let row = |path: &str, content: &str| (path.as_bytes().to_vec(), content.as_bytes().to_vec());
    let roots: [&[u8]; 1] = [b"src"];
    let said = |max_files: u64, max_bytes: u64, rows: Vec<(Vec<u8>, Vec<u8>)>| {
        verdict_text(&admit_draft(max_files, max_bytes, &roots, &rows))
    };
    // A row refused as NotNfc wins over every limit and over a duplicate, whatever
    // comes after it, and the earlier of two refused rows is the one named.
    let bad = || row("src/e\u{301}.lex.tex", "x");
    let dup = || row("src/a.lex.tex", "xx");
    assert_eq!(
        said(0, 0, vec![dup(), dup(), bad()]),
        "Refused NotNfc",
        "a path refusal precedes the limits and the duplicate scan"
    );
    assert_eq!(
        said(
            9,
            9,
            vec![row("src/a.txt", ""), row("src/../b.lex.tex", "")]
        ),
        "Refused NotLexTex"
    );
    // TooManyFiles before TooManyBytes before DuplicatePath.
    assert_eq!(said(1, 1, vec![dup(), dup()]), "Refused TooManyFiles");
    assert_eq!(said(2, 1, vec![dup(), dup()]), "Refused TooManyBytes");
    assert_eq!(said(2, 4, vec![dup(), dup()]), "Refused DuplicatePath");
    assert_eq!(
        said(
            2,
            4,
            vec![row("src/a.lex.tex", "xx"), row("src/b.lex.tex", "xx")]
        ),
        "Admitted"
    );
}

#[test]
fn a_duplicate_is_a_repeat_of_the_whole_path_dl_03() {
    let roots: [&[u8]; 1] = [b"src"];
    let said = |rows: Vec<(Vec<u8>, Vec<u8>)>| {
        verdict_text(&admit_draft(9, 99, &roots, &rows))
    };
    let row = |path: &str| (path.as_bytes().to_vec(), Vec::new());
    // Equal paths that are not adjacent, a path that is a prefix of another, and the
    // same path under another directory.
    assert_eq!(
        said(vec![row("src/a.lex.tex"), row("src/b.lex.tex"), row("src/a.lex.tex")]),
        "Refused DuplicatePath"
    );
    assert_eq!(said(vec![row("src/a.lex.tex"), row("src/a.lex.tex.lex.tex")]), "Admitted");
    assert_eq!(said(vec![row("src/a.lex.tex"), row("src/d/a.lex.tex")]), "Admitted");
}
