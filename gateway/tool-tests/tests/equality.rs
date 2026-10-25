//! TL-03 to TL-06: the tools equal the CLI. For every fixture of F4 and F7 and every example of
//! LexLean at the commit of `gateway/mcp/lexlean.rev`, the tool is called through the production
//! core with the real host on one copy of the project and the `lexlean` binary of the same
//! commit is run on another, and the two must agree on the exit code, the diagnostics, the
//! bytes of the result and the files of the project afterwards.
//!
//! These tests need the `lexlean` binary (`LEXLEAN_CLI`, else the PATH) and a checkout of
//! LexLean at the pin (`LEXLEAN_SOURCES`, laid out by `gateway/lay-out-lexlean.sh`), so they are
//! ignored here; the gateway job runs them with `--ignored` and requires their count.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use gateway_tool_tests::{
    copy_tree, fixtures_of, lexlean_sources, register, run_cli, tree, Cli, Fixture, Gateway, Reply,
    EXAMPLES_OFF_THE_EXPECTED_TREE, F4_FIXTURES, F7_FIXTURES, NOT_IN_THE_REGISTER,
    SNAPSHOTS_OVER_THE_LIMIT,
};
use serde_json::{json, Value};

/// A project staged for both sides: `root` holds the project `name` and, for a project with
/// source packages, the workspace file and the members it names, as the drafts of one store do.
struct Staged {
    root: PathBuf,
    name: String,
    _scratch: tempfile::TempDir,
}

/// A fixture, staged: its `project/` as the one member.
fn stage_fixture(fixture: &Fixture) -> Staged {
    let scratch = tempfile::tempdir().expect("a directory");
    let root = scratch.path().join("workspace");
    copy_tree(&fixture.project, &root.join(&fixture.name));
    let staged = Staged {
        root,
        name: fixture.name.clone(),
        _scratch: scratch,
    };
    // The locks of the fixtures name the compiler semantics of the commit that wrote them; at
    // the pin they are stale, and `check` would answer LLC0102 for each (TOOLS.md). A fixture
    // that is about a lock keeps its own.
    if !fixture
        .expected_codes
        .iter()
        .any(|code| code.starts_with("LLC01"))
    {
        relock_for_the_pin(&staged.root);
    }
    staged
}

/// Write the locks `lexlean lock` makes, for every project of the workspace `root`, when one
/// differs from the committed one in nothing but the compiler semantics it records: the project
/// is then the project of the fixture for the compiler of the pin. Returns how many were replaced.
fn relock_for_the_pin(root: &Path) -> usize {
    let scratch = tempfile::tempdir().expect("a directory");
    let copy = scratch.path().join("workspace");
    copy_tree(root, &copy);
    let mut replaced = 0;
    let projects: Vec<String> = std::fs::read_dir(root)
        .expect("a workspace")
        .flatten()
        .filter(|entry| entry.path().join("lexlean.toml").is_file())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    for name in projects {
        if run_cli(&copy.join(&name), &["lock"]).status != 0 {
            continue;
        }
        let old =
            std::fs::read_to_string(root.join(&name).join("lexlean.lock")).unwrap_or_default();
        let new =
            std::fs::read_to_string(copy.join(&name).join("lexlean.lock")).unwrap_or_default();
        let rest = |text: &str| -> Vec<String> {
            text.lines()
                .filter(|line| !line.starts_with("compiler_semantics = "))
                .map(str::to_string)
                .collect()
        };
        if old != new && rest(&old) == rest(&new) {
            std::fs::write(root.join(&name).join("lexlean.lock"), new).expect("a lock");
            replaced += 1;
        }
    }
    replaced
}

/// What the two sides gave for one project and one command.
struct Pair {
    cli: Cli,
    reply: Reply,
    /// The files of the project after `touch` and before the command.
    before: BTreeMap<String, Vec<u8>>,
    cli_files: BTreeMap<String, Vec<u8>>,
    tool_files: BTreeMap<String, Vec<u8>>,
}

/// Run the CLI in a copy of the staged project and the tool on another copy, installed as the
/// drafts of a store, both from the same bytes (`touch` changes both in the same way first).
fn run_pair(
    staged: &Staged,
    tool: &str,
    arguments: Value,
    cli_arguments: &[&str],
    touch: &dyn Fn(&Path),
) -> Pair {
    let scratch = tempfile::tempdir().expect("a directory");
    let side = scratch.path().join("workspace");
    copy_tree(&staged.root, &side);
    let project = side.join(&staged.name);
    touch(&project);
    let before = tree(&project);
    let mut gateway = Gateway::new();
    copy_tree(&staged.root, &gateway.drafts());
    touch(&gateway.draft(&staged.name));
    let cli = run_cli(&project, cli_arguments);
    let reply = gateway.call(tool, arguments);
    Pair {
        cli,
        reply,
        before,
        cli_files: tree(&project),
        tool_files: tree(&gateway.draft(&staged.name)),
    }
}

fn nothing(_: &Path) {}

/// The ways `pair` differs from the CLI, as sentences; none when the tool equals the CLI.
/// `rename` is the one change a tool makes to the command result: the command it names.
fn differences(pair: &Pair, rename: Option<(&str, &str)>) -> Vec<String> {
    let mut out = Vec::new();
    let Pair { cli, reply, .. } = pair;
    if !cli.stderr.is_empty() {
        out.push(format!(
            "the CLI wrote to stderr: {}",
            String::from_utf8_lossy(&cli.stderr)
        ));
    }
    if reply.body.get("error").is_some() {
        out.push(format!(
            "the tool answered with a protocol error: {}",
            reply.body
        ));
        return out;
    }
    let result = &reply.body["result"];
    let mut expected = String::from_utf8_lossy(&cli.stdout).into_owned();
    if let Some((from, to)) = rename {
        let from = format!("\"command\":\"{from}\"");
        assert!(
            expected.contains(&from),
            "the CLI result names its command: {expected}"
        );
        expected = expected.replacen(&from, &format!("\"command\":\"{to}\""), 1);
    }
    match result["content"][0]["text"].as_str() {
        Some(text) if text == expected => {}
        other => out.push(format!(
            "the text is {other:?}, the CLI printed {expected:?}"
        )),
    }
    let parsed: Value = serde_json::from_str(&expected).expect("the CLI printed JSON");
    if result["structuredContent"] != parsed {
        out.push(format!(
            "structuredContent is {}, the CLI printed {parsed}",
            result["structuredContent"]
        ));
    }
    if parsed["exit_code"] != json!(cli.status) {
        out.push(format!(
            "the CLI exited {} with exit_code {}",
            cli.status, parsed["exit_code"]
        ));
    }
    let flagged = result
        .get("isError")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    if flagged != (cli.status != 0) {
        out.push(format!(
            "isError is {flagged} for the exit status {}",
            cli.status
        ));
    }
    out
}

fn files_differ(pair: &Pair) -> Option<String> {
    if pair.cli_files == pair.tool_files {
        return None;
    }
    let mut paths: Vec<&String> = pair
        .cli_files
        .keys()
        .chain(pair.tool_files.keys())
        .collect();
    paths.sort();
    paths.dedup();
    let differing: Vec<&str> = paths
        .into_iter()
        .filter(|path| pair.cli_files.get(*path) != pair.tool_files.get(*path))
        .map(|path| path.as_str())
        .take(5)
        .collect();
    Some(format!(
        "the files of the project differ afterwards: {differing:?}"
    ))
}

fn finish(label: &str, ran: usize, failures: &[String]) {
    assert!(
        failures.is_empty(),
        "{label}: {} of {ran} runs differ from the CLI:\n{}",
        failures.len(),
        failures.join("\n")
    );
    eprintln!("{label}: {ran} runs, every one equal to the CLI");
}

/// The fixtures of F4 and F7, after the checks that make the count and the arguments mean what
/// the test says they do.
fn in_scope(sources: &Path) -> (Vec<Fixture>, Vec<Fixture>) {
    let f4 = fixtures_of(sources, "F4");
    let f7 = fixtures_of(sources, "F7");
    assert_eq!(f4.len(), F4_FIXTURES, "the check fixtures of F4 at the pin");
    assert_eq!(f7.len(), F7_FIXTURES, "the build fixtures of F7 at the pin");
    let rows = register();
    let in_register = f4
        .iter()
        .chain(&f7)
        .filter(|f| rows.contains_key(&f.name))
        .count();
    assert_eq!(
        in_register + NOT_IN_THE_REGISTER.len(),
        F4_FIXTURES + F7_FIXTURES,
        "every fixture of F4 and F7 is a row of the register, but the names it lacks"
    );
    for fixture in f4.iter().chain(&f7) {
        assert_eq!(
            fixture.args, "[]",
            "{}: a fixture of F4 or F7 runs its command with no argument",
            fixture.name
        );
        assert!(fixture.project.is_dir(), "{}: a project", fixture.name);
    }
    (f4, f7)
}

fn project_arguments(name: &str) -> Value {
    json!({"project": name})
}

// ---- TL-03 -----------------------------------------------------------------------------

fn expected_exit(found: &mut Vec<String>, pair: &Pair, fixture: &Fixture) {
    if pair.cli.status != fixture.expected_exit {
        found.push(format!(
            "the CLI exited {}, the fixture expects {}",
            pair.cli.status, fixture.expected_exit
        ));
    }
    let printed: Value = serde_json::from_slice(&pair.cli.stdout).unwrap_or(Value::Null);
    let codes: Vec<&str> = printed["diagnostics"]
        .as_array()
        .map(|all| all.iter().filter_map(|d| d["code"].as_str()).collect())
        .unwrap_or_default();
    if codes != fixture.expected_codes {
        found.push(format!(
            "the diagnostics are {codes:?}, the fixture expects {:?}",
            fixture.expected_codes
        ));
    }
}

#[test]
#[ignore = "needs the lexlean binary and a checkout of LexLean at the pin: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn tool_equals_cli_on_the_check_and_build_fixtures_tl_03() {
    let sources = lexlean_sources();
    let (f4, f7) = in_scope(&sources);
    let mut failures = Vec::new();
    let mut ran = 0;
    for (fixtures, command, tool) in [
        (&f4, "check", "lexlean_check"),
        (&f7, "build", "lexlean_build"),
    ] {
        for fixture in fixtures {
            let staged = stage_fixture(fixture);
            let pair = run_pair(
                &staged,
                tool,
                project_arguments(&fixture.name),
                &[command],
                &nothing,
            );
            ran += 1;
            let mut found = differences(&pair, None);
            found.extend(files_differ(&pair));
            expected_exit(&mut found, &pair, fixture);
            failures.extend(
                found
                    .into_iter()
                    .map(|d| format!("{command} {}: {d}", fixture.name)),
            );
        }
    }
    // The build tool on the check fixtures: it refuses at the same stage as the CLI does.
    for fixture in &f4 {
        let staged = stage_fixture(fixture);
        let pair = run_pair(
            &staged,
            "lexlean_build",
            project_arguments(&fixture.name),
            &["build"],
            &nothing,
        );
        ran += 1;
        let mut found = differences(&pair, None);
        found.extend(files_differ(&pair));
        failures.extend(
            found
                .into_iter()
                .map(|d| format!("build {}: {d}", fixture.name)),
        );
    }
    // The fixtures whose atoms are outside the profile give LLL1004 on both sides (AC-08).
    for name in ["unknown-word", "unknown-symbol", "unknown-control"] {
        let fixture = f4.iter().find(|f| f.name == name).expect(name);
        let staged = stage_fixture(fixture);
        let pair = run_pair(
            &staged,
            "lexlean_check",
            project_arguments(name),
            &["check"],
            &nothing,
        );
        let codes: Vec<&str> = pair.reply.body["result"]["structuredContent"]["diagnostics"]
            .as_array()
            .map(|all| all.iter().filter_map(|d| d["code"].as_str()).collect())
            .unwrap_or_default();
        if codes != ["LLL1004"] {
            failures.push(format!(
                "check {name}: the codes are {codes:?}, not LLL1004"
            ));
        }
    }
    finish("TL-03", ran, &failures);
}

// ---- TL-04 -----------------------------------------------------------------------------

/// A snapshot of a project that checks: the identities are the ones `lexlean check` prints.
fn snapshot_of_a_project_that_checks(name: &str, pair: &Pair) -> Vec<String> {
    let mut out = Vec::new();
    if pair.cli.status != 0 {
        out.push(format!(
            "snapshot {name}: the project does not check: {}",
            String::from_utf8_lossy(&pair.cli.stdout)
        ));
        return out;
    }
    let result = &pair.reply.body["result"];
    if result
        .get("isError")
        .and_then(Value::as_bool)
        .unwrap_or(false)
        || pair.reply.body.get("error").is_some()
    {
        out.push(format!(
            "snapshot {name}: the tool refused a project that checks: {}",
            pair.reply.body
        ));
        return out;
    }
    let checked: Value = serde_json::from_slice(&pair.cli.stdout).expect("JSON");
    let snapshot = &result["structuredContent"];
    for id in ["source_id", "semantic_id"] {
        if snapshot[id] != checked[id] || !snapshot[id].is_string() {
            out.push(format!(
                "snapshot {name}: {id} is {}, check printed {}",
                snapshot[id], checked[id]
            ));
        }
    }
    if !snapshot["spec"]
        .as_str()
        .is_some_and(|spec| spec.starts_with("lexlean/semantic-snapshot/"))
    {
        out.push(format!("snapshot {name}: spec is {}", snapshot["spec"]));
    }
    match result["content"][0]["text"]
        .as_str()
        .map(serde_json::from_str::<Value>)
    {
        Some(Ok(parsed)) if parsed == *snapshot => {}
        _ => out.push(format!(
            "snapshot {name}: the text block is not the structuredContent"
        )),
    }
    out.extend(files_differ(pair).map(|d| format!("snapshot {name}: {d}")));
    out
}

#[test]
#[ignore = "needs the lexlean binary and a checkout of LexLean at the pin: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn snapshot_agrees_with_check_on_the_fixtures_and_the_examples_tl_04() {
    let sources = lexlean_sources();
    let (f4, f7) = in_scope(&sources);
    let mut failures = Vec::new();
    let mut over_the_limit = Vec::new();
    let mut ran = 0;
    let mut subjects: Vec<(String, Staged)> = f4
        .iter()
        .chain(&f7)
        .map(|fixture| (fixture.name.clone(), stage_fixture(fixture)))
        .collect();
    subjects.extend(examples(&sources));
    for (name, staged) in subjects {
        // The CLI has no snapshot command: the Engine runs the pipeline of `check` (SPEC 24.1),
        // so a refused snapshot is the result of `check`, naming its own command.
        let pair = run_pair(
            &staged,
            "lexlean_snapshot",
            project_arguments(&name),
            &["check"],
            &nothing,
        );
        ran += 1;
        let mut found = Vec::new();
        if pair.cli.status != 0 {
            found.extend(differences(&pair, Some(("check", "snapshot"))));
            found.extend(files_differ(&pair));
        } else if pair.reply.body["result"]["structuredContent"]["diagnostics"][0]["message"]
            .as_str()
            .is_some_and(|message| message.starts_with("the result is "))
        {
            // The snapshot is larger than the 4 MiB an engine effect may return: class 4.
            over_the_limit.push(name.clone());
            if pair.reply.body["result"]["structuredContent"]["exit_code"] != json!(4) {
                found.push("the refusal of a snapshot over the limit is not class 4".to_string());
            }
        } else {
            found.extend(snapshot_of_a_project_that_checks(&name, &pair));
        }
        failures.extend(found.into_iter().map(|d| format!("snapshot {name}: {d}")));
    }
    eprintln!("TL-04: snapshots over the limit of the effect: {over_the_limit:?}");
    assert_eq!(
        over_the_limit, SNAPSHOTS_OVER_THE_LIMIT,
        "the projects whose snapshot is over the 4 MiB of an engine result"
    );
    finish("TL-04", ran, &failures);
}

// ---- TL-05 -----------------------------------------------------------------------------

/// Append blank lines to every source file of the entrypoints, so that `fmt` has work to do.
fn untidy(project: &Path) {
    let config = std::fs::read_to_string(project.join("lexlean.toml")).unwrap_or_default();
    for line in config.lines() {
        if let Some(list) = line.strip_prefix("entrypoints = [") {
            for entry in list.trim_end_matches(']').split(',') {
                let path = project.join(entry.trim().trim_matches('"'));
                if let Ok(mut source) = std::fs::read(&path) {
                    source.extend_from_slice(b"\n\n\n");
                    std::fs::write(&path, source).expect("a source");
                }
            }
        }
    }
}

#[test]
#[ignore = "needs the lexlean binary and a checkout of LexLean at the pin: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn format_agrees_with_fmt_on_the_fixtures_and_the_examples_tl_05() {
    let sources = lexlean_sources();
    let (f4, f7) = in_scope(&sources);
    let mut failures = Vec::new();
    let mut ran = 0;
    let modes: [(bool, &[&str]); 2] = [(true, &["fmt", "--check"]), (false, &["fmt"])];
    for fixture in f4.iter().chain(&f7) {
        let staged = stage_fixture(fixture);
        for (check_only, cli_arguments) in modes {
            let pair = run_pair(
                &staged,
                "lexlean_format",
                json!({"project": fixture.name, "check_only": check_only}),
                cli_arguments,
                &nothing,
            );
            ran += 1;
            let mut found = differences(&pair, None);
            found.extend(files_differ(&pair));
            failures.extend(
                found
                    .into_iter()
                    .map(|d| format!("format {} (check_only {check_only}): {d}", fixture.name)),
            );
        }
    }
    let mut rewritten = 0;
    for (name, staged) in examples(&sources) {
        for (check_only, cli_arguments) in modes {
            // As they are, the examples are canonical; with blank lines added they are not, and
            // `fmt` rewrites them (or, with `--check`, refuses them).
            for (tidy, touch) in [(true, &nothing as &dyn Fn(&Path)), (false, &untidy)] {
                let pair = run_pair(
                    &staged,
                    "lexlean_format",
                    json!({"project": name, "check_only": check_only}),
                    cli_arguments,
                    touch,
                );
                ran += 1;
                if !tidy && !check_only && pair.cli_files != pair.before {
                    rewritten += 1;
                }
                let mut found = differences(&pair, None);
                found.extend(files_differ(&pair));
                failures.extend(
                    found.into_iter().map(|d| {
                        format!("format {name} (check_only {check_only}, tidy {tidy}): {d}")
                    }),
                );
            }
        }
    }
    assert!(
        rewritten > 0,
        "at least one example was rewritten by fmt, so the comparison of the rewritten bytes means something"
    );
    finish("TL-05", ran, &failures);
}

// ---- TL-06 -----------------------------------------------------------------------------

/// The `path = "..."` lines of the `[[source_package]]` tables of a project.
fn source_package_paths(project: &Path) -> Vec<String> {
    let config = std::fs::read_to_string(project.join("lexlean.toml")).unwrap_or_default();
    let mut paths = Vec::new();
    let mut in_package = false;
    for line in config.lines() {
        if line.starts_with('[') {
            in_package = line.trim() == "[[source_package]]";
        } else if in_package {
            if let Some(path) = line.strip_prefix("path = ") {
                paths.push(path.trim_matches('"').to_string());
            }
        }
    }
    paths
}

/// Every example of LexLean, staged: a project with source packages gets the workspace that
/// holds it and the members it names, as the drafts of one store would.
fn examples(sources: &Path) -> Vec<(String, Staged)> {
    let mut names: Vec<String> = std::fs::read_dir(sources.join("examples"))
        .expect("examples")
        .flatten()
        .filter(|entry| entry.path().join("lexlean.toml").is_file())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let scratch = tempfile::tempdir().expect("a directory");
            let root = scratch.path().join("workspace");
            std::fs::create_dir_all(&root).expect("a directory");
            // A member is a project of the workspace: its directory without `expected/`.
            let copy_member = |from: &Path, to: &Path| {
                copy_tree(from, to);
                let _ = std::fs::remove_dir_all(to.join("expected"));
            };
            let example = sources.join("examples").join(&name);
            copy_member(&example, &root.join(&name));
            let mut members = vec![name.clone()];
            let mut pending = source_package_paths(&example);
            while let Some(path) = pending.pop() {
                if members.contains(&path) {
                    continue;
                }
                let member = sources.join(&path);
                pending.extend(source_package_paths(&member));
                copy_member(&member, &root.join(&path));
                members.push(path);
            }
            if members.len() > 1 {
                members.sort();
                let list: Vec<String> = members.iter().map(|m| format!("\"{m}\"")).collect();
                std::fs::write(
                    root.join("lexlean-workspace.toml"),
                    format!(
                        "spec = \"lexlean/workspace/1\"\nmembers = [{}]\n",
                        list.join(", ")
                    ),
                )
                .expect("a workspace file");
            }
            relock_for_the_pin(&root);
            (
                name.clone(),
                Staged {
                    root,
                    name,
                    _scratch: scratch,
                },
            )
        })
        .collect()
}

/// The files a build published: `.lexlean/build/<id>/`, relative to it.
fn published(files: &BTreeMap<String, Vec<u8>>, id: &str) -> BTreeMap<String, Vec<u8>> {
    let head = format!(".lexlean/build/{id}/");
    files
        .iter()
        .filter_map(|(path, bytes)| {
            path.strip_prefix(&head)
                .map(|rest| (rest.to_string(), bytes.clone()))
        })
        .collect()
}

/// How the tree a build published compares with the `expected/build` of the example.
fn against_expected(
    files: &BTreeMap<String, Vec<u8>>,
    command_result: &Value,
    expected: &BTreeMap<String, Vec<u8>>,
) -> Result<(), String> {
    let Some(id) = command_result["build_id"].as_str() else {
        return Err(format!(
            "the build published nothing: {}",
            command_result["diagnostics"][0]["code"]
        ));
    };
    let built = published(files, id);
    let missing: Vec<&String> = expected
        .keys()
        .filter(|p| !built.contains_key(*p))
        .collect();
    let extra: Vec<&String> = built
        .keys()
        .filter(|p| !expected.contains_key(*p))
        .collect();
    let wrong: Vec<&String> = expected
        .iter()
        .filter(|(path, bytes)| built.get(*path).is_some_and(|got| got != *bytes))
        .map(|(path, _)| path)
        .collect();
    if missing.is_empty() && extra.is_empty() && wrong.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "missing {missing:?}, extra {extra:?}, different {wrong:?}"
        ))
    }
}

#[test]
#[ignore = "needs the lexlean binary and a checkout of LexLean at the pin: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn every_example_builds_to_its_expected_tree_tl_06() {
    let sources = lexlean_sources();
    let mut failures = Vec::new();
    let all = examples(&sources);
    assert!(all.len() >= 18, "the examples of LexLean: {}", all.len());
    let mut off_the_tree = Vec::new();
    for (name, staged) in &all {
        let pair = run_pair(
            staged,
            "lexlean_build",
            project_arguments(name),
            &["build"],
            &nothing,
        );
        let mut found = differences(&pair, None);
        found.extend(files_differ(&pair));
        let expected = tree(&sources.join("examples").join(name).join("expected/build"));
        let cli_result: Value = serde_json::from_slice(&pair.cli.stdout).expect("JSON");
        let cli = against_expected(&pair.cli_files, &cli_result, &expected);
        let tool = against_expected(
            &pair.tool_files,
            &pair.reply.body["result"]["structuredContent"],
            &expected,
        );
        // The tool publishes the tree the CLI publishes; where that is not the expected one,
        // the CLI of the same commit is not either, and the example is named below.
        if cli != tool {
            found.push(format!("the tool: {tool:?}; the CLI: {cli:?}"));
        }
        if let Err(reason) = cli {
            eprintln!("build {name}: not the expected tree on either side: {reason}");
            off_the_tree.push(name.clone());
        }
        failures.extend(found.into_iter().map(|d| format!("build {name}: {d}")));
    }
    eprintln!("TL-06: off the expected tree at the pin: {off_the_tree:?}");
    assert_eq!(
        off_the_tree, EXAMPLES_OFF_THE_EXPECTED_TREE,
        "the examples whose expected/build the CLI at the pin does not reproduce"
    );
    finish("TL-06", all.len(), &failures);
}

// ---- TL-13 -----------------------------------------------------------------------------

/// Q8, the running clause: every diagnostic code that a tool answers on the fixtures of F4 and
/// F7 resolves through the running `lexlean_explain` to the entry `lexlean explain` prints.
#[test]
#[ignore = "needs the lexlean binary and a checkout of LexLean at the pin: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn every_code_in_a_tool_result_resolves_through_explain_tl_13() {
    let sources = lexlean_sources();
    let (f4, f7) = in_scope(&sources);
    let mut seen: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    let mut expected: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for (fixtures, tool) in [(&f4, "lexlean_check"), (&f7, "lexlean_build")] {
        for fixture in fixtures {
            let staged = stage_fixture(fixture);
            let mut gateway = Gateway::new();
            copy_tree(&staged.root, &gateway.drafts());
            let reply = gateway.call(tool, project_arguments(&fixture.name));
            let diagnostics = reply.body["result"]["structuredContent"]["diagnostics"]
                .as_array()
                .unwrap_or_else(|| panic!("{}: a command result: {}", fixture.name, reply.body));
            for diagnostic in diagnostics {
                seen.insert(diagnostic["code"].as_str().expect("a code").to_string());
            }
            expected.extend(fixture.expected_codes.iter().cloned());
        }
    }
    assert!(!seen.is_empty(), "the fixtures return diagnostics");
    assert_eq!(
        seen, expected,
        "the codes in the results of the tools are the codes the fixtures expect"
    );
    let nowhere = tempfile::tempdir().expect("a directory with no project");
    let mut gateway = Gateway::new();
    let mut failures = Vec::new();
    for code in &seen {
        let cli = run_cli(nowhere.path(), &["explain", code]);
        let reply = gateway.call("lexlean_explain", json!({"code": code}));
        let result = &reply.body["result"];
        let printed = String::from_utf8_lossy(&cli.stdout).into_owned();
        if cli.status != 0
            || result.get("isError").is_some()
            || result["content"][0]["text"].as_str() != Some(printed.as_str())
            || !result["structuredContent"]["explanation"]
                .as_str()
                .is_some_and(|entry| entry.starts_with(&format!("## `{code}` --- ")))
        {
            failures.push(format!(
                "{code}: {} against the CLI's {printed}",
                reply.body
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} codes do not resolve through lexlean_explain as they do through the CLI:\n{}",
        failures.len(),
        seen.len(),
        failures.join("\n")
    );
    eprintln!(
        "TL-13: {} distinct codes in the results of the tools, every one resolved",
        seen.len()
    );
}
