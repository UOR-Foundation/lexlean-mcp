# The engine tools

Issue #10 asks that `lexlean_check`, `lexlean_snapshot`, `lexlean_format` and `lexlean_build`
(F4 to F7) agree with the `lexlean` CLI. This file says what each tool returns, how an
answer of the Engine becomes an MCP result, and how the claim "the tool equals the CLI" is
tested. The rows are `TL-01` to `TL-09` (suite `tools`); the tests are
`cargo test -p repo-model --test tools` (the register half) and, in the gateway workspace,
`cargo test -p gateway-tool-tests` (the behaviour half). The effects are those of
`src/Effects.lex.tex` and are not changed: `engine.check`, `engine.snapshot`,
`engine.format` and `engine.build` are E3 to E6, and the set stays E1 to E10.

## What a tool returns

The binding builds the `result` of an effect with the function that the CLI builds its
output with, `lexlean::api::command_result_json` (`lexlean/command-result/1`, SPEC 24.5).
`lexlean --diagnostic-format json --color never <command>` prints exactly those bytes, so
the tool's `result` and the CLI's standard output are the same bytes. The tool layer parses
them and returns them twice: as the text of the result, and as its `structuredContent`.

| Tool | CLI command | Input | `structuredContent` |
| --- | --- | --- | --- |
| `lexlean_check` | `lexlean check` | `project`, `files`, `all` | the command result, `command: "check"` |
| `lexlean_snapshot` | none (`Engine::snapshot`, SPEC 24.1) | `project`, `files`, `all` | the canonical semantic snapshot when the project checks; the command result of `check`, with `command: "snapshot"`, when it does not |
| `lexlean_format` | `lexlean fmt` (`fmt --check` for `check_only`) | `project`, `files`, `all`, `check_only` | the command result, `command: "fmt"` |
| `lexlean_build` | `lexlean build` | `project`, `files`, `all` | the command result, `command: "build"`, with `build_id` and the artifact |

`files` selects the files named (a path with a line feed is refused as invalid arguments),
`all` selects every module, and neither selects the entrypoints, as the CLI's default does.
`all` together with a non-empty `files` is invalid arguments, as the CLI's flags are
exclusive. The input schemas are closed (`additionalProperties: false`) and `project` is
required; a call that breaks a schema is a protocol error (JSON-RPC `-32602`), not a tool
result.

## From the answer of the Engine to the result of the tool

| The Engine answers | The tool returns |
| --- | --- |
| class 0 (exit code 0) | a result, no `isError`; `structuredContent` is validated against the `outputSchema` of the tool |
| class 1, 2 or 4 (exit codes 1, 2, 4) | a result with `isError: true` that carries the whole command result, diagnostics and spans included |
| a failure of the host (`project-refused`, `engine-unavailable`) | a result with `isError: true` and `{ "failure": <name> }` |
| classes 3 and 70 (environment, internal) | the host failure `engine-unavailable`: the Engine could not answer |
| bytes that are not JSON, or a command result whose `exit_code` is not the class | a typed internal refusal (JSON-RPC `-32603`): the host broke its contract, which the client cannot repair |

A refusal of the project is the answer the tool exists to give, so it is a tool result the
model can read and act on (`isError`), never a protocol error. A protocol error is for a
call that is not a call.

## Snapshot

The CLI has no `snapshot` command. A project that checks has a snapshot, and the tool
returns its canonical bytes (`Snapshot::canonical_bytes`); a project that does not check
has none, and the tool returns the command result of the `check` it would have been, with
`command` renamed `snapshot`, so a client reads one shape of refusal. The tests compare
the snapshot's identifiers with those the CLI's `check` prints (`source_id`, `semantic_id`)
and the refusal with the CLI's `check` output.

## The test that runs both

`gateway/tool-tests/tests/equality.rs` stages each fixture, runs the `lexlean` binary on
it (`LEXLEAN_CLI`, else `lexlean` on `PATH`), and calls the tool on a copy through the
real host and the production core. It compares the class and exit code, the diagnostics
(code, message, span), and the output bytes: for `check` and `build` the `result`, for
`fmt` the command result and the files written, for `build` the tree of the build root. The
fixtures are the F4 and F7 rows of the register of `crates/negative-fixtures` (252 F4 and
one F7 rows), the examples of LexLean (AC-04: `lexlean build` of each equals its
`expected/build`), and `gateway/tool-tests/fixture/outside-atom`, a project with one atom
outside the profile, whose LLL1004 must carry its span (AC-08).

The four tests that need the binary and the sources of LexLean are `#[ignore]`d with the
reason stated, and the gateway job runs them with `--ignored` and requires the count
(`gateway_ignored_tests gateway-tool-tests 4`), after `gateway/lay-out-lexlean.sh` has laid
LexLean out at the pin (`LEXLEAN_SOURCES`). The remaining tests (schemas, the mapping,
protocol errors, the instrument for Q4) run in every `cargo test`.

### The pin of the register and the pin of the gateway

The register is bound to LexLean `9796067` (297 fixtures); the gateway is compiled by the
LexLean commit MCP names (`gateway/mcp/lexlean.rev`, SM-37 `0b0c95d96`), whose
`tests/negative` has 298. The extra fixture, `induction-nested-type`, is not in the
register (`NOT_IN_THE_REGISTER` in `gateway/tool-tests/src/lib.rs`, with the counts asserted
by `TL-03`), and the expected files of the register are not compared byte for byte at the
second pin, because 290 trees differ there (the `compiler_semantics` of the lock and 12
certificate files). The comparison that matters is the one the tests make: the tool against
the CLI of the same commit, on the same project.

## Q4: no child process

`TL-08` is the instrument that shows that no tool starts a child process: the process's
own accounting of waited-for children and the children of every task, read before and after
a call of each tool, in a binary of one test, with a positive control that starts a child
and is seen to do so.

## Planted defects

A gate is evidence only after a planted defect shows it can fail (AGENTS.md). Each defect
was applied to the working tree, the command ran, and the file was restored with
`git checkout`; the unplanted tree passes the same commands.

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
