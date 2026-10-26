# The engine tools

Issue #10 asks that `lexlean_check`, `lexlean_snapshot`, `lexlean_format` and `lexlean_build`
(F4 to F7) and `lexlean_explain` (F11) agree with the `lexlean` CLI. This file says what each
tool returns, how an answer of the Engine becomes an MCP result, and how the claim "the tool
equals the CLI" is tested. The rows are `TL-01` to `TL-09` (F4 to F7) and `TL-10` to `TL-14`
(F11), and `TL-15` (the running clause of Q2: QUALITY.md), suite `tools`; the tests are `cargo test -p repo-model --test tools` (the register half)
and, in the gateway workspace, `cargo test -p gateway-tool-tests` (the behaviour half). The effects are those of
`src/Effects.lex.tex` and are not changed: `engine.check`, `engine.snapshot`,
`engine.format` and `engine.build` are E3 to E6, and the set stays E1 to E10.

The `project` of an Engine tool is a directory project of the `drafts` area, or, since the draft
tools of #9, the handle `<profile>/<id>` of a draft that `lexlean_draft_write` returned: the host
materializes the profile bundle and the draft as copies and runs the Engine on them (DRAFTS.md,
`DR-07`). The result is the result of the same tool on a directory holding the same files.

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
required. A call that breaks a schema is refused by the core before the tool runs, as an
error result the model can read (`isError: true`, the text `Invalid arguments: ...`, no
`structuredContent`) and no effect is asked for; a call the schema admits and the tool cannot
make (`all` with `files`, a file name with a line feed) is the protocol error `-32602`. Neither
is an answer of the Engine.

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
call that is not a call. A result over 4 MiB (the bound of an engine result, GATEWAY.md,
"The bounds") is the answer of class 4, `LLS8002`, that says so; the one case in the
tests is the snapshot of `uor-atlas`, 59 MB, whose `check` and `build` results are small.

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
second pin, because 290 trees differ there. The comparison that matters is the one the
tests make: the tool against the CLI of the same commit, on the same project.

The locks of the fixtures and of the examples at that commit record a compiler-semantics
identity (`30c58...` for language 1.2) that is not the one of the compiler built from the
same commit (`81a5f...`, the one `gateway/lexlean.lock` records), so `lexlean check` on a
fixture as committed answers `LLC0102` and exit code 2 where the fixture expects 1 or 4.
The tests therefore write the lock that `lexlean lock` makes into the staged copy of a
project (both sides get the same bytes), when it differs from the committed one in nothing
but that identity and the fixture is not about a lock itself; they then require the CLI to
give the fixture's `expected_exit` and the diagnostic codes of its `expected/`, and the tool
to equal the CLI. For the examples the same relock is done, and the trees they publish still
differ from `expected/build` in 12 of 18 (the identities are in the manifest and the maps):
`EXAMPLES_OFF_THE_EXPECTED_TREE` names them, the test fails when the list changes, and the
tool's tree equals the CLI's for every one of the 18. AC-04 as the issue words it ("the
examples build to their `expected/build`") is therefore not shown at this pin for 12 examples
(PR body, "Not met").

## `lexlean_explain` (F11)

`lexlean explain <CODE>` (SPEC 23.4) opens no project: it renders the row of `model/errors.toml`
that the binary embeds (`cli::explain_text`), or refuses a code the registry lacks. That is a
function of the code, so the tool is one too and the host is asked for nothing: the set of
effects stays `E1` to `E10` of `src/Effects.lex.tex`, and `TL-10` observes that a call performs
none. The entries are compiled into the core instead:

| Part | What it is |
| --- | --- |
| `gateway/src/Explain.lex.tex` | the entry of each registered code, as `explain_text` renders it, in groups of eight rows and a lookup `Explain.entry : String -> Option String` |
| `gateway/explain/generate.py` | renders the module from `model/errors.toml` of the LexLean commit of `gateway/mcp/lexlean.rev` (`render`, `check`, `codes`); the module has no author but this script (R1) |
| `GatewayTools.explainDocument` | builds the command result as a JSON value with the members in the CLI's order (UTF-8 byte order of the keys) |
| `GatewayTools.explainReply` | writes it with MCP's `JsonWrite.writeJson`, appends the final line feed, and returns it as text and `structuredContent` |

MCP's writer escapes a string with the rules of LexLean's canonical JSON (the quotation mark, the
reverse solidus, `\b \t \n \f \r`, `\u00xx` for the other controls, everything else raw
UTF-8), so a code the registry lacks is quoted in the refusal as the CLI quotes it.

| The code | The tool returns | The CLI |
| --- | --- | --- |
| registered | no `isError`; the command result with `command: "explain"`, `exit_code: 0` and the `explanation`; the text block is the CLI's standard output | exit 0 |
| any other string, the empty one included | `isError: true`; the command result with one `LLC0001` diagnostic, ``` `CODE` is not a registered diagnostic code ```, `exit_code: 2`; the same bytes as text | exit 2 |

The core writes every `tools/call` result through MCP's output sanitizer (`Sanitize.sanitizeJson`,
applied by `ResultJson`), which replaces with U+FFFD each control character but the tab, the line
feed and the carriage return, the delete character, each character of the C1 range, and the
bidirectional embedding, override and isolate characters (U+202A to U+202E, U+2066 to U+2069). It
touches no byte of the explanation of a registered code and none of the refusal of a code without
such a character; it does change the refusal of a code that holds one, in `structuredContent`
(where the character is a character) and in the text where the CLI prints it raw (the delete
character and the C1 range: the writer escapes the others as `\u00xx`, so the text keeps the
escape). So for such a code the tool returns the CLI's refusal with U+FFFD for the character. This
is the pinned core's rule for every tool, not a choice of `lexlean_explain`; `TL-12` states it as
`core_sanitized` and requires the difference on exactly the strings that hold such a character.

The match is exact and case-sensitive, as the CLI's is. A code that begins with `-` is an option
to the CLI's parser (it answers `LLC0001`, "unexpected argument", which is a parser error and not
a lookup); the tool equals `lexlean explain -- CODE`, and the test runs the CLI that way. The input
schema is closed: `code` (a string) is required, and a call that breaks it is refused by the core as
`Invalid arguments`, with no effect, like the other tools.

`TL-12` (`gateway/tool-tests/tests/explain.rs`, ignored, run by the gateway job) reads the codes
from `model/errors.toml` of the checkout of LexLean at the pin (86, asserted) and, for each, and
for a corpus of strings the registry lacks (the empty string, case and spacing variants, the
backquote, the quotation mark, the reverse solidus, controls, DEL, accents, `§`, CJK, an emoji,
dashes, a path, and for every code its lower-case form, the code and a space, and the code
without its last character), runs `lexlean --diagnostic-format json explain` in a directory with no
project and calls the tool, and requires the same text bytes, `structuredContent` equal to the
parsed standard output (both after `core_sanitized`, which changes the 12 strings of the corpus
that hold a character of the sanitizer and no registered code), `isError` equal to `exit_code != 0`, the exit status, an empty standard
error and no effect. `TL-13` (in `equality.rs`) collects the codes in the results of
`lexlean_check` and `lexlean_build` on the fixtures of `TL-03`, requires them to be the codes the
fixtures expect, and resolves each through the tool: Q8's running clause.

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
| a class mapped to the wrong exit code: `class_number` in `gateway/bindings/gateway-binding/src/engine.rs` answers `2` for `ErrorClass::SecurityOrLimit` (class 4), while the command result keeps exit code 4 | `cargo test -p gateway-tool-tests --test tools` (gateway workspace, offline, the registry's `CARGO_HOME`) | `TL-02` fails: `classes_one_two_and_four_are_error_results_with_the_command_result_tl_02` panics with `a protocol error where a result was expected: {"error":{"code":-32603,"message":"Internal error"}}`, because the core refuses a result whose `exit_code` is not its class (7 passed, 1 failed) | `git checkout gateway/bindings/gateway-binding/src/engine.rs` |
| a dropped diagnostic: `answered` in the same file gives the command result `error.diagnostics.get(1..)`, the diagnostics but the first | the same command | `TL-07` and `TL-02` fail: `an_atom_outside_the_profile_gives_lll1004_with_its_span_tl_07` finds `"diagnostics":[]` with `"exit_code":1`, and `classes_one_two_and_four_...tl_02` finds the same for a bad source (6 passed, 2 failed); the tests that run the CLI beside the tool (`TL-03`) compare every diagnostic, and were not run with this defect | `git checkout gateway/bindings/gateway-binding/src/engine.rs` |
| a format result whose bytes differ from the CLI's: `command_result` in the same file drops the last byte (the line feed) of the result of `Call::Format` | the same command | `TL-02` fails: `class_zero_is_a_result_without_is_error_tl_02` panics with `lexlean_format: the text is the CLI's file spelling` (7 passed, 1 failed); `TL-05` compares the bytes of `fmt` and `fmt --check` with the tool's, and was not run with this defect | `git checkout gateway/bindings/gateway-binding/src/engine.rs` |

## Planted defects of lexlean_explain

Each defect was applied on the development machine, the command ran, and the file was restored
(`git checkout` for a tracked file; for the laid-out core, `cp -r gateway/generated/lexlean_mcp_gateway_core
gateway/lexlean_mcp_gateway_core`, after which `diff -r` of the two directories is empty). A change to the
laid-out core is the change that `prismpm build` would write for the same change of the source (the
core is Rust that `prismpm build` generates, and the string literals and the arms of `Explain.entry` are
in it as written); the build itself is the job's gate and was not repeated for each defect. The unplanted
tree passes every command below.

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| a wrong byte in one explanation: `Exit code: 1.` becomes `Exit code: 2.` in the entry of `LLL1004` of `gateway/src/Explain.lex.tex` | `LEXLEAN_SOURCES=<LexLean at the pin> python3 -I gateway/explain/generate.py check` | exit status 1: `gateway/src/Explain.lex.tex is not the rendering of the registry` | `git checkout gateway/src/Explain.lex.tex`: `the module is the rendering of the registry` |
| the same wrong byte in the laid-out core (the entry of `LLL1004` in `gateway/lexlean_mcp_gateway_core/src/lib.rs`) | `cargo test -p gateway-tool-tests --test explain -- --ignored` (gateway workspace, offline, the registry's `CARGO_HOME`, `LEXLEAN_CLI` and `LEXLEAN_SOURCES` set) | `TL-12` fails: `2 of 392 strings differ from the CLI`; for `"LLL1004"` the text and `structuredContent` end `Exit code: 2.` where the CLI printed `Exit code: 1.` (the other 391 strings are equal); `diff -r gateway/lexlean_mcp_gateway_core gateway/generated/lexlean_mcp_gateway_core` also exits 1 | restore the core from `gateway/generated`: `TL-12` passes |
| an unknown code answered as a registered one: the last arm of `Explain.entry` answers `Option.some ""` instead of `Option.none`, which in the laid-out core is `Some(Str::lit(""))` where `None::<Str>` was | `cargo test -p gateway-tool-tests --test explain -- --include-ignored` | `TL-10` fails: `lexlean_explain_asks_the_host_for_no_effect_tl_10` panics on `assert!(is_error(result))` for `LLX9999`, whose result is `"explanation":""` with `"success":true`; `TL-12` fails for every string the registry lacks (`"LLX9999": isError is false for the exit status 2`); the other two `TL-10` tests pass | restore the core from `gateway/generated`: four tests pass |
| a wrong escape: the digit of a `\u00xx` escape is written in upper case (`hexDigit` of `JsonWrite` adds 55 where it adds 87), so a code with a control character is quoted as `\u001F` where the CLI writes `\u001f` | `gateway/mcp-identities.sh`, and `cargo test -p gateway-tool-tests --test explain -- --include-ignored` with the same change in the laid-out core | `mcp-identities: the laid-out MCP files differ from the pin 39a7d8e413004f031ba941dc8d88808090d51570`, exit status 1 (a `diff -r` names `src/JsonWrite.lex.tex`, line 86); `TL-12` fails for `"\u{1f}"`: the text holds the message `\u001F` where the CLI has `\u001f` (the three `TL-10` tests pass: only the corpus of `TL-12` has a code with a control character that needs a letter) | `cp` of the saved `gateway/mcp/src/JsonWrite.lex.tex` (laid out again by `gateway/lay-out-mcp.sh`); the core restored from `gateway/generated` |
