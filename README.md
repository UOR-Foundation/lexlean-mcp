# LexLean MCP

LexLean MCP serves [LexLean](https://github.com/UOR-Foundation/LexLean) to LLM
clients over the Model Context Protocol. LexLean establishes meaning over the
UOR Atlas; this server adds no meaning of its own. A document an LLM writes
through it reaches its reader only as the canonical rendering of a verified
LexLean module, bound to its signed attestation.

The protocol is implemented once, in LexLean, by
[UOR-Foundation/MCP](https://github.com/UOR-Foundation/MCP), and accepted only
on MCP's own conformance oracles. This repository holds the product model and
the LexLean tool layer, and links that protocol core unchanged.

## Status

The product, its architecture description, its threat model, its quality model,
and the pure part of profile and draft identity and of path admission are
modeled. One capability is built: the Unicode 17.0.0 NFC decision (#25),
registered as NF-01 to NF-15 and used by path admission. The AC-07 fixture
register of #10 is checked: FX-01 to FX-08 classify the 297 negative fixtures of
LexLean `release/v0.4` at `9796067`, 285 of which reach a tool, with the expected
command result of each. No tool runs them yet.

- The product model is LexLean language 1.2 notation (`src/`), built with
  LexLean `release/v0.4` and locked in `lexlean.lock` (`lexlean/lock/2`).
  `lexlean verify` accepts it with an exact axiom policy on every declaration.
  Every owner-set value of #19 is bound to a proposal that
  [OWNER-VALUES.md](OWNER-VALUES.md) justifies and the owner can override.
- The claim registers in `model/` hold the NFC decision's rows (NF-01 to NF-15),
  its authorities (the Unicode standard annex, the four pinned Unicode Character
  Database files and `unicode-normalization` 0.1.25) and their claims, and the
  fixture register's rows (FX-01 to FX-08) with the negative fixtures of the
  pinned LexLean as their authority (`crates/negative-fixtures`, `FIXTURES.md`);
  `CONFORMANCE.md` lists them.
- One measurement of the threat model is executed: R5, path escape (#14), by the
  admission rows NF-09, NF-12 and DL-01 to DL-04, registered as SM-01. The symlink
  clause of its control waits for materialization (#9), and the other eight
  controlled risks stay pending on the issues that build them.
- The repository is not bound to a PrismPM SDK release (#4): `prismpm.lock` and
  `template.lock` are absent, so `just vv` and the devcontainer cannot run, and
  `prismpm check` has not validated the model.
- v0.1.0 is tracked in #20, with its upstream dependencies in LexLean, PrismPM,
  the UOR template and UOR-Foundation/MCP.

v0.1 is scoped to Streamable HTTP on the loopback interface, with one principal
and no authorization: the only binding every part of which an executable MCP
oracle covers. stdio, MCP Apps and remote, authorized deployment wait for
oracles of their own (UOR-Foundation/MCP#15).

## Model

| Path | Owner |
| --- | --- |
| `src/` | authoritative LexLean model modules |
| `language/prism.arch/` | the architecture vocabulary lexicon, a copy of prism-code's |
| `language/prism.sec/`, `language/prism.qual/` | the security and quality vocabulary lexicons, copies of prism-code's |
| `language/lexleanmcp.titles/` | the lexicon phrases that title this repository's modules |
| `lexlean.toml`, `lexlean.lock` | LexLean project and its locked package closure |
| `lakefile.toml`, `lean-toolchain`, `lake-manifest.json` | Lean workspace bound by `lexlean.lock` |

| Module | Content |
| --- | --- |
| `Product` | identity, owner, server identity, protocol core, source policy, platforms, and the owner-set version (bound, #19) |
| `Architecture` | the ISO/IEC/IEEE 42010 architecture description in the `prism.arch` vocabulary: 8 stakeholders, concerns C1 to C8, the component, security and quality viewpoints, 10 components and 11 edges, checked by `architectureDescriptionHolds` |
| `Security` | the threat model in the `prism.sec` vocabulary: assets A1 to A6 tied to architecture components, risks R1 to R10 with impact and an owner-set likelihood (bound, #19), controls K1 to K9, four security activities, and a measurement per risk; `securityValid` and `riskChainHolds` prove that every risk has a measurement and that R10 is the only risk without a control, recorded as residual. One measurement is `Executed`: R5 (path escape), observed by the path admission rows (`SM-01` names them, and `onlyPathEscapeIsExecuted` pins it as the only one). The symlink clause of control K5 is recorded in `pendingClauses` until materialization (#9) builds it, and the other eight controlled risks stay `Pending` (`measurementsPendingExceptR5`). the risks are `Foundation.Sec::Risk` records, which the SDK projector reads. [SECURITY.md](SECURITY.md) holds the evidence and the planted defects |
| `Quality` | the quality model in the `prism.qual` vocabulary, ISO/IEC 25010:2023: requirements Q1 to Q8, each a characteristic, subcharacteristic, requirement, measure chain with an all-or-none threshold, realized by components of the quality view; `qualityValid` and `qualityChainHolds` prove the chains complete |
| `Identity` | the pure part of profile identity (#8) and draft identity (#9): length-prefixed framing of fields and counted lists over octet strings, with a proved left inverse (`fieldsRT`, `listRT`, `profileRT`, `draftRT`), so two different profiles or drafts have different preimages whenever every field is shorter than 2^64 - 1 bytes; domain separation of the two preimages; canonical order for everything that is a set: file and source rows sort by path octets and review records sort by their octets (unsigned, lexicographic), so an ID depends only on what a profile or draft contains and never on the order its parts were produced in, and a duplicate path or a duplicate review record is a typed refusal (`DuplicatePath`, `DuplicateReview`); the frame labels `domain`, `files`, `file`, `reviews`, `review`, `toolchain`, `profile`, `sources` and `source` are part of every ID, and the draft preimage frames the profile ID as its 32 raw digest octets; the `Bytes` to octet bridge; `profileId` and `draftId` as SHA-256 of the preimage, checked against vectors from an independent implementation. It does not claim SHA-256 collision resistance |
| `NfcTables` | generated by `cargo run -p nfc-tables --example generate_tables` from the pinned Unicode Character Database 17.0.0 files in `crates/nfc-tables/ucd/` (SHA-256 pinned in `model/authorities.toml`): the combining classes, the composition exclusions (`Full_Composition_Exclusion`) and the canonical decompositions packed as five sorted tables behind a 30-octet header (the full tables, as two artifacts because a list-backed `Bytes` literal of about 26 000 octets exceeds the elaborator's recursion depth), a small sample image of the same layout for the kernel-checked instances, and the constants `fastBound`, `decomposesFrom`, `decompositionDepth` and `lowLength`. Never edit it: `cargo test -p nfc-tables` compares it with a fresh generation byte for byte |
| `Nfc` | the NFC decision: UTF-8 reader, canonical decomposition (Hangul by arithmetic), canonical ordering by combining class, canonical composition with the composition exclusions, and equality with the input (exact; not Quick_Check). It reads the tables by binary search. Every recursion over the input is a tail call, which the export renders as a loop, so the production root `nfcRun` declares no `recursion` effect and needs no stack that grows with the input. `nfcRun` is exported to Rust (`crates/nfc-oracle`), where every line of `NormalizationTest.txt` is run against it. 30 instance theorems are kernel-checked on the sample image |
| `Admission` | path admission as a total function from a path and the source roots to `Admitted` or a typed `Refusal`: UTF-8, NFC (`NotNfc`, decided by `Nfc` on the Unicode 17.0.0 tables of `NfcTables`), relative, no empty, `.` or `..` component, under a source root, `.lex.tex` suffix; `max_files` counts rows and `max_bytes` counts content lengths only, not path bytes (as LexLean's `max_total_source_bytes` counts source bytes), both as parameters whose values are owner-set (#19); `admittedPathHasNoDotDot` proves that an admitted path has no `..` component. `admitPath` is `admitPathWith` the full tables; the kernel-checked NFC instances run `admitPathWith` on the sample image (see `NfcTables`). The root `admitRun` (tables, roots and path in; the verdict as `[0]` or `[1, index of the Refusal constructor]` out) is a production root exported to Rust and run on the full tables by `crates/nfc-oracle`; its stages (`splitInto`, `componentsStage`, `hasSuffix`, `rootsStage`) are tail-recursive, and they use `Nfc` list functions rather than `Identity`'s, which are not executable. Draft admission (`admitDraft`, #9) decides in this order: the path refusals of the rows in row order (`rowsStageWith`), `TooManyFiles`, `TooManyBytes`, then `DuplicatePath` (a path that occurs twice among the rows, compared as octets; `Refusal.DuplicatePath` is last, so the indices of the other refusals are unchanged). The duplicate scan runs only on a draft the limits admit. It sorts the paths by octets (unsigned, lexicographic, as `Identity` orders rows) with a bottom-up merge sort whose merges are tail calls (`singletonRuns`, `mergeInto`, `mergePairs`, `sortRuns`, with fuel for termination) and compares neighbours (`adjacentRepeat`), so it makes O(n log n) comparisons. The stages that run over the rows (`rowsStageWith`, `countRowsInto`, `totalBytesInto`, `pathsInto` and the sort) are tail-recursive. The root `admitDraftRun` (tables, `max_files`, `max_bytes`, roots and rows in; the verdict as `[0]` or `[1, index]` out) is a production root exported to Rust (`crates/nfc-oracle`, package `lexlean_mcp_draft_admission`) that declares no `recursion` effect |
| `Sha256` | the LexLean standard library module `Sha256` (FIPS 180-4), a byte-identical copy of `stdlib/src/Sha256.lex.tex` from LexLean `release/v0.4` at `9796067` (register row SL-02). The module name must equal its path, so it is `src/Sha256.lex.tex` and not under `src/Foundation/` |
| `OwnerValues` | the values of #19 that no source determines, each bound to the proposal of [OWNER-VALUES.md](OWNER-VALUES.md): signing and `request-state-key` custody and rotation, `verify.width`, `verify.memory_kib`, `verify.concurrency`, `rate.tool_calls`, `rate.window_ms`, `draft.max_bytes`, `draft.max_files`, the two retentions, `cache.ttl_ms`, `listen.port`, the service level objectives S1 to S8 and the 17 profiles; the kernel-checked theorems bind the ranges, and `cargo test -p repo-model` (OV-01 to OV-03) holds the sources, the table, `lexlean.toml` and `Cargo.toml` to one statement |
| `Foundation.Arch`, `Foundation.Sec`, `Foundation.Qual` | the architecture, security and quality vocabularies (copies of the PrismPM stdlib modules, as in prism-code) |

## Repository

This is an instance of the [UOR template](https://github.com/UOR-Foundation/template).

- [AGENTS.md](AGENTS.md) sets the rules every change follows (R1–R6).
- [TEMPLATE-CONTRACT.md](TEMPLATE-CONTRACT.md) defines inherited policy and the
  SDK and template update process.
- [VERIFICATION.md](VERIFICATION.md) defines the acceptance boundary; `just vv`
  is the complete gate.
- [FIXTURES.md](FIXTURES.md) states the counts and the classes of the AC-07
  fixture register and the defects planted in its gates.
- [CI.md](CI.md) describes the workflow that runs every gate that works without
  the SDK, and how to read its failures.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
