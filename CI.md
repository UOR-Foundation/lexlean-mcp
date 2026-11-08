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
| `lexlean / verify` | `lexlean fmt` on a scratch copy, compared for every module but the PrismPM copies under `src/Production` (`root_fmt_check`), `lexlean lock --check`, `lexlean check`, `lexlean verify` |
| `lexlean / export <crate>` | one job per `crates/*/generated/request.json`, found by `find` and never listed: `lexlean production build --request <it>` in a clean `.lexlean/production`, then every exported `rust-std` package is compared with `diff -r` against the package committed beside the request, in both directions (a committed package the request no longer exports, or an export nobody committed, fails by name) |
| `gateway / prismpm build` | the one `prismpm build` of a run, which the two jobs below need: the MCP sources at `.github/mcp.rev` (`gateway/lay-out-mcp.sh`) and their byte equality to the pin (`gateway/mcp-identities.sh`, AC-03 a); the `prism.arch` lexicon of PrismPM at `.github/prismpm.rev`, laid out whole (`gateway/lay-out-sdk.sh`, not committed), and the two whole-file copies of PrismPM's standard library (`sdk_copies_agree`); `lexlean lock --check` and `check`, and every module that is not a copy canonical; `prismpm build`, which verifies the gateway and whose exported core must equal `gateway/generated/lexlean_mcp_gateway_core` (`diff -r`); then the generated packages as the artifact `gateway-build` |
| `gateway / prismpm build and verify` | over the packages of that build and against the registry that `gateway/registry.sh` assembles from the pins: `cargo fmt`, `clippy` and the tests of the binding, with its ignored tests run by count; the generated host source free of tool and method names (Q6); `gateway/vectors/generate.py check`; `gateway-host` built and served under MCP's server suite and schema oracle, whose failing scenarios must be exactly `gateway/waiting.json` (GATEWAY-CONFORMANCE.md, `GW-12`); and `prismpm verify`, which builds again in the locked SDK image |
| `gateway / verify fixtures` | runs VP-07 and VP-08 by name and requires that each ran: the 32 `verify` fixtures through the tool beside the CLI (AC-07), and every example but `uor-atlas` (AC-05). The tests link the packages of that build, and it runs none of the gateway job's other gates. |
| `gateway / images` | builds the release programs from that build's packages, then the images of `deploy/images` (IM-01, IM-02), runs the services apart from them and requires every check written in `deploy/images/run.sh` (IM-03 to IM-05). `IMAGES.md`. |
| `scope / does the change reach the gateway` | publishes whether the gateway job and the job of the verify fixtures run, and so the build that either needs: on a pull request, only when a changed file matches `GATEWAY_INPUTS` or `VERIFY_FIXTURES_INPUTS` in `.github/lexlean-ci/lib.sh` (the files each job reads); on a push to `main`, always |
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

The root pin is the LexLean v0.4.0 release (`e0b74b81`, published to crates.io as
`lexlean` 0.4.0 on 2026-10-26), as release policy asks (#6); it moved from the
`release/v0.4` commit `9796067` together with the vendored fixtures and API
surface (`negative-fixtures`, `quality-map`). No root crate links `lexlean`, so
there is no Cargo dependency to lock yet; the first one is added with `cargo
deny check sources` passing. The gateway keeps MCP's pin (`gateway/mcp/lexlean.rev`),
which is unreleased.

Every third-party action is pinned by its full commit SHA, and the elan and
cargo-deny archives are pinned by version and SHA-256 and checked before they
are unpacked.

## The gateway job

The job takes hours, so a pull request that changes none of its inputs skips it
(`scope`). `main` runs it on every push, so an input missing from `GATEWAY_INPUTS` fails there
rather than passing unseen. Planted defect: with `'^gateway/'` removed from the list, a change to
`gateway/Cargo.lock` alone published `gateway=false`; restored, it publishes `gateway=true`.

The verify fixtures take about 55 minutes, so they have a job of their own, in parallel, gated on
the second output of `scope` (`verify_fixtures`, `VERIFY_FIXTURES_INPUTS`). `main` runs it on every
push. The gateway job skips the test by name, and the job names it and requires that it ran, so a
renamed test fails. It is measured on scratch commits: a tool test, the verifier and the pin of MCP
publish `verify_fixtures=true`; the signer publishes `gateway=true verify_fixtures=false`; `README.md`
publishes both false; a push to `main` publishes both true. Planted defect: with `'^gateway/tool-tests/'`
removed from `VERIFY_FIXTURES_INPUTS`, a change to `gateway/tool-tests/tests/equality.rs` alone published
`verify_fixtures=false`; restored, it publishes `verify_fixtures=true`.
The build job saves the `lexlean` it builds at MCP's pin under that pin's key: `prepare` saves only
the root pin's, and without the save every run rebuilt it (about five minutes). The two jobs after
it restore that key.

### One build in a run

A run verifies the gateway in `prismpm build` and again in `prismpm verify` (step 7q), whose second
verification belongs to PrismPM#342. Two further repeats are gone. Step 7d ran `lexlean verify`
immediately before `prismpm build`: `production::build` at MCP's pin calls `engine.verify` with
`Selection::Entrypoints`, which is the selection `lexlean verify` makes without arguments, and
refuses the export when it fails. And both jobs below ran their own `prismpm build`.

A job reads another's artifact only after that job has ended, so the build is a job of its own and
the gates that need no build (7a, 7c, 7d) run ahead of it, so that a formatting error is not
reported after the verification. The job runs when either scope is true; the gateway job and the job
of the verify fixtures each need it to have succeeded and each keep their own scope. Because a failed
build skips them, the run fails on the build job, not on theirs.

The artifact `gateway-build` is one tar of the three packages `gateway/lay-out.sh` lays out, named by
the commit it was built from, and lives for one day. It is a tar because an artifact does not keep
file modes. It holds nothing else, because nothing later reads more: Cargo reads the packages; the
tests that compare projects copy them without `.prism` and `.lexlean`; `gateway/vectors/generate.py`
and step 7i read the laid-out packages; and `prismpm verify` makes its own build and starts from an
empty `gateway/.prism`, where it writes as the container's user. `gateway_build_unpack` refuses what
this revision's build did not produce:

| Planted defect | Command | Diagnostic | Restored |
| --- | --- | --- | --- |
| the archive of another revision: a commit made after packing | `gateway_build_unpack` | exit 1: `the artifact holds no build of <head>: <other>.tar` | the same commit: `the generated packages of <head> are laid out, and the core is the committed export`, and `gateway-host/run` has mode 755 |
| an archive without `gateway-host` | `gateway_build_unpack` | exit 1: `the build of <head> holds no package gateway-host` | as above |
| a line added to the committed export | `gateway_build_unpack` | exit 1: the `diff -r` hunk, then `the core of the build of <head> differs from gateway/generated/lexlean_mcp_gateway_core` | as above |
| `\|\| needs.scope.outputs.verify_fixtures == 'true'` removed from the `if` of the build job, which would skip the fixtures with their scope true | `cargo test -p repo-model --test gateway` | `the build runs when either job that reads it runs` | the test passes |
| `--env LEXLEAN_VERIFY_WIDTH` removed from `gateway_verify`, which would run step 7q at width 1 | `cargo test -p repo-model --test gateway` | `prismpm build and prismpm verify run at the width of GATEWAY_VERIFY_WIDTH` | the test passes |

`.github/lexlean-ci/test-gate.sh`, which `prepare` runs, holds the first three as cases.

### The width of the verifications

`prismpm build` and `prismpm verify` run at `GATEWAY_VERIFY_WIDTH` (`lib.sh`), passed as
`LEXLEAN_VERIFY_WIDTH` to the process that calls `verify`, and through `docker run --env` in step 7q,
where the container has no other environment. The width schedules processes and changes no output
(LexLean SPEC §22.11). The tests that run verifications keep the default: `run_cli` removes the
variable (`gateway/tool-tests/src/lib.rs`), and the projects of those tests are small.

`LEXLEAN_CI_TELEMETRY` is passed the same way, and each job keeps the directory as an artifact
(`gateway-verify-telemetry-build`, `-verify`). No stage reads it. It is where the peak memory, swap
and wall time of each process are read, and it can be turned into the hints of §22.11. The memory
budget compares the expected peaks of such hints, so without hints it would do nothing and none is
set. `child_timeout_ms` (`gateway/lexlean.toml`) stays 20 minutes: a width that a process cannot
finish under fails the build with `LLS8002`.

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

### `prismpm verify` runs in the locked SDK image

`prismpm verify` takes its executables (`lake`, `lean`, `rustc`, `rustfmt`, `cargo`, `timeout`) from the
signed inventory of the SDK it runs in, and stops with PP5008 on a bare runner, whose toolchain is
not the pinned one (no pin is patched or relaxed). So step 7q runs it inside the image that
`prismpm.lock` selects (`sdk_image`), as `bootstrap` runs `just vv`: the `prismpm` that
`install_prismpm` built (PrismPM at `.github/prismpm.rev`, LexLean at MCP's pin) is mounted
read-only and called by its path, which is not on the image's PATH, and the workspace is mounted at
its own path, so the registry's directory source resolves inside the container. `prismpm build` and
the rest of the job stay on the runner. PrismPM#342's stack leaves the pins of `tools.lock` and
`verification.rs` as they are (no hunk of the stack touches an executable digest).
The step before it removes toolchains the runner preinstalls and no job uses (`free_runner_disk`), so
the image fits beside the job's builds.

### The conformance steps (AC-03 b)

Steps 7k to 7p serve the built `gateway-host` and run MCP's own oracle over it
(GATEWAY-CONFORMANCE.md has the table and the planted defects). The oracle is not copied:
`gateway_conformance_checkout` fetches UOR-Foundation/MCP at `.github/mcp.rev`, refuses any
other HEAD, and `server-suite.mjs`, `schema-oracle.mjs` and MCP's `fetch-suite.sh` run from
that checkout, against the suite commit MCP pins. The job passes only when the failing
scenarios are exactly `gateway/waiting.json`; the oracle's diagnostic names an unlisted
failure, a stale entry or an entry without an issue, and the last 40 lines of it reach the
annotation. Node and the suite exist only in this job: the two tests of
`crates/model/tests/gateway_conformance.rs` that need them are `#[ignore]`d for the workspace
gate (the SDK image has neither) and step 7m runs them with `-- --ignored`, requiring that as
many passed as the source ignores (the pattern of MCP's CF-08).

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

The defects planted in the gateway and what rejects each are in GATEWAY.md, "Planted defects"; those planted in the tools of F4 to F7 and in `lexlean_explain` are in TOOLS.md.

The gateway job also runs the tests that put each tool beside the `lexlean` CLI (step 7h2): it lays LexLean out at the pin with `gateway/lay-out-lexlean.sh` (the `LEXLEAN_SOURCES` the tests read, with the diagnostic registry `model/errors.toml`), requires that `gateway/explain/generate.py check` finds `gateway/src/Explain.lex.tex` to be the rendering of that registry (the explanations compiled into the core, TOOLS.md), runs twelve ignored tests of `gateway-tool-tests` with `--ignored`, and requires that exactly twelve ran (`gateway_ignored_tests gateway-tool-tests 12 the_verify_fixtures_give_the_result_of_the_cli_vp_07`). The ordinary tests of the same crate (step 7h) include `tests/origin.rs`, which runs the served adapter over a socket (GW-13).

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

The vectors of the draft tools (#9) name the profile `tiny`, which #8 provisions in a real store.
`gateway_verify` therefore starts `prismpm verify` from a store that `gateway/vectors/generate.py
provision` has filled with the profiles the vectors were recorded with (the same function that
`record` uses), and the ordinary tests of `gateway-binding` and `gateway-tool-tests` include the
draft tests (`tests/materialize.rs`, `tests/drafts.rs`), none of which is ignored.

The tests that verify run the service of `gateway-verifier-host` on a thread, over a real socket, and
`gateway-verifier` as its program. No vector starts a verification, so the replay needs no verifier
host.

The vectors of the four Engine effects are answered by `gateway-engine-host` (ENGINE-HOST.md).
`gateway_verify` starts it beside the container of `prismpm verify`, on the same store, and hands
its socket to the replay with `LEXLEAN_MCP_ENGINE` through a directory both can reach. The replay
inherits that variable as it inherits `LEXLEAN_MCP_STORE`.
