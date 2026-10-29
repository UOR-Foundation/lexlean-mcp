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
command result of each. The four engine tools run them beside the CLI of the
same commit (TL-01 to TL-09, [TOOLS.md](TOOLS.md)).
The observability model of #16 is modeled: OB-01 to OB-10 hold the telemetry
record types with redaction as a type, the eight service level indicators, the
alert predicates, trace context extraction and the closed set of notifications the
server emits. Nothing is emitted and no alert fires yet; both wait for the verify
task (#11) and the host (#17).
The model of the store and its
lifecycle (#13) is checked as well: ST-01 to ST-09 state the layout, the backup
set, retention, the drift check, the rollout and rollback machine, and AC-14 to
AC-17 as theorems over a record-resolution model. The store itself waits for
verification and signing (#11); [STORE.md](STORE.md) says what is and is not met.

The model-side slice of the profiles and vocabulary (#8) is checked too: PF-01
to PF-09 state which files a profile identity covers, the language rule
(`LLC0103`) and the Atlas profile (293 entries served by 86 modules, counts
read from the pinned LexLean). Provisioning, the tools, the resource template
and completion wait for the generated gateway; [PROFILES.md](PROFILES.md) says
what is and is not met.

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
- Two measures of the quality model are executed (#15): Q2, that each of the 33
  public functions of LexLean's stable API reaches one tool or states why none
  does, and Q8, that each of its 86 diagnostic codes is explained (QM-01 to
  QM-08, `crates/quality-map`, `QUALITY.md`). The tools themselves do not exist
  yet, so Q1 and Q3 to Q7 stay pending and so does the runtime half of Q2 and Q8.
- The 13 features and 6 journeys of #18 are declared (`Features`), checked by kernel
  theorems and by FJ-01 to FJ-10. `prismpm check` has not validated them: the SDK
  image bound in `prismpm.lock` predates PrismPM#330. [FEATURES.md](FEATURES.md)
  states where each feature and journey comes from and which rows await the owner.
- The closed host effect set of #7 is declared (EF-01 to EF-08, `EFFECTS.md`): E1 to E10, a total codec, the negative theorem and the acceptance vectors. The generated host adapter and the `gateway` binary wait on PrismPM#216 and MCP#14 and are not built.
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
| `language/prism.arch/` | the architecture vocabulary lexicon, a copy of PrismPM main at `d11ace6f` (authority `A-PRISMPM-ARCH`, FJ-09) |
| `language/prism.sec/`, `language/prism.qual/` | the security and quality vocabulary lexicons, copies of prism-code's |
| `language/lexleanmcp.titles/` | the lexicon phrases that title this repository's modules |
| `lexlean.toml`, `lexlean.lock` | LexLean project and its locked package closure |
| `lakefile.toml`, `lean-toolchain`, `lake-manifest.json` | Lean workspace bound by `lexlean.lock` |

| Module | Content |
| --- | --- |
| `Product` | identity, owner, server identity, protocol core, source policy, platforms, and the owner-set version (bound, #19) |
| `Architecture` | the ISO/IEC/IEEE 42010 architecture description in the `prism.arch` vocabulary: 8 stakeholders, concerns C1 to C8, the component, security and quality viewpoints, 10 components and 11 edges, checked by `architectureDescriptionHolds` |
| `Security` | the threat model in the `prism.sec` vocabulary: assets A1 to A6 tied to architecture components, risks R1 to R10 with impact and an owner-set likelihood (bound, #19), controls K1 to K9, four security activities, and a measurement per risk; `securityValid` and `riskChainHolds` prove that every risk has a measurement and that R10 is the only risk without a control, recorded as residual. One measurement is `Executed`: R5 (path escape), observed by the path admission rows (`SM-01` names them, and `onlyPathEscapeIsExecuted` pins it as the only one). The symlink clause of control K5 is recorded in `pendingClauses` until materialization (#9) builds it, and the other eight controlled risks stay `Pending` (`measurementsPendingExceptR5`). the risks are `Foundation.Sec::Risk` records, which the SDK projector reads. [SECURITY.md](SECURITY.md) holds the evidence and the planted defects |
| `Quality` | the quality model in the `prism.qual` vocabulary, ISO/IEC 25010:2023: requirements Q1 to Q8, each a characteristic, subcharacteristic, requirement, measure chain with an all-or-none threshold, realized by components of the quality view; `qualityValid` and `qualityChainHolds` prove the chains complete. Two measures are `Executed`: Q2 (the API-to-tool map, `QM-01` to `QM-03`) and Q8 (every diagnostic code explained, `QM-04` to `QM-06`), and `onlyApiMapAndExplainAreExecuted` pins them as the only ones; the clause of Q2 that waits for the tools is recorded in `pendingClauses` and Q8's running clause is executed by `TL-13` (`onlyQ2KeepsAPendingClause` pins the list), and Q1 and Q3 to Q7 stay `Pending` (`measuresPendingExceptQ2AndQ8`). [QUALITY.md](QUALITY.md) holds the evidence and the planted defects |
| `Effects` | the closed host effect set of issue #7 (the declaration half): the ten effects E1 to E10 (`store.get`, `store.put`, `engine.check`, `engine.snapshot`, `engine.format`, `engine.build`, `verify.start`, `verify.control`, `lexicon.read`, `attestation.lookup`) as an `Effect` inductive with a wire tag (its position, a four-octet big-endian frame) and a dotted wire name each; `encode` and `encodeName`; a total `decode` from a `Wire` to `Result Effect Refusal` with the typed refusals `UnknownId`, `UnknownName` and `MalformedFrame`; `decodeEncode` and `decodeEncodeName` (decode inverts encode, by cases on the effect); the negative theorem `decodeSound`, which holds for every wire and is proved by cases and induction, not decided on examples; distinct tags and names; the four `Engine` calls tied one to one to E3 to E6; the Q6 check that no name contains a tool name or an MCP method name; and one acceptance vector per effect, by frame and by name, with the three refusals. The generated host adapter and the `gateway` need PrismPM#216 and MCP#14 and are not here. [EFFECTS.md](EFFECTS.md) holds the wire, the vectors, the mapping to PrismPM#216 and the planted defects |
| `Identity` | the pure part of profile identity (#8) and draft identity (#9): length-prefixed framing of fields and counted lists over octet strings, with a proved left inverse (`fieldsRT`, `listRT`, `profileRT`, `draftRT`), so two different profiles or drafts have different preimages whenever every field is shorter than 2^64 - 1 bytes; domain separation of the two preimages; canonical order for everything that is a set: file and source rows sort by path octets and review records sort by their octets (unsigned, lexicographic), so an ID depends only on what a profile or draft contains and never on the order its parts were produced in, and a duplicate path or a duplicate review record is a typed refusal (`DuplicatePath`, `DuplicateReview`); the frame labels `domain`, `files`, `file`, `reviews`, `review`, `toolchain`, `profile`, `sources` and `source` are part of every ID, and the draft preimage frames the profile ID as its 32 raw digest octets; the `Bytes` to octet bridge; `profileId` and `draftId` as SHA-256 of the preimage, checked against vectors from an independent implementation. It does not claim SHA-256 collision resistance |
| `NfcTables` | generated by `cargo run -p nfc-tables --example generate_tables` from the pinned Unicode Character Database 17.0.0 files in `crates/nfc-tables/ucd/` (SHA-256 pinned in `model/authorities.toml`): the combining classes, the composition exclusions (`Full_Composition_Exclusion`) and the canonical decompositions packed as five sorted tables behind a 30-octet header (the full tables, as two artifacts because a list-backed `Bytes` literal of about 26 000 octets exceeds the elaborator's recursion depth), a small sample image of the same layout for the kernel-checked instances, and the constants `fastBound`, `decomposesFrom`, `decompositionDepth` and `lowLength`. Never edit it: `cargo test -p nfc-tables` compares it with a fresh generation byte for byte |
| `Nfc` | the NFC decision: UTF-8 reader, canonical decomposition (Hangul by arithmetic), canonical ordering by combining class, canonical composition with the composition exclusions, and equality with the input (exact; not Quick_Check). It reads the tables by binary search. Every recursion over the input is a tail call, which the export renders as a loop, so the production root `nfcRun` declares no `recursion` effect and needs no stack that grows with the input. `nfcRun` is exported to Rust (`crates/nfc-oracle`), where every line of `NormalizationTest.txt` is run against it. 30 instance theorems are kernel-checked on the sample image |
| `Admission` | path admission as a total function from a path and the source roots to `Admitted` or a typed `Refusal`: UTF-8, NFC (`NotNfc`, decided by `Nfc` on the Unicode 17.0.0 tables of `NfcTables`), relative, no empty, `.` or `..` component, under a source root, `.lex.tex` suffix; `max_files` counts rows and `max_bytes` counts content lengths only, not path bytes (as LexLean's `max_total_source_bytes` counts source bytes), both as parameters whose values are owner-set (#19); `admittedPathHasNoDotDot` proves that an admitted path has no `..` component. `admitPath` is `admitPathWith` the full tables; the kernel-checked NFC instances run `admitPathWith` on the sample image (see `NfcTables`). The root `admitRun` (tables, roots and path in; the verdict as `[0]` or `[1, index of the Refusal constructor]` out) is a production root exported to Rust and run on the full tables by `crates/nfc-oracle`; its stages (`splitInto`, `componentsStage`, `hasSuffix`, `rootsStage`) are tail-recursive, and they use `Nfc` list functions rather than `Identity`'s, which are not executable. Draft admission (`admitDraft`, #9) decides in this order: the path refusals of the rows in row order (`rowsStageWith`), `TooManyFiles`, `TooManyBytes`, then `DuplicatePath` (a path that occurs twice among the rows, compared as octets; `Refusal.DuplicatePath` is last, so the indices of the other refusals are unchanged). The duplicate scan runs only on a draft the limits admit. It sorts the paths by octets (unsigned, lexicographic, as `Identity` orders rows) with a bottom-up merge sort whose merges are tail calls (`singletonRuns`, `mergeInto`, `mergePairs`, `sortRuns`, with fuel for termination) and compares neighbours (`adjacentRepeat`), so it makes O(n log n) comparisons. The stages that run over the rows (`rowsStageWith`, `countRowsInto`, `totalBytesInto`, `pathsInto` and the sort) are tail-recursive. The root `admitDraftRun` (tables, `max_files`, `max_bytes`, roots and rows in; the verdict as `[0]` or `[1, index]` out) is a production root exported to Rust (`crates/nfc-oracle`, package `lexlean_mcp_draft_admission`) that declares no `recursion` effect |
| `Sha256` | the LexLean standard library module `Sha256` (FIPS 180-4), a byte-identical copy of `stdlib/src/Sha256.lex.tex` from LexLean `release/v0.4` at `9796067` (register row SL-02). The module name must equal its path, so it is `src/Sha256.lex.tex` and not under `src/Foundation/` |
| `Observability` | the observability model of #16: the telemetry records with every free-text field typed `Redacted`, a type with one value, so that no record holds module text or a lexicon search query (kernel-checked over all constructors); the eight service level indicators S1 to S8 as data; the alert predicates on classes 2, 3 and 70, a failed child group end, a profile drift and a signing failure, with the thresholds read from `OwnerValues`; `traceparent`, `tracestate` and `baggage` extraction from a request's `_meta` with typed refusals; and the closed set of emitted notifications, which excludes `notifications/message`. Emission and firing wait for #11 and #17; the planted defects are in [OBSERVABILITY.md](OBSERVABILITY.md) |
| `OwnerValues` | the values of #19 that no source determines, each bound to the proposal of [OWNER-VALUES.md](OWNER-VALUES.md): signing and `request-state-key` custody and rotation, `verify.width`, `verify.memory_kib`, `verify.concurrency`, `rate.tool_calls`, `rate.window_ms`, `draft.max_bytes`, `draft.max_files`, the two retentions, `cache.ttl_ms`, `listen.port`, the service level objectives S1 to S8 and the 17 profiles; the kernel-checked theorems bind the ranges, and `cargo test -p repo-model` (OV-01 to OV-03) holds the sources, the table, `lexlean.toml` and `Cargo.toml` to one statement |
| `Features` | the features F1 to F13 and journeys J1 to J6 (#18) as `Foundation.Arch::Feature` and `Foundation.Arch::Journey` definitions with the interface calls and acceptance rows they name, and the kernel theorems that restate PrismPM's rules PP2014 to PP2018 over them (`featuresModelHolds`) |
| `Store` | the store, persistence and lifecycle model of #13 (register `ST-`): the seven directories of `lexlean-mcp/store/1` with their wire names, the backup set (`verifications`, `keys`, the drafts references), retention as total functions of the bound owner values, the drift check as a total function with the decision order lock, ID, toolchain, the rollout, rollback, upgrade and retirement machine, and AC-14 to AC-17 as theorems over a record-resolution model that rest on `Identity`'s round trip of the profile preimage (`profileRT`) and claim nothing about SHA-256. Attestation signing stays abstract until #11. [STORE.md](STORE.md) holds the claims, what is not met and the planted defects |
| `Profiles` | the model-side slice of #8 (register `PF-`): the profile file set outside the source roots, matched by whole path component, the profile ID over it through `Identity`, the language rule with `LLC0103`, and the Atlas profile data (293 entries, 86 modules) with the check that all of them are served (`profilesModelHolds`) |
| `Foundation.Arch` | the architecture vocabulary, a byte-identical copy of `stdlib/src/Foundation/Arch.lex.tex` of PrismPM main at `d11ace6f` (FJ-09) |
| `Foundation.Sec`, `Foundation.Qual` | the security and quality vocabularies (copies of the PrismPM stdlib modules, as in prism-code) |

## Repository

This is an instance of the [UOR template](https://github.com/UOR-Foundation/template).

- [AGENTS.md](AGENTS.md) sets the rules every change follows (R1–R6).
- [TEMPLATE-CONTRACT.md](TEMPLATE-CONTRACT.md) defines inherited policy and the
  SDK and template update process.
- [VERIFICATION.md](VERIFICATION.md) defines the acceptance boundary; `just vv`
  is the complete gate.
- [FEATURES.md](FEATURES.md) states the origin of each feature and journey, the rows that
  need the owner, and the defects planted in the theorems that check them.
- [EFFECTS.md](EFFECTS.md) states the host effect set, its wire, its acceptance vectors, the
  mapping to PrismPM#216 and the defects planted in its gates.
- [TOOLS.md](TOOLS.md) states what `lexlean_check`, `lexlean_snapshot`, `lexlean_format` and
  `lexlean_build` return, how an answer of the Engine becomes a tool result, how the tool is
  compared with the CLI, and the defects planted in those tests.
- [FIXTURES.md](FIXTURES.md) states the counts and the classes of the AC-07
  fixture register and the defects planted in its gates.
- [QUALITY.md](QUALITY.md) states what the executed measures of Q2 and Q8 show,
  what they do not, and the defects planted in their gates.
- [OBSERVABILITY.md](OBSERVABILITY.md) records the defects planted in the
  observability theorems, the command that rejects each and the restore.
- [STORE.md](STORE.md) states what the store model claims and does not claim
  and the defects planted in its laws.
- [SIGNING.md](SIGNING.md) states what the signer of the verification records claims
  (SG-01 to SG-08), what is not met, and the defects planted in its gates.
- [PROFILES.md](PROFILES.md) states what the profiles model claims and does not
  claim and the defects planted in its laws.
- [CI.md](CI.md) describes the workflow that runs every gate that works without
  the SDK, and how to read its failures.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
