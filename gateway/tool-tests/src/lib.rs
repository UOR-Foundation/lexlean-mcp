//! The tests of the tools of F4 to F7 (issue #10) call them as a client does: a `tools/call`
//! request goes through the production core of the gateway, the effects it asks for are
//! performed by the real host (`gateway_binding::Native`) on a store in a temporary
//! directory, and the response is read back. Nothing here knows what a tool does: the core
//! does, and the CLI of the same LexLean commit is the oracle.
//!
//! This crate holds what the test files share: the gateway, the CLI runner, the trees of a
//! project and the fixtures of LexLean.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use gateway_binding::{Config, Native};
use gateway_host::exchange::{core, step, turn, Step};
use gateway_host::perform::perform;
use gateway_host::protocol::Request;
use serde_json::{json, Value};

/// The files of the tiny project that a profile holds: everything but its sources.
pub const BUNDLE: [&str; 5] = [
    "lexlean.toml",
    "lexlean.lock",
    "lakefile.toml",
    "lean-toolchain",
    "lake-manifest.json",
];

/// The source of the tiny project, as a path below its source root.
pub const SOURCE: &str = "src/Tiny.lex.tex";

/// The stack of the thread the core runs on: the declaration's `rootStackBytes`, with room.
pub const STACK_BYTES: usize = 512 << 20;

/// The tools of F4 to F7, in the order the gateway lists them.
pub const TOOLS: [&str; 4] = [
    "lexlean_check",
    "lexlean_snapshot",
    "lexlean_format",
    "lexlean_build",
];

/// The fixtures of the pinned LexLean that the register (`crates/negative-fixtures`) does not
/// hold: the register is bound to `release/v0.4` at 9796067, and the commit of
/// `gateway/mcp/lexlean.rev` has these names besides its 297.
pub const NOT_IN_THE_REGISTER: [&str; 1] = ["induction-nested-type"];

/// The examples whose `expected/build` the CLI at the pin does not reproduce (measured with the
/// CLI itself, TOOLS.md): the tool gives the same tree as the CLI for them too.
pub const EXAMPLES_OFF_THE_EXPECTED_TREE: &[&str] = &[
    "collections",
    "fold-laws",
    "helpdesk",
    "higher-order",
    "language-1.2",
    "models",
    "nested-induction",
    "production",
    "production-coverage",
    "reasoning",
    "recursion",
    "recursive-data",
];

/// The projects whose semantic snapshot is larger than the 4 MiB an engine effect may return
/// (`RESULT_MAXIMUM` of the binding): `lexlean_snapshot` answers them with class 4 (TOOLS.md).
pub const SNAPSHOTS_OVER_THE_LIMIT: &[&str] = &["uor-atlas"];

/// The codes the diagnostic registry (`model/errors.toml`) of the pinned LexLean commit holds:
/// the codes `lexlean_explain` explains, the count asserted by the test beside the CLI.
pub const REGISTERED_CODES: usize = 86;

/// The fixtures that run `check` and that issue #10 puts in F4: the register's 252 and the
/// names it lacks.
pub const F4_FIXTURES: usize = 253;

/// The fixtures that run `build` and that issue #10 puts in F7.
pub const F7_FIXTURES: usize = 1;

fn chunk(value: &[u8]) -> Vec<u8> {
    let mut out = u32::try_from(value.len())
        .expect("a length that fits a word")
        .to_be_bytes()
        .to_vec();
    out.extend_from_slice(value);
    out
}

/// The request frame the adapter hands the core for a `POST /mcp` with these members.
fn frame(method: &str, tool: Option<&str>, id: u64, params: Value) -> Vec<u8> {
    frame_with(method, tool, id, params, &[])
}

/// [`frame`] with more header fields, in the order given, before `Content-Length`.
fn frame_with(
    method: &str,
    tool: Option<&str>,
    id: u64,
    params: Value,
    more: &[(&str, &str)],
) -> Vec<u8> {
    let body = serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params,
    }))
    .expect("JSON");
    frame_of_body(method, tool, body, more)
}

/// The request frame for a body that is already bytes.
fn frame_of_body(
    method: &str,
    tool: Option<&str>,
    body: Vec<u8>,
    more: &[(&str, &str)],
) -> Vec<u8> {
    let mut fields: Vec<(&str, String)> = vec![
        ("Host", "127.0.0.1".into()),
        ("Content-Type", "application/json".into()),
        ("Accept", "application/json, text/event-stream".into()),
        ("MCP-Protocol-Version", "2026-07-28".into()),
        ("Mcp-Method", method.into()),
    ];
    if let Some(tool) = tool {
        fields.push(("Mcp-Name", tool.into()));
    }
    for (name, value) in more {
        fields.push((name, (*value).to_string()));
    }
    fields.push(("Content-Length", body.len().to_string()));
    let mut out = chunk(b"POST");
    out.extend(chunk(b"/mcp"));
    out.extend(u32::try_from(fields.len()).expect("a count").to_be_bytes());
    for (name, value) in &fields {
        out.extend(chunk(name.as_bytes()));
        out.extend(chunk(value.as_bytes()));
    }
    out.extend(chunk(&body));
    out.extend(0u32.to_be_bytes());
    out
}

/// What the gateway answered.
#[derive(Debug, Clone)]
pub struct Reply {
    /// The HTTP status of the response frame.
    pub status: u32,
    /// The JSON-RPC response, the body of the frame.
    pub body: Value,
    /// The names of the effects the core asked for, in order.
    pub effects: Vec<String>,
}

impl Reply {
    /// The `result` of a JSON-RPC response; a protocol error panics with the response.
    pub fn result(&self) -> &Value {
        assert!(
            self.body.get("error").is_none(),
            "a protocol error where a result was expected: {}",
            self.body
        );
        &self.body["result"]
    }

    /// Whether the result is a tool error.
    pub fn is_error(&self) -> bool {
        self.result()
            .get("isError")
            .and_then(Value::as_bool)
            .unwrap_or(false)
    }

    /// The structured content of a result.
    pub fn structured(&self) -> &Value {
        &self.result()["structuredContent"]
    }

    /// The text of the first content block of a result.
    pub fn text(&self) -> &str {
        self.result()["content"][0]["text"]
            .as_str()
            .expect("a text block")
    }

    /// The `error` of a JSON-RPC response; a result panics with the response.
    pub fn error(&self) -> &Value {
        assert!(
            self.body.get("result").is_none(),
            "a result where a protocol error was expected: {}",
            self.body
        );
        &self.body["error"]
    }
}

fn read_response(frame: &[u8]) -> (u32, Value) {
    let mut at = 0;
    let word = |at: &mut usize| {
        let value = u32::from_be_bytes(frame[*at..*at + 4].try_into().expect("a word"));
        *at += 4;
        value
    };
    let status = word(&mut at);
    let take = |at: &mut usize| -> Vec<u8> {
        let length = u32::from_be_bytes(frame[*at..*at + 4].try_into().expect("a word")) as usize;
        *at += 4;
        let value = frame[*at..*at + length].to_vec();
        *at += length;
        value
    };
    for _ in 0..word(&mut at) {
        take(&mut at);
        take(&mut at);
    }
    for _ in 0..word(&mut at) {
        take(&mut at);
    }
    let body = take(&mut at);
    assert_eq!(at, frame.len(), "trailing bytes in the response frame");
    (status, serde_json::from_slice(&body).expect("a JSON body"))
}

/// Run one exchange: the turns of the core and the effects the host performs, until the core
/// answers. `host` is whatever implements the host trait of the generated crate.
fn exchange<H: gateway_host_api::protocol::Host>(host: &mut H, request: &[u8]) -> Reply {
    let mut completed: Vec<(u32, Vec<u8>)> = Vec::new();
    let mut effects = Vec::new();
    loop {
        let input = turn(request, &completed).expect("a turn");
        let output = core(input).expect("the core answers");
        match step(&output).expect("a step") {
            Step::Done(response) => {
                let (status, body) = read_response(response);
                return Reply {
                    status,
                    body,
                    effects,
                };
            }
            Step::Perform(tag, payload) => {
                let effect = Request::decode(tag, payload).expect("a declared effect");
                let outcome = perform(host, effect).expect("the host performs it");
                effects.push(
                    gateway_host::protocol::EFFECTS[usize::try_from(tag).expect("a tag")]
                        .to_string(),
                );
                completed.push((tag, outcome));
            }
        }
    }
}

/// The meta of every request: the protocol version and a client.
fn meta() -> Value {
    json!({
        "io.modelcontextprotocol/protocolVersion": "2026-07-28",
        "io.modelcontextprotocol/clientInfo": {"name": "gateway-tool-tests", "version": "1.0.0"},
        "io.modelcontextprotocol/clientCapabilities": {},
    })
}

/// A request for `tools/call` with these arguments, as a frame.
pub fn call_frame(tool: &str, arguments: Option<Value>, id: u64) -> Vec<u8> {
    let mut params = json!({"name": tool, "_meta": meta()});
    if let Some(arguments) = arguments {
        params["arguments"] = arguments;
    }
    frame("tools/call", Some(tool), id, params)
}

/// A request for `tools/call` whose `arguments` are these bytes, which need not be JSON that
/// `serde_json` could write: the body is assembled around them.
pub fn call_frame_raw(tool: &str, arguments: &[u8], id: u64) -> Vec<u8> {
    let mut body = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":{id},\"method\":\"tools/call\",\"params\":{{\"name\":{},\"arguments\":",
        serde_json::to_string(tool).expect("JSON")
    )
    .into_bytes();
    body.extend_from_slice(arguments);
    body.extend_from_slice(b",\"_meta\":");
    body.extend(serde_json::to_vec(&meta()).expect("JSON"));
    body.extend_from_slice(b"}}");
    frame_of_body("tools/call", Some(tool), body, &[])
}

/// A request for any method with these parameters, as a frame. `name` is the `Mcp-Name` field
/// the method needs (the URI of `resources/read`, the name of `tools/call`), or `None`.
pub fn method_frame(method: &str, name: Option<&str>, params: Value, id: u64) -> Vec<u8> {
    let mut params = params;
    params["_meta"] = meta();
    frame(method, name, id, params)
}

/// A request for `tools/list`, as a frame.
pub fn list_frame(id: u64) -> Vec<u8> {
    frame("tools/list", None, id, json!({"_meta": meta()}))
}

/// A request for `tools/list` as a frame that carries one `Origin` field: what the adapter
/// would hand the core for a browser's request.
pub fn list_frame_with_origin(id: u64, origin: &str) -> Vec<u8> {
    frame_with(
        "tools/list",
        None,
        id,
        json!({"_meta": meta()}),
        &[("Origin", origin)],
    )
}

/// The bytes of an HTTP `POST /mcp` for `tools/list` as a client sends it to the served
/// adapter, with the `Origin` fields given (none, one, or several).
pub fn list_http_request(id: u64, origins: &[&str]) -> Vec<u8> {
    let body = serde_json::to_vec(&json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "tools/list",
        "params": {"_meta": meta()},
    }))
    .expect("JSON");
    let mut head = String::from(
        "POST /mcp HTTP/1.1\r\nHost: 127.0.0.1\r\nContent-Type: application/json\r\n\
         Accept: application/json, text/event-stream\r\nMCP-Protocol-Version: 2026-07-28\r\n\
         Mcp-Method: tools/list\r\n",
    );
    for origin in origins {
        head.push_str(&format!("Origin: {origin}\r\n"));
    }
    head.push_str(&format!("Content-Length: {}\r\n\r\n", body.len()));
    let mut out = head.into_bytes();
    out.extend(body);
    out
}

/// Run `work` on a thread with the stack the core needs and return what it returns.
pub fn on_big_stack<T: Send>(work: impl FnOnce() -> T + Send) -> T {
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(STACK_BYTES)
            .spawn_scoped(scope, work)
            .expect("a thread")
            .join()
            .expect("the thread finished")
    })
}

/// The gateway: the production core and the real host over a store in a temporary directory.
pub struct Gateway {
    host: Native,
    store: PathBuf,
    next: u64,
    _directory: tempfile::TempDir,
}

impl Default for Gateway {
    fn default() -> Gateway {
        Gateway::new()
    }
}

impl Gateway {
    /// A gateway whose store starts empty.
    pub fn new() -> Gateway {
        let directory = tempfile::tempdir().expect("a temporary directory");
        let store = directory.path().join("store");
        Gateway {
            host: Native::new(Config::at(store.clone())),
            store,
            next: 100,
            _directory: directory,
        }
    }

    /// The directory of the drafts of the store.
    pub fn drafts(&self) -> PathBuf {
        self.store.join("drafts")
    }

    /// The root of the store.
    pub fn store(&self) -> &Path {
        &self.store
    }

    /// The directory of the profile `name`.
    pub fn profile(&self, name: &str) -> PathBuf {
        self.store.join("profiles").join(name)
    }

    /// Provision the profile `name`: the bundle of the tiny project, which is every file of it
    /// but its sources, and the record `record` (`DRAFTS.md`).
    pub fn provision(&self, name: &str, record: &str) {
        let directory = self.profile(name);
        std::fs::create_dir_all(&directory).expect("a profile directory");
        for file in BUNDLE {
            std::fs::copy(tiny().join(file), directory.join(file)).expect("a bundle file");
        }
        std::fs::write(directory.join("profile"), record).expect("a record");
    }

    /// The record of a profile with this ID whose only source root is `src`.
    pub fn record(id: &str) -> String {
        format!("{id}\nsrc\n")
    }

    /// Install the project in `from` as the draft `name`.
    pub fn install(&self, name: &str, from: &Path) {
        copy_tree(from, &self.drafts().join(name));
    }

    /// The draft `name`, where the host keeps it.
    pub fn draft(&self, name: &str) -> PathBuf {
        self.drafts().join(name)
    }

    /// `tools/list`.
    pub fn list(&mut self) -> Reply {
        self.next += 1;
        let request = list_frame(self.next);
        let host = &mut self.host;
        on_big_stack(|| exchange(host, &request))
    }

    /// `tools/call` of `tool` with `arguments`.
    pub fn call(&mut self, tool: &str, arguments: Value) -> Reply {
        self.next += 1;
        let request = call_frame(tool, Some(arguments), self.next);
        let host = &mut self.host;
        on_big_stack(|| exchange(host, &request))
    }

    /// Any method of the protocol with these parameters: `name` is the `Mcp-Name` field the
    /// method needs, the URI of `resources/read` for one.
    pub fn rpc(&mut self, method: &str, name: Option<&str>, params: Value) -> Reply {
        self.next += 1;
        let request = method_frame(method, name, params, self.next);
        let host = &mut self.host;
        on_big_stack(|| exchange(host, &request))
    }

    /// Provision the project `from` as the profile `name`, as the operator's `install` does, with
    /// the review records in the files given. The profile ID it returns is the host's.
    pub fn install_profile(
        &self,
        name: &str,
        from: &Path,
        reviews: &[&Path],
    ) -> gateway_binding::provision::Installed {
        gateway_binding::provision::install(&self.store, name, from, reviews)
            .unwrap_or_else(|refused| panic!("installing {name}: {}", refused.code()))
    }

    /// `tools/call` of `tool` whose `arguments` are the bytes given.
    pub fn call_raw_arguments(&mut self, tool: &str, arguments: &[u8]) -> Reply {
        self.next += 1;
        let request = call_frame_raw(tool, arguments, self.next);
        let host = &mut self.host;
        on_big_stack(|| exchange(host, &request))
    }

    /// A request frame of the caller's own making, answered by the core with the real host.
    pub fn answer_frame(&mut self, request: &[u8]) -> Reply {
        let host = &mut self.host;
        on_big_stack(|| exchange(host, request))
    }

    /// `tools/call` with no `arguments` member at all.
    pub fn call_without_arguments(&mut self, tool: &str) -> Reply {
        self.next += 1;
        let request = call_frame(tool, None, self.next);
        let host = &mut self.host;
        on_big_stack(|| exchange(host, &request))
    }

    /// `tools/call` served by `host` instead of the real one, on this store's id sequence.
    pub fn call_with<H: gateway_host_api::protocol::Host + Send>(
        &mut self,
        host: &mut H,
        tool: &str,
        arguments: Value,
    ) -> Reply {
        self.next += 1;
        let request = call_frame(tool, Some(arguments), self.next);
        on_big_stack(|| exchange(host, &request))
    }

    /// The real host of the gateway.
    pub fn host(&mut self) -> &mut Native {
        &mut self.host
    }

    /// The configuration of the real host.
    pub fn config(&self) -> Config {
        Config::at(self.store.clone())
    }
}

/// Copy a directory tree, leaving out the build root of LexLean.
pub fn copy_tree(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("a directory");
    for entry in std::fs::read_dir(from).expect("a directory").flatten() {
        let name = entry.file_name();
        if name == ".lexlean" {
            continue;
        }
        let target = to.join(&name);
        if entry.file_type().expect("a type").is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), &target).expect("a copy");
        }
    }
}

/// Every file beneath `root`, as its path relative to `root` (with `/`) and its bytes.
pub fn tree(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn walk(base: &Path, at: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for entry in std::fs::read_dir(at).expect("a directory").flatten() {
            let path = entry.path();
            if entry.file_type().expect("a type").is_dir() {
                walk(base, &path, out);
            } else {
                let relative = path
                    .strip_prefix(base)
                    .expect("inside")
                    .to_string_lossy()
                    .replace('\\', "/");
                out.insert(relative, std::fs::read(&path).expect("a file"));
            }
        }
    }
    let mut out = BTreeMap::new();
    if root.is_dir() {
        walk(root, root, &mut out);
    }
    out
}

/// The tiny project the vectors draft: a module that checks, builds and is canonical.
pub fn tiny() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../vectors/fixture/tiny")
}

/// A project of this crate's own fixtures.
pub fn fixture(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixture")
        .join(name)
}

/// What the CLI printed and returned.
#[derive(Debug, Clone)]
pub struct Cli {
    /// The exit status of the process.
    pub status: i32,
    /// The bytes on standard output: the one canonical command result.
    pub stdout: Vec<u8>,
    /// The bytes on standard error: empty in JSON mode.
    pub stderr: Vec<u8>,
}

/// The `lexlean` binary: `LEXLEAN_CLI`, else the one on the PATH.
pub fn cli_path() -> PathBuf {
    std::env::var_os("LEXLEAN_CLI").map_or_else(|| PathBuf::from("lexlean"), PathBuf::from)
}

/// Run `lexlean --diagnostic-format json --color never <arguments>` in `directory`.
pub fn run_cli(directory: &Path, arguments: &[&str]) -> Cli {
    let output = Command::new(cli_path())
        .args(["--diagnostic-format", "json", "--color", "never"])
        .args(arguments)
        .current_dir(directory)
        .env_remove("LEXLEAN_VERIFY_WIDTH")
        .output()
        .unwrap_or_else(|error| panic!("{}: {error}", cli_path().display()));
    Cli {
        // A process that a signal ended has no exit code: it is reported as the negated signal.
        status: output
            .status
            .code()
            .or_else(|| std::os::unix::process::ExitStatusExt::signal(&output.status).map(|s| -s))
            .expect("the CLI exited or was signalled"),
        stdout: output.stdout,
        stderr: output.stderr,
    }
}

/// The checkout of LexLean at the pin: `LEXLEAN_SOURCES`, laid out by
/// `gateway/lay-out-lexlean.sh`.
pub fn lexlean_sources() -> PathBuf {
    let path = std::env::var_os("LEXLEAN_SOURCES")
        .map(PathBuf::from)
        .expect(
        "LEXLEAN_SOURCES names the checkout of LexLean that gateway/lay-out-lexlean.sh lays out",
    );
    assert!(
        path.join("tests/negative").is_dir(),
        "{} holds no tests/negative",
        path.display()
    );
    path
}

/// A fixture of `tests/negative`: its name, its command and its project.
#[derive(Debug, Clone)]
pub struct Fixture {
    /// The directory name.
    pub name: String,
    /// The `command` of `case.toml`.
    pub command: String,
    /// The `args` of `case.toml`, as written between the brackets.
    pub args: String,
    /// The `expected_exit` of `case.toml`.
    pub expected_exit: i32,
    /// The `project/` directory.
    pub project: PathBuf,
    /// The codes of `expected/diagnostics.json`, in order.
    pub expected_codes: Vec<String>,
}

/// The value of `key = value` in a `case.toml`.
fn case_value(case: &str, key: &str) -> Option<String> {
    case.lines()
        .filter_map(|line| line.split_once('='))
        .find(|(name, _)| name.trim() == key)
        .map(|(_, value)| value.trim().to_string())
}

/// The `code` of each diagnostic of `expected/diagnostics.json`.
fn expected_codes(directory: &Path) -> Vec<String> {
    let text =
        std::fs::read_to_string(directory.join("expected/diagnostics.json")).unwrap_or_default();
    serde_json::from_str::<serde_json::Value>(&text)
        .ok()
        .and_then(|value| {
            value.as_array().map(|all| {
                all.iter()
                    .filter_map(|d| d["code"].as_str().map(str::to_string))
                    .collect()
            })
        })
        .unwrap_or_default()
}

/// Every fixture of `tests/negative` that has one command and a `project/`, in name order.
pub fn fixtures(sources: &Path) -> Vec<Fixture> {
    let mut names: Vec<String> = std::fs::read_dir(sources.join("tests/negative"))
        .expect("tests/negative")
        .flatten()
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| {
            let directory = sources.join("tests/negative").join(&name);
            let case = std::fs::read_to_string(directory.join("case.toml")).expect("a case.toml");
            Fixture {
                command: case_value(&case, "command")
                    .map(|value| value.trim_matches('"').to_string())
                    .unwrap_or_default(),
                args: case_value(&case, "args").unwrap_or_default(),
                expected_exit: case_value(&case, "expected_exit")
                    .and_then(|value| value.parse().ok())
                    .unwrap_or(-1),
                project: directory.join("project"),
                expected_codes: expected_codes(&directory),
                name,
            }
        })
        .collect()
}

/// The classes of the register: fixture name to `class`, read from
/// `crates/negative-fixtures/register.toml`.
pub fn register() -> BTreeMap<String, String> {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../crates/negative-fixtures/register.toml");
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    let mut rows = BTreeMap::new();
    let mut name = None;
    for line in text.lines() {
        if let Some(value) = line.strip_prefix("name = ") {
            name = Some(value.trim_matches('"').to_string());
        } else if let Some(value) = line.strip_prefix("class = ") {
            if let Some(key) = name.take() {
                rows.insert(key, value.trim_matches('"').to_string());
            }
        }
    }
    rows
}

/// The class of a fixture of the pinned tree: the register's, else the one its command gives
/// (`check` is F4, `build` is F7, `verify` is `#11`), for a name the register lacks.
pub fn class_of(fixture: &Fixture, register: &BTreeMap<String, String>) -> String {
    if let Some(class) = register.get(&fixture.name) {
        return class.clone();
    }
    assert!(
        NOT_IN_THE_REGISTER.contains(&fixture.name.as_str()),
        "{} is in neither the register nor NOT_IN_THE_REGISTER: the pin moved",
        fixture.name
    );
    match fixture.command.as_str() {
        "check" => "F4",
        "build" => "F7",
        "verify" => "#11",
        other => panic!("{}: the command {other} has no class", fixture.name),
    }
    .to_string()
}

/// The fixtures of one class, with the register's classification.
pub fn fixtures_of(sources: &Path, class: &str) -> Vec<Fixture> {
    let register = register();
    fixtures(sources)
        .into_iter()
        .filter(|fixture| class_of(fixture, &register) == class)
        .collect()
}
