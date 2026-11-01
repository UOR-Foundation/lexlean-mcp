# `just vv` is the normative acceptance gate. Everything else is a slice of it.

default: vv

# The whole gate. Each check runs once, the cheapest failure first.
vv: fmt-check model template-check lint test deny
    @echo "vv: the acceptance gate passed"

# R1, R4, R5, the bootstrap trust root and dependency ownership: every
# repository gate, each falsifiable, in one `xtask validate`.
model:
    cargo run -q --locked -p xtask -- validate

# The SDK's own checks of the canonical contract and both locks. The
# repository gates they complement already ran in `model`.
template-check:
    prismpm template check
    prismpm lock check

# Regenerate everything the model owns: CONFORMANCE.md.
model-write:
    cargo run -q --locked -p xtask -- check-model --write

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

# A feature only its author has built is a feature that does not work, so
# Clippy compiles and lints every target at every optional feature: a rename
# upstream of an optional dependency, or a test behind a flag, fails here.
lint:
    cargo clippy --workspace --all-targets --all-features --locked -- -D warnings

# The workspace suite at its default features, including the R3 meta-gate.
test:
    cargo test --workspace --locked

# R3 alone, while a scenario is being written: every capability begins as a
# Gherkin scenario, and every scenario has a test whose name ends in its ID.
# `test` already runs it, so it is not part of `vv`.
bdd:
    cargo test --locked -p repo-conformance

# R6: nothing shipped depends on a dev-only crate, no wildcard version
# requirement, no advisory against anything in the tree. `cargo-deny` is
# supplied by the locked SDK, so this is part of `just vv`.
#
# Advisories, bans, licences and sources, over the dependency graph.
deny:
    cargo deny --all-features check
