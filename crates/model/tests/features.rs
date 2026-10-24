//! FJ-01 to FJ-10: the features F1 to F13 and the journeys J1 to J6 of issue #18.
//!
//! The declarations are LexLean source (`src/Features.lex.tex`) and `lexlean verify`
//! checks the kernel side: the theorems of that module are `decide` proofs over the
//! same tables. These tests read the sources as text and apply the five rules of
//! PrismPM#330 (PP2014 to PP2018) once more, independently of the Lean data they
//! check, because `prismpm check` cannot run them yet: the SDK image bound in
//! `prismpm.lock` predates PrismPM#330. They also hold the vocabulary copy against
//! the PrismPM commit it comes from and `FEATURES.md` against the declarations.

use std::collections::BTreeSet;
use std::path::PathBuf;

use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The source with runs of white space collapsed and braces tightened, so the
/// formatter's one-line and multi-line spellings of a declaration read the same.
fn tight(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace("{ ", "{")
        .replace(" }", "}")
}

/// Index of the brace that closes the one opened just before `text`.
fn closing_brace(text: &str) -> usize {
    let mut depth = 1usize;
    let mut in_string = false;
    for (index, c) in text.char_indices() {
        match c {
            '"' => in_string = !in_string,
            '{' if !in_string => depth += 1,
            '}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    return index;
                }
            }
            _ => {}
        }
    }
    panic!("a declaration is not closed");
}

/// A definition without parameters: its name, result type and body.
struct Definition {
    name: String,
    result: String,
    body: String,
}

fn definitions(source: &str) -> Vec<Definition> {
    let text = tight(source);
    let mut found = Vec::new();
    let mut rest = text.as_str();
    while let Some(at) = rest.find("definition{") {
        let chunk = &rest[at + "definition{".len()..];
        let end = closing_brace(chunk);
        let inner = &chunk[..end];
        rest = &chunk[end..];
        let Some(after_name) = inner.strip_prefix("name: ") else {
            continue;
        };
        let Some((name, after_parameters)) = after_name.split_once(", parameters: [], result: ")
        else {
            continue;
        };
        let Some((result, body)) = after_parameters.split_once(", body: ") else {
            continue;
        };
        found.push(Definition {
            name: name.to_string(),
            result: result.to_string(),
            body: body.to_string(),
        });
    }
    found
}

/// Every double-quoted string of `segment`, in order.
fn quoted_strings(segment: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut current: Option<String> = None;
    for c in segment.chars() {
        match (&mut current, c) {
            (None, '"') => current = Some(String::new()),
            (Some(_), '"') => found.extend(current.take()),
            (Some(text), other) => text.push(other),
            (None, _) => {}
        }
    }
    found
}

struct FeatureDecl {
    name: String,
    statement: String,
    calls: Vec<String>,
    acceptance: Vec<String>,
}

struct JourneyDecl {
    name: String,
    actor: String,
    steps: Vec<String>,
}

fn features(source: &str) -> Vec<FeatureDecl> {
    definitions(source)
        .into_iter()
        .filter(|d| d.result == "Foundation.Arch::Feature")
        .map(|d| {
            let body = d
                .body
                .strip_prefix("(record Foundation.Arch::Feature {statement := ")
                .unwrap_or_else(|| panic!("{}: `{}`", d.name, d.body));
            let (statement, rest) = body
                .split_once(", calls := ")
                .unwrap_or_else(|| panic!("{}: no calls field", d.name));
            let (calls, acceptance) = rest
                .split_once(", acceptance := ")
                .unwrap_or_else(|| panic!("{}: no acceptance field", d.name));
            FeatureDecl {
                name: d.name,
                statement: quoted_strings(statement).concat(),
                calls: quoted_strings(calls),
                acceptance: quoted_strings(acceptance),
            }
        })
        .collect()
}

fn journeys(source: &str) -> Vec<JourneyDecl> {
    definitions(source)
        .into_iter()
        .filter(|d| d.result == "Foundation.Arch::Journey")
        .map(|d| {
            let body = d
                .body
                .strip_prefix("(record Foundation.Arch::Journey {actor := ")
                .unwrap_or_else(|| panic!("{}: `{}`", d.name, d.body));
            let (actor, steps) = body
                .split_once(", steps := ")
                .unwrap_or_else(|| panic!("{}: no steps field", d.name));
            JourneyDecl {
                name: d.name,
                actor: quoted_strings(actor).concat(),
                steps: quoted_strings(steps),
            }
        })
        .collect()
}

fn features_source() -> String {
    read("src/Features.lex.tex")
}

/// The body of the definition `name`.
fn body_of(source: &str, name: &str) -> String {
    definitions(source)
        .into_iter()
        .find(|d| d.name == name)
        .unwrap_or_else(|| panic!("no definition {name}"))
        .body
}

/// The `id := "..."` values of a table body, in order.
fn table_ids(source: &str, table: &str) -> Vec<String> {
    let body = body_of(source, table);
    body.split("id := ")
        .skip(1)
        .map(|part| quoted_strings(part).into_iter().next().expect("an id"))
        .collect()
}

/// The `number := N` values of a table body, in order.
fn table_numbers(source: &str, table: &str) -> Vec<u32> {
    body_of(source, table)
        .split("number := ")
        .skip(1)
        .map(|part| {
            part.split(|c: char| !c.is_ascii_digit())
                .next()
                .and_then(|digits| digits.parse().ok())
                .expect("a number")
        })
        .collect()
}

/// A theorem of `source` named `name` with the `decide` proof.
fn theorem_is_decided(source: &str, name: &str) -> bool {
    let text = tight(source);
    let marker = format!("theorem{{name: {name}, parameters: [], statement: ");
    let Some(at) = text.find(&marker) else {
        return false;
    };
    let chunk = &text[at + "theorem{".len()..];
    let end = closing_brace(chunk);
    chunk[..end].ends_with("proof: decide{}")
}

fn assert_decided(source: &str, names: &[&str]) {
    for name in names {
        assert!(
            theorem_is_decided(source, name),
            "src/Features.lex.tex has no theorem {name} proved by decide{{}}"
        );
    }
}

fn qualified(module: &str, name: &str) -> String {
    format!("{module}::{name}")
}

/// FJ-01: thirteen features, numbered in order.
#[test]
fn thirteen_features_are_declared_in_order_fj_01() {
    let source = features_source();
    let declared = features(&source);
    assert_eq!(declared.len(), 13, "issue #18 names F1 to F13");
    for (index, feature) in declared.iter().enumerate() {
        let prefix = format!("F{}: ", index + 1);
        assert!(
            feature.statement.starts_with(&prefix),
            "{}: the statement `{}` does not start with `{prefix}`",
            feature.name,
            feature.statement
        );
    }
    let ids: Vec<String> = declared
        .iter()
        .map(|f| qualified("Features", &f.name))
        .collect();
    assert_eq!(
        table_ids(&source, "featureRows"),
        ids,
        "featureRows must list the feature definitions, in order"
    );
    assert_eq!(
        table_numbers(&source, "featureRows"),
        (1..=13).collect::<Vec<u32>>()
    );
    assert_decided(&source, &["thirteenFeaturesInOrder"]);
}

/// FJ-02: six journeys, numbered in order.
#[test]
fn six_journeys_are_declared_in_order_fj_02() {
    let source = features_source();
    let declared = journeys(&source);
    assert_eq!(declared.len(), 6, "issue #18 names J1 to J6");
    for (index, journey) in declared.iter().enumerate() {
        assert!(
            journey.name.starts_with(&format!("journeyJ{}", index + 1)),
            "{} is not journey J{}",
            journey.name,
            index + 1
        );
    }
    let ids: Vec<String> = declared
        .iter()
        .map(|j| qualified("Features", &j.name))
        .collect();
    assert_eq!(table_ids(&source, "journeyRows"), ids);
    assert_eq!(
        table_numbers(&source, "journeyRows"),
        (1..=6).collect::<Vec<u32>>()
    );
    assert_decided(&source, &["sixJourneysInOrder"]);
}

/// FJ-03: PP2014, a feature with no call is refused.
#[test]
fn every_feature_has_a_call_fj_03() {
    let source = features_source();
    for feature in features(&source) {
        assert!(
            !feature.calls.is_empty(),
            "{} has no interface call (PP2014)",
            feature.name
        );
    }
    assert_decided(&source, &["everyFeatureHasACall"]);
}

/// FJ-04: PP2015, a feature with no acceptance row is refused.
#[test]
fn every_feature_has_an_acceptance_row_fj_04() {
    let source = features_source();
    for feature in features(&source) {
        assert!(
            !feature.acceptance.is_empty(),
            "{} has no acceptance row (PP2015)",
            feature.name
        );
    }
    assert_decided(&source, &["everyFeatureHasAnAcceptanceRow"]);
}

/// FJ-05: PP2016, a journey step must name a declared feature.
#[test]
fn every_journey_step_names_a_declared_feature_fj_05() {
    let source = features_source();
    let declared: BTreeSet<String> = features(&source)
        .iter()
        .map(|f| qualified("Features", &f.name))
        .collect();
    for journey in journeys(&source) {
        assert!(!journey.steps.is_empty(), "{} has no step", journey.name);
        for step in &journey.steps {
            assert!(
                declared.contains(step),
                "{}: step `{step}` names no declared feature (PP2016)",
                journey.name
            );
        }
    }
    assert_decided(&source, &["everyJourneyStepNamesAFeature"]);
}

/// FJ-06: PP2017, a journey actor must be a declared stakeholder.
#[test]
fn every_journey_actor_is_a_declared_stakeholder_fj_06() {
    let source = features_source();
    let architecture = read("src/Architecture.lex.tex");
    let stakeholders: BTreeSet<String> = definitions(&architecture)
        .into_iter()
        .filter(|d| d.result == "Foundation.Arch::Stakeholder")
        .map(|d| qualified("Architecture", &d.name))
        .collect();
    assert_eq!(
        stakeholders.len(),
        8,
        "the architecture description declares 8 stakeholders"
    );
    for journey in journeys(&source) {
        assert!(
            stakeholders.contains(&journey.actor),
            "{}: actor `{}` is no declared stakeholder (PP2017)",
            journey.name,
            journey.actor
        );
    }
    let listed: BTreeSet<String> = table_ids(&source, "stakeholderRows").into_iter().collect();
    assert_eq!(
        listed, stakeholders,
        "stakeholderRows must list the stakeholders of Architecture"
    );
    assert_decided(
        &source,
        &[
            "everyJourneyActorIsAStakeholder",
            "stakeholderRowsAreTheArchitectureStakeholders",
        ],
    );
}

/// FJ-07: PP2018, a call or an acceptance row must name a definition.
#[test]
fn every_call_and_acceptance_row_names_a_definition_fj_07() {
    let source = features_source();
    let strings: BTreeSet<String> = definitions(&source)
        .into_iter()
        .filter(|d| d.result == "String")
        .map(|d| qualified("Features", &d.name))
        .collect();
    let calls: BTreeSet<String> = table_ids(&source, "callRows").into_iter().collect();
    let acceptance: BTreeSet<String> = table_ids(&source, "acceptanceRows").into_iter().collect();
    assert!(
        calls.is_disjoint(&acceptance),
        "a definition is both a call and an acceptance row"
    );
    let tabled: BTreeSet<String> = calls.union(&acceptance).cloned().collect();
    assert_eq!(
        tabled, strings,
        "every String definition of the module is exactly one call or acceptance row"
    );
    let mut used_calls = BTreeSet::new();
    let mut used_acceptance = BTreeSet::new();
    for feature in features(&source) {
        for call in &feature.calls {
            assert!(
                calls.contains(call),
                "{}: call `{call}` names no definition (PP2018)",
                feature.name
            );
            used_calls.insert(call.clone());
        }
        for row in &feature.acceptance {
            assert!(
                acceptance.contains(row),
                "{}: acceptance row `{row}` names no definition (PP2018)",
                feature.name
            );
            used_acceptance.insert(row.clone());
        }
    }
    assert_eq!(used_calls, calls, "a declared call no feature uses");
    assert_eq!(
        used_acceptance, acceptance,
        "a declared acceptance row no feature uses"
    );
    assert_decided(
        &source,
        &[
            "everyCallNamesADefinition",
            "everyAcceptanceRowNamesADefinition",
            "featuresModelHolds",
        ],
    );
}

/// Lowercase hexadecimal.
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn files_below(directory: &std::path::Path, found: &mut Vec<String>) {
    for entry in std::fs::read_dir(directory).expect("a directory").flatten() {
        let path = entry.path();
        if path.is_dir() {
            files_below(&path, found);
        } else {
            let relative = path
                .strip_prefix(root())
                .expect("below the root")
                .to_string_lossy()
                .replace('\\', "/");
            found.push(relative);
        }
    }
}

/// The checksum the authority `A-PRISMPM-ARCH` records: the SHA-256 over the lines
/// `<sha256>  <path>` of the vocabulary files, in byte order of path.
fn vocabulary_checksum() -> (usize, String) {
    let mut files = Vec::new();
    files_below(&root().join("language/prism.arch"), &mut files);
    files.push("src/Foundation/Arch.lex.tex".to_string());
    files.sort();
    let mut manifest = String::new();
    for file in &files {
        let bytes = std::fs::read(root().join(file)).expect("a vocabulary file");
        manifest.push_str(&format!("{}  {file}\n", hex(&Sha256::digest(&bytes))));
    }
    (files.len(), hex(&Sha256::digest(manifest.as_bytes())))
}

/// FJ-08: the vocabulary this repository holds declares what PrismPM#330 defines.
#[test]
fn the_vocabulary_declares_features_and_journeys_fj_08() {
    let arch = tight(&read("src/Foundation/Arch.lex.tex"));
    for needle in [
        "name: Feature,",
        "{name: statement, type: String}",
        "{name: calls, type: (List String)}",
        "{name: acceptance, type: (List String)}",
        "name: Journey,",
        "{name: actor, type: String}",
        "{name: steps, type: (List String)}",
    ] {
        assert!(arch.contains(needle), "Foundation.Arch lacks `{needle}`");
    }
    for (entry, module, component) in [
        ("arch-feature", "Foundation.Arch", "Feature"),
        ("arch-journey", "Foundation.Arch", "Journey"),
        (
            "holo-normalized-features",
            "Foundation.Holo",
            "NormalizedFeatures",
        ),
    ] {
        let path = format!("language/prism.arch/entries/{entry}.toml");
        let text = read(&path);
        for needle in [
            format!("id = \"{entry}\""),
            format!("module = \"{module}\""),
            format!("component = \"{component}\""),
        ] {
            assert!(text.contains(&needle), "{path} lacks `{needle}`");
        }
    }
}

/// FJ-09: the vocabulary is PrismPM's, byte for byte.
#[test]
fn the_vocabulary_is_the_prismpm_copy_fj_09() {
    let model = repo_model::Model::load_from_repo_root().expect("the model loads");
    let authority = model
        .authorities
        .authority
        .iter()
        .find(|a| a.id == "A-PRISMPM-ARCH")
        .expect("model/authorities.toml names A-PRISMPM-ARCH");
    let (count, checksum) = vocabulary_checksum();
    assert_eq!(
        count, 143,
        "language/prism.arch has 142 files, plus Foundation.Arch"
    );
    assert_eq!(
        checksum, authority.checksum,
        "the vocabulary files differ from PrismPM d11ace6f: a copy is edited here, or the authority moved"
    );
    assert!(
        authority.citation.contains("d11ace6f"),
        "the authority names the PrismPM commit"
    );
}

/// FJ-10: FEATURES.md and the declarations list the same features and journeys.
#[test]
fn the_provenance_table_and_the_model_list_the_same_items_fj_10() {
    let document = read("FEATURES.md");
    let mut rows: Vec<(String, Vec<String>)> = Vec::new();
    for line in document.lines() {
        let cells: Vec<String> = line
            .trim()
            .trim_matches('|')
            .split('|')
            .map(|c| c.trim().to_string())
            .collect();
        let Some(first) = cells.first() else { continue };
        let key = first.trim_matches('`').to_string();
        if (key.starts_with('F') || key.starts_with('J'))
            && key[1..].chars().all(|c| c.is_ascii_digit())
            && key.len() > 1
            && cells.len() >= 4
        {
            rows.push((key, cells));
        }
    }
    let declared: Vec<String> = (1..=13)
        .map(|n| format!("F{n}"))
        .chain((1..=6).map(|n| format!("J{n}")))
        .collect();
    let listed: Vec<String> = rows.iter().map(|(k, _)| k.clone()).collect();
    assert_eq!(
        listed, declared,
        "FEATURES.md must have one row per F and J, in order"
    );
    for (key, cells) in &rows {
        for cell in &cells[1..] {
            assert!(!cell.is_empty(), "{key}: an empty cell");
        }
        let provenance = &cells[2];
        assert!(
            provenance.starts_with("issue") || provenance.starts_with("needs owner"),
            "{key}: provenance `{provenance}` is neither an issue nor `needs owner`"
        );
    }
}
