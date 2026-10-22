//! The tables of the LexLean NFC decision, generated from the pinned Unicode
//! Character Database 17.0.0 files.
//!
//! Four files come out. The tables are one byte string holding five sorted tables
//! of fixed-width records, written as two halves, `artifacts/nfc-tables-low.bin` and
//! `nfc-tables-high.bin`, plus `artifacts/nfc-sample.bin`, an image of the same
//! layout holding the records the instance theorems touch; `src/NfcTables.lex.tex`
//! is the LexLean module that declares them as artifacts (by its SHA-256 and
//! length) and states where each table starts and how many records it has. The
//! algorithm that reads the tables (decomposition, canonical ordering, composition,
//! Hangul arithmetic) is hand-written LexLean in `src/Nfc.lex.tex`; nothing here
//! decides whether a string is normalized, and the oracle run, not this
//! generator, is what establishes that the tables are right.
//!
//! Every code point is three octets, big endian (the last code point is
//! U+10FFFF, below 2^24). The tables, in order:
//!
//! | table | record | sorted by | holds |
//! | --- | --- | --- | --- |
//! | `ccc` | first, last, class (3+3+1) | first | maximal runs of equal nonzero canonical combining class |
//! | `excluded` | first, last (3+3) | first | maximal ranges of `Full_Composition_Exclusion` |
//! | `single` | code, target (3+3) | code | the canonical decompositions of one code point |
//! | `split` | code, first, second (3+3+3) | code | the canonical decompositions of two code points |
//! | `pairs` | first, second, code (3+3+3) | first, second | the same two-part decompositions, keyed by the pair |

use std::collections::BTreeMap;

use sha2::{Digest, Sha256};

use crate::ucd::{Data, UcdError};

/// Where a table starts in the byte string and how many records it has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Section {
    /// Offset of the first record.
    pub start: usize,
    /// Number of records.
    pub count: usize,
    /// Octets per record.
    pub width: usize,
}

/// The packed tables.
#[derive(Clone, Debug)]
pub struct Packed {
    /// The byte string, exactly as committed.
    pub bytes: Vec<u8>,
    /// Combining class runs.
    pub ccc: Section,
    /// Composition exclusion ranges.
    pub excluded: Section,
    /// One-code-point decompositions.
    pub single: Section,
    /// Two-code-point decompositions keyed by the composite.
    pub split: Section,
    /// Two-code-point decompositions keyed by the pair.
    pub pairs: Section,
    /// The deepest chain of one-step expansions.
    pub depth: usize,
    /// The least code point that is not trivially normalized.
    pub fast_bound: u32,
    /// The least code point with a canonical decomposition.
    pub decomposes_from: u32,
}

fn put3(out: &mut Vec<u8>, value: u32) {
    assert!(value < 1 << 24, "a code point is below 2^24");
    out.extend_from_slice(&value.to_be_bytes()[1..]);
}

/// Cross-check `CompositionExclusions.txt` against `Full_Composition_Exclusion`.
///
/// UAX #15 derives the property as the listed exclusions, the singleton
/// decompositions and the non-starter decompositions. The two files are
/// independent; this fails unless they agree exactly, which they do for the
/// pinned files.
pub fn check_exclusions(data: &Data) -> Result<(), UcdError> {
    let mut derived = data.listed_exclusions.clone();
    for (&code, parts) in &data.decomposition {
        let singleton = parts.len() == 1;
        let non_starter = data.class(code) != 0 || data.class(parts[0]) != 0;
        if singleton || non_starter {
            derived.insert(code);
        }
    }
    let mut property = std::collections::BTreeSet::new();
    for &(first, last) in &data.full_composition_exclusion {
        property.extend(first..=last);
    }
    if derived == property {
        return Ok(());
    }
    let only_derived: Vec<String> = derived
        .difference(&property)
        .map(|c| format!("{c:04X}"))
        .collect();
    let only_property: Vec<String> = property
        .difference(&derived)
        .map(|c| format!("{c:04X}"))
        .collect();
    Err(UcdError(format!(
        "Full_Composition_Exclusion disagrees with CompositionExclusions.txt plus the singleton and non-starter decompositions: only derived {only_derived:?}, only in the property {only_property:?}"
    )))
}

/// The records of the five tables, before they are written.
struct Records {
    ccc: Vec<(u32, u32, u8)>,
    excluded: Vec<(u32, u32)>,
    single: Vec<(u32, u32)>,
    split: Vec<(u32, u32, u32)>,
    pairs: Vec<(u32, u32, u32)>,
}

/// The size of the header: for each of the five tables, its start (3 octets)
/// and its record count (3 octets).
pub const HEADER: usize = 30;

/// Every record of the pinned data.
fn all_records(data: &Data) -> Result<Records, UcdError> {
    check_exclusions(data)?;
    let mut ccc: Vec<(u32, u32, u8)> = Vec::new();
    for (&code, &class) in &data.combining_class {
        match ccc.last_mut() {
            Some(run) if run.1 + 1 == code && run.2 == class => run.1 = code,
            _ => ccc.push((code, code, class)),
        }
    }
    let excluded = data.full_composition_exclusion.clone();
    let mut single = Vec::new();
    let mut split = Vec::new();
    let mut by_pair: BTreeMap<(u32, u32), u32> = BTreeMap::new();
    for (&code, parts) in &data.decomposition {
        match parts[..] {
            [target] => single.push((code, target)),
            [first, second] => {
                split.push((code, first, second));
                if let Some(other) = by_pair.insert((first, second), code) {
                    return Err(UcdError(format!(
                        "{other:04X} and {code:04X} have the same canonical decomposition"
                    )));
                }
            }
            _ => {
                return Err(UcdError(format!(
                    "{code:04X} decomposes into {} code points; the tables hold one or two",
                    parts.len()
                )))
            }
        }
    }
    let pairs = by_pair
        .iter()
        .map(|(&(first, second), &code)| (first, second, code))
        .collect();
    Ok(Records {
        ccc,
        excluded,
        single,
        split,
        pairs,
    })
}

/// Write the header and the five tables.
fn assemble(records: &Records, depth: usize, fast_bound: u32, decomposes_from: u32) -> Packed {
    let mut body = Vec::new();
    let mut sections = Vec::new();
    for (count, width, write) in [
        (
            records.ccc.len(),
            7,
            Box::new(|out: &mut Vec<u8>| {
                for &(first, last, class) in &records.ccc {
                    put3(out, first);
                    put3(out, last);
                    out.push(class);
                }
            }) as Box<dyn Fn(&mut Vec<u8>) + '_>,
        ),
        (
            records.excluded.len(),
            6,
            Box::new(|out: &mut Vec<u8>| {
                for &(first, last) in &records.excluded {
                    put3(out, first);
                    put3(out, last);
                }
            }),
        ),
        (
            records.single.len(),
            6,
            Box::new(|out: &mut Vec<u8>| {
                for &(code, target) in &records.single {
                    put3(out, code);
                    put3(out, target);
                }
            }),
        ),
        (
            records.split.len(),
            9,
            Box::new(|out: &mut Vec<u8>| {
                for &(code, first, second) in &records.split {
                    put3(out, code);
                    put3(out, first);
                    put3(out, second);
                }
            }),
        ),
        (
            records.pairs.len(),
            9,
            Box::new(|out: &mut Vec<u8>| {
                for &(first, second, code) in &records.pairs {
                    put3(out, first);
                    put3(out, second);
                    put3(out, code);
                }
            }),
        ),
    ] {
        let start = HEADER + body.len();
        write(&mut body);
        sections.push(Section {
            start,
            count,
            width,
        });
    }
    let mut bytes = Vec::with_capacity(HEADER + body.len());
    for section in &sections {
        put3(
            &mut bytes,
            u32::try_from(section.start).expect("a start is below 2^24"),
        );
        put3(
            &mut bytes,
            u32::try_from(section.count).expect("a count is below 2^24"),
        );
    }
    bytes.extend_from_slice(&body);
    Packed {
        bytes,
        ccc: sections[0],
        excluded: sections[1],
        single: sections[2],
        split: sections[3],
        pairs: sections[4],
        depth,
        fast_bound,
        decomposes_from,
    }
}

/// The least code point that is not trivially normalized: one with a nonzero
/// combining class, one in `Full_Composition_Exclusion`, or one that is the
/// second code point of a canonical pair (NFC_Quick_Check no or maybe, UAX #15
/// section 9.1). Below it every code point has combining class 0, is
/// `NFC_Quick_Check=Yes`, and does not combine with a preceding one, so a
/// string made only of code points below it is in NFC.
pub fn fast_bound(data: &Data) -> Result<u32, UcdError> {
    let records = all_records(data)?;
    let mut least = u32::MAX;
    for &(first, _, class) in &records.ccc {
        if class != 0 {
            least = least.min(first);
        }
    }
    for &(first, _) in &records.excluded {
        least = least.min(first);
    }
    for &(_, second, _) in &records.pairs {
        least = least.min(second);
    }
    // Hangul vowel and trailing jamo combine with what precedes them.
    least = least.min(0x1161);
    Ok(least)
}

/// The least code point with a canonical decomposition; below it no code
/// point decomposes.
pub fn decomposes_from(data: &Data) -> Result<u32, UcdError> {
    let records = all_records(data)?;
    let single = records.single.iter().map(|&(code, _)| code);
    let split = records.split.iter().map(|&(code, _, _)| code);
    Ok(single
        .chain(split)
        .min()
        .expect("some code point decomposes"))
}

/// Pack the tables.
pub fn pack(data: &Data) -> Result<Packed, UcdError> {
    let records = all_records(data)?;
    Ok(assemble(
        &records,
        data.decomposition_depth(),
        fast_bound(data)?,
        decomposes_from(data)?,
    ))
}

/// The code points, closed under taking a decomposition apart and under
/// composing a canonical pair, of the strings the kernel-checked instances
/// state (`SAMPLE_SEEDS`).
pub fn sample_closure(
    data: &Data,
    seeds: &[u32],
) -> Result<std::collections::BTreeSet<u32>, UcdError> {
    let records = all_records(data)?;
    let mut set: std::collections::BTreeSet<u32> = seeds.iter().copied().collect();
    loop {
        let before = set.len();
        for &(code, first, second) in &records.split {
            if set.contains(&code) {
                set.insert(first);
                set.insert(second);
            }
        }
        for &(code, target) in &records.single {
            if set.contains(&code) {
                set.insert(target);
            }
        }
        for &(first, second, code) in &records.pairs {
            if set.contains(&first) && set.contains(&second) {
                set.insert(code);
            }
        }
        if set.len() == before {
            return Ok(set);
        }
    }
}

/// The sample image: the same layout as the full tables, holding only the
/// records that concern `sample_closure(seeds)`. A lookup of a code point of
/// the closure, or of a pair of them, answers exactly what it answers in the
/// full tables; the sample is small enough for the elaborator to evaluate,
/// which the full tables are not (a list-backed `Bytes` literal is read in
/// time linear in the position, and the full tables are 28 KB).
pub fn sample(data: &Data, seeds: &[u32]) -> Result<Packed, UcdError> {
    let closure = sample_closure(data, seeds)?;
    let full = all_records(data)?;
    let touches = |first: u32, last: u32| closure.range(first..=last).next().is_some();
    let records = Records {
        ccc: full
            .ccc
            .iter()
            .copied()
            .filter(|&(f, l, _)| touches(f, l))
            .collect(),
        excluded: full
            .excluded
            .iter()
            .copied()
            .filter(|&(f, l)| touches(f, l))
            .collect(),
        single: full
            .single
            .iter()
            .copied()
            .filter(|&(c, _)| closure.contains(&c))
            .collect(),
        split: full
            .split
            .iter()
            .copied()
            .filter(|&(c, _, _)| closure.contains(&c))
            .collect(),
        pairs: full
            .pairs
            .iter()
            .copied()
            .filter(|&(f, s, _)| closure.contains(&f) && closure.contains(&s))
            .collect(),
    };
    Ok(assemble(
        &records,
        data.decomposition_depth(),
        fast_bound(data)?,
        decomposes_from(data)?,
    ))
}

/// SHA-256 of `bytes`, as lowercase hexadecimal.
pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

/// Where the byte string is cut in two artifacts. A language-1.2 `Bytes`
/// literal of about 26000 octets or more exceeds the recursion depth Lean's
/// elaborator has for the kernel-checked instances (probed: 24000 passes,
/// 26500 fails), so the one byte string is held as two artifacts and read as
/// a pair.
pub const SPLIT_AT: usize = 16384;

/// The two halves of `bytes`: the first `SPLIT_AT` octets and the rest.
pub fn halves(bytes: &[u8]) -> (&[u8], &[u8]) {
    bytes.split_at(SPLIT_AT.min(bytes.len()))
}

/// The strings the kernel-checked instances state, as code points (the
/// code points below `fast_bound` need no record). The sample image is built
/// for the closure of these.
pub const SAMPLE_SEEDS: &[&[u32]] = &[
    // e, then U+0301; U+00E9
    &[0x65, 0x301],
    &[0xE9],
    // Hangul: L V, L V T, LV T, the syllable LVT
    &[0x1100, 0x1161],
    &[0x1100, 0x1161, 0x11A8],
    &[0xAC00, 0x11A8],
    &[0xAC01],
    // composition exclusion U+0958 (and its parts)
    &[0x958],
    &[0x915, 0x93C],
    // singleton U+212B
    &[0x212B],
    // non-starters out of order, blocked, repeated
    &[0x61, 0x301, 0x316],
    &[0x44, 0x307, 0x323],
    &[0x61, 0x301, 0x301],
    // a three-step decomposition
    &[0x1F82],
];

/// All the seeds of the sample image.
pub fn sample_seeds() -> Vec<u32> {
    SAMPLE_SEEDS
        .iter()
        .flat_map(|string| string.iter().copied())
        .collect()
}

/// A digest as the formatter writes it in an artifact declaration: bare, and
/// quoted when it begins with a digit, which would read as a number.
pub fn digest_token(hex: &str) -> String {
    if hex.starts_with(|c: char| c.is_ascii_digit()) {
        format!("\"{hex}\"")
    } else {
        hex.to_owned()
    }
}

/// The text of `src/NfcTables.lex.tex`, for the byte string `packed.bytes` and
/// the sample image `sample.bytes`.
pub fn module(packed: &Packed, sample: &Packed) -> String {
    let (low, high) = halves(&packed.bytes);
    assert!(
        sample.bytes.len() < SPLIT_AT,
        "the sample image is read as the low half"
    );
    let mut text = String::new();
    text.push_str("\\begin{lexlean}{NfcTables}\n");
    text.push_str("\\useglossary{lexlean.std.nat@1.2.0}\n");
    text.push_str("\\useglossary{lexleanmcp.titles@1.0.0}\n");
    text.push_str("\\title{LexLean MCP Unicode tables}\n");
    text.push_str("\\begin{semanticmodule}\n");
    for (name, half) in [
        ("tablesLow", low),
        ("tablesHigh", high),
        ("sampleImage", &sample.bytes[..]),
    ] {
        text.push_str(&format!(
            "artifact{{\n  name: {name},\n  type: Bytes,\n  length: {},\n  role: binary,\n  schema: bytes{{}},\n  sha256: {}\n}}\n\n",
            half.len(),
            digest_token(&sha256_hex(half))
        ));
    }
    for (name, first, second) in [
        ("tables", "tablesLow", "tablesHigh"),
        ("sample", "sampleImage", "sampleImage"),
    ] {
        text.push_str(&format!(
            "definition{{\n  name: {name},\n  parameters: [],\n  result: (Bytes * Bytes),\n  body: (({first}), ({second})),\n  executable: true\n}}\n\n"
        ));
    }
    let constants: [(&str, usize); 4] = [
        ("lowLength", SPLIT_AT),
        ("decompositionDepth", packed.depth),
        ("fastBound", packed.fast_bound as usize),
        ("decomposesFrom", packed.decomposes_from as usize),
    ];
    for (index, (name, value)) in constants.iter().enumerate() {
        // The formatter writes a definition that fits one line on one line.
        text.push_str(&format!(
            "definition{{name: {name}, parameters: [], result: Nat, body: {value}, executable: true}}\n"
        ));
        if index + 1 != constants.len() {
            text.push('\n');
        }
    }
    text.push_str("\\end{semanticmodule}\n");
    text.push_str("\\end{lexlean}\n");
    text
}
