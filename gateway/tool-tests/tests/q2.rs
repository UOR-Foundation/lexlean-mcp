//! TL-15: the running clause of Q2 (issue #15). Each in-scope function of the API-to-tool map
//! (`crates/quality-map/map.toml`) is reached by a tool that `tools/list` of the production core
//! shows and that runs: a call of the tool through the core with the real host asks for the
//! effect that reaches the function and answers. For the four functions of `lexlean::sdk` the
//! result is held against the function's own result on the same text.

use std::collections::BTreeMap;
use std::path::Path;

use gateway_tool_tests::{tiny, Gateway, SOURCE};
use lexlean::sdk::{self, ModuleKind, SourceHeader};
use serde_json::{json, Value};

const PROFILE_ID: &str = "ad66a1c0c9fa6ad8b6bf64559be0c949e8141a0fa9ad64b80978f57fc83817c2";

/// The functions of the map that reach a tool, in the order of the map, with their tools.
fn mapped() -> Vec<(String, String)> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/quality-map/map.toml");
    let text = std::fs::read_to_string(&path).expect("the map");
    let mut functions = Vec::new();
    let mut current: Option<String> = None;
    let mut in_functions = false;
    for line in text.lines() {
        if line.trim() == "[[function]]" {
            in_functions = true;
            current = None;
        } else if line.trim() == "[[tool]]" {
            in_functions = false;
        } else if in_functions {
            if let Some(value) = line.trim().strip_prefix("path = ") {
                current = Some(value.trim_matches('"').to_string());
            } else if let Some(value) = line.trim().strip_prefix("tool = ") {
                functions.push((
                    current.clone().expect("a path before the tool"),
                    value.trim_matches('"').to_string(),
                ));
            }
        }
    }
    functions
}

fn tiny_text() -> String {
    std::fs::read_to_string(tiny().join(SOURCE)).expect("the source")
}

#[test]
fn each_in_scope_function_is_reached_by_a_tool_that_runs_tl_15() {
    let functions = mapped();
    let names: Vec<&str> = functions.iter().map(|(path, _)| path.as_str()).collect();
    assert_eq!(
        names,
        [
            "Engine::check",
            "Engine::snapshot",
            "Engine::build",
            "Engine::verify",
            "Engine::format",
            "sdk::read_module",
            "sdk::write_module",
            "sdk::declaration_spans",
            "sdk::assemble",
        ],
        "the nine functions of Q2 that reach a tool"
    );

    let mut gateway = Gateway::new();
    gateway.provision("tiny", &Gateway::record(PROFILE_ID));
    gateway.install("dir", &tiny());
    let listed = gateway.list();
    let listed: Vec<String> = listed.result()["tools"]
        .as_array()
        .expect("tools")
        .iter()
        .map(|tool| tool["name"].as_str().expect("a name").to_string())
        .collect();
    for (function, tool) in &functions {
        assert!(
            listed.contains(tool),
            "{tool}, the tool of {function}, is listed: {listed:?}"
        );
    }

    // The Engine calls: the tool asks for the effect that is the call, and answers.
    let engine: BTreeMap<&str, (&str, Value)> = BTreeMap::from([
        (
            "Engine::check",
            ("engine.check", json!({"project": "dir", "files": []})),
        ),
        (
            "Engine::snapshot",
            ("engine.snapshot", json!({"project": "dir", "files": []})),
        ),
        (
            "Engine::build",
            ("engine.build", json!({"project": "dir", "files": []})),
        ),
        (
            "Engine::format",
            (
                "engine.format",
                json!({"project": "dir", "files": [], "check_only": true}),
            ),
        ),
        (
            "Engine::verify",
            ("verify.start", json!({"project": "dir", "files": []})),
        ),
    ]);
    for (function, tool) in &functions {
        let Some((effect, arguments)) = engine.get(function.as_str()) else {
            continue;
        };
        let reply = gateway.call(tool, arguments.clone());
        assert!(!reply.is_error(), "{tool}: {}", reply.body);
        assert_eq!(reply.effects, [*effect], "{tool} reaches {function}");
    }

    // The four functions of the sdk, on the text of the tiny module.
    let text = tiny_text();
    let data = sdk::read_module(&text).expect("reads");
    let written = gateway.call(
        "lexlean_draft_write",
        json!({"profile": "tiny", "files": [{"path": SOURCE, "content": text}]}),
    );
    let draft = written.structured()["draft"]
        .as_str()
        .expect("a handle")
        .to_string();

    let reply = gateway.call(
        "lexlean_draft_read",
        json!({"draft": draft, "path": SOURCE, "view": "data"}),
    );
    assert_eq!(reply.effects, ["store.get"]);
    assert_eq!(
        reply.text(),
        serde_json::to_string(&data).expect("JSON"),
        "sdk::read_module"
    );

    let reply = gateway.call(
        "lexlean_draft_read",
        json!({"draft": draft, "path": SOURCE, "view": "spans"}),
    );
    let spans = sdk::declaration_spans(&text).expect("spans");
    assert_eq!(
        reply.text(),
        serde_json::to_string(&spans).expect("JSON"),
        "sdk::declaration_spans"
    );

    let header = json!({"name": "Tiny", "glossaries": ["lexlean.std.bool@1.2.0",
        "lexlean.std.nat@1.2.0"], "imports": [], "title": "Boolean"});
    let reply = gateway.call(
        "lexlean_draft_assemble",
        json!({"profile": "tiny", "path": "src/Other.lex.tex", "kind": "semantic",
               "header": header, "data": data}),
    );
    assert!(!reply.is_error(), "{}", reply.body);
    let assembled = sdk::assemble(
        &SourceHeader {
            name: "Tiny",
            glossaries: &["lexlean.std.bool@1.2.0", "lexlean.std.nat@1.2.0"],
            imports: &[],
            title: "Boolean",
        },
        ModuleKind::Semantic,
        &data,
    )
    .expect("assembles");
    let handle = reply.structured()["draft"].as_str().expect("a handle");
    let stored = gateway.call(
        "lexlean_draft_read",
        json!({"draft": handle, "path": "src/Other.lex.tex"}),
    );
    assert_eq!(stored.text(), assembled, "sdk::assemble");

    let mut changed = data.clone();
    changed["declarations"][0]["name"] = json!("twice");
    let reply = gateway.call(
        "lexlean_draft_assemble",
        json!({"profile": "tiny", "draft": draft, "path": SOURCE, "data": changed}),
    );
    let handle = reply.structured()["draft"].as_str().expect("a handle");
    let stored = gateway.call(
        "lexlean_draft_read",
        json!({"draft": handle, "path": SOURCE}),
    );
    assert_eq!(
        stored.text(),
        sdk::write_module(&text, &changed).expect("rewrites"),
        "sdk::write_module"
    );
}
