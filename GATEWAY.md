# The gateway

Issue #7 asks for the generated host adapter and the `gateway` binary. `gateway/` is that
half: a LexLean production root, the PrismPM server application that declares it, the
binding that implements its effects, and the vectors that tie the three together. The
declaration half is `src/Effects.lex.tex` and EFFECTS.md (EF-01 to EF-08); this file
describes the generated half, registered as `GW-01` to `GW-11` (suite `gateway`), held to
the declaration by `cargo test -p repo-model --test gateway`.

## What is where

| Path | What it is |
| --- | --- |
| `gateway/src/Effects.lex.tex` | the declaration half, byte for byte (`src/Effects.lex.tex`); the single source of the ten effects |
| `gateway/src/Gateway.lex.tex`, `GatewayTools.lex.tex`, `HostWire.lex.tex` | the tool layer and the production root `LexLeanMcpGateway.Gateway.gatewayRun` |
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
from it. `gateway/mcp-identities.sh` holds the laid-out modules and the compiler to the pin
(AC-03, part a): every file equals the file of MCP at `.github/mcp.rev`, the `mcp.core`
lexicon has the tree hash MCP's lock records, and the two locks record one compiler.

## The effects

The ten effects of the model, in tag order, and the frame effect. A field is `name:Kind`;
the kinds are `Flag`, `U32`, `U64`, `Text` and `Bytes`; a failure is a declared name.

| Effect | Request | Result | Failures |
| --- | --- | --- | --- |
| `store.get` | `area:U32`, `key:Text` | `found:Flag`, `content:Bytes` | `area-unknown`, `key-refused`, `unreadable` |
| `store.put` | `area:U32`, `key:Text`, `content:Bytes` | `length:U64` | `area-unknown`, `key-refused`, `unwritable` |
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

## The bounds

| Value | Where it comes from |
| --- | --- |
| `store.put` request up to 8 MiB of content, `store.get` result up to 8 MiB | `draft.max_bytes` (OWNER-VALUES.md); the tool layer refuses a larger draft before it asks for an effect |
| the Engine's result up to 4 MiB | the binding answers class `4` with a diagnostic rather than a result over it |
| a status carries the last 64 KiB of a log | `verify.control` result |
| `requestMaximum` and `responseMaximum` 20 MiB, `bodyMaximum` 16 MiB | a draft of 8 MiB escaped in JSON is at most 16 MiB |
| an exchange asks for at most 2 effects | `effectBudget`; every tool asks for one |
| `rate.tool_calls` 100 per `rate.window_ms` 60000 | OWNER-VALUES.md; the window is the binding's, the limit the core's |

## The binding

`Native` implements `Host`: eleven methods, and no dispatch. Which effect the core asked for
is the generated driver's business, and what a tool does with the answer is the core's. The
store is a directory of seven areas (`LEXLEAN_MCP_STORE`, else `$XDG_DATA_HOME` or
`~/.local/share`, below `lexlean-mcp/store`): no key leaves its area, no path is a symbolic
link, reads do not follow one, and a write is a temporary file renamed over its target.
A project is a directory of `drafts/` with a `lexlean.toml`. The verifier is a thread that
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

Three vectors ask for no effect (`tools-list`, `invalid-arguments`, `unknown-tool`); the
others call a tool, in an order that matters (the store is shared): the six files of the
project `tiny`, a read of one, the four Engine calls on it, and a refusal of each of
`store.put`, `verify.start`, `verify.control`, `lexicon.read` and `attestation.lookup`.
`verify.start` and `verify.control` are exercised by their refusals only: every vector is a
fresh process, so a task started by one is gone before the next. Their success paths are the
binding's tests (`verify_start_*`, `verify_control_*`), including a real verification and a
real cancellation.

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
- The conformance run of MCP's server suite against the gateway (AC-03, part b) is a later
  change: it needs the waiting list of the gateway and MCP's oracle over a served gateway.
