//! NF-09: path admission refuses a path that is not NFC with the typed refusal
//! `NotNfc`, after UTF-8 validation and before the root and suffix checks.
//!
//! The decision and the order are LexLean source, checked by `lexlean verify`
//! (the kernel-checked instances of `src/Admission.lex.tex`). This test is the
//! gate that the source keeps the shape the register row states, so a rewrite
//! that moved the stage or dropped an instance fails here as well as there.

mod common;

use common::root;

fn admission() -> String {
    std::fs::read_to_string(root().join("src/Admission.lex.tex"))
        .expect("src/Admission.lex.tex reads")
}

/// The text of the `kind{name: ..., ...}` block of `name`.
fn block<'a>(source: &'a str, kind: &str, name: &str) -> &'a str {
    let head = format!("{kind}{{\n  name: {name},");
    let start = source
        .find(&head)
        .unwrap_or_else(|| panic!("no {kind} {name}"));
    let end = source[start..].find("\n}\n").expect("a block ends") + start;
    &source[start..end]
}

#[test]
fn not_nfc_is_a_refusal_constructor_after_not_utf8_nf_09() {
    let source = admission();
    let refusal = block(&source, "inductive", "Refusal");
    let utf8 = refusal
        .find("{name: NotUtf8, fields: []}")
        .expect("NotUtf8");
    let nfc = refusal
        .find("{name: NotNfc, fields: []}")
        .expect("NotNfc is a typed refusal");
    assert!(utf8 < nfc);
}

#[test]
fn the_nfc_stage_follows_the_utf8_stage_and_precedes_the_root_and_suffix_stages_nf_09() {
    let source = admission();
    let admit = block(&source, "definition", "admitPathWith");
    let utf8 = admit.find("utf8Stage path").expect("the UTF-8 stage");
    let nfc = admit.find("nfcStage tables path").expect("the NFC stage");
    let octets = admit.find("admitOctets").expect("the remaining stages");
    assert!(
        utf8 < nfc && nfc < octets,
        "UTF-8, then NFC, then the rest: {admit}"
    );
    // The root and suffix checks are inside admitOctets, which the NFC stage precedes.
    let rest = block(&source, "definition", "admitOctets");
    assert!(rest.contains("rootsStage") && rest.contains("suffixStage"));
    assert!(
        !rest.contains("nfcStage"),
        "the NFC stage is not among the later stages"
    );
    let stage = block(&source, "definition", "nfcStage");
    assert!(
        stage.contains("Refusal.NotNfc") && stage.contains("Nfc::isNfc"),
        "{stage}"
    );
    assert!(
        block(&source, "definition", "admitPath").contains("admitPathWith (NfcTables::tables)"),
        "admission reads the full tables"
    );
}

#[test]
fn the_instances_the_issue_names_are_kernel_checked_nf_09() {
    let source = admission();
    // e followed by U+0301 is refused as NotNfc (on the sample image, which the
    // elaborator can evaluate); U+00E9 is admitted on the full tables.
    let refused = block(&source, "theorem", "refusesDecomposedLetter");
    assert!(
        refused.contains("65cc81") && refused.contains("Refusal.NotNfc"),
        "{refused}"
    );
    let admitted = block(&source, "theorem", "admitsPrecomposedLetter");
    assert!(
        admitted.contains("c3a9") && admitted.contains("Verdict.Admitted"),
        "{admitted}"
    );
    assert!(admitted.contains("admitPath ("), "on the full tables");
    // The order: NFC before the root check and before the suffix check, UTF-8 before NFC.
    for (name, refusal) in [
        ("notNfcIsRefusedBeforeTheRootCheck", "Refusal.NotNfc"),
        ("notNfcIsRefusedBeforeTheSuffixCheck", "Refusal.NotNfc"),
        ("notUtf8IsRefusedBeforeNfc", "Refusal.NotUtf8"),
    ] {
        let theorem = block(&source, "theorem", name);
        assert!(theorem.contains(refusal), "{name}: {theorem}");
    }
    let link = block(&source, "theorem", "admitPathIsAdmitPathWithTheTables");
    assert!(link.contains("NfcTables::tables"), "{link}");
}
