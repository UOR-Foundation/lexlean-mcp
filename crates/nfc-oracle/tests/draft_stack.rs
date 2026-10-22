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

use common::{on_stack, verdict_text};
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
