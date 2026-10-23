//! NF-14 and NF-15: canonical ordering of the NFC decision.
//!
//! Canonical ordering sorts each run of non-starters by combining class and keeps
//! marks of equal class in their order (UAX #15, section 3.11: only adjacent marks
//! whose classes are strictly out of order are exchanged). The decision does it with
//! a stable merge sort, so a run of n marks costs O(n log n). An insertion ordering
//! exchanges each mark with every earlier mark of a greater class and costs O(n^2)
//! in a run of marks of descending class: for 10^6 marks that is about 10^11 steps,
//! which does not finish.
//!
//! The roots below are the ones LexLean exports for `nfcRun` and `admitRun`; the
//! reference is `unicode-normalization` 0.1.25 (whose own canonical sort is a stable
//! sort by class) and, for the NFD, a stable sort by class written here.

mod common;

use std::time::Duration;

use common::{on_stack, on_stack_within, root, verdict_text};
use nfc_oracle::{admit, is_nfc, nfc, nfd};
use unicode_normalization::char::canonical_combining_class;
use unicode_normalization::UnicodeNormalization;

const STACK: usize = 2 * 1024 * 1024;
const MILLION: usize = 1_000_000;

/// How long one decision on a run of marks may take. A merge sort of 10^6 marks takes
/// a small fraction of this. An insertion ordering of the descending run makes about
/// 10^11 exchanges and does not finish. The bound separates the two; it is not a
/// measure of speed.
const ORDER_LIMIT: Duration = Duration::from_secs(60);

/// Every combining class that a mark which has no decomposition can have, with the
/// marks of that class in scalar order.
fn marks_by_class() -> Vec<(u8, Vec<u32>)> {
    let mut by_class: Vec<Vec<u32>> = vec![Vec::new(); 256];
    for code in 0..=0x10FFFFu32 {
        let Some(c) = char::from_u32(code) else {
            continue;
        };
        let class = canonical_combining_class(c);
        if class == 0 {
            continue;
        }
        // A mark that is its own canonical decomposition.
        if std::iter::once(c).nfd().eq(std::iter::once(c)) {
            by_class[usize::from(class)].push(code);
        }
    }
    by_class
        .into_iter()
        .enumerate()
        .filter(|(_, marks)| !marks.is_empty())
        .map(|(class, marks)| (u8::try_from(class).expect("a class is a byte"), marks))
        .collect()
}

fn class_of(code: u32) -> u8 {
    canonical_combining_class(char::from_u32(code).expect("a scalar"))
}

/// A starter and `count` marks, one representative per class, the classes in
/// descending order and cycling: 230, 228, ..., 1, 230, 228, ...
fn descending(count: usize) -> Vec<u32> {
    let mut classes = marks_by_class();
    classes.reverse();
    let mut out = vec![0x61];
    out.extend((0..count).map(|i| classes[i % classes.len()].1[0]));
    out
}

/// A starter and `count` marks in ascending class order: the least class first, each
/// class in one block, so the run is already in canonical order.
fn ascending(count: usize) -> Vec<u32> {
    let classes = marks_by_class();
    let mut out = vec![0x61];
    out.extend((0..count).map(|i| classes[i * classes.len() / count].1[0]));
    out
}

/// A starter and `count` marks of class 230, drawn in turn from 64 distinct marks, so
/// that the marks are told apart by their scalar and not by their class.
fn one_class(count: usize) -> Vec<u32> {
    let classes = marks_by_class();
    let (_, marks) = classes
        .iter()
        .find(|(class, _)| *class == 230)
        .expect("class 230 has marks");
    assert!(marks.len() >= 64);
    let mut out = vec![0x61];
    out.extend((0..count).map(|i| marks[i % 64]));
    out
}

/// The stable sort by combining class of each run of non-starters.
fn stable_order(scalars: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(scalars.len());
    let mut run: Vec<u32> = Vec::new();
    for &code in scalars {
        if class_of(code) == 0 {
            run.sort_by_key(|&c| class_of(c));
            out.append(&mut run);
            out.push(code);
        } else {
            run.push(code);
        }
    }
    run.sort_by_key(|&c| class_of(c));
    out.append(&mut run);
    out
}

fn reference_nfd(scalars: &[u32]) -> Vec<u32> {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).expect("a scalar"))
        .nfd()
        .map(u32::from)
        .collect()
}

fn reference_nfc(scalars: &[u32]) -> Vec<u32> {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).expect("a scalar"))
        .nfc()
        .map(u32::from)
        .collect()
}

/// Run `f` on a two MiB stack within the bound; the time it took is printed.
fn bounded<T: Send + 'static>(what: &str, f: impl FnOnce() -> T + Send + 'static) -> T {
    let (value, took) = on_stack_within(STACK, ORDER_LIMIT, f);
    println!("{what}: {took:?}");
    value
}

/// Decide a starter and its marks four ways, each within the bound, against the
/// reference. When `keeps_order`, the run is one the canonical ordering leaves as it is
/// (marks of one class, or marks already in ascending class order), so the NFD is the
/// input, mark for mark.
fn decide(shape: &str, scalars: Vec<u32>, keeps_order: bool) {
    let size = scalars.len() - 1;
    let expected_nfd = reference_nfd(&scalars);
    let expected_nfc = reference_nfc(&scalars);
    assert!(
        expected_nfd == stable_order(&scalars),
        "{shape}: the reference NFD is the stable sort by class"
    );
    let got = bounded(&format!("{shape} {size} marks, NFD"), {
        let scalars = scalars.clone();
        move || nfd(&scalars)
    });
    assert!(got == expected_nfd, "{shape} {size}: the NFD");
    assert_eq!(
        got == scalars,
        keeps_order,
        "{shape} {size}: the NFD is the input"
    );
    let got = bounded(&format!("{shape} {size} marks, NFC"), {
        let scalars = scalars.clone();
        move || nfc(&scalars)
    });
    assert!(got == expected_nfc, "{shape} {size}: the NFC");
    let verdict = bounded(&format!("{shape} {size} marks, is NFC"), {
        let scalars = scalars.clone();
        move || is_nfc(&scalars)
    });
    assert_eq!(
        verdict,
        expected_nfc == scalars,
        "{shape} {size}: the verdict"
    );
    // The same run in a path, between the root and the suffix.
    let text: String = format!(
        "src/{}.lex.tex",
        scalars
            .iter()
            .map(|&c| char::from_u32(c).expect("a scalar"))
            .collect::<String>()
    );
    let expected_path = if text.nfc().eq(text.chars()) {
        "Admitted"
    } else {
        "Refused NotNfc"
    };
    let path = text.into_bytes();
    let got = bounded(&format!("{shape} {size} marks, path"), move || {
        verdict_text(&admit(&[b"src"], &path))
    });
    assert_eq!(got, expected_path, "{shape} {size}: the path verdict");
}

#[test]
fn a_run_of_descending_marks_is_ordered_within_the_bound_nf_14() {
    for size in [10_000, 100_000, MILLION] {
        decide("descending", descending(size), false);
    }
}

#[test]
fn a_run_of_marks_in_ascending_order_is_ordered_within_the_bound_nf_14() {
    for size in [10_000, 100_000, MILLION] {
        decide("ascending", ascending(size), true);
    }
}

#[test]
fn a_run_of_marks_of_one_class_keeps_the_order_of_its_marks_nf_14() {
    for size in [10_000, 100_000, MILLION] {
        decide("one class", one_class(size), true);
    }
}

#[test]
fn the_descending_run_is_out_of_order_and_is_ordered_by_class_nf_14() {
    // The run is not already in order, and it is in order after the sort: the shape
    // exercised above is the one that separates the orderings, not a sorted run.
    let scalars = descending(1000);
    let ordered = stable_order(&scalars);
    assert!(ordered != scalars);
    let classes: Vec<u8> = ordered[1..].iter().map(|&c| class_of(c)).collect();
    assert!(classes.windows(2).all(|w| w[0] <= w[1]));
    assert_eq!(on_stack(STACK, move || nfd(&scalars)), ordered);
}

/// A small generator, so the strings are the same on every run.
struct Xorshift(u64);

impl Xorshift {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).expect("small")).expect("small")
    }
}

#[test]
fn strings_of_few_classes_agree_with_the_reference_nf_15() {
    // Six classes, several marks in each, so that most pairs of marks are of equal
    // class; four starters, one of which composes with some of the marks.
    let all = marks_by_class();
    let classes: Vec<(u8, Vec<u32>)> = [1u8, 202, 220, 230, 232, 240]
        .iter()
        .map(|&wanted| {
            all.iter()
                .find(|(class, _)| *class == wanted)
                .map(|(class, marks)| (*class, marks.iter().copied().take(4).collect()))
                .expect("the class has marks")
        })
        .collect();
    let starters = [0x61u32, 0x65, 0x6F, 0x1100];
    let mut random = Xorshift(0x9E37_79B9_7F4A_7C15);
    for round in 0..20_000 {
        let length = random.below(25);
        let mut scalars = Vec::with_capacity(length);
        for _ in 0..length {
            if random.below(6) == 0 {
                scalars.push(starters[random.below(starters.len())]);
            } else {
                let (_, marks) = &classes[random.below(classes.len())];
                scalars.push(marks[random.below(marks.len())]);
            }
        }
        let expected = reference_nfd(&scalars);
        assert_eq!(expected, stable_order(&scalars), "round {round}: reference");
        assert_eq!(
            nfd(&scalars),
            expected,
            "round {round}: the NFD of {scalars:X?}"
        );
        assert_eq!(
            nfc(&scalars),
            reference_nfc(&scalars),
            "round {round}: the NFC of {scalars:X?}"
        );
    }
}

#[test]
fn the_equal_class_instances_are_stated_nf_15() {
    let source =
        std::fs::read_to_string(root().join("src/Nfc.lex.tex")).expect("src/Nfc.lex.tex reads");
    for wanted in [
        "marksOfEqualClassKeepTheirOrder",
        "marksOfEqualClassKeepTheirOrderWhenReversed",
        "anOutOfOrderMarkPassesEqualClassMarks",
        "aRunOfMarksIsSortedByClassKeepingEqualClassOrder",
        "eachRunOfMarksIsOrderedOnItsOwn",
        "leadingMarksAreOrdered",
        "equalClassMarksAreNfc",
        "equalClassMarksAreNfcWhenReversed",
    ] {
        assert!(
            source.contains(&format!("name: {wanted},")),
            "no instance named {wanted}"
        );
    }
}
