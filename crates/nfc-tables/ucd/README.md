# Unicode Character Database 17.0.0, pinned

Four files of the Unicode Character Database 17.0.0, the input of the generated
normalization tables (`artifacts/nfc-tables-low.bin`, `nfc-tables-high.bin`, `nfc-sample.bin`, `src/NfcTables.lex.tex`) and the
oracle of the NFC decision. They are data for the generator and the tests; no
other file reads them. Terms of use: <https://www.unicode.org/terms_of_use.html>
(the Unicode License v3; each file's header cites it).

| File | SHA-256 | Bytes |
| --- | --- | --- |
| `UnicodeData.txt` | `2e1efc1dcb59c575eedf5ccae60f95229f706ee6d031835247d843c11d96470c` | 2198209 |
| `CompositionExclusions.txt` | `2f239196ef3b5b61db5cc476e9bd80f534d15aa1b74e1be1dea5d042a344c85f` | 9007 |
| `DerivedNormalizationProps.txt` | `71fd6a206a2c0cdd41feb6b7f656aa31091db45e9cedc926985d718397f9e488` | 1377582 |
| `NormalizationTest.txt` | `5019ffd530751a741900c849c0e010332f142a3612234639bd200b82138a87db` | 2827429 |

The digests are recorded again in `model/authorities.toml` and in
`crates/nfc-tables/src/lib.rs`; the tests compare all three.

## Where the copies come from

`https://www.unicode.org/Public/17.0.0/ucd/` is not reachable from the host that
produced this commit (its egress allows package registries and GitHub). The copies
are the publisher's own source files: the Unicode Consortium's repository
<https://github.com/unicode-org/unicodetools>, from which the published UCD is
built, path `unicodetools/data/ucd/17.0.0/<name>`, at commit
`578f6c0516f5f268292ebda50b51848407a06478` (`main` on 2026-10-22). Each committed
file is byte-identical to the file at that commit (fetched from
`raw.githubusercontent.com/unicode-org/unicodetools/<commit>/unicodetools/data/ucd/17.0.0/<name>`
and compared with `cmp`). The first line of each of the three versioned files
names the release (`# NormalizationTest-17.0.0.txt`, `# DerivedNormalizationProps-17.0.0.txt`,
`# CompositionExclusions-17.0.0.txt`); `UnicodeData.txt` has no header line.

## How the copies were shown to be the 17.0.0 originals

The Rust crate `unicode-normalization` 0.1.25 is what LexLean `release/v0.4` at
`e0b74b8` decides NFC with (its `Cargo.lock`), and its `src/tables.rs` states
`UNICODE_VERSION = (17, 0, 0)`. The crate publishes the script that generated its
tables, `scripts/unicode.py` (identical in the crates.io package and at the commit
`5a69b3bafb625caccdec7871a42bed0d9a6604d1` the package's `.cargo_vcs_info.json`
names), which reads `UnicodeData.txt`, `DerivedNormalizationProps.txt`,
`StandardizedVariants.txt` and `NormalizationTest.txt` from `UCD_URL`. That script was
run unchanged except for reading the four files from a local directory instead of
`urllib.request.urlopen(UCD_URL + filename)`, over the copies committed here and a
copy of `StandardizedVariants.txt` from the same directory of `unicodetools`
(SHA-256 `f55100b2fb11d3d75a37b8c1ab752192dbd1c4b12328c5ec6b38e3807c0ca597`; the
script reads it for the CJK compatibility variants, which no table here uses).

- the `tables.rs` it wrote is byte-identical to `src/tables.rs` of the crate
  (SHA-256 `177d5f08019cc8e335444fcab61aabb7f6309f158f6ebbd7525c73c0e532ec44`);
- the `normalization_tests.rs` it wrote is byte-identical to `tests/data/normalization_tests.rs`
  of the crate's repository at that commit
  (SHA-256 `8f497f9c12661a6482865ebf13212e56361097a6da72be53c26d46a42af1cd71`).

So `UnicodeData.txt`, `DerivedNormalizationProps.txt` and `NormalizationTest.txt`
produce exactly the tables and the test vectors of the crate LexLean pins. That is
a consequence of the files being the ones the crate was generated from; it does not
rest on trusting where the copies were fetched from. `CompositionExclusions.txt`
is not read by the crate's script. Its source is the publisher's repository at the
commit above (its header names 17.0.0, 2025-08-01), it is pinned by digest, and the
generator checks it against the derived property: the listed exclusions, the
singleton decompositions and the non-starter decompositions are exactly
`Full_Composition_Exclusion` of the pinned `DerivedNormalizationProps.txt`
(`tables::check_exclusions`). The decision does not read it: it reads
`Full_Composition_Exclusion`.
