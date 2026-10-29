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
  generate.py record RUNNER [BINARY]
                                 make exchanges.json with RUNNER, then render; the store it
                                 records on is provisioned with BINARY (as `provision` does)
  generate.py provision DIRECTORY [BINARY]
                                 provision the profiles the vectors draft under into the store
                                 DIRECTORY (empty) with the operator's command `gateway-provision`
                                 (BINARY, or gateway/target/debug/gateway-provision): `record` does
                                 it for its own store, and CI does it for the store of
                                 `prismpm verify`, which replays them
  generate.py profile-id PROJECT [REVIEW...]
                                 print the profile ID of the project PROJECT with the review records
                                 REVIEW, recomputed here from the model's `Identity.profileId`
  generate.py table              print the rows of the table of GATEWAY.md, "The effects"
"""

import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile
import tomllib

HERE = pathlib.Path(__file__).resolve().parent
GATEWAY = HERE.parent
APPLICATION = GATEWAY / "src" / "Application.lex.tex"
EXCHANGES = HERE / "exchanges.json"
FIXTURES = HERE / "fixture"
FIXTURE = FIXTURES / "tiny"
VOCAB = FIXTURES / "vocab"
VOCAB_REVIEW = FIXTURES / "vocab.review"
PROVISION = GATEWAY / "target" / "debug" / "gateway-provision"

MODEL = "Foundation.Server.V1.Model"

# ---- the declaration ---------------------------------------------------------------

# name, request fields, result fields, failures, request maximum, result maximum.
# A kind is one of Flag, U32, U64, Text, Bytes. The first ten rows are src/Effects.lex.tex in
# tag order; the last is the frame effect.
DRAFT = 8_388_608
# A draft is stored as its preimage (DRAFTS.md): the content, up to `draft.max_bytes`, and the framing of
# the files, which gateway-binding's DRAFT_BLOB_HEADROOM bounds.
BLOB = DRAFT + 1_048_576
KEY = 1_024
EFFECTS = [
    # `view` and `form` are the closed fields of EF-09 (src/Effects.lex.tex: View, Form, Class). A
    # `path` names the file of a draft that a view reads; `class` says whether the sdk took the
    # bytes (0), refused them (1, and the content or rendered field holds its message) or the
    # draft has no such path (2); `rendered` is the text a form made and stored.
    ("store.get", [("area", "U32"), ("key", "Text"), ("path", "Text"), ("view", "U32")],
     [("found", "Flag"), ("content", "Bytes"), ("class", "U32")],
     ["area-unknown", "key-refused", "unreadable", "view-unknown"],
     4 + 4 + KEY + 4 + KEY + 4 + 8, BLOB + 4_096),
    ("store.put", [("area", "U32"), ("key", "Text"), ("content", "Bytes"), ("form", "U32")],
     [("length", "U64"), ("class", "U32"), ("rendered", "Bytes")],
     ["area-unknown", "key-refused", "unwritable", "form-unknown"],
     4 + 4 + KEY + 4 + BLOB + 4 + 8, BLOB + 4_096),
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
    # The entries that the protocol core needs to serve tasks (TASKHOST.md). The core keeps no state:
    # the adapter gives it the time, an identifier a task may take and the table of tasks in the frame
    # (the three frame effects), and the core asks the host to keep the table it changed (the effect
    # task.store, whose tag is the `taskStoreTag` of src/Gateway.lex.tex). The maximums are those of
    # MCP's own conformance declaration.
    ("clock.now", [], [("ms", "U64"), ("iso", "Text")], [], 8, 64),
    ("task.id", [], [("id", "Text")], [], 8, 128),
    ("task.table", [], [("table", "Bytes")], [], 8, 524_296),
    ("task.store", [("table", "Bytes")], [], [], 524_296, 8),
]
FRAME_EFFECT = "rate.calls"
# The sections of the request frame after the body, in the order the core reads them (MCP's
# LIFECYCLE.md, "The boundary encoding"): name, kind, the effect that fills it, its field.
FRAME_SECTIONS = [
    ("tool_calls", "U32", "rate.calls", "calls"),
    ("now_ms", "U64", "clock.now", "ms"),
    ("now", "Text", "clock.now", "iso"),
    ("fresh_task_id", "Text", "task.id", "id"),
    ("task_table", "Bytes", "task.table", "table"),
]

REQUEST_MAXIMUM = 20_971_520
RESPONSE_MAXIMUM = 20_971_520
BODY_MAXIMUM = 16_777_216
# A tool that makes a draft from a base draft asks four effects: the record of the profile, the base,
# the render of the module (store.put with a form) and the draft (store.put).
EFFECT_BUDGET = 4
PORT = 5395
# The bound `prismpm verify` puts on `cargo build --release --locked --offline` of the staged
# workspace (the binding, which links lexlean, the core of about 14 000 lines, the two host
# packages and the vector runner), cold. Measured on this repository's 2-core, 7 GB development
# machine, shared with other jobs: 238 s wall (398 s CPU) for that command on a fresh target
# directory. The bound is that measure with a factor of about 7.5.
VERIFY_BUILD_TIMEOUT_SECONDS = 1800

DESCRIPTION = (
    "The gateway asks its host for the store, the four calls of the LexLean Engine, the "
    "verifier, the lexicon of a profile and the lookup of an attestation (ten effects the "
    "core requests), for one that only the adapter asks for: how many tools/call "
    "requests the client has made in the current window, not counting the one being served, "
    "for the three that give the core what it cannot compute for the tasks it serves (the "
    "time, an identifier and the table of tasks), and for the one the core asks to keep that "
    "table (task.store).\n"
)


# LexLean lowers a byte literal to an array with one nested `let` per repeated byte value, and
# Lean stops at its recursion depth when such a literal sits inside the nested records and lists
# of the application. So every distinct literal is a definition of its own with no parameter
# (PrismPM reads a closed call of such a definition as an alias of its body, and nothing else
# as a value, so a literal is never an append of pieces).
LITERALS: dict = {}


def hexed(value: bytes) -> str:
    """A reference to the definition that holds `value` (made once per distinct value)."""
    if value not in LITERALS:
        LITERALS[value] = (f"bytes{len(LITERALS):03d}", "0x" + value.hex())
    return f"({LITERALS[value][0]})"


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
    LITERALS.clear()
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
    frames = [
        record("FrameSection", [
            ("name", text(name)),
            ("kind", ctor("FrameKind", kind)),
            ("effect", text(effect)),
            ("field", text(field)),
            ("values", "(nil String)"),
        ])
        for name, kind, effect, field in FRAME_SECTIONS
    ]
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
        ("frame", lst(f"{MODEL}::FrameSection", frames)),
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
    def literal_definition(name: str, body: str) -> str:
        # `lexlean fmt` writes a definition on one line when the line is at most 100 columns.
        line = f"definition{{name: {name}, parameters: [], result: Bytes, body: {body}}}"
        if len(line) <= 100:
            return line + "\n\n"
        return f"definition{{\n  name: {name},\n  parameters: [],\n  result: Bytes,\n  body: {body}\n}}\n\n"

    literal_definitions = "".join(literal_definition(name, body) for name, body in LITERALS.values())
    return (
        "\\begin{lexlean}{Application}\n"
        "\\useglossary{lexlean.std.bool@1.2.0}\n"
        "\\useglossary{lexlean.std.nat@1.2.0}\n"
        "\\useglossary{prism.arch@1.0.0}\n"
        "\\importmodule{Foundation.Bytes}\n"
        f"\\importmodule{{{MODEL}}}\n"
        "\\title{Server application model}\n"
        "\\begin{semanticmodule}\n"
        f"{literal_definitions}"
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

# The instant of the vectors (the host's clock, given in the frame) and the tasks that the
# vectors of the task methods find in the table the host gives (TASKHOST.md). The task `t-lost`
# is one that the verifier does not know, as after a restart of the verifier.
NOW_MS = 1_800_000_000_000
NOW_ISO = "2027-01-15T08:00:00.000Z"
CREATED_ISO = "2027-01-15T07:59:00.000Z"
TASKS_META = dict(META)
TASKS_META["io.modelcontextprotocol/clientCapabilities"] = {
    "extensions": {"io.modelcontextprotocol/tasks": {}}}
LOST_TABLE = json.dumps([{
    "task": {"taskId": "t-lost", "status": "working", "createdAt": CREATED_ISO,
             "lastUpdatedAt": CREATED_ISO, "ttlMs": 86_400_000, "pollIntervalMs": 5_000},
    "createdMs": NOW_MS - 60_000,
    "plan": {"parkAfterMs": 10 ** 15, "parked": {}, "finishAfterMs": 10 ** 15,
             "outcome": {"completes": {}}},
}], separators=(",", ":")).encode()
# For each vector that is given a table or a client that declares the tasks extension: its _meta
# and its table.
TASK_VECTORS = {
    "task.store: a task that the verifier lost is recorded": (TASKS_META, LOST_TABLE),
    "tasks/get: a task that the table lacks": (TASKS_META, LOST_TABLE),
}

# The profile `tiny` of the vectors is provisioned by `gateway-provision install`, as an operator
# does it, from the fixture project `tiny`; the vectors draft under it.
SOURCE = "src/Tiny.lex.tex"
TEMPLATE = "lexlean://profiles/{profile}/lexicon/{package}@{version}"
PACKAGE_URI = "lexlean://profiles/vocab/lexicon/shapes.words@1.0.0"
MISSING_URI = "lexlean://profiles/vocab/lexicon/no.such@1.0.0"
BROKEN_RECORD = "not the id of a profile\nsrc\n"
# A source the Engine refuses (class 1): the project of the vectors after this write is the one
# whose command result a refused check and a refused snapshot carry (issue #10).
REFUSED = "\\begin{lexlean}{Tiny}\nnonsense\n\\end{lexlean}\n"
DIGEST = "5ee76e509683c39ef3347d2587071270720c04fd7e270b6f4b1538b4d96426cc"
# The header and the data of the modules the assemble vectors make: an empty semantic module, which
# `sdk::assemble` and `sdk::write_module` accept.
HEADER = {"name": "Other", "glossaries": ["lexlean.std.bool@1.2.0", "lexlean.std.nat@1.2.0"],
          "imports": [], "title": "Other"}
EMPTY_MODULE = {"spec": "lexlean/semantic-module/2", "declarations": []}


def fixture(path: str) -> str:
    return (FIXTURE / path).read_text(encoding="ascii")


def frame(head: str, payload: bytes) -> bytes:
    """Identity.encFrame: the head with its length, the length of the payload in 8 octets, the payload."""
    name = head.encode("ascii")
    return len(name).to_bytes(4, "big") + name + len(payload).to_bytes(8, "big") + payload


def draft_preimage(profile: bytes, rows: dict) -> bytes:
    """Identity.encDraft over the rows sorted by path (the octets of the path decide)."""
    out = frame("domain", b"lexlean-mcp-draft-v1") + frame("profile", profile)
    out += len("sources".encode()).to_bytes(4, "big") + b"sources" + (2 * len(rows)).to_bytes(8, "big")
    for path in sorted(rows, key=lambda p: p.encode("utf-8")):
        out += frame("source", path.encode("utf-8")) + frame("source", rows[path].encode("utf-8"))
    return out


def draft_handle(profile: str, rows: dict) -> str:
    """The handle of a draft: its profile name and the SHA-256 of its preimage."""
    return f"{profile}/{hashlib.sha256(draft_preimage(PROFILE_ID, rows)).hexdigest()}"


# ---- the identity of a profile ---------------------------------------------------------

PROFILE_DOMAIN = b"lexlean-mcp-profile-v1"


def lock_toolchain(lock: bytes) -> bytes:
    """The toolchain of the identity: the header of the lock, as the host reads it."""
    header = {}
    for line in lock.decode("utf-8").split("\n"):
        if line.startswith("["):
            break
        key, separator, value = line.partition(" = ")
        if separator and value.startswith('"') and value.endswith('"'):
            header[key] = value[1:-1]
    return (f"compiler_semantics {header['compiler_semantics']}\n"
            f"lean_toolchain {header['lean_toolchain']}\n").encode("ascii")


def profile_rows(project: pathlib.Path) -> dict:
    """The profile files of a project (PF-03): its configuration, its lock, the files of its local
    lexicon packages and its artifact sources, by project-relative path. A source file is none."""
    config = tomllib.loads((project / "lexlean.toml").read_text(encoding="utf-8"))
    paths = {"lexlean.toml", config["lockfile"]}
    for source in config.get("lexicon_source", []):
        if source["kind"] == "path":
            root = project / source["path"]
            paths.update(p.relative_to(project).as_posix() for p in root.rglob("*") if p.is_file())
    for artifact in config.get("artifact_source", []):
        paths.add(artifact["path"])
    return {path: (project / path).read_bytes() for path in paths}


def listed(count_head: str, item_head: str, items: list) -> bytes:
    """Identity.encList: the head of the count, the count in 8 octets and the framed items."""
    name = count_head.encode("ascii")
    out = len(name).to_bytes(4, "big") + name + len(items).to_bytes(8, "big")
    for item in items:
        out += frame(item_head, item)
    return out


def profile_preimage(rows: dict, reviews: list, toolchain: bytes) -> bytes:
    """Identity.encProfile: the domain, the files sorted by the octets of the path, the review
    records sorted by their octets and the toolchain."""
    if len(set(reviews)) != len(reviews):
        raise ValueError("DuplicateReview")
    files = []
    for path in sorted(rows, key=lambda p: p.encode("utf-8")):
        files += [path.encode("utf-8"), rows[path]]
    return (frame("domain", PROFILE_DOMAIN) + listed("files", "file", files)
            + listed("reviews", "review", sorted(reviews)) + frame("toolchain", toolchain))


def profile_id(project: pathlib.Path, reviews: list) -> bytes:
    """The profile ID of a project: the SHA-256 of its preimage, recomputed without the host."""
    lock = (project / tomllib.loads((project / "lexlean.toml").read_text(encoding="utf-8"))["lockfile"])
    preimage = profile_preimage(profile_rows(project), reviews, lock_toolchain(lock.read_bytes()))
    return hashlib.sha256(preimage).digest()


# The ID of the profile `tiny` is the real one, recomputed from its fixture; the host records the
# same one when it installs the fixture, and `record` checks that it did.
PROFILE_ID = profile_id(FIXTURE, [])
PROFILE_RECORD = PROFILE_ID.hex() + "\nsrc\n"


def install(binary: pathlib.Path, store: pathlib.Path, name: str, project: pathlib.Path, reviews: list) -> bytes:
    """`gateway-provision install`, as the operator runs it: the ID it prints is the one it recorded."""
    command = [str(binary), "install", "--store", str(store), name, str(project)]
    for review in reviews:
        command += ["--review", str(review)]
    done = subprocess.run(command, capture_output=True, check=False)
    if done.returncode != 0:
        sys.exit(f"generate: {' '.join(command)}: {done.stderr.decode(errors='replace').strip()}")
    return bytes.fromhex(done.stdout.decode().strip())


def provision(store: pathlib.Path, binary: pathlib.Path = PROVISION) -> None:
    """The profiles of the vectors, provisioned by the operator's command: `tiny` and `vocab` are
    installed from their fixture projects, and `broken` is a record that is not one."""
    recorded = install(binary, store, "tiny", FIXTURE, [])
    if recorded != PROFILE_ID:
        sys.exit(f"generate: the host recorded {recorded.hex()} for tiny, the generator {PROFILE_ID.hex()}")
    expected = profile_id(VOCAB, [VOCAB_REVIEW.read_bytes()])
    recorded = install(binary, store, "vocab", VOCAB, [VOCAB_REVIEW])
    if recorded != expected:
        sys.exit(f"generate: the host recorded {recorded.hex()} for vocab, the generator {expected.hex()}")
    broken = store / "profiles" / "broken"
    broken.mkdir(parents=True)
    (broken / "profile").write_text(BROKEN_RECORD, encoding="ascii")


def spec() -> list:
    """The requests, in the order they run: (name, kind, method, tool, arguments).

    The kind is what the vector shows: a request the gateway answers without any effect
    (`tools-list`, `invalid-arguments`, `unknown-tool`, `explain`), or the call of a tool, which
    asks for the effect named in its name."""
    call = "tools/call"
    tiny = {SOURCE: fixture(SOURCE)}
    handle = draft_handle("tiny", tiny)
    refused = draft_handle("tiny", {SOURCE: REFUSED})
    out = [
        ("the tools the gateway serves", "tools-list", "tools/list", None, None),
        ("a tool call without its arguments", "invalid-arguments", call, "lexlean_check", {}),
        ("a tool the gateway does not serve", "unknown-tool", call, "lexlean_unknown", {}),
        ("lexicon.read and store.put: a draft is written", "tool-call", call, "lexlean_draft_write",
         {"profile": "tiny", "files": [{"path": SOURCE, "content": fixture(SOURCE)}]}),
        ("store.get: the files of a draft", "tool-call", call, "lexlean_draft_read", {"draft": handle}),
        ("store.get: a file of a draft", "tool-call", call, "lexlean_draft_read",
         {"draft": handle, "path": SOURCE}),
        ("store.get refuses a key", "tool-call", call, "lexlean_draft_read", {"draft": "../" + handle.split("/")[1]}),
        ("lexicon.read refuses a profile", "tool-call", call, "lexlean_draft_write",
         {"profile": "absent", "files": [{"path": SOURCE, "content": "x"}]}),
        ("a path with .. is refused before store.put", "tool-call", call, "lexlean_draft_write",
         {"profile": "tiny", "files": [{"path": "src/../Tiny.lex.tex", "content": "x"}]}),
        ("a record that is not a profile record is refused", "tool-call", call, "lexlean_draft_write",
         {"profile": "broken", "files": [{"path": SOURCE, "content": "x"}]}),
        ("engine.check", "tool-call", call, "lexlean_check", {"project": handle, "files": []}),
        ("engine.snapshot", "tool-call", call, "lexlean_snapshot", {"project": handle, "files": []}),
        ("engine.format", "tool-call", call, "lexlean_format",
         {"project": handle, "files": [], "check_only": True}),
        ("engine.build", "tool-call", call, "lexlean_build", {"project": handle, "files": []}),
        ("store.put a source the engine refuses", "tool-call", call, "lexlean_draft_write",
         {"profile": "tiny", "files": [{"path": SOURCE, "content": REFUSED}]}),
        ("engine.check refuses the project", "tool-call", call, "lexlean_check",
         {"project": refused, "files": []}),
        ("engine.snapshot refuses the project", "tool-call", call, "lexlean_snapshot",
         {"project": refused, "files": []}),
        ("engine.check refuses a draft that is absent", "tool-call", call, "lexlean_check",
         {"project": "tiny/" + "0" * 64, "files": []}),
        ("verify.start refuses a project", "tool-call", call, "lexlean_verify",
         {"project": "absent", "files": []}),
        ("verify.control refuses a task", "tool-call", call, "lexlean_verify_control",
         {"task": "task-9", "action": "status"}),
        ("lexicon.read refuses a profile", "tool-call", call, "lexlean_lexicon_read",
         {"profile": "absent", "entry": "entries"}),
        ("attestation.lookup finds nothing", "tool-call", call, "lexlean_attestation",
         {"by": "id", "value": DIGEST}),
        # The structured read and the assemble tool (issue #9): store.get with a view, and store.put
        # with a form that the host renders by the sdk.
        ("store.get with the view 1: the data of a file", "tool-call", call, "lexlean_draft_read",
         {"draft": handle, "path": SOURCE, "view": "data"}),
        ("store.get with the view 2: the spans of a file", "tool-call", call, "lexlean_draft_read",
         {"draft": handle, "path": SOURCE, "view": "spans"}),
        ("store.put with the form 1: a module is assembled", "tool-call", call, "lexlean_draft_assemble",
         {"profile": "tiny", "path": "src/Other.lex.tex", "kind": "semantic", "header": HEADER,
          "data": EMPTY_MODULE}),
        ("store.put with the form 2: a module is rewritten", "tool-call", call, "lexlean_draft_assemble",
         {"profile": "tiny", "draft": handle, "path": SOURCE, "data": EMPTY_MODULE}),
        ("store.put with the form 1: the sdk refuses a module", "tool-call", call, "lexlean_draft_assemble",
         {"profile": "tiny", "path": SOURCE, "kind": "semantic", "header": HEADER,
          "data": {"spec": "lexlean/semantic-module/9"}}),
        # An explanation is a function of the code, compiled into the core: no effect is asked.
        ("lexlean_explain a registered code", "explain", call, "lexlean_explain",
         {"code": "LLL1004"}),
        ("lexlean_explain a code the registry lacks", "explain", call, "lexlean_explain",
         {"code": "LLX\"9999\\"}),
        # The profiles and their vocabulary (issue #8): the catalogue is a store.get, the vocabulary
        # of a profile a lexicon.read, and the resource template and its completion ask the same two.
        ("store.get: the profiles the gateway serves", "tool-call", call, "lexlean_profiles", {}),
        ("store.get: a profile that is not installed", "tool-call", call, "lexlean_profiles",
         {"profile": "absent"}),
        ("lexicon.read: the vocabulary of a profile is searched", "tool-call", call,
         "lexlean_lexicon_search", {"profile": "vocab", "query": "shapes"}),
        ("lexicon.read: a profile without a vocabulary", "tool-call", call, "lexlean_lexicon_search",
         {"profile": "broken"}),
        ("resources/templates/list: the template of the profiles", "method", "resources/templates/list",
         None, {}),
        ("store.get: the resources of the profiles", "method", "resources/list", None, {}),
        ("lexicon.read: a package of a profile is read", "method", "resources/read", PACKAGE_URI,
         {"uri": PACKAGE_URI}),
        ("lexicon.read: a package that the profile lacks", "method", "resources/read", MISSING_URI,
         {"uri": MISSING_URI}),
        ("store.get: the profiles are completed", "method", "completion/complete", None,
         {"ref": {"type": "ref/resource", "uri": TEMPLATE},
          "argument": {"name": "profile", "value": "v"}}),
        # The tasks (TASKHOST.md): a task that the table holds as working and that the verifier no
        # longer knows is asked for (verify.control), found lost, and the table is kept (task.store).
        ("task.store: a task that the verifier lost is recorded", "method", "tasks/get", "t-lost",
         {"taskId": "t-lost"}),
        ("tasks/get: a task that the table lacks", "method", "tasks/get", "t-absent",
         {"taskId": "t-absent"}),
        ("tasks/get: a client that does not declare the tasks extension", "method", "tasks/get", "t-lost",
         {"taskId": "t-lost"}),
    ]
    return out


REASONS = {200: "OK", 202: "Accepted", 400: "Bad Request", 403: "Forbidden", 404: "Not Found",
           405: "Method Not Allowed", 429: "Too Many Requests", 500: "Internal Server Error"}


def chunk(value: bytes) -> bytes:
    return len(value).to_bytes(4, "big") + value


def http_request(index: int, method: str, tool, arguments, name: str = "") -> tuple:
    """The request of one vector. A call of a tool names it and gives its arguments; any other
    method is given its parameters whole in `arguments`, and `tool` is then the value of its
    Mcp-Name header when the method has one (`resources/read` names the URI)."""
    params = {}
    if method == "tools/call":
        if tool is not None:
            params["name"] = tool
            if arguments is not None:
                params["arguments"] = arguments
    elif arguments is not None:
        params.update(arguments)
    meta, table = TASK_VECTORS.get(name, (META, b""))
    params["_meta"] = meta
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
    # The host sections after the count of the tool calls (the request frame of the core): the
    # clock, an identifier a task may take, and the table.
    frame += NOW_MS.to_bytes(8, "big") + chunk(NOW_ISO.encode()) + chunk(b"") + chunk(table)
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


def record_vectors(runner: str, binary: pathlib.Path = PROVISION) -> list:
    requests = spec()
    built = [http_request(i, m, t, a, n) for i, (n, _, m, t, a) in enumerate(requests)]
    with tempfile.TemporaryDirectory() as store:
        provision(pathlib.Path(store), binary)
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
    # The identity of a draft is computed here, independently of the core, to name the projects of the
    # later requests: the core must have answered with the same handle.
    written = json.loads(bytes.fromhex(vectors[3]["http_response"]).split(b"\r\n\r\n", 1)[1])
    expected = draft_handle("tiny", {SOURCE: fixture(SOURCE)})
    got = written["result"]["structuredContent"]["draft"]
    if got != expected:
        sys.exit(f"generate: the core named the draft {got}, the generator {expected}")
    # The sdk accepts the empty module of the assemble vectors and refuses the last one.
    expected_error = {"store.put with the form 1: the sdk refuses a module": True,
                      "store.put with the form 1: a module is assembled": False,
                      "store.put with the form 2: a module is rewritten": False,
                      "store.get with the view 1: the data of a file": False,
                      "store.get with the view 2: the spans of a file": False}
    for vector in vectors:
        if vector["name"] in expected_error:
            body = json.loads(bytes.fromhex(vector["http_response"]).split(b"\r\n\r\n", 1)[1])
            failed = body["result"].get("isError", False)
            if failed != expected_error[vector["name"]]:
                sys.exit(f"generate: {vector['name']}: isError is {failed}: {body}")
    return vectors


def load() -> list:
    return json.loads(EXCHANGES.read_text())["vectors"]


def main(argv: list) -> int:
    command = argv[1] if len(argv) > 1 else "render"
    if command == "profile-id" and len(argv) >= 3:
        project = pathlib.Path(argv[2])
        print(profile_id(project, [pathlib.Path(r).read_bytes() for r in argv[3:]]).hex())
        return 0
    if command == "record" and len(argv) in (3, 4):
        vectors = record_vectors(argv[2], pathlib.Path(argv[3]) if len(argv) == 4 else PROVISION)
        EXCHANGES.write_text(json.dumps({"vectors": vectors}, indent=1) + "\n")
        command = "render"
    if command == "provision" and len(argv) in (3, 4):
        store = pathlib.Path(argv[2])
        if not store.is_dir() or any(store.iterdir()):
            print(f"generate: {store} is not an empty directory", file=sys.stderr)
            return 1
        provision(store, pathlib.Path(argv[3]) if len(argv) == 4 else PROVISION)
        print(f"generate: provisioned the profiles of the vectors in {store}")
        return 0
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
