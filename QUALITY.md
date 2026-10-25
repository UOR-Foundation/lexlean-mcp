# The executed measures of the quality model

Issue #15 models the quality requirements Q1 to Q8 as chains with an
all-or-none measure. `src/Quality.lex.tex` records, for each measure, whether it
is `Executed` or `Pending`. This file states which two are `Executed` now, what
the evidence is, what it does not show, and the defects planted in its gates.

| Measure | Requirement | Status | Rows |
| --- | --- | --- | --- |
| Q1 | a tool result equals the engine's result | Pending | needs the tools (#10, #11) |
| Q2 | each in-scope API function is reached by exactly one tool | **Executed** | QM-01 to QM-03 |
| Q3 | no record for a verification that did not end `verified` | Pending | needs verification as tasks (#11) and the store (#13) |
| Q4 | check, snapshot, format, build start no child process | Pending | needs the tools (#10) |
| Q5 | the MCP implementation conforms to 2026-07-28 | Pending | needs the deployed core (#17) |
| Q6 | protocol and tool semantics live only in `.lex.tex` | Pending | needs the host adapter (#7) |
| Q7 | the same `build_id` on `linux/amd64` and `linux/arm64` | Pending | needs `build` as a tool (#10), on both platforms |
| Q8 | every diagnostic code in a result resolves through F11 | **Executed** | QM-04 to QM-06 |

`measuresPendingExceptQ2AndQ8` pins the pending list, `onlyApiMapAndExplainAreExecuted`
pins the executed one, and `onlyQ2KeepsAPendingClause` pins the list of clauses that still
wait for the tools: Q2's, and no other. `qualityValid` requires that the measure of each
clause in `pendingClauses` is an executed one. `QM-07` ties the statuses to the register: the
rows each status cites are read from the statement of `QM-07`, and each must be registered
with a test; for Q8 they are `QM-04` to `QM-06` and `TL-13`, the row of the tools suite that
runs its last clause.

## Q2: the API-to-tool map

**Evidence.** `crates/quality-map/lexlean/` holds, byte-identical to LexLean
`release/v0.4` at `9796067` (`.github/lexlean.rev`), the four source files of the
stable surface: `crates/lexlean/src/api.rs`, `api/production.rs`, `sdk.rs` and
`verify/lifeline.rs` (with a `.txt` suffix, so that no tool of this workspace reads
them as code). `sources.toml` records the git blob id of each, and a test recomputes it
(`QM-01`), with the checksum of the authority `A-LEXLEAN-API`. `derive` finds the
`pub fn` items in the text: 33 public functions, namely 7 methods of `Engine`, 8
functions of `lexlean::sdk`, 11 of `lexlean::api::production` and the methods of its
request, result and report, 5 free functions of `lexlean::api`, and the 2 lifecycle
functions of `lexlean::verify` that SPEC 24.6 declares (`QM-01`). `map.toml` has one
row for each, with the signature the source declares, and no other row; every function
SPEC 24.1, 24.6, 24.7 and 24.8 declare is among them, and `Engine` and `lexlean::sdk`
have exactly the SPEC's methods and functions (`QM-02`).

**The measure** (`QM-03`). Each of the 33 functions reaches exactly one tool or states
a reason no tool does; every tool named is one of the 14 that issues 8 to 12 define
(`lexlean_profiles`, `lexlean_lexicon_search`, `lexlean_draft_write`,
`lexlean_draft_assemble`, `lexlean_draft_read`, `lexlean_check`, `lexlean_snapshot`,
`lexlean_format`, `lexlean_build`, `lexlean_explain`, `lexlean_verify`,
`lexlean_present`, `lexlean_attestation`, `lexlean_export`); and the 9 functions that
reach a tool are the 9 the model of Q2 names. The model is the source of that scope:
the test reads it from the description of Q2 in `src/Quality.lex.tex`.

| Function | Tool | Why not, when no tool |
| --- | --- | --- |
| `Engine::check` | `lexlean_check` | |
| `Engine::snapshot` | `lexlean_snapshot` | |
| `Engine::format` | `lexlean_format` | |
| `Engine::build` | `lexlean_build` | |
| `Engine::verify` | `lexlean_verify` | |
| `sdk::assemble`, `sdk::write_module` | `lexlean_draft_assemble` | |
| `sdk::read_module`, `sdk::declaration_spans` | `lexlean_draft_read` | |
| `Engine::load` | none | opens the project for every tool call; no request of its own (#8, #10) |
| `Engine::lock` | none | setup, part of provisioning a profile (#8); #10 leaves the lock fixtures out |
| the production functions and the methods of `ProductionRequest`, `ProductionResult`, `ReplayReport` (11) | none | outside the product (#10 leaves the production fixtures out) |
| `sdk::module_kind`, `is_notation`, `to_data_form`, `to_notation` | none | #9 lists four sdk functions for its tools and these are not among them; the data form is refused for language 1.2 |
| `api::render_build`, `publish_build` | none | steps behind `Engine::build`, outside the five calls Q2 puts in scope; no issue speaks (reason `issue = 0`, on the pinned SPEC) |
| `api::parse_lock_bytes`, `command_result_json`, `verify_part` | none | lock setup (#8, #10); the shape of the command result (#10, #11); the parts of one verification (#11) |
| `verify::end_live_children`, `allow_children` | none | steps of the tasks protocol (#11, AC-12), not a tool |

The split of the four `sdk` functions between `lexlean_draft_assemble` (the two that
write) and `lexlean_draft_read` (the two that read) is this repository's reading of
issue 9, which names the four and the two tools without pairing them. Q2 requires
only that each reach one tool.

**What it does not show.** That the tools exist, and that each reaches its function. The
clause is in `pendingClauses`: #9 for the `sdk` functions, #10 and #11 for the `Engine`
calls. Seven tools have no API function behind them (`lexlean_profiles`,
`lexlean_lexicon_search`, `lexlean_draft_write`, `lexlean_explain`, `lexlean_present`,
`lexlean_attestation`, `lexlean_export`): Q2 asks that functions reach tools, not the
converse. The map does not say whether a function outside the five calls and four `sdk`
functions the model names *should* have a tool; that is the model's scope.

## Q8: every diagnostic code explained

**Where the explanations live.** At `9796067` they exist as data. `model/errors.toml`
is the closed registry (SPEC 26.1): 86 codes, each with a class, an exit status, a title
and a statement. `ERRORS.md` is generated from it with one entry per code, and
`lexlean explain <CODE>` prints the entry (SPEC 23.4; `explain_text` in
`crates/lexlean/src/cli.rs`). An unknown code exits as CLI misuse.

**Evidence.** Both files are vendored byte-identically with their git blob ids
(`QM-04`, checksum of `A-LEXLEAN-ERRORS`). Every row is complete: a code in its range
(`LL`, a range letter, and four digits whose first is the digit of the range, SPEC
26.2), unique and sorted, a class with the exit status SPEC 24.5 gives it, a title and
a statement; `ERRORS.md` holds one entry per code, no other, and each entry is
what `explain_text` renders from the row (`QM-05`). Every code returned by the 297
negative fixtures of AC-07 (`crates/negative-fixtures`) is registered and has an entry
(`QM-06`).

**Observed once, not gated.** The `lexlean` binary built at the pinned revision was
run as `lexlean explain <CODE>` for each of the 86 codes: its output equals the entry of
`ERRORS.md` byte for byte for all 86, and `lexlean explain LLX9999` exits 2 with
`LLC0001`. No test of this repository runs the binary.

**What it does not show.** That a code in the result of a running tool resolves
through the running `lexlean_explain` (F11) is not shown by `QM-04` to `QM-06`. `TL-13`
(`gateway/tool-tests/tests/equality.rs`, TOOLS.md) executes it where the tool is built:
every code in the results of `lexlean_check` and `lexlean_build` on the fixtures of
`TL-03` is resolved through the running tool to the entry the CLI prints. The quality
model says the same: the clause of Q8 is not in `pendingClauses`, the evidence of Q8
names `TL-13`, and `QM-07` requires both. `TL-13` runs in the gateway job (its test is
`#[ignore]`d for the workspace gate, which has neither the `lexlean` binary nor the
sources of LexLean), so `cargo test --workspace` checks that the row and its test exist
and the gateway job observes the resolution. The measure does not show that the registry is closed over what the
compiler can emit: that is LexLean's own gate (`validate-model` rejects an unregistered code in use
and an unused registered one, SPEC 26.1), cited here and not re-run. The fixtures
return only some of the 86 codes; the others are explained but not observed in a
result.

## What is not met

Q1 (CLI equality as a run), Q3 (fault tolerance), Q4 (no child processes), Q5 (the MCP
oracles), Q6 (modularity) and Q7 (the same `build_id` on both platforms) stay
`Pending`: each needs the tools or the deployed service, and each says in the model
which issue executes it. For Q2 the runtime half is not met, as above; for Q8 the running clause is executed by `TL-13` and the quality model no longer lists it as pending. Issue
#15 stays open.

## Planted defects

Each defect is planted in the working tree, the gate that must reject it is run, and
the file is restored. `QM-08` plants the same defects in memory in
`crates/quality-map/tests/plants.rs` and `crates/conformance/tests/quality.rs`, so
that the check runs on every `cargo test`.

| Defect | Command | Diagnostic |
| --- | --- | --- |
| `Engine::snapshot` removed from `map.toml` | `cargo test -p quality-map --test surface` | `Engine::snapshot is public in the pinned source and is not in map.toml` |
| tool name `lexlean_check` misspelled `lexlean_chek` | same | `Engine::check is mapped to lexlean_chek, which no issue defines` |
| `pub fn export` added to the vendored `api.rs.txt` | same | `Engine::export is public in the pinned source and is not in map.toml`; `api.rs.txt: the git blob id is 8bcf705d... and the table records 2fac21e5...` |
| the entry of `LLB6002` removed from the vendored `ERRORS.md` | `cargo test -p quality-map --test explain` | `LLB6002 is registered and has no entry in ERRORS.md`; `ERRORS.md: the git blob id is c09fd5d1... and the table records 9a995e31...` |
| a fixture returns `LLX9999`, a code the registry lacks (`register.toml`) | same | `LLX9999 is returned and the registry does not explain it` |
| the count of codes in the model changed from 86 to 85 | `cargo test -p repo-conformance --test quality` | `the model counts Some(85) diagnostic codes and the registry holds 86` |
| Q2 flipped back to `Pending` | same | `the measure 1 is not Executed`; `exactly two measures must be Executed` |
| `QM-03` removed from the register | same | `QM-03 is cited and is not in model/ids.toml` |
| the running clause of Q8 put back into `pendingClauses` (`index := 7`) | `cargo test -p repo-conformance --test quality` | `pendingClauses records a clause of Q8, whose running clause TL-13 executes` |
| the clause of Q8 put back into `pendingClauses` (`index := 7`) | `lexlean verify src/Quality.lex.tex` | `LLV7002`: `Tactic `decide` proved that the proposition namedIndexes pendingClauses = [1] is false` at `src/Quality.lex.tex:678`, the theorem `onlyQ2KeepsAPendingClause` |
| the evidence of Q8 pointing at `pendingClauses` again | `cargo test -p repo-conformance --test quality` | `the Executed status of measure 7 points at pendingClauses, and its running clause is executed` |
| `TL-13` removed from the statement of `QM-07` | same | `QM-07 cites [...], and the expected evidence is [...]` |
| `CONFORMANCE.md` edited by hand | `cargo run -p xtask -- check-model` | `CONFORMANCE.md is stale: it disagrees with model/ids.toml` |

The planted-in-memory set is wider: a function with neither tool nor reason, a function
both mapped and not exposed, a function re-signed upstream, a vendored byte altered or
removed, a pin moved without a new import, a code with no statement, an entry altered,
an exit status that disagrees with its class, rows out of order, Q8 flipped back, a
measure flipped to `Executed` without rows, a cited row with no test, and the pending
clauses dropped.

Not run on the working tree: a flip of Q2 in `src/Quality.lex.tex` through `lexlean
verify`, which would make `measuresPendingExceptQ2AndQ8` and
`onlyApiMapAndExplainAreExecuted` fail with `LLV7002`. The local verification runs once,
on the final state, to keep to the shared memory budget; CI runs it on every push.

## Reproducing

```sh
cargo test -p quality-map -p repo-conformance        # QM-01 to QM-08
cargo run -q -p xtask -- check-model                 # CONFORMANCE.md equals the model
lexlean verify                                       # the new theorems of Quality
```
