#!/usr/bin/env python3
"""Render gateway/src/Application.lex.tex, the server application declaration.

The declaration is one LexLean definition of PrismPM's `Foundation.Server.V1.Model`
`ServerApplication`. Everything in it is written here except the vectors, which nobody writes:

  * `exchanges.json` holds each vector as `record` made it: the HTTP request the gateway is
    sent, the request frame the adapter hands the core, the response the core writes, and
    the effects the core asked for with the outcome the real host (gateway-binding) gave.
  * `record RUNNER` makes `exchanges.json`: it sends each request of `SPEC` through the
    committed core and the binding (the program `gateway/vectors/runner`, built by
    `prismpm build` and laid out by `gateway/lay-out.sh`), on a store that starts empty.
  * `render` writes the declaration from `exchanges.json`; `check` fails when the committed
    declaration differs from what `exchanges.json` renders.

What the declaration states about the effects is `src/Effects.lex.tex` (the ten effects the core
may ask for, in tag order) and EFFECTS.md ("The frame effect"), and the widths of the
payloads below. `crates/model/tests/gateway.rs` (GW-01) compares the effect names with the
model and the table of GATEWAY.md.

usage:
  generate.py render             write src/Application.lex.tex from exchanges.json
  generate.py check              exit 1 when src/Application.lex.tex is not that rendering
  generate.py record RUNNER      make exchanges.json with RUNNER, then render
  generate.py table              print the rows of the table of GATEWAY.md, "The effects"
"""

import json
import pathlib
import subprocess
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
GATEWAY = HERE.parent
APPLICATION = GATEWAY / "src" / "Application.lex.tex"
EXCHANGES = HERE / "exchanges.json"
FIXTURE = HERE / "fixture" / "tiny"

MODEL = "Foundation.Server.V1.Model"

# ---- the declaration ---------------------------------------------------------------

# name, request fields, result fields, failures, request maximum, result maximum.
# A kind is one of Flag, U32, U64, Text, Bytes. The first ten rows are src/Effects.lex.tex in
# tag order; the last is the frame effect.
DRAFT = 8_388_608
KEY = 1_024
EFFECTS = [
    ("store.get", [("area", "U32"), ("key", "Text")],
     [("found", "Flag"), ("content", "Bytes")],
     ["area-unknown", "key-refused", "unreadable"], 4 + 4 + KEY + 8, DRAFT + 4_096),
    ("store.put", [("area", "U32"), ("key", "Text"), ("content", "Bytes")],
     [("length", "U64")],
     ["area-unknown", "key-refused", "unwritable"], 4 + 4 + KEY + 4 + DRAFT + 8, 16),
    ("engine.check", [("project", "Text"), ("selection", "U32"), ("files", "Text")],
     [("class", "U32"), ("result", "Bytes")],
     ["project-refused", "engine-unavailable"], 65_536, 4_194_304 + 64),
    ("engine.snapshot", [("project", "Text"), ("selection", "U32"), ("files", "Text")],
     [("class", "U32"), ("result", "Bytes")],
     ["project-refused", "engine-unavailable"], 65_536, 4_194_304 + 64),
    ("engine.format", [("project", "Text"), ("selection", "U32"), ("files", "Text"), ("check_only", "Flag")],
     [("class", "U32"), ("result", "Bytes")],
     ["project-refused", "engine-unavailable"], 65_536, 4_194_304 + 64),
    ("engine.build", [("project", "Text"), ("selection", "U32"), ("files", "Text")],
     [("class", "U32"), ("result", "Bytes")],
     ["project-refused", "engine-unavailable"], 65_536, 4_194_304 + 64),
    ("verify.start", [("project", "Text"), ("selection", "U32"), ("files", "Text")],
     [("task", "Text"), ("state", "U32")],
     ["project-refused", "verifier-unavailable"], 65_536, 256),
    ("verify.control", [("task", "Text"), ("action", "U32")],
     [("state", "U32"), ("class", "U32"), ("result", "Bytes"), ("groups_ended", "U32"), ("groups_failed", "U32")],
     ["unknown-task"], 256, 65_536 + 256),
    ("lexicon.read", [("profile", "Text"), ("entry", "Text")],
     [("found", "Flag"), ("content", "Bytes")],
     ["profile-unknown", "key-refused"], 2 * (4 + KEY) + 8, 1_048_576 + 64),
    ("attestation.lookup", [("by", "U32"), ("value", "Text")],
     [("found", "Flag"), ("envelope", "Bytes")],
     ["value-refused"], 4 + 4 + 128, 1_048_576 + 64),
    ("rate.calls", [("counted", "Flag")], [("calls", "U32")], [], 8, 16),
]
FRAME_EFFECT = "rate.calls"

REQUEST_MAXIMUM = 20_971_520
RESPONSE_MAXIMUM = 20_971_520
BODY_MAXIMUM = 16_777_216
EFFECT_BUDGET = 2
PORT = 5395
VERIFY_BUILD_TIMEOUT_SECONDS = 3600

DESCRIPTION = (
    "The gateway asks its host for the store, the four calls of the LexLean Engine, the "
    "verifier, the lexicon of a profile and the lookup of an attestation (ten effects the "
    "core requests), and for one more that only the adapter asks for: how many tools/call "
    "requests the client has made in the current window, not counting the one being served.\n"
)


def hexed(value: bytes) -> str:
    return "0x" + value.hex()


def text(value: str) -> str:
    out = ['"']
    for c in value:
        out.append({'"': '\\"', "\\": "\\\\", "\n": "\\n"}.get(c, c))
    out.append('"')
    return "".join(out)


def lst(item: str, entries: list) -> str:
    body = "".join(f"(cons {entry} " for entry in entries)
    return f"{body}(nil {item})" + ")" * len(entries)


def record(name: str, fields: list) -> str:
    inner = ", ".join(f"{field} := {value}" for field, value in fields)
    return f"(record {MODEL}::{name} {{{inner}}})"


def ctor(owner: str, name: str) -> str:
    return f"(ctor {MODEL}::{owner}.{name})"


def word(value: int, kind: str = "UInt32") -> str:
    return f"({value} : {kind})"


def host_effect(effect) -> str:
    name, request, result, failures, request_max, result_max = effect
    fields = lambda items: lst(
        f"{MODEL}::HostField",
        [record("HostField", [("name", text(n)), ("kind", ctor("HostKind", k))]) for n, k in items],
    )
    return record("HostEffect", [
        ("name", text(name)),
        ("request", fields(request)),
        ("result", fields(result)),
        ("failures", lst("String", [text(f) for f in failures])),
        ("requestMaximum", word(request_max)),
        ("resultMaximum", word(result_max)),
    ])


def render(vectors: list) -> str:
    acceptance = [
        record("AcceptanceVector", [
            ("request", hexed(bytes.fromhex(v["frame_request"]))),
            ("response", hexed(bytes.fromhex(v["frame_response"]))),
        ])
        for v in vectors
    ]
    served = [
        record("ServedVector", [
            ("vector", word(index)),
            ("request", hexed(bytes.fromhex(v["http_request"]))),
            ("response", hexed(bytes.fromhex(v["http_response"]))),
        ])
        for index, v in enumerate(vectors)
    ]
    transcripts = []
    for index, v in enumerate(vectors):
        if not v["steps"]:
            continue
        steps = [
            record("HostStep", [
                ("effect", text(step["effect"])),
                ("request", hexed(bytes.fromhex(step["request"]))),
                ("outcome", hexed(bytes.fromhex(step["outcome"]))),
            ])
            for step in v["steps"]
        ]
        transcripts.append(record("HostTranscript", [
            ("vector", word(index)),
            ("steps", lst(f"{MODEL}::HostStep", steps)),
        ]))

    origins = [
        record("ServingOrigin", [("scheme", text(scheme)), ("host", text(host)), ("port", word(0))])
        for scheme in ("http", "https")
        for host in ("localhost", "127.0.0.1", "[::1]")
    ]
    frame = record("FrameSection", [
        ("name", text("tool_calls")),
        ("kind", ctor("FrameKind", "U32")),
        ("effect", text(FRAME_EFFECT)),
        ("field", text("calls")),
        ("values", "(nil String)"),
    ])
    serving = record("ServerServing", [
        ("address", text("127.0.0.1")),
        ("port", word(PORT)),
        ("connectionMaximum", word(64)),
        ("readTimeoutMillis", word(10_000)),
        ("rootStackBytes", word(8_388_608)),
        ("requestLineMaximum", word(8192)),
        ("headerCountMaximum", word(100)),
        ("headerBytesMaximum", word(65_536)),
        ("bodyMaximum", word(BODY_MAXIMUM)),
        ("origins", lst(f"{MODEL}::ServingOrigin", origins)),
        ("accept", "(nil String)"),
        ("contentTypes", lst("String", [text("application/json")])),
        ("enforcedMethods", lst("String", [text("POST")])),
        ("countedField", text("Mcp-Method")),
        ("countedValue", text("tools/call")),
        ("frame", lst(f"{MODEL}::FrameSection", [frame])),
        ("vectors", lst(f"{MODEL}::ServedVector", served)),
    ])
    host = record("HostProtocol", [
        ("protocol", text("prismpm/host-effects/1")),
        ("description", text(DESCRIPTION)),
        ("usageCode", "(64 : UInt8)"),
        ("ioCode", "(74 : UInt8)"),
        ("faultCode", "(70 : UInt8)"),
        ("effectBudget", word(EFFECT_BUDGET)),
        ("effects", lst(f"{MODEL}::HostEffect", [host_effect(e) for e in EFFECTS])),
        ("transcripts", lst(f"{MODEL}::HostTranscript", transcripts)),
    ])
    application = record("ServerApplication", [
        ("profile", text("prismpm/server-application/1")),
        ("name", text("LexLean MCP Gateway")),
        ("cargoName", text("gateway")),
        ("cargoVersion", text("0.1.0")),
        ("cargoDescription", text(
            "LexLean served as an MCP 2026-07-28 server: the protocol core and the tools that "
            "drive the LexLean Engine, over loopback HTTP.")),
        ("cargoRepository", text("https://github.com/UOR-Foundation/lexlean-mcp")),
        ("cargoHomepage", text("https://github.com/UOR-Foundation/lexlean-mcp")),
        ("core", record("ServerCore", [
            ("root", text("LexLeanMcpGateway.Gateway.gatewayRun")),
            ("package", text("lexlean_mcp_gateway_core")),
            ("version", text("0.1.0")),
        ])),
        ("acceptanceVectors", lst(f"{MODEL}::AcceptanceVector", acceptance)),
        ("requestMaximum", word(REQUEST_MAXIMUM)),
        ("responseMaximum", word(RESPONSE_MAXIMUM)),
        ("bindings", lst(f"{MODEL}::ServerBinding", [record("ServerBinding", [
            ("path", text("bindings/gateway-binding")),
            ("crateName", text("gateway-binding")),
            ("host", text("gateway_binding::Native")),
        ])])),
        ("serving", lst(f"{MODEL}::ServerServing", [serving])),
        ("verifyBuildTimeoutSeconds", word(VERIFY_BUILD_TIMEOUT_SECONDS)),
        ("host", host),
    ])
    return (
        "\\begin{lexlean}{Application}\n"
        "\\useglossary{lexlean.std.bool@1.2.0}\n"
        "\\useglossary{lexlean.std.nat@1.2.0}\n"
        "\\useglossary{prism.arch@1.0.0}\n"
        "\\importmodule{Foundation.Bytes}\n"
        f"\\importmodule{{{MODEL}}}\n"
        "\\title{Server application model}\n"
        "\\begin{semanticmodule}\n"
        "definition{\n"
        "  name: application,\n"
        "  parameters: [],\n"
        f"  result: {MODEL}::ServerApplication,\n"
        f"  body: {application}\n"
        "}\n"
        "\\end{semanticmodule}\n"
        "\\end{lexlean}\n"
    )


# ---- the vectors ---------------------------------------------------------------------

META = {
    "io.modelcontextprotocol/protocolVersion": "2026-07-28",
    "io.modelcontextprotocol/clientInfo": {"name": "gateway-vectors", "version": "1.0.0"},
    "io.modelcontextprotocol/clientCapabilities": {},
}

FIXTURE_FILES = [
    "lexlean.toml",
    "lexlean.lock",
    "lakefile.toml",
    "lean-toolchain",
    "lake-manifest.json",
    "src/Tiny.lex.tex",
]
DIGEST = "5ee76e509683c39ef3347d2587071270720c04fd7e270b6f4b1538b4d96426cc"


def fixture(path: str) -> str:
    return (FIXTURE / path).read_text(encoding="ascii")


def spec() -> list:
    """The requests, in the order they run: (name, kind, method, tool, arguments).

    The kind is what the vector shows: a request the gateway answers without any effect
    (`tools-list`, `invalid-arguments`, `unknown-tool`), or the call of a tool, which
    asks for the effect named in its name."""
    call = "tools/call"
    out = [
        ("the tools the gateway serves", "tools-list", "tools/list", None, None),
        ("a tool call without its arguments", "invalid-arguments", call, "lexlean_check", {}),
        ("a tool the gateway does not serve", "unknown-tool", call, "lexlean_unknown", {}),
    ]
    for path in FIXTURE_FILES:
        out.append((f"store.put {path}", "tool-call", call, "lexlean_draft_write",
                    {"path": f"tiny/{path}", "content": fixture(path)}))
    out += [
        ("store.get", "tool-call", call, "lexlean_draft_read", {"path": "tiny/src/Tiny.lex.tex"}),
        ("store.put refuses a key", "tool-call", call, "lexlean_draft_write",
         {"path": "../outside", "content": "x"}),
        ("engine.check", "tool-call", call, "lexlean_check", {"project": "tiny", "files": []}),
        ("engine.snapshot", "tool-call", call, "lexlean_snapshot", {"project": "tiny", "files": []}),
        ("engine.format", "tool-call", call, "lexlean_format",
         {"project": "tiny", "files": [], "check_only": True}),
        ("engine.build", "tool-call", call, "lexlean_build", {"project": "tiny", "files": []}),
        ("verify.start refuses a project", "tool-call", call, "lexlean_verify",
         {"project": "absent", "files": []}),
        ("verify.control refuses a task", "tool-call", call, "lexlean_verify_control",
         {"task": "task-9", "action": "status"}),
        ("lexicon.read refuses a profile", "tool-call", call, "lexlean_lexicon_read",
         {"profile": "absent", "entry": "entries"}),
        ("attestation.lookup finds nothing", "tool-call", call, "lexlean_attestation",
         {"by": "id", "value": DIGEST}),
    ]
    return out


REASONS = {200: "OK", 202: "Accepted", 400: "Bad Request", 403: "Forbidden", 404: "Not Found",
           405: "Method Not Allowed", 429: "Too Many Requests", 500: "Internal Server Error"}


def chunk(value: bytes) -> bytes:
    return len(value).to_bytes(4, "big") + value


def http_request(index: int, method: str, tool, arguments) -> tuple:
    params = {}
    if tool is not None:
        params["name"] = tool
        if arguments is not None:
            params["arguments"] = arguments
    params["_meta"] = META
    body = json.dumps(
        {"jsonrpc": "2.0", "id": 100 + index, "method": method, "params": params},
        separators=(",", ":"), ensure_ascii=True,
    ).encode("ascii")
    fields = [
        ("Host", "127.0.0.1"),
        ("Content-Type", "application/json"),
        ("Accept", "application/json, text/event-stream"),
        ("MCP-Protocol-Version", "2026-07-28"),
        ("Mcp-Method", method),
    ]
    if tool is not None:
        fields.append(("Mcp-Name", tool))
    fields.append(("Content-Length", str(len(body))))
    head = "POST /mcp HTTP/1.1\r\n" + "".join(f"{n}: {v}\r\n" for n, v in fields) + "\r\n"
    frame = chunk(b"POST") + chunk(b"/mcp") + len(fields).to_bytes(4, "big")
    for n, v in fields:
        frame += chunk(n.encode()) + chunk(v.encode())
    frame += chunk(body) + (0).to_bytes(4, "big")
    return head.encode("ascii") + body, frame


def http_response(frame: bytes) -> bytes:
    at = 0

    def word():
        nonlocal at
        value = int.from_bytes(frame[at:at + 4], "big")
        at += 4
        return value

    def take():
        nonlocal at
        length = word()
        value = frame[at:at + length]
        at += length
        return value

    status = word()
    fields = [(take(), take()) for _ in range(word())]
    notifications = [take() for _ in range(word())]
    body = take()
    assert at == len(frame), "trailing bytes in the response frame"
    assert not notifications, "a notification in a response frame"
    out = f"HTTP/1.1 {status} {REASONS.get(status, 'Status')}\r\n".encode()
    for n, v in fields:
        out += n + b": " + v + b"\r\n"
    out += f"Content-Length: {len(body)}\r\nConnection: close\r\n\r\n".encode() + body
    return out


def record_vectors(runner: str) -> list:
    requests = spec()
    built = [http_request(i, m, t, a) for i, (_, _, m, t, a) in enumerate(requests)]
    with tempfile.TemporaryDirectory() as store:
        environment = {"LEXLEAN_MCP_STORE": store, "PATH": "/usr/bin:/bin"}
        process = subprocess.run(
            [runner],
            input="".join(frame.hex() + "\n" for _, frame in built).encode(),
            capture_output=True, env=environment, check=False,
        )
    if process.returncode != 0:
        sys.exit(f"generate: {runner} failed: {process.stderr.decode(errors='replace')}")
    answers = [json.loads(line) for line in process.stdout.decode().splitlines()]
    if len(answers) != len(built):
        sys.exit(f"generate: {len(answers)} answers for {len(built)} requests")
    vectors = []
    for (name, kind, _, _, _), (http, frame), answer in zip(requests, built, answers):
        response = bytes.fromhex(answer["response"])
        vectors.append({
            "name": name,
            "kind": kind,
            "http_request": http.hex(),
            "frame_request": frame.hex(),
            "frame_response": response.hex(),
            "http_response": http_response(response).hex(),
            "steps": answer["steps"],
        })
    return vectors


def load() -> list:
    return json.loads(EXCHANGES.read_text())["vectors"]


def main(argv: list) -> int:
    command = argv[1] if len(argv) > 1 else "render"
    if command == "record" and len(argv) == 3:
        vectors = record_vectors(argv[2])
        EXCHANGES.write_text(json.dumps({"vectors": vectors}, indent=1) + "\n")
        command = "render"
    if command == "render":
        APPLICATION.write_text(render(load()))
        print(f"generate: wrote {APPLICATION}")
        return 0
    if command == "check":
        if APPLICATION.read_text() != render(load()):
            print(f"generate: {APPLICATION} is not the rendering of {EXCHANGES}", file=sys.stderr)
            return 1
        print("generate: the declaration is the rendering of the vectors")
        return 0
    if command == "table":
        for name, request, result, failures, _, _ in EFFECTS:
            pairs = lambda items: ", ".join(f"`{n}:{k}`" for n, k in items) or "none"
            names = ", ".join(f"`{f}`" for f in failures) or "none"
            print(f"| `{name}` | {pairs(request)} | {pairs(result)} | {names} |")
        return 0
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
