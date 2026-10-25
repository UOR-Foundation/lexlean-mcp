# Continuous integration without the SDK

`.github/workflows/lexlean.yml` runs every gate of `just vv` that does not need
the PrismPM SDK, so a pull request has real evidence before `prismpm.lock`
exists (#4). The template's `bootstrap` workflow is unchanged and remains the
acceptance boundary (`VERIFICATION.md`); until the SDK binding lands it fails at
"Run the template gate through the locked SDK" on every pull request and on
`main`. This workflow does not replace it and is not a reduced path to release.

## Jobs and gates

| Job | Gate |
| --- | --- |
| `cargo / fmt clippy audits deny test` | `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets --locked -- -D warnings`; `cargo check --workspace --all-features --all-targets --locked`; `cargo xtask check-model`, `audit-limits`, `audit-deferral` and `audit-dependency-updates`; `cargo deny --all-features check`; `cargo test --workspace --locked --no-fail-fast` |
| `lexlean / verify` | `lexlean fmt --check`, `lexlean lock --check`, `lexlean check`, `lexlean verify` |
| `lexlean / export <crate>` | one job per `crates/*/generated/request.json`, found by `find` and never listed: `lexlean production build --request <it>` in a clean `.lexlean/production`, then every exported `rust-std` package is compared with `diff -r` against the package committed beside the request, in both directions (a committed package the request no longer exports, or an export nobody committed, fails by name) |
| `prepare / compiler and toolchains` | builds `lexlean`, installs the Lean toolchain, checks that the compiler is the one `lexlean.lock` was made with, and publishes the job matrix |

## Not run here

- `cargo xtask audit-bootstrap` (and so `xtask validate`) stops at
  "prismpm.lock is required", after `audit-dependency-updates` has run.
- `prismpm template check` and `prismpm lock check` need the SDK image.
- `cargo test --workspace` already includes the `repo-conformance` scenarios that
  `just bdd` runs.

These stay in `just vv` and in the `bootstrap` workflow.

## The LexLean pin

`.github/lexlean.rev` is the only place the LexLean revision is written. The
workflow builds `lexlean` at that commit with `cargo install --git ... --rev ...
--locked` and caches the result under a key that holds the revision. A cache
restored for another revision cannot pass, because the installed binary is
accepted only when cargo's own record (`.crates.toml`) names exactly that
revision.

`lexlean --version` prints the language-1.0 compiler-semantics ID, which a
language-1.2 lock does not record, so it is not the comparison. The step "The
compiler is the one lexlean.lock was made with" locks a copy of the committed
tree with the built binary and requires the copy's `compiler_semantics` to equal
the one in `lexlean.lock`; `lexlean lock --check` (LLC0102) then repeats the
comparison in the `verify` job. Move `.github/lexlean.rev` and `lexlean.lock` in
one commit.

Every third-party action is pinned by its full commit SHA, and the elan and
cargo-deny archives are pinned by version and SHA-256 and checked before they
are unpacked.

## Reading a failure

The job log cannot always be downloaded. Each step runs its command through
`gate` (`.github/lexlean-ci/lib.sh`), which:

- on failure, writes the last 40 lines of the command's output as one
  `::error::` annotation titled `<gate> failed (exit N)`;
- on success, writes a `::notice::` with the gate's duration and its last line,
  so a gate that executed nothing cannot look like one that passed;
- runs the command with errexit on, so a failing command inside a helper
  function fails the gate, and restores the step's errexit afterwards, so a
  failed gate in a step of several gates fails the step. Before this, `gate`
  captured `set +o` inside a command substitution, where errexit is never set,
  so the first gate of a step turned errexit off for the rest of it, and each
  helper ran with errexit off. `.github/lexlean-ci/test-gate.sh` holds the six
  cases, and the `prepare` job runs it before any gate is trusted. Planted
  against the old `gate`, four of the six cases fail.

Read them with

```sh
gh api repos/UOR-Foundation/lexlean-mcp/check-runs/<job id>/annotations
```

## Planted defect

One comment, ` // hand edit`, appended to line 10 (`pub fn tick`) of
`crates/json-oracle/generated/mcp_json_codec/src/lib.rs`. It changes no
behavior.

| | |
| --- | --- |
| failing step | `lexlean / export json-oracle`, step "5. the committed packages are the export of their sources" |
| diagnostic | `export of crates/json-oracle/generated/request.json failed (exit 1)`, then `== crates/json-oracle/generated/mcp_json_codec differs from the export of crates/json-oracle/generated/request.json ==` and the `diff -ru` hunk showing the `// hand edit` line |
| also failing | `cargo test`: `error: 1 target failed: -p json-oracle --test provenance` |
| passing | `lexlean / verify` and `lexlean / export lifecycle-oracle` |
| runs | commit `33ef09f`, run 39904326478, jobs 120253214743 and 120253076251; reverted by `c138d0e` |
