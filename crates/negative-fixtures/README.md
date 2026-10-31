# negative-fixtures

The AC-07 fixture register of issue #10: LexLean's `tests/negative` at the
commit `.github/lexlean.rev` pins, each fixture classified once and bound to the
command result the LexLean CLI gives for it. Dev and CI only. Nothing here runs
a LexLean command; the tools of F4 to F7 and of #11 start from this table.

| Path | Content |
| --- | --- |
| `register.toml` | one row per fixture: command, exit status, diagnostic codes, class, reason, digests and git trees |
| `lexlean/<name>/` | `case.toml` and `expected/*` of the fixture, byte-identical to LexLean's |
| `src/lib.rs` | the register, the git object hashing and the gates |
| `examples/import.rs` | rewrites `lexlean/` and `register.toml` from a LexLean checkout |
| `tests/register.rs` | the tests of FX-01 to FX-08 |

`FIXTURES.md` at the repository root states the counts, the classes and the
planted defects.

## What is vendored, and why

The authority rule of `model/authorities.toml` asks for a citation and a
checksum, not for the bytes. The bytes are here because a digest nobody can
recompute is a claim nobody can check: with the 1188 expected files and 298
`case.toml` files (about 390 kB) in this repository, a test recomputes every
digest, reads every exit status and diagnostic code from the result itself, and
recomputes the git tree of every fixture directory and of `tests/negative`.

The `project/` directories (about 20 MB in all), and the `toolchain/` and
`overlay/` directories a few fixtures carry, are not copied. Their git tree
hashes are recorded in `register.toml` (`project_tree`, `extra`); a git tree
hash is a function of the names and contents beneath it, so the recomputed
hashes bind the register to the whole pinned tree without holding the sources.
The projects themselves are LexLean's, and the tools that run them read them
from LexLean at the pinned commit.

## Regenerating

```sh
git clone --filter=blob:none --no-checkout https://github.com/UOR-Foundation/LexLean <dir>
git -C <dir> checkout e0b74b81e6f49c83596a957d232c8383a5db1a25
cargo run -p negative-fixtures --example import -- <dir>
```

The example refuses a checkout that is not at the pinned commit, copies the
files, reads the tree of each `project/` and of any other directory from git,
checks that the tree it recomputes for each fixture directory is the one git
reports, and writes `register.toml`. It never reads a class from the checkout:
the class of a row is a function of its command and of the exclusions issue #10
names (`class_of` in `src/lib.rs`). Moving the pin means moving
`.github/lexlean.rev`, `PIN_COMMIT` and `PIN_TREE`, running the example, and
updating the counts in `model/ids.toml` that FX-01 and FX-06 state.
