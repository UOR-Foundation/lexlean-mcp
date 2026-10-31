# The attestation tool

Issue #12, F10. `lexlean_attestation` resolves an attestation ID, or the SHA-256 of a source file
that a verification covered, to the signed envelope of that verification, through the index of
the store. The core was wired to `attestation.lookup` in an earlier slice and the host answers it
(SG-11); this file claims what the tool returns, tested against a real signed record.

## What is claimed

| Row | Claim |
| --- | --- |
| `AT-01` | By attestation ID the tool returns the envelope in the store, byte for byte, and it validates against `keys/history` and the stored `attestation.json`. |
| `AT-02` | By the digest of a source file the verification covered it returns the same envelope. |
| `AT-03` | A record that does not exist is `found: false`, not an error. A value that is not 64 lowercase hexadecimal digits is the host's `value-refused`; an empty value is refused by the core, which does not ask the host. |
| `AT-04` | A lookup writes nothing to the store. |
| `AT-05` | Each gate rejects its planted defect (below). |

## What the tool returns

The arguments are `by` (`id` or `digest`) and `value`. A record that exists is answered with
`structuredContent` `{"found": true}` and the envelope, a JSON document, as the text of the first
content block. A record that does not exist is answered with `{"found": false}` and the text
`not found`. A refusal of the value is a tool error with `structuredContent` `{"failure":
"value-refused"}`. The tool asks the host once (`attestation.lookup`).

The tests make the record with `conclude`, as the verifier does when a verification ends
`verified` (`gateway/bindings/gateway-binding/tests/records.rs` does the same for the host), and
take the store and the key history as the oracle. They do not run a verification: that needs the
Lean toolchain, and the ignored test of the binding runs it.

## Planted defects

A gate is evidence only after a planted defect shows that it can fail (AGENTS.md). Each defect below
was applied to `gateway/bindings/gateway-binding/src/attestation.rs`, the command was run in
`gateway/` (`CARGO_HOME=gateway/registry/cargo-home`, `--locked --offline`), and the file was
restored. The unplanted tree passes the command (5 passed).

| Defect | Command | Gate | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| the lookup reading the file that the envelope signs: `{identifier}/attestation.json` in place of `{identifier}.json` | `cargo test -p gateway-tool-tests --test attestation` | `the_tool_returns_the_envelope_by_attestation_id_at_01`, `the_tool_returns_the_same_envelope_by_the_digest_of_a_source_file_at_02` | both fail: "the text is the envelope in the store, byte for byte", the text being the bytes of `attestation.json` | `git checkout -- gateway/bindings/gateway-binding/src/attestation.rs`: the command passes again |
| the digest read as an attestation ID: the `by` 1 arm returns `value` | `cargo test -p gateway-tool-tests --test attestation` | `the_tool_returns_the_same_envelope_by_the_digest_of_a_source_file_at_02`, `an_absent_record_is_not_found_and_a_malformed_value_is_refused_at_03` | the first fails: found is false where true was expected; the second fails: a digest that is the ID of a record is found | `git checkout -- gateway/bindings/gateway-binding/src/attestation.rs`: the command passes again |
| the check of the value dropped: `is_digest(value)` is no longer asked | `cargo test -p gateway-tool-tests --test attestation` | `an_absent_record_is_not_found_and_a_malformed_value_is_refused_at_03`, `a_lookup_writes_nothing_to_the_store_at_04` | both fail: a malformed value reaches the store as a key and the host faults (`the host performs it: HostFault`) where `value-refused` was expected | `git checkout -- gateway/bindings/gateway-binding/src/attestation.rs`: the command passes again |
| a lookup that writes: `index/last-lookup` is written with the value | `cargo test -p gateway-tool-tests --test attestation` | `a_lookup_writes_nothing_to_the_store_at_04` | the test fails: "the store is as it was after the lookups", the store having the extra file | `git checkout -- gateway/bindings/gateway-binding/src/attestation.rs`: the command passes again |

## Not met

- The envelope is the text of the result. `structuredContent` carries only `found`; the issue asks
  for the envelope as structured content only of `lexlean_present`.
- The acceptance row of F10 is the proposal in FEATURES.md ("attestation resolves by ID or file
  digest"); the owner has not confirmed it (FEATURES.md, "Needs owner").
- No tool lists the attestations of the store, and a lookup does not say which key signed an
  envelope: the reader validates it against the key history.
