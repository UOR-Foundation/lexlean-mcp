//! TL-01, TL-02 and TL-07: the schemas of the four tools, what an answer of the Engine becomes
//! as a tool result, and the diagnostic of an atom outside the profile. Each test calls the
//! tool through the production core with the real host, as a client does.

use gateway_binding::Native;
use gateway_host_api::protocol::{
    attestation_lookup, clock_now, engine_build, engine_check, engine_format, engine_snapshot,
    lexicon_read, rate_calls, store_get, store_put, task_id, task_store, task_table,
    verify_control, verify_start, Host, HostError,
};
use gateway_tool_tests::{fixture, tiny, Gateway, Reply, TOOLS};
use serde_json::{json, Value};

const INVALID_PARAMS: i64 = -32602;
const INTERNAL_ERROR: i64 = -32603;

/// A gateway whose store holds the draft project `tiny`.
fn with_tiny() -> Gateway {
    let gateway = Gateway::new();
    gateway.install("tiny", &tiny());
    gateway
}

fn is_error(result: &Value) -> bool {
    result
        .get("isError")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn text(result: &Value) -> &str {
    result["content"][0]["text"].as_str().expect("a text block")
}

// ---- TL-01 -----------------------------------------------------------------------------

fn input_schema(with_check_only: bool) -> Value {
    let mut properties = json!({
        "project": {"type": "string", "minLength": 1},
        "files": {"type": "array", "items": {"type": "string"}},
        "all": {"type": "boolean"},
    });
    if with_check_only {
        properties["check_only"] = json!({"type": "boolean"});
    }
    json!({
        "type": "object",
        "properties": properties,
        "required": ["project"],
        "additionalProperties": false,
    })
}

fn command_result_schema(command: &str) -> Value {
    json!({
        "type": "object",
        "properties": {
            "spec": {"type": "string", "const": "lexlean/command-result/1"},
            "command": {"type": "string", "const": command},
            "success": {"type": "boolean"},
            "exit_code": {"type": "integer"},
            "modules": {"type": "array", "items": {"type": "string"}},
            "artifacts": {"type": "array", "items": {"type": "string"}},
            "diagnostics": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {"code": {"type": "string"}, "message": {"type": "string"}},
                    "required": ["code", "message"],
                },
            },
            "source_id": {"type": "string"},
            "semantic_id": {"type": "string"},
            "build_id": {"type": "string"},
        },
        "required": ["spec", "command", "success", "exit_code", "modules", "artifacts", "diagnostics"],
        "additionalProperties": false,
    })
}

#[test]
fn tools_list_shows_the_four_tools_with_their_schemas_tl_01() {
    let mut gateway = Gateway::new();
    let reply = gateway.list();
    let listed = reply.result()["tools"].as_array().expect("tools").clone();
    for name in TOOLS {
        let tool = listed
            .iter()
            .find(|tool| tool["name"] == name)
            .unwrap_or_else(|| panic!("{name} is listed"));
        let expected_input = input_schema(name == "lexlean_format");
        assert_eq!(tool["inputSchema"], expected_input, "{name}: inputSchema");
        let expected_output = match name {
            "lexlean_check" => command_result_schema("check"),
            "lexlean_format" => command_result_schema("fmt"),
            "lexlean_build" => command_result_schema("build"),
            _ => json!({
                "type": "object",
                "properties": {"spec": {"type": "string"}},
                "required": ["spec"],
            }),
        };
        assert_eq!(
            tool["outputSchema"], expected_output,
            "{name}: outputSchema"
        );
        assert!(
            tool["description"].as_str().is_some_and(|d| !d.is_empty()),
            "{name} is described"
        );
    }
    let names: Vec<&str> = listed
        .iter()
        .map(|tool| tool["name"].as_str().expect("a name"))
        .collect();
    assert_eq!(
        names.iter().filter(|name| TOOLS.contains(name)).count(),
        4,
        "each of the four tools is listed once: {names:?}"
    );
}

/// A call that breaks the input schema is refused by the core before the tool runs, as an
/// error result the model can read (`Invalid arguments: ...`, MCP's tool execution error),
/// and no effect is asked for.
fn assert_refused_by_the_schema(reply: &Reply, label: &str) {
    let result = reply.result();
    assert!(is_error(result), "{label}: {}", reply.body);
    assert!(
        text(result).starts_with("Invalid arguments:"),
        "{label}: {}",
        reply.body
    );
    assert!(
        result.get("structuredContent").is_none(),
        "{label}: an invalid call has no command result"
    );
    assert!(
        reply.effects.is_empty(),
        "{label}: a refused call asks for no effect"
    );
}

#[test]
fn a_call_that_breaks_a_schema_is_refused_before_the_engine_tl_01() {
    let mut gateway = with_tiny();
    for tool in TOOLS {
        let broken = [
            json!({}),
            json!({"project": ""}),
            json!({"project": 3}),
            json!({"project": "tiny", "files": "src/Tiny.lex.tex"}),
            json!({"project": "tiny", "files": [1]}),
            json!({"project": "tiny", "all": "yes"}),
            json!({"project": "tiny", "unknown": true}),
        ];
        for arguments in broken {
            let reply = gateway.call(tool, arguments.clone());
            assert_refused_by_the_schema(&reply, &format!("{tool} {arguments}"));
        }
        assert_refused_by_the_schema(
            &gateway.call_without_arguments(tool),
            &format!("{tool} without arguments"),
        );
        // A call the schema admits and the tool cannot make is a protocol error: a file name
        // with a line feed cannot be sent to the host.
        let reply = gateway.call(
            tool,
            json!({"project": "tiny", "files": ["src/a\nb.lex.tex"]}),
        );
        assert_eq!(
            reply.error()["code"],
            INVALID_PARAMS,
            "{tool}: {}",
            reply.body
        );
        assert!(reply.effects.is_empty(), "{tool}");
    }
    assert_refused_by_the_schema(
        &gateway.call(
            "lexlean_format",
            json!({"project": "tiny", "check_only": "yes"}),
        ),
        "lexlean_format with check_only a string",
    );
}

#[test]
fn all_together_with_files_is_invalid_arguments_tl_01() {
    let mut gateway = with_tiny();
    for tool in TOOLS {
        let reply = gateway.call(
            tool,
            json!({"project": "tiny", "all": true, "files": ["src/Tiny.lex.tex"]}),
        );
        assert_eq!(reply.error()["code"], INVALID_PARAMS, "{tool}");
        assert!(reply.effects.is_empty(), "{tool}");
        // `all` with no files is the selection of every module, and `all: false` with files
        // is the files.
        let every = gateway.call(tool, json!({"project": "tiny", "all": true, "files": []}));
        assert_eq!(every.effects.len(), 1, "{tool}: {}", every.body);
        let listed = gateway.call(
            tool,
            json!({"project": "tiny", "all": false, "files": ["src/Tiny.lex.tex"]}),
        );
        assert_eq!(listed.effects.len(), 1, "{tool}: {}", listed.body);
    }
}

// ---- TL-02 -----------------------------------------------------------------------------

fn structured(reply: &Reply) -> &Value {
    &reply.result()["structuredContent"]
}

#[test]
fn class_zero_is_a_result_without_is_error_tl_02() {
    let mut gateway = with_tiny();
    for (tool, command) in [
        ("lexlean_check", "check"),
        ("lexlean_format", "fmt"),
        ("lexlean_build", "build"),
    ] {
        let reply = gateway.call(tool, json!({"project": "tiny"}));
        let result = reply.result();
        assert!(!is_error(result), "{tool}: {result}");
        let command_result = structured(&reply);
        assert_eq!(command_result["spec"], "lexlean/command-result/1", "{tool}");
        assert_eq!(command_result["command"], command, "{tool}");
        assert_eq!(command_result["success"], true, "{tool}");
        assert_eq!(command_result["exit_code"], 0, "{tool}");
        assert_eq!(command_result["diagnostics"], json!([]), "{tool}");
        assert_eq!(command_result["modules"], json!(["Tiny"]), "{tool}");
        let block = text(result);
        assert!(
            block.ends_with('\n'),
            "{tool}: the text is the CLI's file spelling"
        );
        assert_eq!(
            serde_json::from_str::<Value>(block).expect("the text is JSON"),
            *command_result,
            "{tool}: the text block and structuredContent are one document"
        );
    }
    let snapshot = gateway.call("lexlean_snapshot", json!({"project": "tiny"}));
    let result = snapshot.result();
    assert!(!is_error(result));
    assert!(structured(&snapshot)["spec"]
        .as_str()
        .is_some_and(|spec| spec.starts_with("lexlean/semantic-snapshot/")));
    assert_eq!(
        serde_json::from_str::<Value>(text(result)).expect("JSON"),
        *structured(&snapshot)
    );
}

#[test]
fn classes_one_two_and_four_are_error_results_with_the_command_result_tl_02() {
    for tool in TOOLS {
        let mut gateway = with_tiny();
        // Class 1: a source that is not a module.
        std::fs::write(
            gateway.draft("tiny").join("src/Tiny.lex.tex"),
            "\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n",
        )
        .expect("a source");
        let language = gateway.call(tool, json!({"project": "tiny"}));
        let result = language.result();
        assert!(is_error(result), "{tool}: {result}");
        let command_result = structured(&language);
        assert_eq!(command_result["spec"], "lexlean/command-result/1", "{tool}");
        assert_eq!(command_result["success"], false, "{tool}");
        assert_eq!(command_result["exit_code"], 1, "{tool}");
        assert!(
            command_result["diagnostics"][0]["code"]
                .as_str()
                .is_some_and(|code| code.starts_with("LL")),
            "{tool}: {command_result}"
        );
        assert_eq!(
            serde_json::from_str::<Value>(text(result)).expect("JSON"),
            *command_result,
            "{tool}: the text block is the command result"
        );

        // Class 2: a selection that names no module of the project.
        let gateway = &mut with_tiny();
        let selection = gateway.call(
            tool,
            json!({"project": "tiny", "files": ["src/Nope.lex.tex"]}),
        );
        assert!(is_error(selection.result()), "{tool}: {}", selection.body);
        assert_eq!(structured(&selection)["exit_code"], 2, "{tool}");

        // Class 4: a limit of the project that the module exceeds (its lock is current).
        gateway.install("limited", &fixture("over-limit"));
        let limit = gateway.call(tool, json!({"project": "limited"}));
        assert!(is_error(limit.result()), "{tool}: {}", limit.body);
        assert_eq!(structured(&limit)["exit_code"], 4, "{tool}");
        assert_eq!(
            structured(&limit)["diagnostics"][0]["code"],
            "LLS8002",
            "{tool}: {}",
            limit.body
        );
    }
}

#[test]
fn a_host_failure_is_an_error_result_with_its_failure_tl_02() {
    let mut gateway = with_tiny();
    for tool in TOOLS {
        let absent = gateway.call(tool, json!({"project": "absent"}));
        let result = absent.result();
        assert!(is_error(result), "{tool}: {result}");
        assert_eq!(
            result["structuredContent"],
            json!({"failure": "project-refused"}),
            "{tool}"
        );
        assert_eq!(absent.effects.len(), 1, "{tool}: the effect was asked for");
    }
    // An environment error of the Engine is not a class: the effect declares it as a failure.
    let config = std::fs::read_to_string(gateway.draft("tiny").join("lexlean.toml"))
        .expect("a config")
        .replace(
            "lean_workspace = \".\"",
            "lean_workspace = \"no-such-workspace\"",
        );
    std::fs::write(gateway.draft("tiny").join("lexlean.toml"), config).expect("a config");
    for tool in TOOLS {
        let unavailable = gateway.call(tool, json!({"project": "tiny"}));
        let result = unavailable.result();
        assert!(is_error(result), "{tool}: {result}");
        assert_eq!(
            result["structuredContent"],
            json!({"failure": "engine-unavailable"}),
            "{tool}"
        );
    }
}

/// A host that answers every Engine effect with what it was given, and every other effect as
/// the real one does: the way to make the core meet an answer that the real Engine never gives.
struct Forge {
    inner: Native,
    class: u32,
    result: Vec<u8>,
}

macro_rules! forged {
    ($name:ident, $module:ident) => {
        fn $name(
            &mut self,
            _request: $module::Request,
        ) -> Result<$module::Completed, HostError<$module::Failure>> {
            Ok($module::Completed {
                class: self.class,
                result: self.result.clone(),
            })
        }
    };
}

macro_rules! delegated {
    ($name:ident, $module:ident) => {
        fn $name(
            &mut self,
            request: $module::Request,
        ) -> Result<$module::Completed, HostError<$module::Failure>> {
            self.inner.$name(request)
        }
    };
}

impl Host for Forge {
    forged!(engine_check, engine_check);
    forged!(engine_snapshot, engine_snapshot);
    forged!(engine_format, engine_format);
    forged!(engine_build, engine_build);
    delegated!(store_get, store_get);
    delegated!(store_put, store_put);
    delegated!(verify_start, verify_start);
    delegated!(verify_control, verify_control);
    delegated!(lexicon_read, lexicon_read);
    delegated!(attestation_lookup, attestation_lookup);
    delegated!(rate_calls, rate_calls);
    delegated!(clock_now, clock_now);
    delegated!(task_id, task_id);
    delegated!(task_table, task_table);
    delegated!(task_store, task_store);
}

fn forge(gateway: &Gateway, class: u32, result: &[u8]) -> Forge {
    Forge {
        inner: Native::new(gateway.config()),
        class,
        result: result.to_vec(),
    }
}

fn command_result(exit_code: i64) -> Vec<u8> {
    format!(
        "{{\"artifacts\":[],\"command\":\"check\",\"diagnostics\":[],\"exit_code\":{exit_code},\"modules\":[],\"spec\":\"lexlean/command-result/1\",\"success\":{}}}\n",
        exit_code == 0
    )
    .into_bytes()
}

#[test]
fn bytes_that_are_not_a_command_result_are_refused_by_the_core_tl_02() {
    let mut gateway = with_tiny();
    let cases: [(&str, u32, Vec<u8>); 8] = [
        ("not JSON", 0, b"this is not JSON".to_vec()),
        ("empty", 0, Vec::new()),
        ("not UTF-8", 0, vec![0xff, 0xfe, 0xfd]),
        ("an array", 0, b"[1, 2]\n".to_vec()),
        ("a class and an exit code that differ", 1, command_result(2)),
        ("class 0 with a failing exit code", 0, command_result(1)),
        ("class 4 with a success", 4, command_result(0)),
        (
            "class 1 with a document that is no command result",
            1,
            b"{\"spec\":\"lexlean/semantic-snapshot/2\"}\n".to_vec(),
        ),
    ];
    for tool in TOOLS {
        for (label, class, bytes) in &cases {
            let mut host = forge(&gateway, *class, bytes);
            let reply = gateway.call_with(&mut host, tool, json!({"project": "tiny"}));
            assert_eq!(
                reply.error()["code"],
                INTERNAL_ERROR,
                "{tool}, {label}: {}",
                reply.body
            );
            assert_eq!(reply.status, 500, "{tool}, {label}");
        }
    }
    // A success of the snapshot is a snapshot, and a command result is not one.
    let mut host = forge(&gateway, 0, &command_result(0));
    let reply = gateway.call_with(&mut host, "lexlean_snapshot", json!({"project": "tiny"}));
    assert_eq!(reply.error()["code"], INTERNAL_ERROR, "{}", reply.body);
    // The same host with a result that is one: nothing is refused.
    for (class, bytes) in [
        (0, command_result(0)),
        (1, command_result(1)),
        (4, command_result(4)),
    ] {
        let mut host = forge(&gateway, class, &bytes);
        let reply = gateway.call_with(&mut host, "lexlean_check", json!({"project": "tiny"}));
        assert_eq!(is_error(reply.result()), class != 0);
        assert_eq!(text(reply.result()).as_bytes(), bytes);
    }
}

// ---- TL-07 -----------------------------------------------------------------------------

#[test]
fn an_atom_outside_the_profile_gives_lll1004_with_its_span_tl_07() {
    let mut gateway = Gateway::new();
    gateway.install("atom", &fixture("outside-atom"));
    let reply = gateway.call("lexlean_check", json!({"project": "atom"}));
    let result = reply.result();
    assert!(is_error(result), "{result}");
    let command_result = structured(&reply);
    assert_eq!(command_result["exit_code"], 1);
    let diagnostics = command_result["diagnostics"]
        .as_array()
        .expect("diagnostics");
    assert_eq!(diagnostics.len(), 1, "{command_result}");
    let diagnostic = &diagnostics[0];
    assert_eq!(diagnostic["code"], "LLL1004");
    assert!(diagnostic["message"]
        .as_str()
        .is_some_and(|message| message.contains("`banana`")));
    // The span is where `banana` is in the module, found here and not recorded.
    let source = std::fs::read_to_string(gateway.draft("atom").join("src/Main.lex.tex"))
        .expect("the module");
    let start = source.find("banana").expect("the atom");
    let line = source[..start].matches('\n').count() + 1;
    let column = source[..start]
        .rsplit('\n')
        .next()
        .expect("a line")
        .chars()
        .count()
        + 1;
    assert_eq!(
        diagnostic["primary"],
        json!({
            "path": "src/Main.lex.tex",
            "byte_start": start,
            "byte_end": start + "banana".len(),
            "line_start": line,
            "line_end": line,
            "column_start": column,
            "column_end": column + "banana".len(),
        })
    );
}
