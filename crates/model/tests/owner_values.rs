//! OV-01 to OV-03: the owner-set values of issue #19.
//!
//! The values are LexLean source (`src/OwnerValues.lex.tex`, `src/Product.lex.tex`,
//! `src/Security.lex.tex`) and `lexlean verify` checks the kernel side. These tests
//! read the same sources as text and hold them against `OWNER-VALUES.md`, which
//! states why each value is what it is, and against the files that bound a value
//! from outside the model (`lexlean.toml`, `Cargo.toml`). A value changed in one
//! place and not the others fails here and names its key.

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

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

/// A definition without parameters: its name, result type and body.
struct Definition {
    name: String,
    result: String,
    body: String,
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

fn value_definitions(source: &str) -> Vec<Definition> {
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

/// `draft.max_files` is bound by the definition `draftMaxFiles`.
fn definition_name(key: &str) -> String {
    let mut name = String::new();
    let mut upper = false;
    for c in key.chars() {
        if c == '.' || c == '_' {
            upper = true;
        } else if upper {
            name.extend(c.to_uppercase());
            upper = false;
        } else {
            name.push(c);
        }
    }
    name
}

fn quoted(body: &str, prefix: &str) -> String {
    body.strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix("\")"))
        .unwrap_or_else(|| panic!("`{body}` is not `{prefix}...\")`"))
        .to_string()
}

/// How the table writes the value a definition holds.
fn rendered(definition: &Definition) -> String {
    match definition.result.as_str() {
        "Nat" => definition
            .body
            .parse::<u64>()
            .unwrap_or_else(|_| {
                panic!(
                    "{}: `{}` is not a numeral",
                    definition.name, definition.body
                )
            })
            .to_string(),
        "Product::OwnerValue" => quoted(&definition.body, "(ctor Product::OwnerValue.Bound \""),
        "Retention" => {
            if definition.body == "(ctor Retention.Forever)" {
                "forever".to_string()
            } else {
                let days = definition
                    .body
                    .strip_prefix("(ctor Retention.Days ")
                    .and_then(|rest| rest.strip_suffix(')'))
                    .unwrap_or_else(|| panic!("{}: `{}`", definition.name, definition.body));
                format!("{days} days")
            }
        }
        other => panic!(
            "{}: result type {other} is not an owner value",
            definition.name
        ),
    }
}

/// The text of a field `name := ` inside a record literal: a quoted string or a numeral.
fn field(record: &str, name: &str) -> String {
    let marker = format!("{name} := ");
    let at = record
        .find(&marker)
        .unwrap_or_else(|| panic!("`{record}` has no field {name}"));
    let rest = &record[at + marker.len()..];
    if let Some(string) = rest.strip_prefix('"') {
        string[..string.find('"').expect("a string closes")].to_string()
    } else {
        rest.chars().take_while(char::is_ascii_digit).collect()
    }
}

/// The records `record <type> {...}` of a source, as the text between the braces.
fn records(source: &str, type_name: &str) -> Vec<String> {
    let text = tight(source);
    let marker = format!("record {type_name} {{");
    let mut found = Vec::new();
    let mut rest = text.as_str();
    while let Some(at) = rest.find(&marker) {
        let chunk = &rest[at + marker.len()..];
        let end = closing_brace(chunk);
        found.push(chunk[..end].to_string());
        rest = &chunk[end..];
    }
    found
}

struct Row {
    key: String,
    value: String,
    cells: Vec<String>,
}

/// The first backtick span of a cell.
fn tick(cell: &str) -> String {
    cell.split('`')
        .nth(1)
        .unwrap_or_else(|| panic!("`{cell}` has no backtick span"))
        .to_string()
}

fn table() -> Vec<Row> {
    read("OWNER-VALUES.md")
        .lines()
        .filter(|line| line.starts_with("| `"))
        .map(|line| {
            let cells: Vec<String> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect();
            assert_eq!(cells.len(), 6, "a row has six cells: {line}");
            Row {
                key: tick(&cells[0]),
                value: tick(&cells[1]),
                cells,
            }
        })
        .collect()
}

/// The values issue #19 names, as keys of the table.
const REQUIRED: [&str; 16] = [
    "version",
    "signing.algorithm",
    "signing.key_custody",
    "signing.key_rotation_days",
    "request_state_key.custody",
    "request_state_key.rotation_days",
    "verify.width",
    "verify.memory_kib",
    "verify.concurrency",
    "rate.tool_calls",
    "draft.max_bytes",
    "draft.max_files",
    "store.draft_retention",
    "store.attestation_retention",
    "cache.ttl_ms",
    "listen.port",
];

fn product_version() -> String {
    let text = tight(&read("src/Product.lex.tex"));
    let marker = "version := (ctor OwnerValue.Bound \"";
    let at = text
        .find(marker)
        .expect("the product version is bound in src/Product.lex.tex");
    let rest = &text[at + marker.len()..];
    rest[..rest.find('"').expect("a string closes")].to_string()
}

fn risks() -> BTreeMap<String, String> {
    records(&read("src/Security.lex.tex"), "Foundation.Sec::Risk")
        .iter()
        .map(|record| {
            let index: usize = field(record, "index").parse().expect("a risk has an index");
            let marker = "likelihood := (ctor Foundation.Sec::Likelihood.";
            let at = record
                .find(marker)
                .unwrap_or_else(|| panic!("risk {index} has no likelihood"));
            let rest = &record[at + marker.len()..];
            let level = rest[..rest.find(')').expect("a constructor closes")].to_string();
            (format!("risk.R{}", index + 1), level)
        })
        .collect()
}

fn objectives() -> BTreeMap<String, String> {
    records(&read("src/OwnerValues.lex.tex"), "Slo")
        .iter()
        .map(|record| {
            let index: usize = field(record, "index")
                .parse()
                .expect("an objective has an index");
            (
                format!("slo.S{}", index + 1),
                format!(
                    "{} {} / {} s",
                    field(record, "threshold"),
                    field(record, "unit"),
                    field(record, "windowSeconds")
                ),
            )
        })
        .collect()
}

fn profiles() -> BTreeMap<String, String> {
    records(&read("src/OwnerValues.lex.tex"), "ProfileSpec")
        .iter()
        .map(|record| {
            (
                format!("profile.{}", field(record, "name")),
                field(record, "language"),
            )
        })
        .collect()
}

/// The value the model binds for a key of the table.
fn model_value(
    key: &str,
    definitions: &BTreeMap<String, Definition>,
    families: &BTreeMap<String, String>,
) -> Option<String> {
    if key == "version" {
        return Some(product_version());
    }
    if let Some(value) = families.get(key) {
        return Some(value.clone());
    }
    definitions.get(&definition_name(key)).map(rendered)
}

fn sources() -> Vec<(String, String)> {
    let mut found = Vec::new();
    let mut stack = vec![root().join("src")];
    while let Some(dir) = stack.pop() {
        for entry in std::fs::read_dir(&dir).expect("src is readable").flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.to_string_lossy().ends_with(".lex.tex") {
                let text = std::fs::read_to_string(&path).expect("a source is readable");
                found.push((path.display().to_string(), text));
            }
        }
    }
    found
}

/// OV-01: nothing is left unset.
#[test]
fn no_declaration_depends_on_an_unset_owner_value_ov_01() {
    for (path, text) in sources() {
        for constructor in [
            "(ctor Product::OwnerValue.Unbound",
            "(ctor OwnerValue.Unbound",
        ] {
            assert!(
                !text.contains(constructor),
                "{path} applies the Unbound constructor of OwnerValue: an owner value is unset (#19)"
            );
        }
    }

    assert!(
        !product_version().is_empty(),
        "the product version is bound to an empty text"
    );

    let risk_levels = risks();
    assert_eq!(
        risk_levels.len(),
        10,
        "R1 to R10 each carry a Foundation.Sec::Risk record"
    );
    for (key, level) in &risk_levels {
        assert!(
            ["low", "medium", "high"].contains(&level.as_str()),
            "{key}: `{level}` is not a Foundation.Sec::Likelihood constructor"
        );
    }

    let rows = table();
    let keys: BTreeSet<&str> = rows.iter().map(|row| row.key.as_str()).collect();
    for required in REQUIRED {
        assert!(
            keys.contains(required),
            "issue #19 names `{required}` and OWNER-VALUES.md has no row for it"
        );
    }
    for n in 1..=10 {
        assert!(
            keys.contains(format!("risk.R{n}").as_str()),
            "no row for risk.R{n}"
        );
    }
    assert!(
        keys.iter().any(|key| key.starts_with("profile.")),
        "OWNER-VALUES.md binds no profile"
    );
    assert!(
        keys.iter().any(|key| key.starts_with("slo.")),
        "OWNER-VALUES.md binds no service level objective"
    );
}

/// OV-02: the table and the model are one statement, from both sides.
#[test]
fn the_table_and_the_model_state_the_same_values_ov_02() {
    let rows = table();
    let definitions: BTreeMap<String, Definition> =
        value_definitions(&read("src/OwnerValues.lex.tex"))
            .into_iter()
            .map(|definition| (definition.name.clone(), definition))
            .collect();

    let mut families = BTreeMap::new();
    families.extend(risks());
    families.extend(objectives());
    families.extend(profiles());

    let mut failures = Vec::new();
    let mut used: BTreeSet<String> = BTreeSet::new();
    for row in &rows {
        if !used.insert(row.key.clone()) {
            failures.push(format!("{}: the table has the key twice", row.key));
        }
        match model_value(&row.key, &definitions, &families) {
            None => failures.push(format!("{}: the model binds no such value", row.key)),
            Some(bound) if bound != row.value => failures.push(format!(
                "{}: the model binds `{bound}` and OWNER-VALUES.md proposes `{}`",
                row.key, row.value
            )),
            Some(_) => {}
        }
        // Cells 2 to 5: evidence, decider, status, consequence.
        for (index, name) in [
            (2, "evidence"),
            (3, "decider"),
            (4, "status"),
            (5, "consequence"),
        ] {
            if row.cells[index].is_empty() {
                failures.push(format!("{}: the {name} cell is empty", row.key));
            }
        }
        if !["owner", "sponsor", "owner and sponsor"].contains(&row.cells[3].as_str()) {
            failures.push(format!(
                "{}: `{}` is not owner, sponsor, or owner and sponsor",
                row.key, row.cells[3]
            ));
        }
        if !["evidenced", "needs owner"].contains(&row.cells[4].as_str()) {
            failures.push(format!(
                "{}: `{}` is not evidenced or needs owner",
                row.key, row.cells[4]
            ));
        }
    }

    // The other direction: a value the model binds with no row has no evidence.
    for definition in definitions.values() {
        let is_value =
            ["Nat", "Product::OwnerValue", "Retention"].contains(&definition.result.as_str());
        let has_row = rows
            .iter()
            .any(|row| definition_name(&row.key) == definition.name);
        if is_value && !has_row {
            failures.push(format!(
                "{}: the model binds it and OWNER-VALUES.md has no row",
                definition.name
            ));
        }
    }
    for key in families.keys() {
        if !used.contains(key) {
            failures.push(format!(
                "{key}: the model binds it and OWNER-VALUES.md has no row"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "OWNER-VALUES.md and the model disagree:\n{}",
        failures.join("\n")
    );
}

fn number(value: &str) -> u64 {
    value
        .parse()
        .unwrap_or_else(|_| panic!("`{value}` is not a numeral"))
}

/// The Nat a key is bound to in the model.
fn bound(key: &str) -> u64 {
    let definitions: BTreeMap<String, Definition> =
        value_definitions(&read("src/OwnerValues.lex.tex"))
            .into_iter()
            .map(|definition| (definition.name.clone(), definition))
            .collect();
    let definition = definitions
        .get(&definition_name(key))
        .unwrap_or_else(|| panic!("{key} is not bound in src/OwnerValues.lex.tex"));
    number(&rendered(definition))
}

/// The `[limits]` of lexlean.toml, which LexLean enforces as `LLS8002`.
fn lexlean_limit(name: &str) -> u64 {
    let toml: toml::Table = read("lexlean.toml").parse().expect("lexlean.toml is TOML");
    toml["limits"][name]
        .as_integer()
        .unwrap_or_else(|| panic!("lexlean.toml [limits] has no integer {name}")) as u64
}

/// OV-03: each value stays inside the constraint it follows from.
#[test]
fn each_value_respects_its_constraint_ov_03() {
    // At LexLean 9796067 the examples other than the Atlas total at most
    // 5413010 bytes of `.lex.tex` (production-coverage) in at most 148 modules.
    // The Atlas is a profile that carries its own sources, never a draft.
    const LARGEST_EXAMPLE_BYTES: u64 = 5_413_010;
    const MOST_EXAMPLE_MODULES: u64 = 148;
    // The Atlas modules are 24841413 bytes and count toward the same limit.
    const ATLAS_SOURCE_BYTES: u64 = 24_841_413;
    // The memory of the host that runs the verifier: a 16 GB runner.
    const HOST_MEMORY_KIB: u64 = 16 * 1024 * 1024;

    let max_bytes = bound("draft.max_bytes");
    let max_files = bound("draft.max_files");
    let total_limit = lexlean_limit("max_total_source_bytes");
    assert!(
        max_bytes >= LARGEST_EXAMPLE_BYTES,
        "draft.max_bytes {max_bytes} cannot hold the largest example ({LARGEST_EXAMPLE_BYTES} bytes)"
    );
    assert!(
        max_files >= MOST_EXAMPLE_MODULES,
        "draft.max_files {max_files} cannot hold the largest example ({MOST_EXAMPLE_MODULES} modules)"
    );
    assert!(
        max_bytes + ATLAS_SOURCE_BYTES <= total_limit,
        "draft.max_bytes {max_bytes} and the Atlas modules pass max_total_source_bytes {total_limit}: LexLean would refuse first with LLS8002"
    );

    let width = bound("verify.width");
    assert!(
        (1..=64).contains(&width),
        "verify.width {width} is outside the range 1 to 64 of LexLean SPEC 22.11"
    );
    let budget = bound("verify.memory_kib");
    let concurrency = bound("verify.concurrency");
    assert!(
        concurrency >= 1,
        "verify.concurrency is 0: no task could run"
    );
    assert!(
        concurrency * budget <= HOST_MEMORY_KIB,
        "verify.concurrency {concurrency} times verify.memory_kib {budget} passes the {HOST_MEMORY_KIB} KiB of the host"
    );

    let port = bound("listen.port");
    assert!(
        (1024..32768).contains(&port),
        "listen.port {port} is not in the registered range below the Linux ephemeral range"
    );
    assert!(
        bound("rate.tool_calls") >= 1,
        "rate.tool_calls is 0: every call is refused"
    );
    assert!(bound("rate.window_ms") >= 1, "rate.window_ms is 0");

    let cargo: toml::Table = read("Cargo.toml").parse().expect("Cargo.toml is TOML");
    let workspace_version = cargo["workspace"]["package"]["version"]
        .as_str()
        .expect("the workspace package has a version");
    assert_eq!(
        product_version(),
        workspace_version,
        "the product version of src/Product.lex.tex and the workspace package version differ"
    );
}
