//! GW-01 to GW-09: the generated half of issue #7, the gateway.
//!
//! The declaration (`gateway/src/Application.lex.tex`), the root
//! (`gateway/src/Gateway.lex.tex`), the binding crate and the workflow are the
//! sources; `lexlean verify`, `prismpm build` and `prismpm verify` check them in the
//! `gateway` job. These tests read the same files as text and hold them against
//! `src/Effects.lex.tex` (the single source of the effect names and their order),
//! against `GATEWAY.md` (the table of request, result and failures, and the
//! planted defects) and against each other. Nothing here encodes or decodes a
//! wire: the generated crates do, and the binding crate's tests run in the job.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn root() -> PathBuf {
    repo_model::repo_root()
}

fn read(relative: &str) -> String {
    std::fs::read_to_string(root().join(relative))
        .unwrap_or_else(|error| panic!("{relative}: {error}"))
}

fn tight(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The text of the string literals of a body, in order.
fn strings(body: &str) -> Vec<String> {
    body.split('"')
        .enumerate()
        .filter(|(index, _)| index % 2 == 1)
        .map(|(_, text)| text.to_string())
        .collect()
}

/// The body of the definition `name` of a module (read through `tight`), without its braces.
fn definition_body(source: &str, name: &str) -> String {
    let marker = format!("definition{{ name: {name}, ");
    let at = source
        .find(&marker)
        .unwrap_or_else(|| panic!("no definition {name}"));
    let chunk = &source[at..];
    let (_, rest) = chunk
        .split_once(", body: ")
        .unwrap_or_else(|| panic!("definition {name} has no body"));
    let end = [
        ", recursive_argument: ",
        ", axioms: ",
        ", executable: ",
        " } definition{",
        " } structure{",
        " } inductive{",
        " } theorem{",
        " } \\end{semanticmodule}",
    ]
    .iter()
    .filter_map(|terminator| rest.find(terminator))
    .min()
    .unwrap_or(rest.len());
    rest[..end].to_string()
}

/// `effectName` of the model: the wire names, in the order of the constructors.
fn model_effect_names() -> Vec<String> {
    let source = tight(&read("src/Effects.lex.tex"));
    definition_body(&source, "effectName")
        .split("| Effect.")
        .skip(1)
        .map(|branch| {
            let (_, rest) = branch.split_once(" => ").expect("a branch");
            strings(rest).remove(0)
        })
        .collect()
}

/// One declared effect of the application.
#[derive(Debug, PartialEq, Eq)]
struct Declared {
    name: String,
    request: Vec<(String, String)>,
    result: Vec<(String, String)>,
    failures: Vec<String>,
    request_maximum: u64,
    result_maximum: u64,
}

fn application() -> String {
    tight(&read("gateway/src/Application.lex.tex"))
}

fn fields(text: &str) -> Vec<(String, String)> {
    text.split("HostField {name := \"")
        .skip(1)
        .map(|part| {
            let (name, rest) = part.split_once('"').expect("a field name");
            let kind = rest
                .split("HostKind.")
                .nth(1)
                .expect("a kind")
                .split(')')
                .next()
                .expect("a kind name");
            (name.to_string(), kind.to_string())
        })
        .collect()
}

fn number_after(text: &str, key: &str) -> u64 {
    let (_, rest) = text.split_once(key).unwrap_or_else(|| panic!("no {key}"));
    rest.trim_start_matches([' ', '('])
        .chars()
        .take_while(char::is_ascii_digit)
        .collect::<String>()
        .parse()
        .unwrap_or_else(|_| panic!("{key} is not a number"))
}

/// The effects of the host protocol of the application, in order.
fn declared_effects(source: &str) -> Vec<Declared> {
    let (_, host) = source
        .split_once("Model::HostProtocol {")
        .expect("the application declares a host protocol");
    let (effects, _) = host
        .split_once(", transcripts := ")
        .expect("the host protocol has transcripts");
    effects
        .split("Model::HostEffect {name := \"")
        .skip(1)
        .map(|chunk| {
            let (name, rest) = chunk.split_once('"').expect("an effect name");
            let (_, rest) = rest.split_once(", request := ").expect("a request");
            let (request, rest) = rest.split_once(", result := ").expect("a result");
            let (result, rest) = rest.split_once(", failures := ").expect("failures");
            let (failures, rest) = rest
                .split_once(", requestMaximum := ")
                .expect("a request maximum");
            Declared {
                name: name.to_string(),
                request: fields(request),
                result: fields(result),
                failures: strings(failures),
                request_maximum: number_after(
                    &format!("requestMaximum := {rest}"),
                    "requestMaximum := ",
                ),
                result_maximum: number_after(rest, "resultMaximum := "),
            }
        })
        .collect()
}

/// The rows of the table under `heading` of `GATEWAY.md`: the cells of each row.
fn table(document: &str, heading: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut inside = false;
    let mut seen = 0usize;
    for line in document.lines() {
        if line.starts_with("## ") {
            inside = line.trim_start_matches("## ").trim() == heading;
            seen = 0;
            continue;
        }
        if inside && line.starts_with('|') {
            seen += 1;
            if seen > 2 {
                rows.push(
                    line.trim_matches('|')
                        .split('|')
                        .map(|cell| cell.trim().trim_matches('`').to_string())
                        .collect(),
                );
            }
        }
    }
    assert!(!rows.is_empty(), "GATEWAY.md has no table under {heading}");
    rows
}

fn list(cell: &str) -> Vec<String> {
    cell.split(", ")
        .map(|part| part.trim().trim_matches('`'))
        .filter(|part| !part.is_empty() && *part != "none")
        .map(str::to_string)
        .collect()
}

fn pairs(cell: &str) -> Vec<(String, String)> {
    list(cell)
        .into_iter()
        .map(|field| {
            let (name, kind) = field.split_once(':').expect("name:Kind");
            (
                name.trim().trim_matches('`').to_string(),
                kind.trim().trim_matches('`').to_string(),
            )
        })
        .collect()
}

/// GW-01: the gateway declares the ten effects of the model, in its order, and then the frame effect.
#[test]
fn the_gateway_declares_the_ten_effects_in_order_gw_01() {
    let declared: Vec<String> = declared_effects(&application())
        .into_iter()
        .map(|effect| effect.name)
        .collect();
    let model = model_effect_names();
    assert_eq!(model.len(), 10, "src/Effects.lex.tex declares ten effects");
    assert_eq!(
        declared.len(),
        11,
        "the host protocol declares the ten effects and the frame effect"
    );
    assert_eq!(
        declared[..10],
        model[..],
        "the host protocol lists the effects of src/Effects.lex.tex first, so the tag of an effect is its position"
    );
    assert_eq!(
        declared[10], FRAME_EFFECT,
        "the eleventh effect is the frame effect"
    );
    let distinct: BTreeSet<&String> = declared.iter().collect();
    assert_eq!(distinct.len(), 11, "no effect is declared twice");
    let document = read("GATEWAY.md");
    let tabled: Vec<String> = table(&document, "The effects")
        .into_iter()
        .map(|cells| cells[0].clone())
        .collect();
    assert_eq!(
        tabled, declared,
        "GATEWAY.md tabulates the same effects in the same order"
    );
}

/// The frame effect: the adapter, not the core, asks for it.
const FRAME_EFFECT: &str = "rate.calls";

/// GW-02: request, result and failures are those of the table.
#[test]
fn each_effect_has_typed_fields_and_closed_failures_gw_02() {
    let declared = declared_effects(&application());
    let document = read("GATEWAY.md");
    let rows = table(&document, "The effects");
    assert_eq!(rows.len(), declared.len(), "a row per effect");
    let kinds: BTreeSet<&str> = ["U32", "U64", "Flag", "Bytes", "Text"]
        .into_iter()
        .collect();
    for (effect, cells) in declared.iter().zip(&rows) {
        assert_eq!(effect.name, cells[0]);
        assert_eq!(effect.request, pairs(&cells[1]), "{}: request", effect.name);
        assert_eq!(effect.result, pairs(&cells[2]), "{}: result", effect.name);
        assert_eq!(
            effect.failures,
            list(&cells[3]),
            "{}: failures",
            effect.name
        );
        for (field, kind) in effect.request.iter().chain(&effect.result) {
            assert!(
                kinds.contains(kind.as_str()),
                "{}: field {field} has kind {kind}",
                effect.name
            );
        }
        let distinct: BTreeSet<&String> = effect.failures.iter().collect();
        assert_eq!(
            distinct.len(),
            effect.failures.len(),
            "{}: failures are distinct",
            effect.name
        );
        assert!(effect.request_maximum >= 1 && effect.result_maximum >= 1);
    }
    // The bounds follow the owner values: a draft is at most draft.max_bytes.
    let put = declared
        .iter()
        .find(|e| e.name == "store.put")
        .expect("store.put");
    let get = declared
        .iter()
        .find(|e| e.name == "store.get")
        .expect("store.get");
    assert!(
        put.request_maximum >= 8_388_608,
        "store.put admits a draft of draft.max_bytes"
    );
    assert!(
        get.result_maximum >= 8_388_608,
        "store.get returns a draft of draft.max_bytes"
    );
}

fn is_hex40(text: &str) -> bool {
    text.len() == 40 && text.chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
}

/// The value of `rev = ` in a pin file.
fn pinned(file: &str) -> String {
    read(file)
        .lines()
        .find_map(|line| line.strip_prefix("rev = "))
        .unwrap_or_else(|| panic!("{file} has no rev line"))
        .trim()
        .to_string()
}

/// GW-03: one production root over the MCP sources and the tool layer.
#[test]
fn the_core_is_one_production_root_over_mcp_and_the_tool_layer_gw_03() {
    assert!(
        is_hex40(&pinned(".github/mcp.rev")),
        "the MCP pin is a full commit"
    );
    assert!(
        is_hex40(&pinned(".github/prismpm.rev")),
        "the PrismPM pin is a full commit"
    );
    let project = read("gateway/lexlean.toml");
    assert!(
        project.contains("source_roots = [\"mcp/src\", \"src\"]"),
        "the project reads the gateway sources and the MCP sources laid out from the pin"
    );
    assert!(
        project.contains("src/Gateway.lex.tex"),
        "the root module is an entrypoint"
    );
    let source = tight(&read("gateway/src/Gateway.lex.tex"));
    let root = source
        .split("definition{ name: gatewayRun, ")
        .nth(1)
        .expect("Gateway declares gatewayRun");
    let root = root
        .split("\\end{semanticmodule}")
        .next()
        .expect("a module");
    assert!(
        root.contains("parameters: [input : Bytes]"),
        "the root takes the turn as Bytes"
    );
    assert!(
        root.contains("result: (List Nat)"),
        "the root answers a step as a list of octets"
    );
    assert!(
        root.contains("production: {effects: [allocation, overflow], targets: [rust-std]}"),
        "the root declares the allocation and overflow effects and no recursion"
    );
    assert!(
        source.contains("\\importmodule{StepServe}")
            && source.contains("\\importmodule{Extension}"),
        "the root is built over MCP's StepServe (the MCP#39 hook) and Extension"
    );
    let application = application();
    assert!(
        application.contains("root := \"LexLeanMcpGateway.Gateway.gatewayRun\""),
        "the application names the root"
    );
    let request = read("gateway/generated/request.json");
    assert!(
        request.contains("LexLeanMcpGateway.Gateway.gatewayRun"),
        "the export request names the root"
    );
}

fn walk(directory: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if path.file_name().is_some_and(|name| name == "target") {
                continue;
            }
            walk(&path, files);
        } else {
            files.push(path);
        }
    }
}

/// GW-04: every effect is exercised by a transcript step of a vector of the file.
#[test]
fn every_effect_is_exercised_by_an_acceptance_vector_gw_04() {
    let source = application();
    let (_, transcripts) = source
        .split_once(", transcripts := ")
        .expect("the host protocol has transcripts");
    let exercised: BTreeSet<String> = transcripts
        .split("HostStep {effect := \"")
        .skip(1)
        .map(|part| part.split('"').next().expect("an effect").to_string())
        .collect();
    let model: BTreeSet<String> = model_effect_names().into_iter().collect();
    assert_eq!(
        exercised, model,
        "the transcripts request exactly the ten effects"
    );
    let vectors = read("gateway/vectors/exchanges.json");
    for name in &model {
        assert!(
            vectors.contains(&format!("\"effect\": \"{name}\"")),
            "gateway/vectors/exchanges.json has a vector that performs {name}"
        );
    }
    for kind in ["invalid-arguments", "unknown-tool", "tools-list"] {
        assert!(
            vectors.contains(&format!("\"kind\": \"{kind}\"")),
            "a vector performs no effect: {kind}"
        );
    }
}

/// The test names of a Rust file.
fn test_names(relative: &str) -> BTreeSet<String> {
    let mut files = Vec::new();
    walk(&root().join(relative), &mut files);
    let mut names = BTreeSet::new();
    for file in files
        .iter()
        .filter(|f| f.extension().is_some_and(|e| e == "rs"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        let mut after_test = false;
        for line in text.lines() {
            let line = line.trim();
            if line.starts_with("#[test]") || line.starts_with("#[tokio::test]") {
                after_test = true;
            } else if after_test && line.starts_with("fn ") {
                names.insert(line[3..].split('(').next().expect("a name").to_string());
                after_test = false;
            } else if !line.starts_with("#[") && !line.starts_with("///") {
                after_test = false;
            }
        }
    }
    names
}

/// GW-05: the closed set is asserted by negative tests of the binding crate.
#[test]
fn no_effect_outside_e1_to_e10_can_be_encoded_gw_05() {
    let names = test_names("gateway/bindings/gateway-binding");
    for required in [
        "the_first_tag_past_the_set_is_undeclared",
        "the_largest_tag_is_undeclared",
        "the_decoder_has_one_variant_per_effect",
        "the_frame_effect_is_not_the_cores",
    ] {
        assert!(
            names.contains(required),
            "the binding crate has the test {required}"
        );
    }
    let workflow = read(".github/workflows/lexlean.yml");
    assert!(
        workflow.contains("gateway-binding") && workflow.contains("--ignored"),
        "the gateway job runs the binding tests"
    );
}

/// The tool names of the tool layer: the `lexlean_` string literals of `gateway/src`.
fn tool_names() -> BTreeSet<String> {
    let mut files = Vec::new();
    walk(&root().join("gateway/src"), &mut files);
    let mut names = BTreeSet::new();
    for file in files
        .iter()
        .filter(|f| f.to_string_lossy().ends_with(".lex.tex"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        for literal in strings(&text) {
            if literal.starts_with("lexlean_")
                && literal.chars().all(|c| c.is_ascii_lowercase() || c == '_')
            {
                names.insert(literal);
            }
        }
    }
    names
}

/// The method names the MCP sources at the pin dispatch, plus the reserved prefixes.
fn method_names() -> BTreeSet<String> {
    let mut names: BTreeSet<String> = [
        "initialize",
        "ping",
        "server/discover",
        "tools/list",
        "tools/call",
        "resources/list",
        "resources/read",
        "resources/templates/list",
        "prompts/list",
        "prompts/get",
        "completion/complete",
        "tasks/get",
        "tasks/update",
        "tasks/cancel",
        "subscriptions/listen",
        "notifications/progress",
    ]
    .into_iter()
    .map(str::to_string)
    .collect();
    let mut files = Vec::new();
    walk(&root().join("gateway/mcp/src"), &mut files);
    for file in files
        .iter()
        .filter(|f| f.to_string_lossy().ends_with(".lex.tex"))
    {
        let text = std::fs::read_to_string(file).unwrap_or_default();
        for literal in strings(&text) {
            let method = literal.contains('/')
                && !literal.contains(' ')
                && literal.split('/').all(|part| {
                    !part.is_empty() && part.chars().all(|c| c.is_ascii_lowercase() || c == '_')
                })
                && [
                    "tools",
                    "resources",
                    "prompts",
                    "completion",
                    "tasks",
                    "server",
                    "notifications",
                    "subscriptions",
                    "logging",
                    "sampling",
                    "elicitation",
                    "roots",
                ]
                .contains(&literal.split('/').next().expect("a prefix"));
            if method {
                names.insert(literal);
            }
        }
    }
    names
}

/// Whether `name` occurs in `text` as a name and not inside a longer word: the characters
/// around it are not letters, digits or underscores (the method `ping` is not in "mapping").
fn contains_name(text: &str, name: &str) -> bool {
    let word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    text.match_indices(name).any(|(at, _)| {
        let before = text[..at].chars().next_back();
        let after = text[at + name.len()..].chars().next();
        !before.is_some_and(word) && !after.is_some_and(word)
    })
}

/// The names of `names` that occur in a file of `directories`, as `(file, name)`.
fn occurrences(directories: &[&str], names: &BTreeSet<String>) -> Vec<(String, String)> {
    let mut found = Vec::new();
    for directory in directories {
        let mut files = Vec::new();
        walk(&root().join(directory), &mut files);
        for file in files {
            let Ok(text) = std::fs::read_to_string(&file) else {
                continue;
            };
            for name in names {
                if contains_name(&text, name) {
                    found.push((file.display().to_string(), name.clone()));
                }
            }
        }
    }
    found
}

/// GW-06: no tool name or method name occurs in host adapter source (Q6).
#[test]
fn no_tool_or_method_name_occurs_in_adapter_source_gw_06() {
    let tools = tool_names();
    assert!(
        tools.len() >= 10,
        "the tool layer names its tools: {tools:?}"
    );
    let methods = method_names();
    assert!(
        methods.contains("tools/call"),
        "the method list is not empty"
    );
    let names: BTreeSet<String> = tools.union(&methods).cloned().collect();
    let found = occurrences(&["gateway/bindings"], &names);
    assert!(
        found.is_empty(),
        "adapter source names a tool or a method: {found:?}"
    );
    // The check has teeth: a planted name is found.
    let planted: BTreeSet<String> = ["lexlean_check".to_string()].into_iter().collect();
    let scratch = std::env::temp_dir().join(format!("gw06-{}", std::process::id()));
    std::fs::create_dir_all(&scratch).expect("scratch");
    std::fs::write(
        scratch.join("lib.rs"),
        "const NAME: &str = \"lexlean_check\";\n",
    )
    .expect("write");
    let hits = {
        let mut files = Vec::new();
        walk(&scratch, &mut files);
        files
            .iter()
            .filter(|f| {
                std::fs::read_to_string(f)
                    .is_ok_and(|t| planted.iter().any(|n| contains_name(&t, n)))
            })
            .count()
    };
    std::fs::remove_dir_all(&scratch).expect("cleanup");
    assert_eq!(hits, 1, "a planted tool name is found");
}

/// The generated host crates, once `prismpm build` has laid them out (the gateway job).
#[test]
#[ignore = "needs the generated host packages that `prismpm build` lays out: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn no_tool_or_method_name_occurs_in_generated_host_source_gw_06() {
    let tools = tool_names();
    let methods = method_names();
    let names: BTreeSet<String> = tools.union(&methods).cloned().collect();
    let generated: Vec<String> = std::fs::read_dir(root().join("gateway"))
        .expect("gateway")
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n.ends_with("-host-api") || n.ends_with("-host"))
        .map(|n| format!("gateway/{n}"))
        .collect();
    assert_eq!(
        generated.len(),
        2,
        "the host-api and host packages are laid out: {generated:?}"
    );
    let directories: Vec<&str> = generated.iter().map(String::as_str).collect();
    let found = occurrences(&directories, &names);
    assert!(
        found.is_empty(),
        "generated host source names a tool or a method: {found:?}"
    );
}

/// GW-07: each effect has a method in the binding and tests of its success and failures.
#[test]
fn the_binding_implements_each_effect_and_tests_each_failure_gw_07() {
    let source = read("gateway/bindings/gateway-binding/src/lib.rs");
    let names = test_names("gateway/bindings/gateway-binding");
    for effect in model_effect_names() {
        let method = effect.replace('.', "_");
        assert!(
            source.contains(&format!("fn {method}(")),
            "the binding implements {effect} as {method}"
        );
        assert!(
            names
                .iter()
                .any(|name| name.starts_with(&format!("{method}_"))),
            "the binding has tests for {effect}"
        );
    }
    for forbidden in [
        "match tag",
        "match request",
        "Request::StoreGet",
        "fn dispatch",
    ] {
        assert!(
            !source.contains(forbidden),
            "the binding holds effect implementations only, never dispatch: {forbidden}"
        );
    }
}

/// GW-08: the gateway job builds, compares the core and verifies; the defects are recorded.
#[test]
fn the_gateway_job_builds_and_verifies_and_the_defects_are_recorded_gw_08() {
    // The steps are in the workflow; the build and the comparison of the core are functions
    // of the library the workflow sources (`gateway_build`), so both files are read.
    let workflow = format!(
        "{}\n{}",
        read(".github/workflows/lexlean.yml"),
        read(".github/lexlean-ci/lib.sh")
    );
    for step in [
        "prismpm build",
        "prismpm verify",
        "diff -r",
        "install_prismpm",
        "gateway/registry.sh",
    ] {
        assert!(workflow.contains(step), "the workflow runs `{step}`");
    }
    let document = read("GATEWAY.md");
    let defects = table(&document, "Planted defects");
    assert!(defects.len() >= 5, "five defects or more are recorded");
    for cells in &defects {
        assert!(
            cells.len() >= 4,
            "a defect names its change, command, diagnostic and restore"
        );
        assert!(
            cells.iter().all(|cell| !cell.is_empty()),
            "no cell of a defect is empty: {cells:?}"
        );
    }
    let planted: String = defects
        .iter()
        .map(|c| c[0].to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    for kind in [
        "undeclared effect",
        "missing vector",
        "wrong effect result",
        "tool name",
    ] {
        assert!(planted.contains(kind), "a planted defect is: {kind}");
    }
}

/// GW-09: the gateway builds from the pins, with registry dependencies only.
#[test]
fn the_gateway_builds_from_the_pins_without_a_published_crate_gw_09() {
    let script = read("gateway/registry.sh");
    for needle in ["cargo package", "sha256sum", "expected", "directory"] {
        assert!(
            script.contains(needle),
            "gateway/registry.sh uses `{needle}`"
        );
    }
    let mut manifests = Vec::new();
    walk(&root().join("gateway"), &mut manifests);
    let manifests: Vec<PathBuf> = manifests
        .into_iter()
        .filter(|f| f.file_name().is_some_and(|n| n == "Cargo.toml"))
        .filter(|f| !f.to_string_lossy().contains("/generated/"))
        // The packages `prismpm build` generates and `gateway/lay-out.sh` lays out, and the
        // registry, are not committed (gateway/.gitignore): the manifests here are the
        // repository's own.
        .filter(|f| {
            let path = f.to_string_lossy();
            ![
                "/gateway/gateway-host/",
                "/gateway/gateway-host-api/",
                "/gateway/lexlean_mcp_gateway_core/",
                "/gateway/registry/",
            ]
            .iter()
            .any(|laid| path.contains(laid))
        })
        .collect();
    assert!(!manifests.is_empty(), "the gateway workspace has manifests");
    for manifest in manifests {
        let text = std::fs::read_to_string(&manifest).expect("manifest");
        let relative = manifest
            .strip_prefix(root())
            .expect("inside")
            .display()
            .to_string();
        assert!(!text.contains("git = "), "{relative} has a Git dependency");
        for line in text.lines() {
            let line = line.trim();
            let dependency =
                line.contains("path = ") && !line.starts_with("members") && !line.starts_with('#');
            // A path to a sibling of the gateway workspace is a workspace member; a path to
            // lexlean or to the MCP core would not be.
            if dependency {
                assert!(
                    !line.contains("lexlean") || line.contains("gateway-"),
                    "{relative}: `{line}` is a path dependency on an upstream crate"
                );
            }
        }
    }
    let deny = read("deny.toml");
    assert!(
        deny.contains("allow-registry = [\"https://github.com/rust-lang/crates.io-index\"]")
            && deny.contains("unknown-git = \"deny\""),
        "deny.toml admits crates.io only"
    );
    let ci = read("CI.md");
    assert!(
        ci.contains("release step") && ci.contains("registry.sh"),
        "CI.md states the release step that removes the replacement"
    );
}

/// GW-10: the rate counter is a frame effect, outside the ten effects of the core.
#[test]
fn the_rate_counter_is_a_frame_effect_outside_the_cores_ten_gw_10() {
    let source = application();
    let declared = declared_effects(&source);
    let frame = declared
        .iter()
        .find(|effect| effect.name == FRAME_EFFECT)
        .expect("the host protocol declares the frame effect");
    assert_eq!(
        frame.request,
        vec![("counted".to_string(), "Flag".to_string())],
        "the adapter tells the effect whether the request is counted"
    );
    assert_eq!(
        frame.result,
        vec![("calls".to_string(), "U32".to_string())],
        "the effect answers with the count"
    );
    assert!(frame.failures.is_empty(), "the counter cannot fail");
    assert_eq!(
        declared
            .iter()
            .position(|effect| effect.name == FRAME_EFFECT),
        Some(10),
        "its tag is 10, the first past the ten of the core"
    );
    let tight_source = tight(&source);
    assert!(
        tight_source.contains(
            "FrameSection {name := \"tool_calls\", kind := (ctor Foundation.Server.V1.Model::FrameKind.U32), effect := \"rate.calls\", field := \"calls\", values := (nil String)}"
        ),
        "the request frame's tool_calls section takes the field calls of the effect, not a constant"
    );
    assert!(
        !tight_source.contains("effect := \"\", field := \"\""),
        "no frame section is a model value"
    );
    // The core cannot encode its tag: the decoder of the model refuses it.
    let model = tight(&read("src/Effects.lex.tex"));
    assert!(
        model.contains(
            "label := \"the first unassigned id\", wire := (ctor Wire.Frame (cons 0 (cons 0 (cons 0 (cons 10 (nil Nat)))))), expected := (ctor Result.error [Effect, Refusal] (ctor Refusal.UnknownId))"
        ),
        "src/Effects.lex.tex has the vector that refuses the tag 10"
    );
    let document = read("EFFECTS.md");
    assert!(
        document.contains("`rate.calls`") && document.contains("frame effect"),
        "EFFECTS.md records the frame effect and that it is outside the ten"
    );
}

/// GW-11: the MCP files laid out are those of the pin, and the lexicon tree is the lock's.
#[test]
fn the_laid_out_mcp_modules_are_those_of_the_pin_gw_11() {
    let script = read("gateway/mcp-identities.sh");
    for needle in [
        "diff -r",
        "tree_sha256",
        "mcp.core",
        ".github/mcp.rev",
        "compiler_semantics",
        "mcp/lexlean.rev",
    ] {
        assert!(
            script.contains(needle),
            "gateway/mcp-identities.sh checks {needle}"
        );
    }
    let workflow = read(".github/workflows/lexlean.yml");
    assert!(
        workflow.contains("gateway/mcp-identities.sh"),
        "the gateway job runs the identity check"
    );
    assert!(
        workflow.contains("LEXLEAN_REV_FILE") && workflow.contains("gateway/mcp/lexlean.rev"),
        "the gateway job builds lexlean and the SDK with the LexLean commit MCP names"
    );
    let registry = read("gateway/registry.sh");
    assert!(
        registry.contains("mcp/lexlean.rev") && !registry.contains(".github/lexlean.rev|"),
        "the registry packages the lexlean MCP names, not the root project's"
    );
}
