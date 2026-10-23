//! DL-02: no stage of draft admission recurses to a depth bounded by the number of
//! rows.
//!
//! The root `admitDraftRun` is exported with the effects `allocation` and
//! `overflow` and without `recursion`: every self call of the stages that run over
//! the rows is a tail call, which the export renders as a loop (SPEC section 17.13,
//! RB-14). The runs below give the root a draft of one million rows on a two MiB
//! thread stack, which a recursion once per row would overflow, and every run
//! returns a typed verdict. The rows, the list the root reads and the verdict are
//! built and dropped on that stack as well.

mod common;

use common::{on_stack, on_stack_within, verdict_text};
use nfc_oracle::admit_draft;

const STACK: usize = 2 * 1024 * 1024;
const MILLION: usize = 1_000_000;
const ROOTS: [&[u8]; 1] = [b"src"];

fn path(i: usize) -> Vec<u8> {
    format!("src/d{i}/f{i}.lex.tex").into_bytes()
}

/// `count` rows with distinct paths, each holding `size` octets.
fn rows(count: usize, size: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    (0..count).map(|i| (path(i), vec![b'x'; size])).collect()
}

fn judge(max_files: u64, max_bytes: u64, rows: Vec<(Vec<u8>, Vec<u8>)>) -> String {
    on_stack(STACK, move || {
        verdict_text(&admit_draft(max_files, max_bytes, &ROOTS, &rows))
    })
}

#[test]
fn a_draft_of_a_million_rows_is_refused_as_too_many_files_on_a_two_mib_stack_dl_02() {
    // Every one of the million rows is admitted by the row stage first, then the
    // row count decides.
    assert_eq!(
        judge(1000, 1 << 40, rows(MILLION, 0)),
        "Refused TooManyFiles"
    );
}

#[test]
fn a_draft_of_a_million_rows_is_refused_as_too_many_bytes_on_a_two_mib_stack_dl_02() {
    // The row stage, the row count and the byte total each run over a million rows;
    // the byte total, one octet per row, is one million against ten.
    assert_eq!(
        judge(MILLION as u64, 10, rows(MILLION, 1)),
        "Refused TooManyBytes"
    );
}

#[test]
fn a_draft_whose_rows_and_bytes_equal_the_limits_is_admitted_dl_02() {
    // 2000 rows of 500 octets: exactly maxFiles rows and exactly maxBytes octets.
    let at = || rows(2000, 500);
    assert_eq!(judge(2000, 1_000_000, at()), "Admitted");
    // One row fewer allowed, or one octet fewer allowed.
    assert_eq!(judge(1999, 1_000_000, at()), "Refused TooManyFiles");
    assert_eq!(judge(2000, 999_999, at()), "Refused TooManyBytes");
}

#[test]
fn a_draft_of_large_contents_is_counted_by_its_octets_dl_02() {
    // Two rows of two MiB: the byte total is the sum of the lengths, however long.
    let size = 2 * 1024 * 1024;
    assert_eq!(judge(2, 2 * size as u64, rows(2, size)), "Admitted");
    assert_eq!(
        judge(2, 2 * size as u64 - 1, rows(2, size)),
        "Refused TooManyBytes"
    );
}

#[test]
fn a_draft_with_a_duplicate_path_is_refused_as_duplicate_path_dl_02() {
    // Within both limits, the last row repeats the first.
    let mut draft = rows(2000, 500);
    draft[1999].0 = path(0);
    assert_eq!(judge(2000, 1_000_000, draft), "Refused DuplicatePath");
    // The same repeat is not looked for in a draft the limits already refuse.
    let mut draft = rows(2000, 500);
    draft[1999].0 = path(0);
    assert_eq!(judge(1999, 1_000_000, draft), "Refused TooManyFiles");
    // Two rows, the smallest draft with a repeat.
    let twice = vec![(path(7), Vec::new()), (path(7), Vec::new())];
    assert_eq!(judge(2, 0, twice), "Refused DuplicatePath");
}

/// How long the duplicate scan of a million rows may take. Sorting a million paths and
/// comparing neighbours takes a fraction of this; a scan that compares every row with
/// every other would make about 5 * 10^11 comparisons and does not finish. The bound
/// separates the two; it is not a measure of speed.
const SCAN_LIMIT: std::time::Duration = std::time::Duration::from_secs(120);

/// `count` rows with distinct paths `src/a0000000.lex.tex` and so on, in an order
/// that is no sorted order: row `i` holds path number `i * 7919 mod count`.
fn shuffled_rows(count: usize) -> Vec<(Vec<u8>, Vec<u8>)> {
    assert_ne!(
        count % 7919,
        0,
        "7919 is prime, so the map is a permutation"
    );
    (0..count)
        .map(|i| {
            let number = (i * 7919) % count;
            (format!("src/a{number:07}.lex.tex").into_bytes(), Vec::new())
        })
        .collect()
}

fn judge_within(max_files: u64, rows: Vec<(Vec<u8>, Vec<u8>)>) -> (String, std::time::Duration) {
    let (verdict, took) = on_stack_within(STACK, SCAN_LIMIT, move || {
        verdict_text(&admit_draft(max_files, 1 << 40, &ROOTS, &rows))
    });
    (verdict, took)
}

#[test]
fn a_draft_of_a_million_distinct_paths_is_admitted_within_the_bound_dl_04() {
    let (verdict, took) = judge_within(MILLION as u64, shuffled_rows(MILLION));
    println!("admitted {MILLION} rows in {took:?}");
    assert_eq!(verdict, "Admitted");
}

#[test]
fn the_same_draft_with_one_repeated_path_is_refused_within_the_bound_dl_04() {
    let mut rows = shuffled_rows(MILLION);
    rows[600_000].0 = rows[0].0.clone();
    let (verdict, took) = judge_within(MILLION as u64, rows);
    println!("refused {MILLION} rows in {took:?}");
    assert_eq!(verdict, "Refused DuplicatePath");
}

#[test]
fn a_repeat_of_the_neighbour_in_sorted_order_is_found_dl_04() {
    // The repeated pair is the first and the last row, which no scan of adjacent rows
    // in the order given would see.
    let mut rows = shuffled_rows(50_000);
    let last = rows.len() - 1;
    rows[last].0 = rows[0].0.clone();
    let (verdict, _) = judge_within(50_000, rows);
    assert_eq!(verdict, "Refused DuplicatePath");
}
