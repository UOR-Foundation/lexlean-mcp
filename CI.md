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
| `gateway / prismpm build and verify` | builds the gateway of #7 against the registry that `gateway/registry.sh` assembles from the pins: the MCP sources at `.github/mcp.rev` (`gateway/lay-out-mcp.sh`) and their byte equality to the pin (`gateway/mcp-identities.sh`, AC-03 a); the `prism.arch` lexicon of PrismPM at `.github/prismpm.rev`, laid out whole (`gateway/lay-out-sdk.sh`, not committed), and the two whole-file copies of PrismPM's standard library (`sdk_copies_agree`); `lexlean lock --check` and `check`, and every module that is not a copy canonical; the registry; `prismpm build`, whose exported core must equal `gateway/generated/lexlean_mcp_gateway_core` (`diff -r`); `cargo fmt`, `clippy` and the tests of the binding, with its ignored tests run by count; the generated host source free of tool and method names (Q6); `gateway/vectors/generate.py check`; and `prismpm verify` |
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

## The gateway job

`gateway/` is its own Cargo workspace with its own `Cargo.lock`, because the binding links
`lexlean`, which is not on crates.io yet. The root workspace, `cargo deny` and the SDK's
offline `just vv` never see it; `deny.toml` is unchanged. No manifest has a Git dependency,
no path dependency names an unpublished upstream crate (the generated host packages are
members of the workspace and are laid out by `gateway/lay-out.sh`), and no `.crate` file is
committed.

`gateway/registry.sh` assembles the one crate that is not published, from its pin:

1. `lexlean` is packaged with `cargo package` from UOR-Foundation/LexLean at the
   commit `gateway/mcp/lexlean.rev` names, which is MCP's `.github/lexlean.rev` at `.github/mcp.rev`
   (the gateway is compiled by the LexLean commit MCP is checked with; the root project keeps its own). The SHA-256 of the archive is recorded in `gateway/registry.expected`
   and in `gateway/Cargo.lock`, and the script stops when the package it just made is neither.
2. The rest of the lock is the crates.io closure of that package. Each crate is downloaded from
   `static.crates.io` and its SHA-256 is compared with the checksum in `gateway/Cargo.lock`.
3. Every package is unpacked into `gateway/registry` with the `.cargo-checksum.json` of a
   directory source, and `gateway/registry/cargo-home/config.toml` replaces crates-io by that
   directory. The job points `CARGO_HOME` at it, so the build is offline and resolves nothing else.

`gateway/registry.sh --record` rewrites `gateway/registry.expected` when the pin moves, and
`--lock` resolves `gateway/Cargo.lock` once, with a temporary `[patch.crates-io]` that is not
committed. Run them after `prismpm build` and `gateway/lay-out.sh`: the lock names the
generated members.

### The executables `prismpm verify` pins

`prismpm verify` runs only with the executables PrismPM's `tools.lock` names, by digest, and stops
with PP5008 otherwise: the elan proxy (`lake`, `lean`) of elan v4.2.3, the rustup proxy
(`rustc`, `rustfmt`) and the `timeout` of Debian's coreutils 9.1-1. The gateway job therefore
sets `ELAN_VERSION` and `ELAN_SHA256` for itself to elan v4.2.3 (the other jobs keep v4.2.4), and
`install_pinned_timeout` (`.github/lexlean-ci/lib.sh`) takes `timeout` out of the Debian package
and checks its digest before the verify step. The runner's rustup (1.28.2) is the pinned proxy.

### The release step

Uploading to crates.io is the last step of a release, at the tag. Development does not wait
for it and this job does not depend on it. At the release tag `lexlean` is published, and the
release step is:

1. delete `gateway/registry.sh`, `gateway/registry.expected`, the registry and the `CARGO_HOME`
   of the gateway job, and the `gateway/registry.sh` step;
2. keep `gateway/Cargo.lock`: it already names `lexlean` as a registry package with the
   checksum of the archive packaged from the pin, so the same version resolves from crates.io
   and cargo refuses the build when the archive crates.io serves is another one. When it is
   another one (the release is packaged from the release commit, not from the pin), run
   `cargo update -p lexlean` once with the registry gone, and review the one changed checksum;
3. pin `.github/mcp.rev`, `.github/prismpm.rev` and (through MCP) the LexLean commit to releases.

## Planted defect (the gateway)

The defects planted in the gateway and what rejects each are in GATEWAY.md, "Planted defects".

## Reading a failure

The job log cannot always be downloaded. Each step runs its command through
`gate` (`.github/lexlean-ci/lib.sh`), which:

- on failure, writes the last 40 lines of the command's output as one
  `::error::` annotation titled `<gate> failed (exit N)`;
- on success, writes a `::notice::` with the gate's duration and its last line,
  so a gate that executed nothing cannot look like one that passed.

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
