//! MC-01 to MC-03: `prismpm check` on the root model.
//!
//! The check itself runs in `just vv`, inside the SDK image. These tests read the sources as text,
//! so that a plain `cargo test` holds the gate in place and applies the two rules that the model
//! had to be conformed to once more, independently of PrismPM: the index of a facet record is its
//! position in the byte order of the IDs, and a standard reference has the form of the profile.

use std::collections::BTreeMap;
use std::path::PathBuf;

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The module sources of the root model, by module name, without the vendored vocabulary.
fn modules() -> Vec<(String, String)> {
    let mut found = Vec::new();
    for entry in std::fs::read_dir(root().join("src")).expect("src") {
        let path = entry.expect("an entry").path();
        let name = path
            .file_name()
            .expect("a name")
            .to_string_lossy()
            .to_string();
        if let Some(module) = name.strip_suffix(".lex.tex") {
            found.push((
                module.to_string(),
                std::fs::read_to_string(&path).expect("a module"),
            ));
        }
    }
    assert!(found.len() > 10, "the modules of the model were read");
    found
}

/// The facet records of `source`: the record type, the ID and the index of each that has one.
fn indexed_records(module: &str, source: &str) -> Vec<(String, String, u64)> {
    let mut found = Vec::new();
    let lines: Vec<&str> = source.lines().collect();
    for window in lines.windows(4) {
        let (Some(name), Some(result)) = (
            window[0]
                .trim()
                .strip_prefix("name: ")
                .and_then(|name| name.strip_suffix(',')),
            window[2]
                .trim()
                .strip_prefix("result: ")
                .and_then(|result| result.strip_suffix(',')),
        ) else {
            continue;
        };
        if !result.starts_with("Foundation.") {
            continue;
        }
        let Some(at) = window[3].find("{index := ") else {
            continue;
        };
        let digits: String = window[3][at + "{index := ".len()..]
            .chars()
            .take_while(char::is_ascii_digit)
            .collect();
        found.push((
            result.to_string(),
            format!("{module}::{name}"),
            digits.parse().expect("an index"),
        ));
    }
    found
}

/// Each record whose index is not its position in the byte order of the IDs of its type.
fn out_of_order(records: Vec<(String, String, u64)>) -> Vec<String> {
    let mut by_type: BTreeMap<String, Vec<(String, u64)>> = BTreeMap::new();
    for (kind, id, index) in records {
        by_type.entry(kind).or_default().push((id, index));
    }
    let mut out = Vec::new();
    for rows in by_type.values_mut() {
        rows.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
        for (position, (id, index)) in rows.iter().enumerate() {
            if *index != position as u64 {
                out.push(format!("{id} has index {index}, expected {position}"));
            }
        }
    }
    out
}

fn all_records() -> Vec<(String, String, u64)> {
    modules()
        .iter()
        .flat_map(|(module, source)| indexed_records(module, source))
        .collect()
}

#[test]
fn the_gate_runs_the_check_of_the_project_mc_01() {
    let project = read("prismpm.toml");
    assert!(
        project.contains("spec = \"prismpm/project/1\"")
            && project.contains("lexlean_project = \"lexlean.toml\""),
        "the root is a PrismPM project over the root model"
    );
    let justfile = read("Justfile");
    let vv = justfile
        .lines()
        .find(|line| line.starts_with("vv:"))
        .expect("the recipe vv");
    assert!(
        vv.split_whitespace().any(|word| word == "model-check"),
        "just vv runs the check: {vv}"
    );
    let recipe = &justfile[justfile.find("\nmodel-check:").expect("the recipe")..];
    assert_eq!(
        recipe.lines().nth(2).map(str::trim),
        Some("prismpm check"),
        "the recipe is the check and nothing narrower"
    );
    assert!(
        read("MODEL-CHECK.md").contains("MC-01"),
        "MODEL-CHECK.md does not name MC-01"
    );
}

#[test]
fn each_facet_index_is_the_byte_order_of_the_ids_mc_02() {
    let records = all_records();
    assert!(
        records.len() > 100,
        "the facet records were read: {}",
        records.len()
    );
    assert_eq!(out_of_order(records), Vec::<String>::new());
    // The rule is held to a defect: two indexes of one type swapped.
    let planted = vec![
        ("T".to_string(), "M::a".to_string(), 1),
        ("T".to_string(), "M::b".to_string(), 0),
    ];
    assert_eq!(out_of_order(planted).len(), 2);
    // Bytes, not numbers: this is why the names carry two digits.
    let unpadded = vec![
        ("T".to_string(), "M::r1".to_string(), 0),
        ("T".to_string(), "M::r2".to_string(), 1),
        ("T".to_string(), "M::r10".to_string(), 2),
    ];
    assert_eq!(out_of_order(unpadded).len(), 2);
}

#[test]
fn each_standard_reference_has_the_form_of_the_profile_mc_02() {
    for (file, reference) in [
        (
            "src/Architecture.lex.tex",
            "standardIndex := 4, editionIndex := 0, standardId := \"ISO-42010-2022\", edition := \"2022\"",
        ),
        (
            "src/Quality.lex.tex",
            "standardIndex := 0, editionIndex := 0, standardId := \"ISO-25010-2023\", edition := \"2023\"",
        ),
    ] {
        assert!(read(file).contains(reference), "{file}: {reference}");
    }
    // The same identifiers are authorities of the lock the SDK binds.
    let standards = read("standards.lock");
    for id in ["ISO-42010-2022", "ISO-25010-2023"] {
        assert!(standards.contains(id), "standards.lock does not hold {id}");
    }
}

#[test]
fn each_gate_rejects_its_planted_defect_and_what_it_misses_is_named_mc_03() {
    let document = read("MODEL-CHECK.md");
    let section = |title: &str| {
        document
            .split(title)
            .nth(1)
            .and_then(|rest| rest.split("\n## ").next())
            .unwrap_or_else(|| panic!("MODEL-CHECK.md has no section {title}"))
            .to_string()
    };
    let rows = |text: &str| {
        text.lines()
            .filter(|line| line.starts_with("| ") && !line.starts_with("| ---"))
            .count()
            - 1
    };
    let planted = section("## Planted defects");
    assert_eq!(rows(&planted), 13, "the planted defects of MC-03");
    for code in [
        "PP4004", "PP2014", "PP2015", "PP2016", "PP2017", "PP2018", "PP2001", "PP2005", "PP2006",
        "PP2007",
    ] {
        assert!(
            planted.contains(code),
            "no planted defect is rejected with {code}"
        );
    }
    let missed = section("## What the check does not reject");
    assert_eq!(rows(&missed), 4, "the defects the check lets through");
}
