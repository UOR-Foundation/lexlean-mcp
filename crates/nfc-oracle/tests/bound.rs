//! NF-10: the bounds the decision uses to skip table lookups are the ones the
//! pinned data implies, and the decision is right below them.

mod common;

use common::{data, root, text};
use nfc_oracle::{is_nfc, nfc};

/// The constant a definition of `src/NfcTables.lex.tex` gives, read from the text.
fn constant(name: &str) -> u32 {
    let source = std::fs::read_to_string(root().join("src/NfcTables.lex.tex")).expect("reads");
    let line = source
        .lines()
        .find(|l| {
            l.starts_with(&format!(
                "definition{{name: {name}, parameters: [], result: Nat, body: "
            ))
        })
        .unwrap_or_else(|| panic!("no constant {name}"));
    line.rsplit("body: ")
        .next()
        .and_then(|rest| rest.split(',').next())
        .and_then(|n| n.trim().parse().ok())
        .unwrap_or_else(|| panic!("cannot read {name}"))
}

#[test]
fn the_fast_bound_is_the_least_code_point_that_is_not_trivially_normalized_nf_10() {
    let data = data();
    // Computed here from the parsed files, not through the generator's own function.
    let mut least = u32::MAX;
    for (&code, &class) in &data.combining_class {
        if class != 0 {
            least = least.min(code);
        }
    }
    for &(first, _) in &data.full_composition_exclusion {
        least = least.min(first);
    }
    for parts in data.decomposition.values() {
        if let [_, second] = parts[..] {
            least = least.min(second);
        }
    }
    // The Hangul vowels and trailing consonants combine with what precedes them.
    least = least.min(0x1161);
    assert_eq!(least, 0x300, "the bound for Unicode 17.0.0");
    assert_eq!(
        constant("fastBound"),
        least,
        "src/NfcTables.lex.tex records another bound"
    );
    let decomposes = data
        .decomposition
        .keys()
        .copied()
        .min()
        .expect("some code point decomposes");
    assert_eq!(decomposes, 0xC0);
    assert_eq!(constant("decomposesFrom"), decomposes);
    assert_eq!(
        constant("decompositionDepth") as usize,
        data.decomposition_depth()
    );
}

#[test]
fn every_string_of_two_code_points_below_the_bound_is_nfc_nf_10() {
    let bound = constant("fastBound");
    let mut wrong = Vec::new();
    for first in 0..bound {
        for second in 0..bound {
            let pair = [first, second];
            // The decision, and the normalization itself (which computes rather than
            // takes the shortcut the decision has for strings below the bound).
            let ours = is_nfc(&pair) && nfc(&pair) == pair;
            let theirs = unicode_normalization::is_nfc(&text(&pair));
            if !ours || !theirs {
                wrong.push(format!(
                    "U+{first:04X} U+{second:04X}: ours {ours}, crate {theirs}"
                ));
            }
        }
    }
    assert!(
        wrong.is_empty(),
        "{} pairs: {:?}",
        wrong.len(),
        &wrong[..wrong.len().min(20)]
    );
}
