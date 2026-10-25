# The Atlas profile of the pinned LexLean

Derived extracts that PF-07 and PF-08 are held against. They are facts of
UOR-Foundation/LexLean at commit 97960679736e960e84a98f5c43981e1f2c70d6d7
(release/v0.4 at 9796067, the revision `.github/lexlean.rev` pins), not
claims made here; `model/authorities.toml` names them as `A-LEXLEAN-ATLAS`.

| File | What it is | Source at the pin |
| --- | --- | --- |
| `lexicon.toml` | verbatim copy | `language/uor/atlas/lexicon.toml` (git blob `06a6fd06742185f8292377fea7c9ec86b6e71a76`) |
| `entries.tsv` | one line per entry file: id, category, denotation kind, denotation module, denotation component, tab separated, in byte order of the file name | the 293 files of `language/uor/atlas/entries` (git tree of `language/uor/atlas` `0f3b5a884831c378723d1e890c890a0821f4c45f`) |
| `modules.txt` | one line per source file: its module name, which is the path below `src/` without `.lex.tex` and with `/` written as `.` | the 86 `.lex.tex` files of `examples/uor-atlas/src` (git tree `3c4ef471b81bbf3aacc2eb0f2eed311b82faeab8`), in byte order of path |

## How the copy was made

A partial clone of the pinned commit (`git fetch --depth 1 --filter=blob:none`
of the commit, sparse paths `language/uor` and `examples/uor-atlas/src`), then
a script read each entry file with a TOML parser and each source file for its
`\begin{lexlean}` module header, and wrote the three files. No field was
edited. The module name equals the path below `src/` for all 86 files, 80 at
the top and 6 under `Prelude/`, each a `coremodule`, as
`examples/uor-atlas/README.md` states ("86 modules: 80 Atlas modules and 6
under `Prelude/`").

The tests recompute the SHA-256 of each file and compare it with the checksum
in `model/authorities.toml`, so an edited extract fails.
