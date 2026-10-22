//! NF-05, NF-06, NF-08: the decision on code points the file does not list, the
//! reader on every scalar value, and the cross-check against the pinned crate.

mod common;

use std::collections::BTreeSet;

use common::{data, rows, text, Memo};
use nfc_oracle::{decode, is_nfc, nfc, nfd, utf8};

fn is_scalar(code: u32) -> bool {
    char::from_u32(code).is_some()
}

#[test]
fn every_assigned_code_the_file_does_not_list_is_its_own_normal_form_nf_05() {
    let data = data();
    let listed: BTreeSet<u32> = rows()
        .iter()
        .filter(|row| row.part == 1)
        .filter_map(|row| match row.columns[0][..] {
            [single] => Some(single),
            _ => None,
        })
        .collect();
    assert!(listed.len() > 10_000, "Part 1 lists a code point per line");
    let candidates: Vec<u32> = data
        .assigned
        .iter()
        .copied()
        // A surrogate is assigned in UnicodeData.txt and is not a scalar value,
        // so it cannot appear in a UTF-8 string.
        .filter(|&c| is_scalar(c) && !listed.contains(&c))
        .collect();
    assert!(
        candidates.len() > 100_000,
        "{} candidates",
        candidates.len()
    );
    let mut failures = Vec::new();
    for code in candidates {
        let one = [code];
        if !is_nfc(&one) || nfc(&one) != one || nfd(&one) != one {
            failures.push(format!("U+{code:04X}"));
        }
    }
    assert!(
        failures.is_empty(),
        "{} code points are not their own normal form: {:?}",
        failures.len(),
        &failures[..failures.len().min(40)]
    );
}

#[test]
fn the_reader_returns_every_scalar_value_from_its_utf8_encoding_nf_06() {
    let mut wrong = Vec::new();
    for code in (0..=0x10FFFF_u32).filter(|&c| is_scalar(c)) {
        let octets = utf8(&[code]);
        if decode(&octets) != [code] {
            wrong.push(format!("U+{code:04X}"));
        }
    }
    assert!(
        wrong.is_empty(),
        "misread: {:?}",
        &wrong[..wrong.len().min(40)]
    );
    // Several scalars in a row, with the widths mixed.
    let mixed = [
        0x41, 0xE9, 0x20AC, 0x1F600, 0x7F, 0x80, 0x7FF, 0x800, 0xFFFF, 0x10000, 0x10FFFF,
    ];
    assert_eq!(decode(&utf8(&mixed)), mixed);
    assert_eq!(decode(&[]), Vec::<u32>::new());
}

#[test]
fn the_decision_agrees_with_the_pinned_crate_on_every_line_and_every_scalar_nf_08() {
    let mut memo = Memo::new();
    let mut disagreements = Vec::new();
    for row in rows() {
        for column in &row.columns {
            let ours = memo.get(column, is_nfc);
            if ours != unicode_normalization::is_nfc(&text(column)) {
                disagreements.push(format!("line {}", row.line));
            }
        }
    }
    for code in (0..=0x10FFFF_u32).filter(|&c| is_scalar(c)) {
        let ours = is_nfc(&[code]);
        if ours != unicode_normalization::is_nfc(&text(&[code])) {
            disagreements.push(format!("U+{code:04X}"));
        }
    }
    assert!(
        disagreements.is_empty(),
        "the decision and the crate disagree on {}: {:?}",
        disagreements.len(),
        &disagreements[..disagreements.len().min(40)]
    );
}
