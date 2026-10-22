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
modeled; no capability is built yet.

- The product model is LexLean language 1.2 notation (`src/`), built with
  LexLean `release/v0.4` and locked in `lexlean.lock` (`lexlean/lock/2`).
  `lexlean verify` accepts it with an exact axiom policy on every declaration.
  The version of the product is an owner-set value that is not yet bound (#19).
- The claim registers in `model/` are empty, so `CONFORMANCE.md` lists no
  capability and this repository claims none.
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
| `Product` | identity, owner, server identity, protocol core, source policy, platforms, and the owner-set version (unbound) |
| `Architecture` | the ISO/IEC/IEEE 42010 architecture description in the `prism.arch` vocabulary: 8 stakeholders, concerns C1 to C8, the component, security and quality viewpoints, 10 components and 11 edges, checked by `architectureDescriptionHolds` |
| `Security` | the threat model in the `prism.sec` vocabulary: assets A1 to A6 tied to architecture components, risks R1 to R10 with impact and an owner-set likelihood (unbound, #19), controls K1 to K9, four security activities, and a measurement per risk; `securityValid` and `riskChainHolds` prove that every risk has a measurement and that R10 is the only risk without a control, recorded as residual |
| `Quality` | the quality model in the `prism.qual` vocabulary, ISO/IEC 25010:2023: requirements Q1 to Q8, each a characteristic, subcharacteristic, requirement, measure chain with an all-or-none threshold, realized by components of the quality view; `qualityValid` and `qualityChainHolds` prove the chains complete |
| `Identity` | the pure part of profile identity (#8) and draft identity (#9): length-prefixed framing of fields and counted lists over octet strings, with a proved left inverse (`fieldsRT`, `listRT`, `profileRT`, `draftRT`), so two different profiles or drafts have different preimages whenever every field is shorter than 2^64 - 1 bytes; domain separation of the two preimages; row canonicalization (sort by path octets, typed refusal of duplicate paths); the `Bytes` to octet bridge; `profileId` and `draftId` as SHA-256 of the preimage, checked against vectors from an independent implementation. It does not claim SHA-256 collision resistance |
| `Admission` | path admission as a total function from a path and the source roots to `Admitted` or a typed `Refusal`: UTF-8, relative, no empty, `.` or `..` component, under a source root, `.lex.tex` suffix; `max_files` and `max_bytes` of a draft; `admittedPathHasNoDotDot` proves that an admitted path has no `..` component. NFC is not checked: an exact check needs the Unicode normalization data, which this repository does not carry (open, #9) |
| `Sha256` | the LexLean standard library module `Sha256` (FIPS 180-4), a byte-identical copy of `stdlib/src/Sha256.lex.tex` from LexLean `release/v0.4` at `9796067` (register row SL-02). The module name must equal its path, so it is `src/Sha256.lex.tex` and not under `src/Foundation/` |
| `Foundation.Arch`, `Foundation.Sec`, `Foundation.Qual` | the architecture, security and quality vocabularies (copies of the PrismPM stdlib modules, as in prism-code) |

## Repository

This is an instance of the [UOR template](https://github.com/UOR-Foundation/template).

- [AGENTS.md](AGENTS.md) sets the rules every change follows (R1–R6).
- [TEMPLATE-CONTRACT.md](TEMPLATE-CONTRACT.md) defines inherited policy and the
  SDK and template update process.
- [VERIFICATION.md](VERIFICATION.md) defines the acceptance boundary; `just vv`
  is the complete gate.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
