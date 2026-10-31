# The AC-07 fixture register

Issue #10 asks that the tools of F4 to F7 and of #11 give, for LexLean's
negative fixtures, the command result the CLI gives. This file states what the
register in `crates/negative-fixtures` holds, the counts it checks, and the
defects planted in its gates. It runs no LexLean command; the tools that do are
issue #10 and #11.

## Claims

| ID | Level | Claim |
| --- | --- | --- |
| FX-01 | some-true | the pinned `tests/negative` is the git tree `f5a4399` with 298 fixtures |
| FX-02 | some-true | the expected result of each is the canonical command result of the CLI |
| FX-03 | build | the vendored files and the recorded trees recompute to the pinned tree |
| FX-04 | build | every row is what the vendored bytes give |
| FX-05 | build | every fixture is classified exactly once |
| FX-06 | build | the counts are recomputed from the rows |
| FX-07 | build | the exclusions are the twelve of the issue, with its reasons |
| FX-08 | build | each gate rejects its planted defect |

FX-01 and FX-02 are reproduced from LexLean (`A-LEXLEAN-NEGATIVE`, AUTH-07 and
AUTH-08). FX-02 rests on the `check-fixtures` gate of LexLean at the pinned
commit; this repository records the results and does not run the CLI.

## The counts differ from the issue

The pinned LexLean, v0.4.0, commit `e0b74b81e6f49c83596a957d232c8383a5db1a25`
(`.github/lexlean.rev`), has 298 negative fixtures. Issue #10 counts 285, with
273 in scope and 240 for F4. Those are the counts of LexLean `main` and
`release/v0.5` (285 fixtures, observed on 2026-10-23 at `main` `6bcc8d5`).
`v0.4.0` has the same fixtures and twelve more, all of the command
`check`: `cases-bool-branch-binder`, `cases-bool-branch-repeated`,
`cases-bool-induction`, `cases-bool-language-11`, `cases-bool-missing-branch`,
`term-cases-definition-absent`, `term-cases-definition-repeated`,
`term-cases-definitions-unsorted`, `term-cases-empty-definitions`,
`term-cases-language-11`, `term-cases-reserved-name` and
`term-cases-scrutinee-not-inductive`.

The register applies the issue's rules to the pinned tree and does not force
its numbers:

| | Pinned (298) | Issue |
| --- | --- | --- |
| `check` through F4 | 252 | 240 |
| `build` through F7 | 1 | 1 |
| `verify` in #11 | 32 | 32 |
| in scope | 285 | 273 |
| excluded | 13 | 12 |
| total | 298 | 285 |

By command the 298 fixtures are 256 `check`, 1 `build`, 1 `migrate`, 32 `verify`,
4 `lock` and 4 `production`. The four `check` fixtures that are excluded are
`check-unknown-flag` and `check-duplicate-selection` (CLI argument parsing) and
`path-symlink` and `model-artifact-symlink` (symlinks); 256 - 4 = 252.

The excluded fixtures are the twelve the issue names and `migrate-module-without-notation`,
the one fixture of the command `migrate`, which no tool runs (added with the move to v0.4.0).
Every name exists at the pinned tree:

| Reason | Fixtures |
| --- | --- |
| CLI argument parsing | `check-unknown-flag`, `check-duplicate-selection` |
| symlinks | `path-symlink`, `model-artifact-symlink` |
| setup | `git-lexicon-offline`, `lexicon-unsafe-surface`, `package-language-mismatch`, `stale-lock` |
| outside the product | `production-expectation-stale`, `production-request-malformed`, `production-root-unknown`, `production-target-unregistered` |

When the pin moves to a release that holds 285 fixtures, the importer rewrites
the register and the counts in `model/ids.toml` (FX-01, FX-06) follow it.

## What the register records

Each row holds the name, the command of `case.toml`, the exit status and the
diagnostic codes of `expected/command.json`, the SHA-256 of that file and of
`case.toml`, the class (`F4`, `F7`, `#11` or `excluded`), the reason of an
excluded row, and the git trees of the fixture directory and of `project/`.
Thirteen fixtures carry another directory (twelve a `toolchain/`, `stale-lock`
an `overlay/`); their trees are the `extra` key of the row, so #11 can tell
which fixtures need a provisioned toolchain.

The command of a row is the command of `case.toml`. The two CLI argument
fixtures and the two symlink fixtures run `check`; their reason says why no tool
reaches them. `stale-lock` runs `lock` twice; its row holds the final result.

`project/` is not vendored, and neither are the `toolchain/` and `overlay/`
directories. The authority rule asks for a citation and a checksum; the
expected files are vendored in addition, 1490 files of about 390 kB, because
then the digests, the exit statuses, the codes and the git trees are recomputed
from bytes in this repository and a hand edit of any of them fails. The
checksum of `A-LEXLEAN-NEGATIVE` is the SHA-256 over the lines
`<sha256>  <path>` of those files in byte order of path.

## Planted defects

Each defect was planted in the working tree, `cargo test -p negative-fixtures`
was run, and the file was restored. The same defects are planted in memory by
FX-08, so the gates are checked on every run. The restored tree passes all 8
tests.

| Defect | Planted | Failing tests | Diagnostic |
| --- | --- | --- | --- |
| a fixture reclassified | `class = "#11"` for `application-arity` | FX-04, FX-05, FX-06, FX-08 | `application-arity: class is #11, its command check and the issue's exclusions give F4` |
| a digest altered | first digit of `result_sha256` of `application-arity` | FX-03, FX-04, FX-08 | `application-arity: expected/command.json has SHA-256 6d49c4d9..., the register says 0d49c4d9...` |
| a vendored byte altered | a newline appended to `lexlean/application-arity/expected/command.json` | FX-03, FX-04, FX-08 | `expected/command.json has SHA-256 08dccae9...`, then `the vendored files make the tree 3af1d27b..., the register says 785a7075...` |
| a vendored file removed | `expected/hashes.toml` of `application-arity` deleted | FX-03, FX-04, FX-08 | `application-arity: the vendored files make the tree 0d707d89..., the register says 785a7075...` |
| an exclusion without a reason | `reason` line of `stale-lock` deleted | FX-04, FX-05, FX-07, FX-08 | `stale-lock: excluded without a reason` |
| an exclusion the issue does not make | `application-arity` excluded with a reason | FX-04, FX-05, FX-06, FX-07, FX-08 | `application-arity: class is excluded, its command check and the issue's exclusions give F4` |
| a name listed twice | the first row repeated | FX-01, FX-03, FX-05, FX-06, FX-08 | `the register names 298 fixtures and 297 are vendored, or their names differ` and `listed twice` |
| a fixture missing | the row of `application-arity` deleted | FX-01, FX-03, FX-05, FX-06, FX-08 | `the register names 296 fixtures and 297 are vendored, or their names differ` and `the rows make the tree b519e8a6... for tests/negative, the pin is 200e4d92...` |
| a hand-edited count | `252 for F4` in the FX-06 statement changed to `253 for F4` | FX-06 | `FX-06 states other counts than the register has: 297 fixtures: 252 for F4, 1 for F7, 32 for #11 and 13 excluded, so 285 reach a tool` |

Restore: put the file back (`git checkout` for `model/ids.toml`, the committed
copy for the others); the run after the restore passes.

## Regenerating

See `crates/negative-fixtures/README.md`.
