//! Runs the packages LexLean exports for the NFC decision and for path admission
//! (issue #25).
//!
//! Dev and CI only. The decision is LexLean source in `src/Nfc.lex.tex`; the
//! first package is LexLean's own export of the root `LexLeanMcp.Nfc.nfcRun`, the
//! second its export of `LexLeanMcp.Admission.admitRun`. This file only converts
//! between scalar values, paths and the packages' octets and `u64` lists. It
//! normalizes and admits nothing itself.

// The package carries its own lint table (`warnings = deny`, `clippy::all = deny`
// with ten named exceptions) and is clean under it as a crate. Included here as a
// module it inherits this workspace's table instead, so the same ten exceptions are
// named, plus the two the include itself causes: items nothing here calls, and no
// documentation on generated items.
#[allow(
    dead_code,
    unused,
    missing_docs,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::result_unit_err,
    clippy::single_match,
    clippy::manual_unwrap_or,
    clippy::manual_unwrap_or_default,
    clippy::manual_map,
    clippy::manual_ok_err
)]
#[rustfmt::skip]
#[path = "../generated/lexlean_mcp_nfc/src/lib.rs"]
mod package;

// The admission package: the same lint exceptions, for the same reasons.
#[allow(
    dead_code,
    unused,
    missing_docs,
    clippy::type_complexity,
    clippy::too_many_arguments,
    clippy::large_enum_variant,
    clippy::result_large_err,
    clippy::result_unit_err,
    clippy::single_match,
    clippy::manual_unwrap_or,
    clippy::manual_unwrap_or_default,
    clippy::manual_map,
    clippy::manual_ok_err
)]
#[rustfmt::skip]
#[path = "../generated/lexlean_mcp_admission/src/lib.rs"]
mod admission_package;

/// The text of the generated admission package this crate compiled.
pub const ADMISSION_SOURCE: &str = include_str!("../generated/lexlean_mcp_admission/src/lib.rs");

/// The text of the generated package this crate compiled, so a test can hash the
/// file the oracle ran against the digest the export recorded.
pub const GENERATED_SOURCE: &str = include_str!("../generated/lexlean_mcp_nfc/src/lib.rs");

/// The scalar values as the standard library's UTF-8 encoding of them.
///
/// # Panics
/// When a value is not a Unicode scalar value.
pub fn utf8(scalars: &[u32]) -> Vec<u8> {
    scalars
        .iter()
        .map(|&c| char::from_u32(c).unwrap_or_else(|| panic!("U+{c:04X} is not a scalar value")))
        .collect::<String>()
        .into_bytes()
}

/// The tables the exported decision reads: the full tables the admission
/// module carries, or the sample image the kernel-checked instances use.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Image {
    /// `artifacts/nfc-tables-low.bin` and `artifacts/nfc-tables-high.bin`.
    Full,
    /// `artifacts/nfc-sample.bin` (read as both halves, as the module does).
    Sample,
}

const LOW: &[u8] = include_bytes!("../../../artifacts/nfc-tables-low.bin");
const HIGH: &[u8] = include_bytes!("../../../artifacts/nfc-tables-high.bin");
const SAMPLE: &[u8] = include_bytes!("../../../artifacts/nfc-sample.bin");

thread_local! {
    static FULL: (package::Bytes, package::Bytes) =
        (package::Bytes::lit(LOW), package::Bytes::lit(HIGH));
    static SMALL: (package::Bytes, package::Bytes) =
        (package::Bytes::lit(SAMPLE), package::Bytes::lit(SAMPLE));
}

fn run_on(image: Image, mode: u64, input: &[u8]) -> Vec<u64> {
    let (low, high) = match image {
        Image::Full => FULL.with(Clone::clone),
        Image::Sample => SMALL.with(Clone::clone),
    };
    let mut list = package::run(mode, low, high, package::Bytes::lit(input))
        .expect("the NFC root does not overflow on this input");
    let mut out = Vec::new();
    while let Some((head, tail)) = list.uncons() {
        out.push(head);
        list = tail;
    }
    out
}

fn run(mode: u64, input: &[u8]) -> Vec<u64> {
    run_on(Image::Full, mode, input)
}

fn scalars_of(words: Vec<u64>) -> Vec<u32> {
    words
        .into_iter()
        .map(|w| u32::try_from(w).expect("the root reported a value above 2^32"))
        .collect()
}

/// The LexLean decision: is the string, given as UTF-8, in NFC?
pub fn is_nfc_octets(octets: &[u8]) -> bool {
    match run(0, octets).as_slice() {
        [1] => true,
        [0] => false,
        other => panic!("unexpected verdict {other:?}"),
    }
}

/// The LexLean decision for a scalar sequence.
pub fn is_nfc(scalars: &[u32]) -> bool {
    is_nfc_octets(&utf8(scalars))
}

/// The scalar values the LexLean reader finds in `octets`.
pub fn decode(octets: &[u8]) -> Vec<u32> {
    scalars_of(run(1, octets))
}

/// The LexLean canonical decomposition with canonical ordering (NFD).
pub fn nfd(scalars: &[u32]) -> Vec<u32> {
    scalars_of(run(2, &utf8(scalars)))
}

/// The LexLean canonical composition of the NFD (NFC).
pub fn nfc(scalars: &[u32]) -> Vec<u32> {
    scalars_of(run(3, &utf8(scalars)))
}

/// The LexLean decision on `image`, for octets.
pub fn is_nfc_octets_on(image: Image, octets: &[u8]) -> bool {
    match run_on(image, 0, octets).as_slice() {
        [1] => true,
        [0] => false,
        other => panic!("unexpected verdict {other:?}"),
    }
}

/// The LexLean NFC on `image`.
pub fn nfc_on(image: Image, scalars: &[u32]) -> Vec<u32> {
    scalars_of(run_on(image, 3, &utf8(scalars)))
}

/// The LexLean NFD on `image`.
pub fn nfd_on(image: Image, scalars: &[u32]) -> Vec<u32> {
    scalars_of(run_on(image, 2, &utf8(scalars)))
}

/// The scalar values the LexLean reader finds in `octets`, on `image`.
pub fn decode_on(image: Image, octets: &[u8]) -> Vec<u32> {
    scalars_of(run_on(image, 1, octets))
}

fn tables_of(image: Image) -> (&'static [u8], &'static [u8]) {
    match image {
        Image::Full => (LOW, HIGH),
        Image::Sample => (SAMPLE, SAMPLE),
    }
}

/// What the exported root `admitRun` returns for a path: `[0]` for admitted,
/// `[1, index]` for the refusal at `index` in the constructor order of
/// `Refusal`. The tests, not this crate, name the refusals.
///
/// # Panics
/// When the root reports a resource failure rather than a verdict.
pub fn admit_on(image: Image, roots: &[&[u8]], path: &[u8]) -> Vec<u64> {
    let (low, high) = tables_of(image);
    let mut list = admission_package::List::nil();
    for root in roots.iter().rev() {
        list = admission_package::List::cons(admission_package::Bytes::lit(root), list);
    }
    let mut verdict = admission_package::run(
        admission_package::Bytes::lit(low),
        admission_package::Bytes::lit(high),
        list,
        admission_package::Bytes::lit(path),
    )
    .expect("the admission root does not overflow on this input");
    let mut out = Vec::new();
    while let Some((head, tail)) = verdict.uncons() {
        out.push(head);
        verdict = tail;
    }
    out
}

/// The verdict of path admission on the full tables.
pub fn admit(roots: &[&[u8]], path: &[u8]) -> Vec<u64> {
    admit_on(Image::Full, roots, path)
}
