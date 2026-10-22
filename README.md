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

Nothing is built yet.

- The claim registers in `model/` are empty, so `CONFORMANCE.md` lists no
  capability and this repository claims none.
- The repository is not bound to a PrismPM SDK release (#4): `prismpm.lock` and
  `template.lock` are absent, so `just vv` and the devcontainer cannot run.
- v0.1.0 is tracked in #20, with its upstream dependencies in LexLean, PrismPM,
  the UOR template and UOR-Foundation/MCP.

v0.1 is scoped to Streamable HTTP on the loopback interface, with one principal
and no authorization: the only binding every part of which an executable MCP
oracle covers. stdio, MCP Apps and remote, authorized deployment wait for
oracles of their own (UOR-Foundation/MCP#15).

## Repository

This is an instance of the [UOR template](https://github.com/UOR-Foundation/template).

- [AGENTS.md](AGENTS.md) sets the rules every change follows (R1–R6).
- [TEMPLATE-CONTRACT.md](TEMPLATE-CONTRACT.md) defines inherited policy and the
  SDK and template update process.
- [VERIFICATION.md](VERIFICATION.md) defines the acceptance boundary; `just vv`
  is the complete gate.

Licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).
