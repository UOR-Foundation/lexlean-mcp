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

/// The scenarios the gateway passes whatever its catalogue is: they exercise the served
/// transport (the endpoint, the method and the headers, DNS rebinding, the event-stream
/// negotiation, the typed refusal of an unsupported method or an invalid argument) and the
/// `tools/list` of the gateway's own catalogue. A waiting list that names one of them would
/// hide a defect in the gateway's serving.
const MUST_PASS: [&str; 7] = [
    "tools-list",
    "dns-rebinding-protection",
    "http-header-validation",
    "server-sse-multiple-streams",
    "sep-2164-resource-not-found",
    "input-required-result-unsupported-methods",
    "input-required-result-validate-input",
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
        if inside && !rows.is_empty() && !line.starts_with('|') {
            break;
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
    assert!(
        !job.contains("continue-on-error"),
        "no step of the gateway job may fail without failing the job"
    );
    // The count of the ignored tests is read from the source, as MCP's run_oracle_tests does.
    assert!(
        library.contains("grep -c '^#\\[ignore'") && library.contains("gateway_conformance.rs"),
        "the job requires as many ignored tests to pass as the source ignores"
    );
    // MCP's oracle is run from the checkout at the pin and is not copied here.
    let mut files = Vec::new();
    walk(&root(), &mut files);
    for file in files {
        let name = file
            .file_name()
            .expect("name")
            .to_string_lossy()
            .to_string();
        assert!(
            ![
                "server-suite.mjs",
                "schema-oracle.mjs",
                "verdict.mjs",
                "recorder.mjs",
                "fetch-suite.sh",
            ]
            .contains(&name.as_str()),
            "{} is a copy of MCP's oracle; the job reuses the checkout at .github/mcp.rev",
            file.strip_prefix(root()).expect("inside").display()
        );
    }
    // A test of this file that needs node or the installed suite is ignored, naming the job,
    // so that the workspace gate (no node, no suite) neither runs nor fails it.
    let source = read("crates/model/tests/gateway_conformance.rs");
    let mut ignored = 0;
    for block in source.split("#[test]\n").skip(1) {
        let needs_suite = block.contains(concat!("Command::", "new(\"node\")"))
            || block.contains(concat!("MCP_CONFORMANCE", "_SUITE_ROOT"))
            || block.contains(concat!("GATEWAY_MCP", "_CHECKOUT"));
        let header = block.lines().next().unwrap_or_default();
        if needs_suite {
            assert!(
                header.starts_with("#[ignore = \"")
                    && header.contains("gateway / prismpm build and verify"),
                "a test that needs node or the suite is ignored with a reason naming the gateway job"
            );
            ignored += 1;
        }
    }
    assert_eq!(
        ignored,
        source.matches("\n#[ignore").count(),
        "every ignored test of this file is one that needs node or the suite, and the job counts them"
    );
}

/// The scenarios of the frozen requirements of the pinned suite for the server leg: the
/// required list, and the entries of `not_scored` that name the server leg.
fn frozen_scenarios(suite: &Path) -> (Vec<String>, Vec<String>) {
    let text = std::fs::read_to_string(suite.join("requirements/2026-07-28.yaml"))
        .expect("the suite's frozen requirements");
    let mut section = "";
    let mut required = Vec::new();
    let mut unscored = Vec::new();
    let mut pending: Option<String> = None;
    for line in text.lines() {
        if !line.starts_with(' ') && !line.starts_with('#') && line.ends_with(':') {
            section = if line == "server:" {
                "server"
            } else if line == "not_scored:" {
                "not_scored"
            } else {
                "other"
            };
            continue;
        }
        let item = line.trim();
        match section {
            "server" => {
                if let Some(name) = item.strip_prefix("- ") {
                    required.push(name.trim().to_string());
                }
            }
            "not_scored" => {
                if let Some(name) = item.strip_prefix("- scenario: ") {
                    pending = Some(name.trim().to_string());
                } else if item == "leg: server" {
                    unscored.extend(pending.take());
                }
            }
            _ => {}
        }
    }
    (required, unscored)
}

/// GW-12: every name of the list is a scenario of the suite's frozen server requirements.
#[test]
#[ignore = "needs the installed pinned suite (MCP_CONFORMANCE_SUITE_ROOT): the gateway job of .github/workflows/lexlean.yml, 'gateway / prismpm build and verify', runs it with --ignored and requires the count"]
fn the_waiting_list_names_scenarios_of_the_frozen_requirements_gw_12() {
    let suite = PathBuf::from(
        std::env::var("MCP_CONFORMANCE_SUITE_ROOT")
            .expect("MCP_CONFORMANCE_SUITE_ROOT names the installed pinned suite"),
    );
    let (required, unscored) = frozen_scenarios(&suite);
    assert!(
        required.len() >= 37 && unscored.len() >= 13,
        "the frozen requirements were read: {} required, {} not scored",
        required.len(),
        unscored.len()
    );
    let known: BTreeSet<String> = required.iter().chain(&unscored).cloned().collect();
    let (entries, skipped) = waiting();
    for name in entries
        .keys()
        .chain(skipped.keys())
        .map(String::as_str)
        .chain(MUST_PASS)
    {
        assert!(
            known.contains(name),
            "{name} is not a server scenario of the frozen requirements"
        );
    }
}

/// GW-12: MCP's own verdict, over the real list, rejects a stale entry, an unlisted failure
/// and an entry without an issue, and accepts exactly the list.
#[test]
#[ignore = "needs node and the MCP checkout at the pin (GATEWAY_MCP_CHECKOUT) and the installed suite: the gateway job of .github/workflows/lexlean.yml, 'gateway / prismpm build and verify', runs it with --ignored and requires the count"]
fn the_verdict_of_mcps_oracle_rejects_a_stale_entry_and_an_unlisted_failure_gw_12() {
    use std::io::Write;
    let checkout = PathBuf::from(
        std::env::var("GATEWAY_MCP_CHECKOUT").expect("GATEWAY_MCP_CHECKOUT names the MCP checkout"),
    );
    let suite = PathBuf::from(
        std::env::var("MCP_CONFORMANCE_SUITE_ROOT").expect("MCP_CONFORMANCE_SUITE_ROOT"),
    );
    let (required, unscored) = frozen_scenarios(&suite);
    let list: serde_json::Value =
        serde_json::from_str(&read("gateway/waiting.json")).expect("gateway/waiting.json");
    let input = serde_json::json!({
        "required": required,
        "unscored": unscored,
        "waiting": list["waiting"],
        "suiteSkipped": list["suite_skipped"],
        "passing": "tools-list",
        "listed": "caching",
    });
    let program = r#"
import { readFileSync } from 'node:fs';
const { verdict } = await import(process.argv[1]);
const input = JSON.parse(readFileSync(0, 'utf8'));
const run = (waiting, fails) => {
  const results = new Map();
  for (const scenario of [...input.required, ...input.unscored]) {
    const skipped = Object.hasOwn(input.suiteSkipped, scenario);
    results.set(scenario, [{ id: scenario, status: skipped ? 'SKIPPED' : fails(scenario) ? 'FAILURE' : 'SUCCESS' }]);
  }
  const v = verdict({ required: input.required, unscored: input.unscored, results, suiteSkipped: input.suiteSkipped, waiting });
  return { matches: v.matchesWaiting, stale: v.stale, unexplained: v.unexplained, unnamed: v.unnamed };
};
const listed = (scenario) => Object.hasOwn(input.waiting, scenario);
const out = {
  exact: run(input.waiting, listed),
  // A scenario that passes is named in the list.
  stale: run({ ...input.waiting, [input.passing]: { issue: 1, reason: 'x' } }, listed),
  // A listed scenario passes.
  delivered: run(input.waiting, (s) => listed(s) && s !== input.listed),
  // A scenario that is not listed fails.
  unlisted: run(input.waiting, (s) => listed(s) || s === input.passing),
  // An entry names neither an issue nor a pull request.
  unnamed: run({ ...input.waiting, [input.listed]: { reason: 'no issue' } }, listed),
};
process.stdout.write(JSON.stringify(out));
"#;
    let mut child = std::process::Command::new("node")
        .args(["--input-type=module", "-e", program])
        .arg(checkout.join("conformance/oracle/verdict.mjs"))
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("node starts");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(input.to_string().as_bytes())
        .expect("the input is written");
    let output = child.wait_with_output().expect("node finishes");
    assert!(
        output.status.success(),
        "node failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let out: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON from node");
    assert_eq!(
        out["exact"]["matches"], true,
        "the list is the failing set it describes: {out}"
    );
    let names = |case: &str, field: &str| out[case][field].clone();
    assert_eq!(out["stale"]["matches"], false, "{out}");
    assert_eq!(
        names("stale", "stale"),
        serde_json::json!(["tools-list"]),
        "{out}"
    );
    assert_eq!(out["delivered"]["matches"], false, "{out}");
    assert_eq!(
        names("delivered", "stale"),
        serde_json::json!(["caching"]),
        "{out}"
    );
    assert_eq!(out["unlisted"]["matches"], false, "{out}");
    assert_eq!(
        names("unlisted", "unexplained"),
        serde_json::json!(["tools-list"]),
        "{out}"
    );
    assert_eq!(out["unnamed"]["matches"], false, "{out}");
    assert_eq!(
        names("unnamed", "unnamed"),
        serde_json::json!(["caching"]),
        "{out}"
    );
}
