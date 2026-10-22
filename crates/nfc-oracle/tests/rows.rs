//! NF-03, NF-04: the LexLean decision and normalizations against every line of
//! the pinned `NormalizationTest.txt`.
//!
//! The expectations are Unicode's, read from the file; nothing here normalizes.

mod common;

use common::{rows, show, Memo};
use nfc_oracle::{is_nfc, nfc, nfd};

#[test]
fn the_file_has_every_line_and_the_parts_the_parser_reads_nf_03() {
    let text = std::fs::read_to_string(nfc_tables::ucd_dir().join("NormalizationTest.txt"))
        .expect("the file reads");
    let lines = text
        .lines()
        .filter(|l| !l.starts_with('#') && !l.starts_with('@') && !l.trim().is_empty())
        .count();
    let parsed = rows();
    assert_eq!(parsed.len(), lines, "the parser reads every line");
    assert_eq!(parsed.len(), 20034, "the pinned file has 20034 lines");
    let parts: Vec<usize> = (0..=3)
        .map(|p| parsed.iter().filter(|r| r.part == p).count())
        .collect();
    assert!(parts.iter().all(|&n| n > 0), "four parts: {parts:?}");
}

#[test]
fn c2_and_c4_are_nfc_and_c1_c3_and_c5_are_nfc_exactly_when_they_equal_their_normal_form_nf_03() {
    // The file's own statement of the NFC invariants (its header, lines 22 to 24):
    //   c2 == toNFC(c1) == toNFC(c2) == toNFC(c3)
    //   c4 == toNFC(c4) == toNFC(c5)
    // So c2 and c4 are their own NFC, c1 and c3 are NFC exactly when they equal
    // c2, and c5 is NFC exactly when it equals c4. (Issue #25 words this as "c1,
    // c3, c4 and c5 are NFC exactly when they equal c2". That holds for c1 and
    // c3 only: c4 and c5 are the compatibility forms, and for U+00A0 (c2) the
    // c4 column U+0020 is NFC though it is not c2. 7576 of the 100170
    // judgements would then contradict Unicode's own file.)
    let mut memo = Memo::new();
    let mut failures = Vec::new();
    let mut judged = 0;
    for row in rows() {
        let normal_form = |index: usize| match index {
            0 | 2 => &row.columns[1],
            4 => &row.columns[3],
            _ => &row.columns[index],
        };
        for (index, column) in row.columns.iter().enumerate() {
            let expected = column == normal_form(index);
            let got = memo.get(column, is_nfc);
            judged += 1;
            if got != expected {
                failures.push(format!(
                    "line {} c{}: {} is {} NFC, expected {}",
                    row.line,
                    index + 1,
                    show(column),
                    if got { "" } else { "not" },
                    if expected { "NFC" } else { "not NFC" }
                ));
            }
        }
    }
    assert_eq!(judged, 5 * 20034, "five columns of every line");
    assert!(
        failures.is_empty(),
        "{} of {judged} judgements disagree:\n{}",
        failures.len(),
        failures
            .iter()
            .take(40)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn the_nfc_and_nfd_invariants_of_the_file_hold_on_every_line_nf_04() {
    let mut to_nfc = Memo::new();
    let mut to_nfd = Memo::new();
    let mut failures = Vec::new();
    for row in rows() {
        let [c1, c2, c3, c4, c5] = &row.columns;
        let mut expect = |what: &str, got: Vec<u32>, want: &Vec<u32>| {
            if &got != want {
                failures.push(format!(
                    "line {}: {what} is {}, expected {}",
                    row.line,
                    show(&got),
                    show(want)
                ));
            }
        };
        // NFC: c2 == toNFC(c1) == toNFC(c2) == toNFC(c3); c4 == toNFC(c4) == toNFC(c5).
        for (name, column, want) in [
            ("toNFC(c1)", c1, c2),
            ("toNFC(c2)", c2, c2),
            ("toNFC(c3)", c3, c2),
            ("toNFC(c4)", c4, c4),
            ("toNFC(c5)", c5, c4),
        ] {
            expect(name, to_nfc.get(column, nfc), want);
        }
        // NFD: c3 == toNFD(c1) == toNFD(c2) == toNFD(c3); c5 == toNFD(c4) == toNFD(c5).
        for (name, column, want) in [
            ("toNFD(c1)", c1, c3),
            ("toNFD(c2)", c2, c3),
            ("toNFD(c3)", c3, c3),
            ("toNFD(c4)", c4, c5),
            ("toNFD(c5)", c5, c5),
        ] {
            expect(name, to_nfd.get(column, nfd), want);
        }
    }
    assert!(
        failures.is_empty(),
        "{} invariants fail:\n{}",
        failures.len(),
        failures
            .iter()
            .take(40)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn a_string_that_is_not_nfc_is_judged_not_nfc_nf_03() {
    // The negative side of the decision, by name: e followed by U+0301 is not NFC
    // and U+00E9 is; U+0958 stays decomposed under the composition exclusions.
    assert!(!is_nfc(&[0x65, 0x301]));
    assert!(is_nfc(&[0xE9]));
    assert!(!is_nfc(&[0x958]));
    assert!(is_nfc(&[0x915, 0x93C]));
    assert!(!is_nfc(&[0x1100, 0x1161, 0x11A8]));
    assert!(is_nfc(&[0xAC01]));
    assert!(!is_nfc(&[0x44, 0x307, 0x323]));
}
