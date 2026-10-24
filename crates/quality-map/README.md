# quality-map

The executable measures of Q2 and Q8 of issue #15, pinned against the LexLean
commit `.github/lexlean.rev` names. Dev and CI only. Nothing here runs a
LexLean command.

| Path | Content |
| --- | --- |
| `sources.toml` | the six vendored files: path in LexLean, git blob id, SHA-256, length, role |
| `lexlean/` | the files, byte-identical to LexLean's (the Rust files carry a `.txt` suffix so that no tool of this workspace reads them as code) |
| `map.toml` | the API-to-tool map: the 14 tools the issues define and one row per public function |
| `src/lib.rs` | the derivation of the public functions, the registry and the checks |
| `examples/import.rs` | rewrites `lexlean/` and `sources.toml` from a LexLean checkout and prints the functions it finds |
| `tests/surface.rs`, `tests/explain.rs`, `tests/plants.rs` | the tests of QM-01 to QM-06 and QM-08 |

`crates/conformance/tests/quality.rs` holds the test of QM-07, which ties the
statuses in `src/Quality.lex.tex` to the rows. `QUALITY.md` at the repository
root states the evidence, what it does not show, and the planted defects.

## What is measured

**Q2.** The public functions of the stable surface (SPEC 24.1, 24.6, 24.7 and
24.8) are the `pub fn` items of four files: `api.rs` and `api/production.rs`
(the methods of `Engine`, `ProductionRequest`, `ProductionResult` and
`ReplayReport`, and the free functions), `sdk.rs`, and the two lifecycle
functions of `verify/lifeline.rs`. `derive` finds them in the vendored text:
a `pub fn` at the left margin is a function of its module, one four columns in
and inside an `impl` at the left margin is a method, `pub(crate)` is not public,
and the scan stops at the unit tests. `map.toml` has a row for each, with the
signature the source declares and either the one tool that exposes it or a
reason no tool does. A function added, removed or re-signed upstream, or left
unmapped here, fails a test.

**Q8.** `model/errors.toml` registers the diagnostic codes and `ERRORS.md` is
generated from it; `lexlean explain <CODE>` prints the entry of the code. The
tests require every row to be complete, `ERRORS.md` to hold exactly the entry
each row renders to, and every code the 297 negative fixtures return (the
register of `negative-fixtures`) to be one of them.

## Regenerating

```sh
git clone --filter=blob:none --no-checkout https://github.com/UOR-Foundation/LexLean <dir>
git -C <dir> checkout 97960679736e960e84a98f5c43981e1f2c70d6d7
cargo run -p quality-map --example import -- <dir>
```

The example refuses a checkout that is not at the pinned commit, and refuses a
file whose git blob id differs from the id of the bytes it copied. When the pin
moves, run it again, review the functions it prints against `map.toml`, update
the model counts in `src/Quality.lex.tex`, the checksums in
`model/authorities.toml` and the statements in `model/ids.toml`.
