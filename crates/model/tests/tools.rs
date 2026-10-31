//! TL-01 to TL-09: the tools of F4 to F7 (issue #10), check, snapshot, format and build, agree
//! with the CLI. TL-10 to TL-14: so does `lexlean_explain` (F11).
//!
//! The behavior is tested where the core and the host run: `gateway/tool-tests` calls each tool
//! through the production core with the real host, beside the `lexlean` binary of the same
//! commit, and the gateway job runs it. These tests read the sources as text and hold them
//! against each other: the schemas of the tool layer against the declared ones, the register of
//! the fixtures against the counts of the tests, the tests against the rows they answer, and the
//! workflow against the tests it must run. Nothing here runs LexLean.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn walk(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| {
                ["target", ".prism", ".lexlean", "registry"]
                    .iter()
                    .any(|skipped| name == *skipped)
            }) {
                continue;
            }
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// The names of the `#[test]` functions of the Rust files beneath `relative`, with the
/// `#[ignore = "..."]` reason of each (empty when the test runs everywhere).
fn tests_in(relative: &str) -> Vec<(String, String)> {
    let mut files = Vec::new();
    walk(&root().join(relative), &mut files);
    let mut found = Vec::new();
    for file in files
        .iter()
        .filter(|file| file.extension().is_some_and(|e| e == "rs"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        let mut in_test = false;
        let mut reason = String::new();
        for line in text.lines() {
            let line = line.trim();
            if line == "#[test]" {
                in_test = true;
                reason.clear();
            } else if in_test && line.starts_with("#[ignore") {
                reason = line.to_string();
            } else if in_test && line.starts_with("fn ") {
                let name = line[3..].split('(').next().expect("a name").to_string();
                found.push((name, reason.clone()));
                in_test = false;
            } else if in_test && !line.starts_with("#[") && !line.starts_with("///") {
                in_test = false;
            }
        }
    }
    found
}

fn names_for(id: &str) -> BTreeSet<String> {
    let suffix = id.to_lowercase().replace('-', "_");
    tests_in("gateway/tool-tests")
        .into_iter()
        .map(|(name, _)| name)
        .filter(|name| name.ends_with(&suffix))
        .collect()
}

fn ignored_in_tool_tests() -> usize {
    tests_in("gateway/tool-tests")
        .into_iter()
        .filter(|(_, reason)| !reason.is_empty())
        .count()
}

fn tight(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The rows of the markdown table that follows the heading `heading` of `document`.
fn table(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut inside = false;
    let mut seen_table = false;
    for line in document.lines() {
        if line.starts_with('#') {
            if inside && seen_table {
                break;
            }
            inside = line.trim_start_matches('#').trim() == heading;
            continue;
        }
        if inside && line.starts_with('|') {
            seen_table = true;
            let cells: Vec<String> = line
                .trim()
                .trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().to_string())
                .collect();
            if cells
                .iter()
                .all(|c| c.chars().all(|ch| ch == '-' || ch == ' '))
            {
                continue;
            }
            rows.push(cells);
        } else if inside && seen_table {
            break;
        }
    }
    rows.into_iter().skip(1).collect()
}

fn needs(id: &str, tests: &[&str]) {
    let names = names_for(id);
    for test in tests {
        assert!(
            names.contains(*test),
            "gateway/tool-tests has the test {test} for {id}: {names:?}"
        );
    }
}

/// The step of the gateway job that runs the tests of the tools with their count.
fn workflow() -> String {
    format!(
        "{}\n{}",
        read(".github/workflows/lexlean.yml"),
        read(".github/lexlean-ci/lib.sh")
    )
}

/// TL-01: one schema each, an output schema each, and the arguments that break them.
#[test]
fn the_four_tools_declare_their_schemas_tl_01() {
    let source = tight(&read("gateway/src/GatewayTools.lex.tex"));
    for tool in [
        "lexlean_check",
        "lexlean_snapshot",
        "lexlean_format",
        "lexlean_build",
    ] {
        assert!(
            source.contains(&format!("\"{tool}\"")),
            "the tool layer names {tool}"
        );
    }
    for member in ["\"project\"", "\"files\"", "\"all\"", "\"check_only\""] {
        assert!(
            source.contains(member),
            "the schemas have the member {member}"
        );
    }
    assert!(
        source.contains("\"lexlean/command-result/1\""),
        "the output schema names the command result"
    );
    assert!(
        source.matches("outputSchema := (ctor Option.some").count() >= 1,
        "the engine tools declare an outputSchema"
    );
    needs(
        "TL-01",
        &[
            "tools_list_shows_the_four_tools_with_their_schemas_tl_01",
            "a_call_that_breaks_a_schema_is_refused_before_the_engine_tl_01",
            "all_together_with_files_is_invalid_arguments_tl_01",
        ],
    );
}

/// TL-02: an answer of the Engine is a tool result, and what is not one is refused with a type.
#[test]
fn an_answer_of_the_engine_is_a_tool_result_tl_02() {
    let source = tight(&read("gateway/src/GatewayTools.lex.tex"));
    assert!(
        source.contains("JsonRead::readJson"),
        "the core reads the result bytes with the reader of MCP, not a reader of its own"
    );
    assert!(
        source.contains("\"exit_code\""),
        "the core holds the exit code of the command result to the class"
    );
    needs(
        "TL-02",
        &[
            "class_zero_is_a_result_without_is_error_tl_02",
            "classes_one_two_and_four_are_error_results_with_the_command_result_tl_02",
            "a_host_failure_is_an_error_result_with_its_failure_tl_02",
            "bytes_that_are_not_a_command_result_are_refused_by_the_core_tl_02",
        ],
    );
}

/// The F4 and F7 rows of the register, and the names the tests add to them.
fn register_counts() -> (usize, usize, usize) {
    let register = read("crates/negative-fixtures/register.toml");
    let classes: Vec<&str> = register
        .lines()
        .filter_map(|line| line.strip_prefix("class = "))
        .map(|class| class.trim_matches('"'))
        .collect();
    let count = |class: &str| classes.iter().filter(|c| **c == class).count();
    (count("F4"), count("F7"), classes.len())
}

fn constant(source: &str, name: &str) -> usize {
    source
        .lines()
        .find_map(|line| {
            line.trim()
                .strip_prefix(&format!("pub const {name}: usize = "))
        })
        .unwrap_or_else(|| panic!("a constant {name}"))
        .trim_end_matches(';')
        .parse()
        .expect("a number")
}

/// TL-03: the fixtures the equality test runs are the register's F4 and F7 rows and the names it
/// lacks, and the test runs the CLI and the tool.
#[test]
fn the_check_and_build_fixtures_are_the_registers_and_the_tool_is_run_beside_the_cli_tl_03() {
    let (f4, f7, total) = register_counts();
    assert_eq!(
        (f4, f7, total),
        (252, 1, 298),
        "the register is the one of FX-06"
    );
    let library = read("gateway/tool-tests/src/lib.rs");
    let added = library
        .lines()
        .find(|line| line.starts_with("pub const NOT_IN_THE_REGISTER"))
        .expect("the names the register lacks")
        .matches('"')
        .count()
        / 2;
    assert_eq!(
        constant(&library, "F4_FIXTURES"),
        f4 + added,
        "F4: the register's rows and the names it lacks"
    );
    assert_eq!(constant(&library, "F7_FIXTURES"), f7);
    let test = read("gateway/tool-tests/tests/equality.rs");
    assert!(
        test.contains("run_cli(") && test.contains("gateway.call("),
        "the equality test runs the CLI and calls the tool"
    );
    for compared in ["exit_code", "structuredContent", "isError", "files_differ"] {
        assert!(
            test.contains(compared),
            "the equality test compares {compared}"
        );
    }
    needs(
        "TL-03",
        &["tool_equals_cli_on_the_check_and_build_fixtures_tl_03"],
    );
}

/// TL-04 to TL-06: the tests exist and are ignored only for the reason the job answers.
#[test]
fn snapshot_format_and_build_are_compared_with_the_cli_tl_04() {
    needs(
        "TL-04",
        &["snapshot_agrees_with_check_on_the_fixtures_and_the_examples_tl_04"],
    );
    let test = read("gateway/tool-tests/tests/equality.rs");
    assert!(
        test.contains("Some((\"check\", \"snapshot\"))"),
        "a refused snapshot is compared with the result of check, naming its own command"
    );
    let binding = tests_in("gateway/bindings/gateway-binding");
    assert!(
        binding.iter().any(|(name, _)| {
            name == "engine_snapshot_returns_the_canonical_bytes_of_the_snapshot"
        }),
        "the binding test that holds the result of E4 to the canonical bytes of the Engine"
    );
}

#[test]
fn format_is_compared_with_fmt_tl_05() {
    needs(
        "TL-05",
        &["format_agrees_with_fmt_on_the_fixtures_and_the_examples_tl_05"],
    );
    let test = read("gateway/tool-tests/tests/equality.rs");
    assert!(
        test.contains("\"fmt\", \"--check\"") && test.contains("&[\"fmt\"]"),
        "both modes of fmt are run"
    );
}

#[test]
fn every_example_is_built_and_compared_with_its_expected_tree_tl_06() {
    needs(
        "TL-06",
        &["every_example_builds_to_its_expected_tree_tl_06"],
    );
    let test = read("gateway/tool-tests/tests/equality.rs");
    assert!(
        test.contains("expected/build"),
        "the build is compared with the expected tree"
    );
}

/// TL-07: the atom outside the profile.
#[test]
fn an_atom_outside_the_profile_is_lll1004_with_its_span_tl_07() {
    needs(
        "TL-07",
        &["an_atom_outside_the_profile_gives_lll1004_with_its_span_tl_07"],
    );
    let project = root().join("gateway/tool-tests/fixture/outside-atom");
    assert!(project.join("lexlean.toml").is_file());
    let module = std::fs::read_to_string(project.join("src/Main.lex.tex")).expect("a module");
    assert!(module.contains("banana"), "the atom is outside the profile");
}

/// TL-08: the instrument and the control.
#[test]
fn no_child_process_is_started_by_a_tool_tl_08() {
    needs(
        "TL-08",
        &["no_child_process_is_started_by_the_four_tools_tl_08"],
    );
    let test = read("gateway/tool-tests/tests/q4.rs");
    for needle in [
        "/proc/self/stat",
        "/proc/self/task",
        "Command::new(\"true\")",
    ] {
        assert!(test.contains(needle), "the Q4 test uses {needle}");
    }
    assert_eq!(
        test.matches("#[test]").count(),
        1,
        "the Q4 binary holds one test, so that no other test spawns beside the tools"
    );
}

/// TL-03 to TL-06 run in the gateway job with --ignored, and the job counts them.
#[test]
fn the_gateway_job_runs_the_ignored_tests_of_the_tools_and_counts_them_tl_03() {
    let ignored = ignored_in_tool_tests();
    assert_eq!(
        ignored, 11,
        "TL-03 to TL-06, TL-12 and TL-13, the two Atlas tests of VO-14, the two of TH-11 and VP-07, which a job of its own runs, are the ignored tests of the tools"
    );
    for (name, reason) in tests_in("gateway/tool-tests") {
        if !reason.is_empty() {
            assert!(
                reason.contains("lexlean.yml") && reason.contains("--ignored"),
                "{name}: an ignored test names the job that runs it: {reason}"
            );
        }
    }
    let workflow = workflow();
    // The verify fixtures have a job of their own, which requires that the test ran (VP-07).
    let in_the_job = ignored - 1;
    assert!(
        workflow.contains(&format!(
            "gateway_ignored_tests gateway-tool-tests {in_the_job} "
        )),
        "the gateway job runs {in_the_job} ignored tests of gateway-tool-tests and requires the count"
    );
    assert!(
        workflow.contains("gateway/lay-out-lexlean.sh") && workflow.contains("LEXLEAN_SOURCES"),
        "the job lays out LexLean at the pin and names it to the tests"
    );
    let script = read("gateway/lay-out-lexlean.sh");
    assert!(
        script.contains("mcp/lexlean.rev") && script.contains("rev-parse HEAD"),
        "the layout is the commit of gateway/mcp/lexlean.rev and nothing else"
    );
}

/// TL-09: the planted defects are recorded, with the command, the diagnostic and the restore.
#[test]
fn the_planted_defects_of_the_tools_are_recorded_tl_09() {
    let document = read("TOOLS.md");
    let defects = table(&document, "Planted defects");
    assert!(defects.len() >= 3, "three defects or more are recorded");
    for cells in &defects {
        assert!(
            cells.len() >= 4 && cells.iter().all(|cell| !cell.is_empty()),
            "a defect names its change, command, diagnostic and restore: {cells:?}"
        );
    }
    let planted: String = defects
        .iter()
        .map(|cells| cells[0].to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    for kind in ["exit code", "diagnostic", "format"] {
        assert!(planted.contains(kind), "a planted defect is about: {kind}");
    }
}

// ---- F11: lexlean_explain ---------------------------------------------------------------

/// The text of the module `Explain` and of the generator that renders it.
fn explanations() -> String {
    read("gateway/src/Explain.lex.tex")
}

/// TL-10: the tool is declared, answers from the core, and asks the host for nothing.
#[test]
fn lexlean_explain_is_a_pure_function_of_the_code_tl_10() {
    let tools = tight(&read("gateway/src/GatewayTools.lex.tex"));
    assert!(
        tools.contains("\"lexlean_explain\""),
        "the tool layer names lexlean_explain"
    );
    assert!(
        tools.contains("Explain::entry"),
        "the tool layer answers from the compiled explanations"
    );
    // No effect carries a code: the set is the model's E1 to E10, and none is about explain.
    let effects = read("src/Effects.lex.tex");
    assert!(
        !effects.to_lowercase().contains("explain"),
        "src/Effects.lex.tex declares no effect for an explanation"
    );
    assert_eq!(
        effects,
        read("gateway/src/Effects.lex.tex"),
        "the gateway's declaration of the effects is the model's, byte for byte"
    );
    // The plan of the tool layer, the function that maps a tool to its effect, has no case for it.
    let plan = tools
        .split("name: plan,")
        .nth(1)
        .and_then(|rest| rest.split("name: nth,").next())
        .expect("the plan of the tool layer");
    assert!(
        !plan.contains("lexlean_explain"),
        "lexlean_explain has no effect to plan"
    );
    // Two recorded vectors show it: a registered code and one the registry lacks ask for no effect.
    let vectors = read("gateway/vectors/exchanges.json");
    assert_eq!(
        vectors.matches("\"kind\": \"explain\"").count(),
        2,
        "the vectors hold an explanation of a registered code and of an unregistered one"
    );
    needs(
        "TL-10",
        &[
            "lexlean_explain_is_listed_with_its_schemas_tl_10",
            "lexlean_explain_asks_the_host_for_no_effect_tl_10",
            "a_call_to_lexlean_explain_that_breaks_the_schema_is_refused_tl_10",
        ],
    );
}

/// TL-11: the explanations are the rendering of the registry, and nothing else.
#[test]
fn the_explanations_are_the_rendering_of_the_registry_tl_11() {
    let module = explanations();
    let entries: Vec<&str> = module
        .lines()
        .flat_map(|line| line.split("(cons (\"LL").skip(1))
        .collect();
    assert!(
        entries.len() >= 80,
        "the module holds the entries of the registry: {}",
        entries.len()
    );
    let mut previous = String::new();
    for entry in &entries {
        let code = format!("LL{}", &entry[..5]);
        assert!(
            code.len() == 7 && code[3..].chars().all(|c| c.is_ascii_digit()),
            "{code}: a code"
        );
        assert!(
            code > previous,
            "{code}: sorted and unique, after {previous}"
        );
        previous = code.clone();
        assert!(
            entry.contains(&format!("## `{code}` --- ")),
            "{code}: the entry begins with its heading"
        );
        assert!(
            entry.contains("\\n\\nClass: `") && entry.contains(". Exit code: "),
            "{code}: the entry ends with its class and exit code"
        );
    }
    // The generator is the only author of the module; the tool layer has no entry of its own.
    let generator = read("gateway/explain/generate.py");
    assert!(
        generator.contains("model\" / \"errors.toml\"") && generator.contains("def entry("),
        "the generator reads the registry of LexLean and renders each row"
    );
    let tools = read("gateway/src/GatewayTools.lex.tex");
    assert!(
        !tools.contains("## `"),
        "the tool layer holds no explanation text"
    );
    // The gateway job renders the module again from the registry at the pin and compares.
    let workflow = workflow();
    assert!(
        workflow.contains("gateway/explain/generate.py check"),
        "the gateway job fails when the committed module is not the rendering of the registry"
    );
    let script = read("gateway/lay-out-lexlean.sh");
    assert!(
        script.contains("model/errors.toml"),
        "the layout of LexLean at the pin holds the registry"
    );
    assert!(
        read("gateway/lexlean.toml").contains("mcp/src"),
        "the gateway is compiled from its own sources and MCP's"
    );
    assert!(
        read("gateway/language/lexleanmcp.titles/entries/lexlean-mcp-diagnostic-explanations.toml")
            .contains("module = \"Explain\""),
        "the module has its title in the lexicon"
    );
}

/// TL-12: the tool is held to the CLI for every code of the registry and for strings it lacks.
#[test]
fn lexlean_explain_is_compared_with_the_cli_tl_12() {
    needs(
        "TL-12",
        &["tool_equals_cli_on_every_registered_code_and_on_unregistered_strings_tl_12"],
    );
    let test = read("gateway/tool-tests/tests/explain.rs");
    for compared in [
        "model/errors.toml",
        "\"explain\", \"--\"",
        "structuredContent",
        "isError",
        "exit_code",
        "REGISTERED_CODES",
    ] {
        assert!(test.contains(compared), "the explain test uses {compared}");
    }
    assert!(
        read("gateway/tool-tests/src/lib.rs").contains("pub const REGISTERED_CODES: usize = 86;"),
        "the count of the registry at the pin is asserted"
    );
}

/// TL-13: the codes of the results of the tools resolve through the running tool.
#[test]
fn the_codes_of_the_results_resolve_through_lexlean_explain_tl_13() {
    needs(
        "TL-13",
        &["every_code_in_a_tool_result_resolves_through_explain_tl_13"],
    );
    let test = read("gateway/tool-tests/tests/equality.rs");
    assert!(
        test.contains("\"lexlean_explain\"") && test.contains("run_cli(nowhere.path()"),
        "the Q8 test calls the tool and the CLI for each code"
    );
}

/// TL-15: each in-scope function of the API-to-tool map is reached by a tool that runs.
#[test]
fn the_running_clause_of_q2_is_a_tool_per_in_scope_function_tl_15() {
    needs(
        "TL-15",
        &["each_in_scope_function_is_reached_by_a_tool_that_runs_tl_15"],
    );
    let test = read("gateway/tool-tests/tests/q2.rs");
    for compared in [
        "sdk::read_module",
        "sdk::declaration_spans",
        "sdk::assemble",
        "sdk::write_module",
        "crates/quality-map/map.toml",
        "gateway.list()",
    ] {
        assert!(test.contains(compared), "the Q2 test uses {compared}");
    }
    let quality = read("src/Quality.lex.tex");
    assert!(
        quality.contains("TL-15"),
        "the quality model names the row that runs the clause of Q2"
    );
}

/// TL-14: the planted defects of lexlean_explain are recorded.
#[test]
fn the_planted_defects_of_lexlean_explain_are_recorded_tl_14() {
    let document = read("TOOLS.md");
    let defects = table(&document, "Planted defects of lexlean_explain");
    assert!(defects.len() >= 3, "three defects or more are recorded");
    for cells in &defects {
        assert!(
            cells.len() >= 4 && cells.iter().all(|cell| !cell.is_empty()),
            "a defect names its change, command, diagnostic and restore: {cells:?}"
        );
    }
    let planted: String = defects
        .iter()
        .map(|cells| cells[0].to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    for kind in ["byte", "registered", "escape"] {
        assert!(planted.contains(kind), "a planted defect is about: {kind}");
    }
}
