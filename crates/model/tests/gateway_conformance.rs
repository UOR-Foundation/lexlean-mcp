//! GW-12: the gateway passes the pinned server suite of MCP with its own exact waiting list
//! (AC-03, part b of issue #20; issue #7).
//!
//! The run itself is the `gateway` job of `.github/workflows/lexlean.yml`: it serves the built
//! `gateway-host` and runs the oracle of the MCP checkout at `.github/mcp.rev` over it. These
//! tests read the waiting list, the job, the library it calls and the record as text and hold
//! them against each other, so that a list that is not the job's, a copy of MCP's oracle, a
//! job that does not require its count or a record that does not match the list fails here.
//! The tests that need node or the installed suite are `#[ignore]`d with the reason that names
//! the job, as MCP does (CF-08): the job runs them with `-- --ignored` and requires that as
//! many passed as the source ignores.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

/// The scenarios of the suite that the gateway must pass whatever its catalogue is: the
/// lifecycle of a stateless server, the transport, the headers, DNS rebinding, caching and the
/// schema. A waiting list that names one of them would hide a defect in the gateway's serving.
const MUST_PASS: [&str; 6] = [
    "tools-list",
    "dns-rebinding-protection",
    "caching",
    "json-schema-2020-12",
    "http-header-validation",
    "http-custom-header-server-validation",
];

/// One entry of `gateway/waiting.json`.
#[derive(Debug)]
struct Entry {
    issue: Option<u64>,
    pr: Option<u64>,
    reason: String,
}

fn waiting() -> (BTreeMap<String, Entry>, BTreeMap<String, String>) {
    let parsed: serde_json::Value =
        serde_json::from_str(&read("gateway/waiting.json")).expect("gateway/waiting.json is JSON");
    assert_eq!(
        parsed["spec"], "2026-07-28",
        "the list is for the 2026-07-28 revision"
    );
    let entries = parsed["waiting"]
        .as_object()
        .expect("`waiting` is an object")
        .iter()
        .map(|(scenario, value)| {
            (
                scenario.clone(),
                Entry {
                    issue: value["issue"].as_u64(),
                    pr: value["pr"].as_u64(),
                    reason: value["reason"].as_str().unwrap_or_default().to_string(),
                },
            )
        })
        .collect();
    let skipped = parsed["suite_skipped"]
        .as_object()
        .expect("`suite_skipped` is an object")
        .iter()
        .map(|(scenario, value)| {
            (
                scenario.clone(),
                value["reason"].as_str().unwrap_or_default().to_string(),
            )
        })
        .collect();
    (entries, skipped)
}

/// The rows of the markdown table under `heading` of the record, as cells.
fn table(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut inside = false;
    for line in document.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            inside = title.trim() == heading;
            continue;
        }
        if inside && line.starts_with('|') {
            let cells: Vec<String> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect();
            if cells
                .iter()
                .all(|c| c.chars().all(|ch| ch == '-' || ch == ':'))
            {
                continue;
            }
            rows.push(cells);
        }
    }
    if !rows.is_empty() {
        rows.remove(0);
    }
    rows
}

fn walk(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if name == "target" || name == ".git" || name == ".prism" || name == ".lexlean" {
            continue;
        }
        if path.is_dir() {
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// GW-12: the list is exact and honest, and the record says the same.
#[test]
fn the_waiting_list_is_exact_and_recorded_per_scenario_gw_12() {
    let (entries, skipped) = waiting();
    assert!(
        !entries.is_empty(),
        "the gateway's catalogue is not the fixture's, so scenarios that call fixture tools wait"
    );
    for (scenario, entry) in &entries {
        assert!(
            entry.issue.is_some() || entry.pr.is_some(),
            "{scenario} names neither an issue nor a pull request, and MCP's oracle refuses it"
        );
        assert!(
            entry.reason.split_whitespace().count() >= 8,
            "{scenario} states no reason"
        );
        assert!(
            !MUST_PASS.contains(&scenario.as_str()),
            "{scenario} is a lifecycle, transport, header, DNS-rebinding, caching or schema scenario and must pass"
        );
    }
    for (scenario, reason) in &skipped {
        assert!(
            !entries.contains_key(scenario) && !reason.is_empty(),
            "{scenario} is both waiting and skipped by the suite, or has no reason"
        );
    }
    let record = read("GATEWAY-CONFORMANCE.md");
    let rows = table(&record, "The run, per scenario");
    let listed: BTreeSet<String> = rows
        .iter()
        .filter(|row| row.get(3).is_some_and(|c| c.contains("waiting")))
        .map(|row| row[1].trim_matches('`').to_string())
        .collect();
    let waiting_names: BTreeSet<String> = entries.keys().cloned().collect();
    assert_eq!(
        listed, waiting_names,
        "the record's waiting rows are the entries of gateway/waiting.json"
    );
    for row in &rows {
        let scenario = row[1].trim_matches('`');
        if let Some(entry) = entries.get(scenario) {
            let named = entry
                .issue
                .map(|n| format!("#{n}"))
                .into_iter()
                .chain(entry.pr.map(|n| format!("#{n}")))
                .any(|tag| row[4].contains(&tag));
            assert!(
                named,
                "the row of {scenario} names its issue or pull request"
            );
        } else {
            assert!(
                row[3].contains("pass") || row[3].contains("skipped"),
                "{scenario} is neither waiting, passing nor skipped by the suite"
            );
        }
    }
    for must in MUST_PASS {
        assert!(
            rows.iter()
                .any(|row| row[1].trim_matches('`') == must && row[3].contains("pass")),
            "the record shows {must} passing"
        );
    }
    for defect in ["stale", "unlisted", "served"] {
        assert!(
            record.contains(&format!("planted: {defect}")),
            "the record has the planted defect `{defect}`"
        );
    }
}

/// GW-12: the job reuses MCP's oracle at the pin, serves the gateway, and requires the ignored
/// tests' count.
#[test]
fn the_job_runs_mcps_oracle_at_the_pin_over_the_served_gateway_gw_12() {
    let workflow = read(".github/workflows/lexlean.yml");
    let library = read(".github/lexlean-ci/lib.sh");
    let job = workflow
        .split("\n  gateway:\n")
        .nth(1)
        .expect("the gateway job")
        .to_string();
    for function in [
        "gateway_conformance_suite",
        "gateway_conformance_schema",
        "gateway_conformance_ignored",
    ] {
        assert!(job.contains(function), "the gateway job runs {function}");
        assert!(
            library.contains(&format!("{function}()")),
            "lib.sh defines {function}"
        );
    }
    for needle in [
        "server-suite.mjs",
        "schema-oracle.mjs",
        "fetch-suite.sh",
        "--waiting",
        "gateway/waiting.json",
        "gateway-host",
        ".github/mcp.rev",
        "--ignored",
    ] {
        assert!(library.contains(needle), "lib.sh uses `{needle}`");
    }
    // The count of the ignored tests is read from the source, as MCP's run_oracle_tests does.
    assert!(
        library.contains("grep -c '^#\\[ignore'") && library.contains("gateway_conformance.rs"),
        "the job requires as many ignored tests to pass as the source ignores"
    );
    // MCP's oracle is run from the checkout at the pin and is not copied here.
    let mut files = Vec::new();
    walk(&root(), &mut files);
    for file in files {
        let relative = file.strip_prefix(root()).expect("inside").to_path_buf();
        let name = file
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();
        let copy = [
            "server-suite.mjs",
            "schema-oracle.mjs",
            "verdict.mjs",
            "recorder.mjs",
            "fetch-suite.sh",
        ]
        .contains(&name.as_str());
        assert!(
            !copy || relative.starts_with("gateway/conformance-mcp"),
            "{} is a copy of MCP's oracle; the job reuses the checkout at .github/mcp.rev",
            relative.display()
        );
    }
    // A test of this file that needs node or the suite is ignored, naming the job.
    let source = read("crates/model/tests/gateway_conformance.rs");
    let mut previous = String::new();
    for line in source.lines() {
        if let Some(name) = line.trim().strip_prefix("fn ") {
            let body: String = source
                .split(&format!("fn {name}"))
                .nth(1)
                .unwrap_or_default()
                .split("\n}\n")
                .next()
                .unwrap_or_default()
                .to_string();
            if body.contains("Command::new(\"node\")") {
                assert!(
                    previous.contains("#[ignore = \"")
                        && previous.contains("gateway / prismpm build and verify"),
                    "a test that runs node is ignored with the reason naming the gateway job"
                );
            }
        }
        if line.trim().starts_with("#[") {
            previous.push_str(line);
        } else if !line.trim().starts_with("///") {
            previous.clear();
        }
    }
}
