# Executed measurements of the threat model

`src/Security.lex.tex` gives each risk R1 to R10 one measurement, with a status of
`Executed`, `Pending` or `Residual`. This file records which measurements are
executed, what executes them, what they do not cover, and the defects planted to
show that the checks can fail (`AGENTS.md`, gate quality). It is project-owned
evidence; the register is `model/ids.toml` and `CONFORMANCE.md` is generated from it.

## Status

| Risk | Measurement | Status | Evidence |
| --- | --- | --- | --- |
| R1 | every envelope validates under the key history | `Executed` | `SM-03`: `SG-03`, `SG-10`, `SG-11` |
| R2 | no profile served without a review record per package digest | `Executed` | `SM-02` |
| R3 | `dns-rebinding-protection` passes; the adapter refuses a foreign `Origin` | `Executed` | `SM-04`: `GW-12`, `GW-13` |
| R5 | a planted `..` path refused before any `Engine` call | `Executed` | `SM-01` |
| R8 | an envelope under a revoked key fails validation | `Executed` | `SM-05`: `SG-04`, `SG-05`, `SG-07` |
| R4, R6, R7, R9 | as stated in `Security` | `Pending` | the issue named in each status (#7 to #13, #17, #45) |
| R10 | recorded as a residual risk of `mcp-loopback` | `Residual` | none in this release |

The symlink clause of control K5 (materialization refuses symlinks in a profile
bundle) is not a measurement of `Security`. It was recorded in `pendingClauses` until
drafts were materialized (#9); `DR-08` executes it (DRAFTS.md), so `pendingClauses` is
empty and `SM-01` cites `DR-08`. Likelihoods stay unbound until #19.

## R5: path escape (`SM-01`)

The check is "a planted `..` path refused before any `Engine` call". It is observed
by these rows, each with tests that run in `cargo test --workspace`:

| Row | What it observes | Tests (`crates/nfc-oracle/tests/`) |
| --- | --- | --- |
| `NF-09` | `NotNfc` is a typed refusal that follows `NotUtf8` and precedes the root and suffix stages; the kernel-checked instances of `Admission` hold on the full tables and on the sample image | `admission.rs`, `instances.rs` |
| `NF-12` | the exported root `admitRun` refuses each escape form (`..`, `.`, empty component, absolute path, outside the roots) with the typed refusal of the constructor order, and `src/../é.lex.tex` is `Refused DotDotComponent` | `admit.rs` |
| `DL-01` | the exported draft root `admitDraftRun` declares only allocation and overflow, so no engine or host effect, and its stages are executable definitions | `draft.rs` |
| `DL-02` | typed verdicts of `admitDraftRun` for a draft of a million rows and at the limits | `draft_stack.rs` |
| `DL-03` | a draft with a `..` row is refused with `DotDotComponent` first, before the limits and the duplicate scan | `draft.rs` |
| `DL-04` | the duplicate scan is O(n log n) and still refuses a repeated path | `draft_stack.rs` |

`admittedPathHasNoDotDot` in `src/Admission.lex.tex` proves that an admitted path has
no `..` component, and `lexlean verify` checks it with the other theorems.

What this does and does not show:

- "Before any `Engine` call" holds because admission is a pure function. The
  provenance of both exported roots (`crates/nfc-oracle/generated/*/provenance.json`)
  records the effects `allocation` and `overflow` only, and `NF-07` and `DL-01` check
  that record. No `Engine` is linked yet (#6), so no test observes an ordering with one.
- The rows observe refusal of `..` and the other escape forms by the pure admission
  function. They do not observe symlinks, which exist only on a file system.
- `SM-01` does not rerun the cited tests. It checks that each cited row is registered
  at the build level and has a test, and that `Security` records the measurement as
  `Executed`. Whether the tests pass is the verdict of the workspace test run.

## Planted defects

Each defect was applied to the working tree, the named command was run, and the
tree was restored from a saved copy (`cmp` equal) before the next defect. The
commands run in WSL from the repository root; `lexlean` is the build at
`.github/lexlean.rev`.

### 1. R5 flipped back to `Pending`

Defect: in `src/Security.lex.tex`, `ctor MeasureStatus.Executed` in the measurement
of R5 replaced by `ctor MeasureStatus.Pending`.

| Command | Result |
| --- | --- |
| `lexlean fmt --check`, `lexlean check` | pass (12 modules): the defect is well formed |
| `lexlean verify src/Security.lex.tex` | exit 1 |
| `cargo test -p repo-conformance --test security` | `SM-01` fails |

Diagnostics:

```
error[LLV7002]: Lean rejected `LexLeanMcp.Security` (error): Tactic `decide` proved that the proposition
  pendingRisks measurements = [0, 2, 3, 5, 6, 7, 8]
is false
error[LLV7002]: Lean rejected `LexLeanMcp.Security` (error): Tactic `decide` proved that the proposition
  executedRisks measurements = [1, 4]
is false
```

```
SM-01 does not hold:
exactly one measurement must be Executed
the measurement of R5 is not Executed
```

Restored: `lexlean verify src/Security.lex.tex` verifies 5 modules and
`cargo test -p repo-conformance --test security` passes both tests.

### 2. A cited row removed from the register

Defect: the `DL-03` row deleted from `model/ids.toml`.

| Command | Result |
| --- | --- |
| `cargo test -p repo-conformance --test security` | `SM-01` fails |
| `cargo xtask check-model` | fails |

Diagnostics:

```
SM-01 does not hold:
DL-03 is cited and is not in model/ids.toml
```

```
gate failed: CONFORMANCE.md is stale: it disagrees with model/ids.toml.
```

Restored: both commands pass.

### 3. Planted inside the test

`the_check_rejects_a_pending_r5_and_a_missing_row_sm_01` plants, in memory, R5 flipped
to `Pending`, the test of `NF-12` removed from the test list, `DL-03` removed from
the register, and the symlink clause put back into `pendingClauses`, and `DR-08` removed from the
register, and it fails unless each is reported. It ends with the unplanted inputs and requires no violation.

### 4. The symlink clause of K5 put back into `pendingClauses`

Defect: in `src/Security.lex.tex`, the body of `pendingClauses` replaced by the clause
`K5 symlink refusal` (index 4), which `DR-08` now executes.

| Command | Result |
| --- | --- |
| `cargo test -p repo-conformance --test security` | `SM-01` fails |

Diagnostic:

```
SM-01 does not hold:
pendingClauses records a clause, and DR-08 executes the symlink clause of K5
```

Restored: `src/Security.lex.tex` is the committed one, and `lexlean verify src/Security.lex.tex`
verifies 5 modules, with `noClauseIsPending` among its theorems.

### 5. R3 flipped back to `Pending` (also R1, R8)

Defect: in `src/Security.lex.tex`, `ctor MeasureStatus.Executed` in the measurement of R3
replaced by `ctor MeasureStatus.Pending`. The same flip of R1 or R8 is reported the same way
(`the measurement of R1 is not Executed`), and `SM-03` to `SM-05` also plant, in memory, the
first cited row removed from the register and its test removed from the test names.

| Command | Result |
| --- | --- |
| `cargo test -p repo-conformance --test security` | `SM-04` fails, and `SM-01` fails on its count of Executed measurements |

Diagnostics:

```
SM-04 does not hold:
the measurement of R3 is not Executed
```

```
SM-01 does not hold:
exactly five measurements must be Executed
```

Restored: `src/Security.lex.tex` is the committed one. The theorems
`measurementsPendingAreR4R6R7AndR9` and `executedMeasurementsAreR1R2R3R5AndR8` are checked
by `lexlean verify src/Security.lex.tex`, which CI runs; it was not run in this change's
environment, which has no `lexlean`.

R9 stays `Pending` (#45): the conformance scenario `input-required-result-tampered-state`
waits because no tool of the gateway asks the client for input, so the core issues no
`requestState`. `SM-05` requires the status to name #45.
