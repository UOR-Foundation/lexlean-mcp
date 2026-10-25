//! TL-10 and TL-12: `lexlean_explain` (F11, issue #10) is the CLI's `explain`.
//!
//! `lexlean explain <CODE>` needs no project: the entry of a registered code is a function of
//! the code, so the tool asks the host for no effect and the entries are compiled into the core
//! from LexLean's own registry (`gateway/explain/generate.py`). The tests of TL-10 call the
//! tool through the production core and the real host, as a client does. The test of TL-12 puts
//! the tool beside the `lexlean` binary of the same commit for every code the registry holds.

use gateway_tool_tests::{lexlean_sources, run_cli, Gateway, REGISTERED_CODES};
use serde_json::{json, Value};

fn is_error(result: &Value) -> bool {
    result
        .get("isError")
        .and_then(Value::as_bool)
        .unwrap_or(false)
}

fn text(result: &Value) -> &str {
    result["content"][0]["text"].as_str().expect("a text block")
}

// ---- TL-10 -----------------------------------------------------------------------------

fn diagnostic_schema() -> Value {
    json!({
        "type": "object",
        "properties": {"code": {"type": "string"}, "message": {"type": "string"}},
        "required": ["code", "message"],
    })
}

/// The output schema of the tool: the command result, with `explanation` for the success.
fn output_schema() -> Value {
    json!({
        "type": "object",
        "properties": {
            "spec": {"type": "string", "const": "lexlean/command-result/1"},
            "command": {"type": "string", "const": "explain"},
            "success": {"type": "boolean"},
            "exit_code": {"type": "integer"},
            "modules": {"type": "array", "items": {"type": "string"}},
            "artifacts": {"type": "array", "items": {"type": "string"}},
            "diagnostics": {"type": "array", "items": diagnostic_schema()},
            "explanation": {"type": "string"},
        },
        "required": ["spec", "command", "success", "exit_code", "modules", "artifacts", "diagnostics"],
        "additionalProperties": false,
    })
}

#[test]
fn lexlean_explain_is_listed_with_its_schemas_tl_10() {
    let mut gateway = Gateway::new();
    let reply = gateway.list();
    let listed = reply.result()["tools"].as_array().expect("tools").clone();
    let tool = listed
        .iter()
        .find(|tool| tool["name"] == "lexlean_explain")
        .expect("lexlean_explain is listed");
    assert_eq!(
        tool["inputSchema"],
        json!({
            "type": "object",
            "properties": {"code": {"type": "string"}},
            "required": ["code"],
            "additionalProperties": false,
        })
    );
    assert_eq!(tool["outputSchema"], output_schema());
    assert!(tool["description"].as_str().is_some_and(|d| !d.is_empty()));
    assert_eq!(
        listed
            .iter()
            .filter(|tool| tool["name"] == "lexlean_explain")
            .count(),
        1,
        "listed once"
    );
}

#[test]
fn lexlean_explain_asks_the_host_for_no_effect_tl_10() {
    let mut gateway = Gateway::new();
    let known = gateway.call("lexlean_explain", json!({"code": "LLL1004"}));
    let result = known.result();
    assert!(!is_error(result), "{}", known.body);
    assert!(
        known.effects.is_empty(),
        "the host was asked for {:?}",
        known.effects
    );
    let structured = &result["structuredContent"];
    assert_eq!(structured["command"], "explain");
    assert_eq!(structured["success"], true);
    assert_eq!(structured["exit_code"], 0);
    assert!(
        structured["explanation"].as_str().is_some_and(|entry| entry
            .starts_with("## `LLL1004` --- ")
            && entry.contains("Exit code: 1.")),
        "{structured}"
    );
    assert!(text(result).ends_with('\n'), "the file spelling of the CLI");
    assert_eq!(
        serde_json::from_str::<Value>(text(result)).expect("JSON"),
        *structured,
        "the text is the structured content"
    );

    let unknown = gateway.call("lexlean_explain", json!({"code": "LLX9999"}));
    let result = unknown.result();
    assert!(is_error(result), "{}", unknown.body);
    assert!(unknown.effects.is_empty());
    let structured = &result["structuredContent"];
    assert_eq!(structured["success"], false);
    assert_eq!(structured["exit_code"], 2);
    assert_eq!(structured["diagnostics"][0]["code"], "LLC0001");
    assert_eq!(
        structured["diagnostics"][0]["message"],
        "`LLX9999` is not a registered diagnostic code"
    );
    assert!(structured.get("explanation").is_none());
}

#[test]
fn a_call_to_lexlean_explain_that_breaks_the_schema_is_refused_tl_10() {
    let mut gateway = Gateway::new();
    let broken = [
        json!({}),
        json!({"code": 1004}),
        json!({"code": null}),
        json!({"code": ["LLL1004"]}),
        json!({"code": "LLL1004", "project": "tiny"}),
    ];
    for arguments in broken {
        let reply = gateway.call("lexlean_explain", arguments.clone());
        let result = reply.result();
        assert!(is_error(result), "{arguments}: {}", reply.body);
        assert!(
            text(result).starts_with("Invalid arguments:"),
            "{arguments}: {}",
            reply.body
        );
        assert!(result.get("structuredContent").is_none(), "{arguments}");
        assert!(reply.effects.is_empty(), "{arguments}");
    }
    let reply = gateway.call_without_arguments("lexlean_explain");
    assert!(is_error(reply.result()), "{}", reply.body);
    assert!(reply.effects.is_empty());
}

// ---- TL-12 -----------------------------------------------------------------------------

/// The `code` of every row of `model/errors.toml` of the checkout of LexLean at the pin.
fn registered_codes() -> Vec<String> {
    let path = lexlean_sources().join("model/errors.toml");
    let registry = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    registry
        .lines()
        .filter_map(|line| line.strip_prefix("code = "))
        .map(|code| code.trim().trim_matches('"').to_string())
        .collect()
}

/// Strings the registry does not register, and what is hard about writing them back: quoting,
/// escaping, controls, non-ASCII, a leading dash, the empty string, near misses of real codes.
fn unregistered(codes: &[String]) -> Vec<String> {
    let mut strings: Vec<String> = [
        "",
        " ",
        "LLX9999",
        "LLL1004 ",
        " LLL1004",
        "LLL1004\n",
        "LLL100",
        "LLL10044",
        "LL",
        "LLC0000",
        "null",
        "`",
        "`LLL1004`",
        "\"",
        "\\",
        "a\"b\\c",
        "\\\"",
        "/",
        "\u{1}",
        "\u{7}",
        "\u{8}",
        "\t",
        "\r",
        "\u{c}",
        "\u{1f}",
        "\u{7f}",
        "é",
        "e\u{301}",
        "§",
        "日本語",
        "😀",
        "\u{ffff}",
        "-",
        "-x",
        "--",
        "--help",
        "-LLL1004",
        "../../etc/passwd",
        "LLL1004 LLL1004",
    ]
    .iter()
    .map(|s| (*s).to_string())
    .collect();
    for code in codes {
        strings.push(code.to_lowercase());
        strings.push(format!("{code} "));
        strings.push(code[..code.len() - 1].to_string());
    }
    strings
}

/// Run the CLI as `lexlean explain CODE`, with `--` before a code that begins with a dash, in a
/// directory that holds no project.
fn cli_explain(directory: &std::path::Path, code: &str) -> gateway_tool_tests::Cli {
    if code.starts_with('-') {
        run_cli(directory, &["explain", "--", code])
    } else {
        run_cli(directory, &["explain", code])
    }
}

#[test]
#[ignore = "needs the lexlean binary and a checkout of LexLean at the pin: the gateway job of .github/workflows/lexlean.yml runs it with --ignored and requires the count"]
fn tool_equals_cli_on_every_registered_code_and_on_unregistered_strings_tl_12() {
    let codes = registered_codes();
    assert_eq!(
        codes.len(),
        REGISTERED_CODES,
        "the registry of the pin holds {REGISTERED_CODES} codes"
    );
    let nowhere = tempfile::tempdir().expect("a directory with no project");
    let mut gateway = Gateway::new();
    let mut failures = Vec::new();
    let mut registered = 0;
    let mut refused = 0;
    let strings: Vec<(String, bool)> = codes
        .iter()
        .map(|code| (code.clone(), true))
        .chain(unregistered(&codes).into_iter().map(|code| (code, false)))
        .collect();
    for (code, expect_registered) in &strings {
        let cli = cli_explain(nowhere.path(), code);
        let reply = gateway.call("lexlean_explain", json!({"code": code}));
        let mut found = Vec::new();
        if !cli.stderr.is_empty() {
            found.push(format!(
                "the CLI wrote to stderr: {}",
                String::from_utf8_lossy(&cli.stderr)
            ));
        }
        if cli.status != if *expect_registered { 0 } else { 2 } {
            found.push(format!("the CLI exited {}", cli.status));
        }
        if reply.body.get("error").is_some() || !reply.effects.is_empty() {
            found.push(format!(
                "a protocol error or an effect: {} {:?}",
                reply.body, reply.effects
            ));
        } else {
            let result = &reply.body["result"];
            let printed = String::from_utf8_lossy(&cli.stdout).into_owned();
            if result["content"][0]["text"].as_str() != Some(printed.as_str()) {
                found.push(format!(
                    "the text is {:?}, the CLI printed {printed:?}",
                    result["content"][0]["text"]
                ));
            }
            let parsed: Value = serde_json::from_str(&printed).expect("the CLI printed JSON");
            if result["structuredContent"] != parsed {
                found.push(format!(
                    "structuredContent is {}, the CLI printed {parsed}",
                    result["structuredContent"]
                ));
            }
            if is_error(result) != (cli.status != 0) {
                found.push(format!(
                    "isError is {} for the exit status {}",
                    is_error(result),
                    cli.status
                ));
            }
            if parsed["exit_code"] != json!(cli.status) {
                found.push(format!("exit_code is {}", parsed["exit_code"]));
            }
        }
        if *expect_registered {
            registered += 1;
        } else {
            refused += 1;
        }
        failures.extend(found.into_iter().map(|d| format!("{code:?}: {d}")));
    }
    assert!(
        failures.is_empty(),
        "{} of {} runs differ from the CLI:\n{}",
        failures.len(),
        strings.len(),
        failures.join("\n")
    );
    assert_eq!(registered, REGISTERED_CODES);
    eprintln!("TL-12: {registered} registered codes and {refused} other strings, every one equal to the CLI");
}
