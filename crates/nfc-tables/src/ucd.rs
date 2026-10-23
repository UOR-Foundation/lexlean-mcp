//! Reading the pinned Unicode Character Database 17.0.0 files.
//!
//! Nothing here decides normalization. It reads the columns of `UnicodeData.txt`
//! and the ranges of `DerivedNormalizationProps.txt` the tables are generated
//! from, and the rows of `NormalizationTest.txt` the oracle checks.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::Path;

/// The Unicode version every pinned file names in its first line.
pub const UNICODE_VERSION: &str = "17.0.0";

/// The pinned files, in the order the generator reads them.
pub const FILES: [&str; 4] = [
    "UnicodeData.txt",
    "CompositionExclusions.txt",
    "DerivedNormalizationProps.txt",
    "NormalizationTest.txt",
];

/// Why a file is not what the generator expects.
#[derive(Debug)]
pub struct UcdError(pub String);

impl fmt::Display for UcdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for UcdError {}

fn fail<T>(message: String) -> Result<T, UcdError> {
    Err(UcdError(message))
}

/// The text of the four pinned files.
#[derive(Clone, Debug)]
pub struct Files {
    /// `UnicodeData.txt`.
    pub unicode_data: String,
    /// `CompositionExclusions.txt`.
    pub composition_exclusions: String,
    /// `DerivedNormalizationProps.txt`.
    pub derived_normalization_props: String,
    /// `NormalizationTest.txt`.
    pub normalization_test: String,
}

impl Files {
    /// Read the four files from `dir`.
    pub fn load(dir: &Path) -> Result<Files, UcdError> {
        let read = |name: &str| {
            std::fs::read_to_string(dir.join(name))
                .map_err(|e| UcdError(format!("{}: {e}", dir.join(name).display())))
        };
        Ok(Files {
            unicode_data: read(FILES[0])?,
            composition_exclusions: read(FILES[1])?,
            derived_normalization_props: read(FILES[2])?,
            normalization_test: read(FILES[3])?,
        })
    }
}

/// What the normalization algorithms read from the database.
#[derive(Clone, Debug)]
pub struct Data {
    /// Canonical combining class of every code point whose class is not 0.
    pub combining_class: BTreeMap<u32, u8>,
    /// The canonical decomposition mapping of every code point that has one
    /// (field 5 without a `<tag>`), one step, Hangul syllables excluded: they
    /// decompose by arithmetic, not by a mapping.
    pub decomposition: BTreeMap<u32, Vec<u32>>,
    /// Every assigned code point, ranges expanded.
    pub assigned: BTreeSet<u32>,
    /// The code points with the property `Full_Composition_Exclusion`, as
    /// maximal ranges `(first, last)` in ascending order.
    pub full_composition_exclusion: Vec<(u32, u32)>,
    /// The code points listed in `CompositionExclusions.txt`.
    pub listed_exclusions: BTreeSet<u32>,
}

fn first_line_names(text: &str, file: &str) -> Result<(), UcdError> {
    let first = text.lines().next().unwrap_or("");
    let expected = format!("# {}-{UNICODE_VERSION}.txt", file.trim_end_matches(".txt"));
    if first == expected {
        Ok(())
    } else {
        fail(format!(
            "{file}: the first line is {first:?}, expected {expected:?}"
        ))
    }
}

fn hex(field: &str, context: &str) -> Result<u32, UcdError> {
    u32::from_str_radix(field.trim(), 16).map_err(|_| {
        UcdError(format!(
            "{context}: {field:?} is not a hexadecimal code point"
        ))
    })
}

impl Data {
    /// Parse the three files the tables come from. Each must name 17.0.0 in
    /// its first line.
    pub fn parse(files: &Files) -> Result<Data, UcdError> {
        for (file, text) in [
            (FILES[0], &files.unicode_data),
            (FILES[1], &files.composition_exclusions),
            (FILES[2], &files.derived_normalization_props),
            (FILES[3], &files.normalization_test),
        ] {
            // UnicodeData.txt carries no header line; its version is held by the
            // digest pin, not by its text.
            if file != FILES[0] {
                first_line_names(text, file)?;
            }
        }
        let mut combining_class = BTreeMap::new();
        let mut decomposition = BTreeMap::new();
        let mut assigned = BTreeSet::new();
        let mut open_range: Option<u32> = None;
        for (number, line) in files.unicode_data.lines().enumerate() {
            let context = format!("UnicodeData.txt:{}", number + 1);
            let fields: Vec<&str> = line.split(';').collect();
            if fields.len() != 15 {
                return fail(format!("{context}: {} fields, expected 15", fields.len()));
            }
            let code = hex(fields[0], &context)?;
            let name = fields[1];
            if name.ends_with(", First>") {
                open_range = Some(code);
                continue;
            }
            if name.ends_with(", Last>") {
                let Some(first) = open_range.take() else {
                    return fail(format!("{context}: a Last line without a First line"));
                };
                assigned.extend(first..=code);
                continue;
            }
            assigned.insert(code);
            let class: u8 = fields[3]
                .parse()
                .map_err(|_| UcdError(format!("{context}: bad combining class {:?}", fields[3])))?;
            if class != 0 {
                combining_class.insert(code, class);
            }
            let mapping = fields[5];
            if !mapping.is_empty() && !mapping.starts_with('<') {
                let mut parts = Vec::new();
                for part in mapping.split(' ') {
                    parts.push(hex(part, &context)?);
                }
                decomposition.insert(code, parts);
            }
        }
        if open_range.is_some() {
            return fail("UnicodeData.txt: a First line without a Last line".to_owned());
        }

        let mut full_composition_exclusion = Vec::new();
        for (number, line) in files.derived_normalization_props.lines().enumerate() {
            let context = format!("DerivedNormalizationProps.txt:{}", number + 1);
            let content = line.split('#').next().unwrap_or("");
            let mut columns = content.split(';');
            let (Some(range), Some(property)) = (columns.next(), columns.next()) else {
                continue;
            };
            // `Full_Composition_Exclusion` has no value column; the other
            // properties of the file (`NFC_QC; N`, `FC_NFKC; ...`) are not it.
            if property.trim() != "Full_Composition_Exclusion" {
                continue;
            }
            let range = range.trim();
            let (first, last) = match range.split_once("..") {
                Some((a, b)) => (hex(a, &context)?, hex(b, &context)?),
                None => (hex(range, &context)?, hex(range, &context)?),
            };
            if let Some(&(_, previous_last)) = full_composition_exclusion.last() {
                if first <= previous_last {
                    return fail(format!("{context}: ranges are not ascending and disjoint"));
                }
            }
            full_composition_exclusion.push((first, last));
        }
        // Adjacent ranges are one range; the tables do not depend on how the
        // file happened to split them.
        let mut merged: Vec<(u32, u32)> = Vec::new();
        for (first, last) in full_composition_exclusion {
            match merged.last_mut() {
                Some(previous) if previous.1 + 1 == first => previous.1 = last,
                _ => merged.push((first, last)),
            }
        }

        let mut listed_exclusions = BTreeSet::new();
        for (number, line) in files.composition_exclusions.lines().enumerate() {
            let context = format!("CompositionExclusions.txt:{}", number + 1);
            let content = line.split('#').next().unwrap_or("").trim();
            if content.is_empty() {
                continue;
            }
            listed_exclusions.insert(hex(content, &context)?);
        }

        Ok(Data {
            combining_class,
            decomposition,
            assigned,
            full_composition_exclusion: merged,
            listed_exclusions,
        })
    }

    /// Is `code` in `Full_Composition_Exclusion`?
    pub fn is_excluded(&self, code: u32) -> bool {
        self.full_composition_exclusion
            .iter()
            .any(|&(first, last)| first <= code && code <= last)
    }

    /// The canonical combining class of `code`.
    pub fn class(&self, code: u32) -> u8 {
        self.combining_class.get(&code).copied().unwrap_or(0)
    }

    /// The number of one-step expansions after which no code point of any
    /// canonical decomposition has a mapping any more.
    pub fn decomposition_depth(&self) -> usize {
        let mut deepest = 0;
        for &code in self.decomposition.keys() {
            let mut level = vec![code];
            let mut depth = 0;
            loop {
                let mut next = Vec::new();
                let mut grew = false;
                for c in &level {
                    match self.decomposition.get(c) {
                        Some(parts) => {
                            next.extend(parts.iter().copied());
                            grew = true;
                        }
                        None => next.push(*c),
                    }
                }
                if !grew {
                    break;
                }
                level = next;
                depth += 1;
            }
            deepest = deepest.max(depth);
        }
        deepest
    }
}

/// One row of `NormalizationTest.txt`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Row {
    /// The line number in the file, from 1.
    pub line: usize,
    /// The part (`@Part0` ... `@Part3`) the row is in.
    pub part: u8,
    /// Columns `c1` to `c5`, each a sequence of code points.
    pub columns: [Vec<u32>; 5],
}

/// The rows of `NormalizationTest.txt`, in file order.
pub fn rows(text: &str) -> Result<Vec<Row>, UcdError> {
    let mut out = Vec::new();
    let mut part: Option<u8> = None;
    for (number, line) in text.lines().enumerate() {
        let context = format!("NormalizationTest.txt:{}", number + 1);
        if let Some(rest) = line.strip_prefix("@Part") {
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            part = Some(
                digits
                    .parse()
                    .map_err(|_| UcdError(format!("{context}: bad part marker {line:?}")))?,
            );
            continue;
        }
        let content = line.split('#').next().unwrap_or("").trim();
        if content.is_empty() {
            continue;
        }
        let Some(part) = part else {
            return fail(format!("{context}: a row before the first @Part"));
        };
        let fields: Vec<&str> = content.split(';').collect();
        // Five columns and the empty field after the last semicolon.
        if fields.len() != 6 || !fields[5].trim().is_empty() {
            return fail(format!("{context}: expected five columns"));
        }
        let mut columns: [Vec<u32>; 5] = Default::default();
        for (index, field) in fields[..5].iter().enumerate() {
            for code in field.split_whitespace() {
                columns[index].push(hex(code, &context)?);
            }
            if columns[index].is_empty() {
                return fail(format!("{context}: column c{} is empty", index + 1));
            }
        }
        out.push(Row {
            line: number + 1,
            part,
            columns,
        });
    }
    Ok(out)
}
