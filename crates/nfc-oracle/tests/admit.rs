//! NF-12: path admission through the exported LexLean root `admitRun`, on the
//! full tables.
//!
//! The decision and the order are LexLean source; `lexlean verify` checks the
//! instances of `src/Admission.lex.tex` (those that need the tables, on the
//! sample image). Here the package LexLean exports for `admitRun` judges the
//! paths, so the full-table refusal of `e` followed by U+0301 is established by
//! running the admission decision itself, not only by running the NFC decision.

mod common;

use common::{admission_source, block, refusal_names, verdict_text};
use nfc_oracle::{admit, admit_on, Image};

const ROOT: &[u8] = b"src";

fn judge(path: &[u8]) -> String {
    verdict_text(&admit(&[ROOT], path))
}

#[test]
fn the_refusal_indices_follow_the_constructor_order_nf_12() {
    let source = admission_source();
    let index = block(&source, "definition", "refusalIndex");
    let names = refusal_names();
    assert!(names.len() >= 10, "the refusals: {names:?}");
    for (position, name) in names.iter().enumerate() {
        let arm = format!("| Refusal.{name} => {position}");
        assert!(index.contains(&arm), "refusalIndex lacks `{arm}`:\n{index}");
    }
    assert_eq!(
        index.matches("| Refusal.").count(),
        names.len(),
        "refusalIndex names every refusal once"
    );
    assert!(names.contains(&"NotNfc".to_owned()));
}

#[test]
fn e_followed_by_u0301_is_refused_as_not_nfc_and_u00e9_is_admitted_nf_12() {
    assert_eq!(judge("src/e\u{301}.lex.tex".as_bytes()), "Refused NotNfc");
    assert_eq!(judge("src/\u{e9}.lex.tex".as_bytes()), "Admitted");
    // The same on the sample image the kernel-checked instances use.
    for (path, expected) in [
        ("src/e\u{301}.lex.tex", "Refused NotNfc"),
        ("src/\u{e9}.lex.tex", "Admitted"),
    ] {
        assert_eq!(
            verdict_text(&admit_on(Image::Sample, &[ROOT], path.as_bytes())),
            expected
        );
    }
}

#[test]
fn the_stages_run_in_the_order_the_instances_state_nf_12() {
    let vectors: &[(&[u8], &str)] = &[
        // UTF-8 first: invalid octets win over a decomposed letter before them.
        (b"src/e\xCC\x81\xFF.lex.tex", "Refused NotUtf8"),
        (b"src/\xFF.lex.tex", "Refused NotUtf8"),
        // NFC before the root check and before the suffix check.
        ("other/e\u{301}.lex.tex".as_bytes(), "Refused NotNfc"),
        ("src/e\u{301}.txt".as_bytes(), "Refused NotNfc"),
        ("/src/e\u{301}.lex.tex".as_bytes(), "Refused NotNfc"),
        ("src/../e\u{301}.lex.tex".as_bytes(), "Refused NotNfc"),
        // The later stages still decide an NFC path.
        ("other/\u{e9}.lex.tex".as_bytes(), "Refused OutsideRoots"),
        ("src/\u{e9}.txt".as_bytes(), "Refused NotLexTex"),
        ("/src/\u{e9}.lex.tex".as_bytes(), "Refused AbsolutePath"),
        (
            "src/../\u{e9}.lex.tex".as_bytes(),
            "Refused DotDotComponent",
        ),
        ("src//\u{e9}.lex.tex".as_bytes(), "Refused EmptyComponent"),
        ("src/./\u{e9}.lex.tex".as_bytes(), "Refused DotComponent"),
        // A decomposed Hangul syllable and a composition exclusion are refused as well.
        ("src/\u{1100}\u{1161}.lex.tex".as_bytes(), "Refused NotNfc"),
        ("src/\u{958}.lex.tex".as_bytes(), "Refused NotNfc"),
        ("src/\u{ac00}.lex.tex".as_bytes(), "Admitted"),
    ];
    for (path, expected) in vectors {
        assert_eq!(
            judge(path),
            *expected,
            "{:?}",
            String::from_utf8_lossy(path)
        );
    }
}

/// `0x737263` as octets.
fn hex_octets(token: &str) -> Vec<u8> {
    let digits = token.strip_prefix("0x").expect("a hexadecimal literal");
    (0..digits.len() / 2)
        .map(|i| u8::from_str_radix(&digits[2 * i..2 * i + 2], 16).expect("hex digits"))
        .collect()
}

/// An admission instance: the roots, the path and the stated verdict.
struct Instance {
    name: String,
    on_sample: bool,
    roots: Vec<Vec<u8>>,
    path: Vec<u8>,
    verdict: String,
}

fn parse_instance(name: &str, statement: &str) -> Option<Instance> {
    let (on_sample, mut rest) = if let Some(rest) = statement.strip_prefix("((admitPath (") {
        (false, rest)
    } else if let Some(rest) = statement.strip_prefix("((admitPathWith (NfcTables::sample) (") {
        (true, rest)
    } else {
        return None;
    };
    // `rest` starts after the "(" that opens the roots list: `cons 0x.. (cons .. (nil Bytes))` or `nil Bytes)`.
    let mut roots = Vec::new();
    loop {
        if let Some(after) = rest.strip_prefix("nil Bytes)") {
            rest = after;
            break;
        }
        let after = rest.strip_prefix("cons ")?;
        let (token, tail) = after.split_once(' ')?;
        roots.push(hex_octets(token));
        rest = tail.strip_prefix('(')?;
    }
    // Every `(cons ... (nil Bytes))` closes with one ")" per root after the first.
    let rest = rest.trim_start_matches(')').trim_start();
    let (token, tail) = rest.split_once(')')?;
    let path = hex_octets(token.trim());
    let verdict = tail.trim().strip_prefix("= ")?.trim_end_matches(',');
    let verdict = verdict.strip_suffix(')').unwrap_or(verdict);
    let verdict = if verdict.contains("Verdict.Admitted") {
        "Admitted".to_owned()
    } else {
        let refusal = verdict
            .split("Refusal.")
            .nth(1)?
            .trim_end_matches(')')
            .trim()
            .to_owned();
        format!("Refused {refusal}")
    };
    Some(Instance {
        name: name.to_owned(),
        on_sample,
        roots,
        path,
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

#[test]
fn every_admission_instance_holds_through_the_exported_root_nf_12() {
    let all = instances();
    assert!(
        all.len() >= 20,
        "the module states at least twenty path instances, found {}",
        all.len()
    );
    assert!(all.iter().any(|i| i.on_sample), "instances on the sample");
    assert!(
        all.iter().any(|i| !i.on_sample),
        "instances on the full tables"
    );
    let mut failures = Vec::new();
    for instance in &all {
        let roots: Vec<&[u8]> = instance.roots.iter().map(Vec::as_slice).collect();
        // An instance on the sample holds on both images; one on the full
        // tables is judged on the full tables.
        let images: &[Image] = if instance.on_sample {
            &[Image::Sample, Image::Full]
        } else {
            &[Image::Full]
        };
        for &image in images {
            let got = verdict_text(&admit_on(image, &roots, &instance.path));
            if got != instance.verdict {
                failures.push(format!(
                    "{} on {image:?}: the module states {}, the exported root returns {got}",
                    instance.name, instance.verdict
                ));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn the_instances_the_issue_names_are_among_those_run_nf_12() {
    let all = instances();
    let named: Vec<&str> = all.iter().map(|i| i.name.as_str()).collect();
    for name in [
        "refusesDecomposedLetter",
        "admitsPrecomposedLetter",
        "notNfcIsRefusedBeforeTheRootCheck",
        "notNfcIsRefusedBeforeTheSuffixCheck",
        "notUtf8IsRefusedBeforeNfc",
    ] {
        assert!(named.contains(&name), "{name} is not parsed: {named:?}");
    }
}
