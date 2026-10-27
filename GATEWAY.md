# The gateway

Issue #7 asks for the generated host adapter and the `gateway` binary. `gateway/` is that
half: a LexLean production root, the PrismPM server application that declares it, the
binding that implements its effects, and the vectors that tie the three together. The
declaration half is `src/Effects.lex.tex` and EFFECTS.md (EF-01 to EF-08); this file
describes the generated half, registered as `GW-01` to `GW-12` (suite `gateway`), held to
the declaration by `cargo test -p repo-model --test gateway`.

## What is where

| Path | What it is |
| --- | --- |
| `gateway/src/Effects.lex.tex` | the declaration half, byte for byte (`src/Effects.lex.tex`); the single source of the ten effects |
| `gateway/src/Gateway.lex.tex`, `GatewayTools.lex.tex`, `HostWire.lex.tex` | the tool layer and the production root `LexLeanMcpGateway.Gateway.gatewayRun` |
| `gateway/src/Drafts.lex.tex`, `DraftAdmission.lex.tex`, `Nfc.lex.tex`, `NfcTables.lex.tex`, `gateway/artifacts` | the draft tools (DRAFTS.md): `Drafts` is the identity, the profile record, the storage, the reading (as text, as data and as spans) and the assembly of a module; the other three are `src/Admission.lex.tex` (module name and title aside), `src/Nfc.lex.tex`, `src/NfcTables.lex.tex` and the tables they embed, byte for byte, so that the core admits a path with the exported admission (DR-03) |
| `gateway/src/Vocabulary.lex.tex`, `ProfileLanguage.lex.tex`, `gateway/bindings/gateway-binding/src/{identity,provision}.rs`, `src/bin/gateway-provision.rs` | profiles and vocabulary (PROFILES.md): the tools `lexlean_profiles` and `lexlean_lexicon_search`, the resource template, completion and the capabilities `resources` and `completions` (`Vocabulary`); the language rule, a copy of the declarations of `src/Profiles.lex.tex` that a test holds byte for byte (`ProfileLanguage`); the profile identity and the operator's provisioning command (`identity`, `provision`, `gateway-provision`), which are no effect |
| `gateway/src/Application.lex.tex` | the server application: the eleven effects, the limits, the serving surface and the vectors; rendered by `gateway/vectors/generate.py` |
| `gateway/mcp/src`, `gateway/language/mcp.core` | the protocol core of UOR-Foundation/MCP at `.github/mcp.rev`, laid out by `gateway/lay-out-mcp.sh`; not committed |
| `gateway/bindings/gateway-binding` | the binding crate: `Native` implements the generated `Host` trait, and nothing else |
| `gateway/vectors` | `generate.py`, `exchanges.json` (the recorded vectors), `runner` (the recorder) and `fixture/tiny` (the project the vectors draft) |
| `gateway/generated` | the production request and the committed export of the core |
| `gateway/gateway-host-api`, `gateway/gateway-host`, `gateway/lexlean_mcp_gateway_core` | what `prismpm build` generates and `gateway/lay-out.sh` lays out; not committed |
| `gateway/registry.sh` | the local registry the workspace is built against (CI.md, "The gateway job") |

## The core

One LexLean production root, `gatewayRun : Bytes -> List Nat`, over the modules of MCP and
the tool layer. The root is `StepServe.serveStep` (MCP#39) over the gateway's provider: it
reads PrismPM's turn, runs every check MCP's core runs before a provider, asks the provider
with the effects completed so far, and writes PrismPM's step. A tool is a function from its
arguments and the outcomes of the effects already performed to a typed outcome: a result, a
refusal, or the typed request for an effect (`ProviderRefusal.effectRequested`, which the
core carries to the root unchanged). No marker travels in a tool result, so none can
collide with what a tool returns, and the core stays a pure function that the driver
replays. The gateway links no MCP crate; its modules are the sources at the pin.

The gateway is compiled by the LexLean commit that MCP is checked with at the pin
(`.github/lexlean.rev` of MCP, laid out as `gateway/mcp/lexlean.rev`), not by the root
project's commit: `gateway/lexlean.lock` records the same `compiler_semantics` as MCP's
`lexlean.lock`. The gateway job builds `lexlean` and the `lexlean` that `prismpm` vendors
from that commit (`LEXLEAN_REV_FILE`), and the binding links the `lexlean` crate packaged
from it. Its project allows a Lean child `child_timeout_ms = 1200000` (twenty minutes), the
limit of MCP's own projects (MCP#42): a production build of the core exceeded 300000 on a
shared runner. The limit is part of `project_config_sha256`, so `gateway/lexlean.lock` is made
again with the compiler of `gateway/mcp/lexlean.rev` whenever it moves; the root project keeps
`child_timeout_ms = 300000`, which OWNER-VALUES.md ties to the objective `slo.S4`. `gateway/mcp-identities.sh` holds the laid-out modules and the compiler to the pin
(AC-03, part a): every file equals the file of MCP at `.github/mcp.rev`, the `mcp.core`
lexicon has the tree hash MCP's lock records, and the two locks record one compiler.

## The effects

The ten effects of the model, in tag order, and the frame effect. A field is `name:Kind`;
the kinds are `Flag`, `U32`, `U64`, `Text` and `Bytes`; a failure is a declared name.

| Effect | Request | Result | Failures |
| --- | --- | --- | --- |
| `store.get` | `area:U32`, `key:Text`, `path:Text`, `view:U32` | `found:Flag`, `content:Bytes`, `class:U32` | `area-unknown`, `key-refused`, `unreadable`, `view-unknown` |
| `store.put` | `area:U32`, `key:Text`, `content:Bytes`, `form:U32` | `length:U64`, `class:U32`, `rendered:Bytes` | `area-unknown`, `key-refused`, `unwritable`, `form-unknown` |
| `engine.check` | `project:Text`, `selection:U32`, `files:Text` | `class:U32`, `result:Bytes` | `project-refused`, `engine-unavailable` |
| `engine.snapshot` | `project:Text`, `selection:U32`, `files:Text` | `class:U32`, `result:Bytes` | `project-refused`, `engine-unavailable` |
| `engine.format` | `project:Text`, `selection:U32`, `files:Text`, `check_only:Flag` | `class:U32`, `result:Bytes` | `project-refused`, `engine-unavailable` |
| `engine.build` | `project:Text`, `selection:U32`, `files:Text` | `class:U32`, `result:Bytes` | `project-refused`, `engine-unavailable` |
| `verify.start` | `project:Text`, `selection:U32`, `files:Text` | `task:Text`, `state:U32` | `project-refused`, `verifier-unavailable` |
| `verify.control` | `task:Text`, `action:U32` | `state:U32`, `class:U32`, `result:Bytes`, `groups_ended:U32`, `groups_failed:U32` | `unknown-task` |
| `lexicon.read` | `profile:Text`, `entry:Text` | `found:Flag`, `content:Bytes` | `profile-unknown`, `key-refused` |
| `attestation.lookup` | `by:U32`, `value:Text` | `found:Flag`, `envelope:Bytes` | `value-refused` |
| `rate.calls` | `counted:Flag` | `calls:U32` | none |

The first ten rows are `src/Effects.lex.tex` (GW-01 holds the names to the model and the
order to the table). `rate.calls` is the eleventh and is not the core's: EFFECTS.md, "The
frame effect". A `class` is `0` for a result, `1` for a language error of the draft, `2`
for a configuration or selection error and `4` for a limit; an environment or internal
error of the Engine is not a class, it is the failure `engine-unavailable`. A `class` is
the exit code of the CLI, and the `result` of `engine.check`, `engine.format` and
`engine.build` is the bytes the CLI prints with `--diagnostic-format json` (TOOLS.md); that of
`engine.snapshot` is the canonical snapshot, or, for a project that does not check, the same
refusal under the command name `snapshot`. A `state` is
`0` queued, `1` running, `2` verified, `3` failed, `4` cancelled. An `area` is the position
of a directory in the store (`profiles`, `drafts`, `builds`, `verifications`, `index`,
`exports`, `keys`); a key is segments of `[A-Za-z0-9._-]` not starting with `.`, joined by
`/`, at most 1024 bytes.

`store.get` and `store.put` carry the closed fields of EF-09 (`src/Effects.lex.tex`: `View`,
`Form`, `Class`, each with a total decoder and a typed refusal for a number past its last
constructor), and no eleventh effect carries them. `view` is `0` for the bytes as stored, `1` for
the module of `sdk::read_module` as JSON and `2` for the byte ranges of `sdk::declaration_spans`
as JSON (`null` for a module written as data); `path`, when it is not empty, names the file of
the draft at `key` that the view reads, and when it is empty the view reads the bytes at `key`.
`form` is `0` to store the content as given, `1` to render `{kind, header, data}` with
`sdk::assemble` and `2` to render `{text, data}` with `sdk::write_module` before storing the text.
A `class` of these two effects is `0` when the bytes were taken, `1` when the sdk refused them
(the `content` of `store.get` and the `rendered` of `store.put` are then its message and nothing
is stored) and `2` when the draft has no such `path`. A `view` or a `form` outside `0` to `2` is
the failure `view-unknown` or `form-unknown`, and is refused before the store is touched and
before the sdk is reached. The host reaches `lexlean::sdk` only for the views `1` and `2` and the
forms `1` and `2` (DRAFTS.md).

## The bounds

| Value | Where it comes from |
| --- | --- |
| `store.put` request up to 9 MiB of content, `store.get` result up to 9 MiB, `store.put` result up to 9 MiB (the rendered text) | `draft.max_bytes` 8 MiB of file content (OWNER-VALUES.md) and 1 MiB of headroom for the framing of the preimage (`DRAFT_BLOB_HEADROOM`: 256 files of 36 octets of framing, their paths, which `draft.max_bytes` does not count, and the three heads); the tool layer refuses a larger draft before it asks for an effect (DRAFTS.md) |
| the Engine's result up to 4 MiB | the binding answers class `4` with a diagnostic rather than a result over it |
| a status carries the last 64 KiB of a log | `verify.control` result |
| `requestMaximum` and `responseMaximum` 20 MiB, `bodyMaximum` 16 MiB | a draft of 8 MiB escaped in JSON is at most 16 MiB |
| an exchange asks for at most 4 effects | `effectBudget`; `lexlean_draft_assemble` on a base draft asks four (`lexicon.read`, `store.get`, `store.put` of the render, `store.put` of the draft), `lexlean_draft_write` two, the Engine tools one |
| `rate.tool_calls` 100 per `rate.window_ms` 60000 | OWNER-VALUES.md; the window is the binding's, the limit the core's |

## The binding

`Native` implements `Host`: eleven methods, and no dispatch. Which effect the core asked for
is the generated driver's business, and what a tool does with the answer is the core's. The
store is a directory of seven areas (`LEXLEAN_MCP_STORE`, else `$XDG_DATA_HOME` or
`~/.local/share`, below `lexlean-mcp/store`): no key leaves its area, no path is a symbolic
link, reads do not follow one, and a write is a temporary file renamed over its target.
A project is a directory of `drafts/` with a `lexlean.toml`, or the handle `<profile>/<id>` of a
draft written by `lexlean_draft_write`: the host copies the profile bundle and the files of the
draft into a directory of its own below `builds/` (`materialize.rs`, DRAFTS.md), runs the Engine
on it and removes it, and a bundle that holds a symbolic link is `project-refused`. The verifier is a thread that
runs `Engine::verify` (one at a time, the others queued) and writes the attestation record
of a success to `verifications/` and its digest index to `index/`; it is cancelled with
`lexlean::verify::end_live_children`. The rate counter counts the counted requests of the
process in a fixed window and answers with the count before the request being served.

The attestation record is unsigned: DSSE signing is #11. `attestation.lookup` finds the
record the verifier wrote, by ID or by the digest of a verified file.

The generated host crates carry one method name, `tools/call`: it is the value the application
declares for counting (`countedValue` of the serving section, the `Mcp-Method` of the requests
`rate.calls` counts), which PrismPM renders into the serving surface and its tests. GW-06 allows
that one name there and nowhere else; the binding, the source the repository writes, carries none.

## The vectors

`vectors/exchanges.json` holds each vector as the real host answered it: the HTTP request,
the frame the adapter hands the core, the effects the core asked for with the payload it
asked with and the outcome it got, and the response. `generate.py record` makes the file
with `vectors/runner`, on a store that starts empty; `generate.py render` writes
`src/Application.lex.tex` from it, and CI fails when the committed file is not that
rendering. `prismpm verify` replays every vector three ways (a script of the recorded
outcomes, the real binding through `gateway-host run`, and the served program over a socket)
and requires the same bytes each time.

Five vectors ask for no effect (`tools-list`, `invalid-arguments`, `unknown-tool`, and two
`explain` calls, one of a registered code and one of a code with a quotation mark and a reverse
solidus that the registry lacks, whose answers are compiled into the core: TOOLS.md); the
others call a tool, in an order that matters (the store is shared): the six files of the
project `tiny`, a read of one, the four Engine calls on it, and a refusal of each of
`store.put`, `verify.start`, `verify.control`, `lexicon.read` and `attestation.lookup`.
`verify.start` and `verify.control` are exercised by their refusals only: every vector is a
fresh process, so a task started by one is gone before the next. Their success paths are the
binding's tests (`verify_start_*`, `verify_control_*`), including a real verification and a
real cancellation.

## The adapter's refusal of a foreign `Origin`

The `Origin` of a request is judged twice. The generated adapter judges it first, in
`serve::read_request`, before the request is framed: a request with no `Origin` is not a
browser's and is not judged; one `Origin` is allowed only when it is `scheme://host[:port]` over
`http` or `https` with a host and port that the declaration's `origins` name (a declared port of 0
is any port), compared without regard to case; anything else (a foreign host, `null`, an empty
value, a path, a query, userinfo, a port that is not a number, or a second `Origin` field) is
answered `403` with the body `OriginForbidden`, the code of PrismPM's closed `ServingRefusal`,
and the core is not called. The core judges it again when it is called (`Forbidden: invalid
Origin`, a JSON-RPC error).

MCP's server suite cannot see the adapter's list: when it sends a foreign `Host` and `Origin`,
whichever check runs first refuses, and both answer `403` (GATEWAY-CONFORMANCE.md). A served vector
cannot hold it either: PrismPM's model gives every `ServedVector` a frame vector, the frame the
adapter hands the core, and a request the adapter refuses has no frame. So `GW-13`
(`gateway/tool-tests/tests/origin.rs`, an ordinary test of the gateway job) runs the adapter
itself, `serve::run` of the generated `gateway-host` with the real host and the real core behind
a call counter, over a real loopback socket:

| Test | What it observes |
| --- | --- |
| `a_foreign_origin_is_refused_by_the_adapter_and_never_reaches_the_core_gw_13` | 25 origins (a foreign host, a host that only starts or ends with a loopback name, other schemes, userinfo, a path, a bad port, `null`, the empty value) are each answered `403` with the body `OriginForbidden`, which is the adapter's word and not the core's, and the counter says the core was called 0 times |
| `two_origin_fields_are_refused_by_the_adapter_gw_13` | two `Origin` fields, both allowed, are refused the same way |
| `an_allowed_origin_and_no_origin_reach_the_core_gw_13` | ten allowed spellings (the three loopback names over both schemes, ports, case) and a request with no `Origin` reach the core once and are answered `200` with the tool list, so a `403` above cannot be some other failure |
| `the_core_refuses_a_foreign_origin_in_its_own_words_gw_13` | the same kind of origin handed to the core as a frame is refused by the core too, with `Forbidden: invalid Origin`: two checks, two voices |
| `the_declared_origins_are_the_six_loopback_origins_with_any_port_gw_13` | `surface::ORIGINS`, the allow-list of the declaration as the build renders it, is exactly `http` and `https` over `localhost`, `127.0.0.1` and `[::1]`, any port, so a widening to an origin the corpus does not name fails too |

## Planted defects

A gate is evidence only after a planted defect shows it can fail (AGENTS.md). Each defect was
applied to the committed files on the development machine, the command ran, and the files were
restored (`git checkout`); the unplanted tree passes the same commands. `prismpm verify` itself
was not run there (it stops with PP5008 outside the PrismPM SDK: the `timeout` of coreutils
9.1-1 is pinned by digest, CI.md), so the replay of the vectors through the host was run with
the host the job builds, over the same three transports (`exchange`, `run`, `serve`), by a
script that is not committed.

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| an undeclared effect: a twelfth effect, `rogue.effect`, in the host protocol of the application | `python3 -I gateway/vectors/generate.py render` then `cargo test -p repo-model --test gateway` | GW-01 and GW-02 fail: `the host protocol declares the ten effects and the frame effect` (left 12, right 11) and `a row per effect` (left 11, right 12) | `git checkout gateway/src/Application.lex.tex gateway/vectors/generate.py`: the gateway tests pass again |
| a missing vector: the steps of the vector that reaches `store.get` emptied in `exchanges.json`, the application rendered again | `cargo test -p repo-model --test gateway`, and `prismpm build` in `gateway/` | GW-04 fails: `the transcripts request exactly the ten effects` (`store.get` is missing from the left set); `prismpm build` stops after 9 s with `PP2013: every host effect the core may request must be requested by a step of some acceptance transcript` | `git checkout gateway/vectors/exchanges.json gateway/src/Application.lex.tex` |
| a wrong effect result: `store.get` answers `found: content.is_none()` | `cargo test -p gateway-binding --locked --offline` (CARGO_HOME of the registry), and the replay of the recorded vectors through `gateway-host run` and `gateway-host serve` | the binding tests `store_get_returns_what_store_put_wrote` and `store_get_reports_a_missing_key_as_not_found` fail (51 passed, 2 failed); vector 9 (`store.get`) differs from its recorded response through `run` and through `serve`, the other eighteen are equal | `git checkout gateway/bindings/gateway-binding/src/lib.rs` |
| a tool name in adapter source: the comment `// lexlean_check` in the binding's `src/store.rs` | `cargo test -p repo-model --test gateway` | GW-06 fails: `adapter source names a tool or a method` with `store.rs` and `lexlean_check` | `git checkout gateway/bindings/gateway-binding/src/store.rs` |
| a hand-edited core: a comment inserted in the committed `generated/lexlean_mcp_gateway_core/src/lib.rs` | `diff -r gateway/lexlean_mcp_gateway_core gateway/generated/lexlean_mcp_gateway_core` (the comparison `gateway_build` runs after `prismpm build`) | exit status 1: `diff -r .../src/lib.rs`, the first differing line is the inserted comment | `git checkout gateway/generated/lexlean_mcp_gateway_core/src/lib.rs`: the two directories are equal |
| a tampered vector: the last byte of the first served response changed in `exchanges.json` | `python3 -I gateway/vectors/generate.py check` | exit status 1: `Application.lex.tex is not the rendering of exchanges.json` | `git checkout gateway/vectors/exchanges.json`: `the declaration is the rendering of the vectors` |
| the served port differs from the owner value: `listenPort` set to 5396 in `src/OwnerValues.lex.tex` | `cargo test -p repo-model --test gateway the_served_port` | `the gateway serves on the owner value listen.port` (left 5395, right 5396) | `git checkout src/OwnerValues.lex.tex`: the test passes again |
| a widened allow-list of origins: `evil.example.com` among the hosts of `surface::ORIGINS` in the laid-out `gateway/gateway-host/src/surface.rs` (the constant `prismpm build` writes from the `ServingOrigin` values of the declaration) | `cargo test -p gateway-tool-tests --test origin` (gateway workspace, offline, the registry's `CARGO_HOME`) | `GW-13` fails twice: `a_foreign_origin_is_refused_by_the_adapter_and_never_reaches_the_core_gw_13` finds, for `http://evil.example.com`, that the answer is the core's (`Forbidden: invalid Origin`, not the adapter's `OriginForbidden`), so the core was reached; `the_declared_origins_are_the_six_loopback_origins_with_any_port_gw_13` finds seven origins where six are declared (3 passed, 2 failed); the core's own refusal, `the_core_refuses_a_foreign_origin_in_its_own_words_gw_13`, still passes: a widened adapter is not seen by any vector or by the core | `cp` of the saved `surface.rs` (equal to the build's again by `diff -r`) |
| the same widening in the declaration: `"evil.example.com"` among the hosts of the origins in `gateway/vectors/generate.py`, the application rendered again | `cargo test -p repo-model --test gateway the_adapter_refuses` | `GW-13` fails: `the declaration's origins are http and https over the three loopback names`; `generate.py check` passes (the rendering is consistent with the widened list), so only this test and the origin tests above reject it | `git checkout gateway/vectors/generate.py gateway/src/Application.lex.tex` |

## Building and verifying

```sh
gateway/lay-out-mcp.sh                  # the sources of MCP at the pin
gateway/mcp-identities.sh               # AC-03 (a)
gateway/lay-out-sdk.sh                  # language/prism.arch of PrismPM at the pin
gateway/registry.sh                     # the registry; CARGO_HOME=gateway/registry/cargo-home
(cd gateway && prismpm build)           # exports the core, generates the host packages
gateway/lay-out.sh gateway/.prism/build/<id>
(cd gateway && prismpm verify)          # LEXLEAN_MCP_STORE=$(mktemp -d)
```

CI.md describes the job that runs this, and the release step that removes the registry.

## Not claimed

- The tools do what their names say only as far as the vectors show: the semantics of each
  tool, its schema and its errors belong to #8 to #12. This file claims that the tools reach
  the host through the ten effects, and that the host does what the effects declare.
- That the gateway passes the scenarios of MCP's server suite that wait. The conformance run
  (AC-03, part b, `GW-12`) is `GATEWAY-CONFORMANCE.md`: 40 scenarios fail and are exactly
  `gateway/waiting.json`, each with its reason; they are failures and are counted as such.
