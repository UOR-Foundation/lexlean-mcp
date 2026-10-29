# The signer of the verification records

Issue #11 asks that every verification record be signed: a DSSE envelope over an in-toto
statement whose subject is `attestation.json`, with the key history published. This file
records the slice that stands without the verifier process and the store: the signer, its
key file, its key history and the validator, registered as SG-01 to SG-08. The verify task
does not call it yet (see "Not met"), so Q3, R1 and R8 stay `Pending` in the model.

The code is `gateway/bindings/gateway-binding/src/signer.rs`; its tests are
`gateway/bindings/gateway-binding/tests/signing.rs`, and `crates/model/tests/signing.rs`
holds them against the rows, the failures and this file.

## What is claimed

| ID | Claim |
| --- | --- |
| SG-01 | The envelope is DSSE over an in-toto Statement v1: payload type `application/vnd.in-toto+json`, one subject `attestation.json` with the SHA-256 of the bytes given, the predicate type of this product, one signature whose `keyid` is the SHA-256 of the public key and whose `sig` is Ed25519 over PAE. |
| SG-02 | PAE, the public key and the signature are the standards' bytes (the DSSE example, RFC 8032 section 7.1 test 1), and equal inputs give equal envelopes. |
| SG-03 | `validate` accepts what the signer wrote and returns the statement; it uses the key the history names. |
| SG-04 | Every refusal of an envelope is a constructor of `EnvelopeError` with a planted envelope that reaches it. |
| SG-05 | The key history is appended on first use, survives rotation, revokes a key on the operator's word, and refuses a damaged list; a revoked key signs nothing. |
| SG-06 | The key is a file of 64 hexadecimal digits; each fault has its own error; the seed is in nothing that is printed or written. |
| SG-07 | A signature whose history cannot be recorded returns no envelope; the history is replaced whole. |
| SG-08 | Each gate rejects its planted defect (below). |

## The formats

**Envelope.** `{"payloadType", "payload", "signatures": [{"keyid", "sig"}]}`. The payload is the
base64 of the statement; `sig` is the base64 of the 64 octets of Ed25519 over
`PAE(payloadType, payload)`, with `PAE = "DSSEv1" SP len(type) SP type SP len(body) SP body` and
lengths in octets. The statement is

```json
{"_type": "https://in-toto.io/Statement/v1",
 "subject": [{"name": "attestation.json", "digest": {"sha256": "<hex>"}}],
 "predicateType": "https://uor.foundation/lexlean-mcp/verification/v1",
 "predicate": {}}
```

The predicate is whatever the caller supplies. This slice does not decide what a verification
puts in it; the verify task will (afflom's design on #11 names the identifiers, the status, the
modules, the toolchain and the observed axioms).

**Key.** A file of 64 hexadecimal digits, whitespace around them allowed, read as the Ed25519
seed. To make one: `head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n' > key && chmod 0400 key`.
The key ID is the SHA-256 of the public key.

**History.** `keys/history`, a JSON list, one entry per key in the order of first use:
`{keyid, algorithm: "Ed25519", public_key (base64, 32 octets), not_before (seconds), revoked_at?}`.
A list with an unknown field, another algorithm, a key that is not 32 octets, a keyid that is not
the key's digest or a key twice is `HistoryError::Malformed`. The signer writes a new entry before it
returns the first envelope of a key; the file is written beside itself and renamed over.

**Validation order.** Payload type, payload, signatures present; then each signature in turn: key
in the history, key not revoked, signature (strict verification). One valid signature is enough.
Then the statement (type, predicate type, exactly one subject named `attestation.json` with only a
`sha256` digest), then the digest against the bytes given.

**Rotation and revocation.** A second key adds an entry and revokes nothing, so envelopes of the
first key stay valid. `revoke` sets `revoked_at` and every envelope of that key then fails with
`KeyRevoked`, whatever it says: an envelope carries no time that someone holding the key could not
choose, so the validator does not compare times.

## Claim level

- Ed25519 is `ed25519-dalek` 2.2.0 and base64 is `base64` 0.22.1. SG-02 reproduces from RFC 8032
  and the DSSE protocol the two vectors it states; it does not prove the library correct.
- The keyid, the history and the validator are built here and shown by the tests only on the
  inputs they use.
- Ed25519 signatures are deterministic, so a signing failure cannot come from a weak random source
  and equal inputs give equal bytes (SG-02). The seed is not random here: the operator makes it.

## Not met

- **Nothing calls the signer.** `verifier::verify` still writes the unsigned record of
  `attestation::record`, and `attestation.lookup` (E10) still returns it. Making `verifications/<id>.json`
  the envelope, `verifications/<id>/attestation.json` the subject, the key a configured file, and a
  missing key `verifier-unavailable` is the next slice and changes the record and the lookup.
- **One verifier process per task**, the queue, the cancel race and the lock that Q3 needs, the
  tasks mapping and the pin move to MCP's tasks extension are afflom's design on #11 and are not
  here. AC-05, AC-07, AC-11, AC-12 and AC-13 are not addressed.
- **No operator command** revokes a key or makes one; `revoke` is a function.
- **The key file's mode is not checked** (a mounted secret is often world-readable inside its
  container), and the text of the key file is read into a string that is not wiped. The library's
  signing key zeroizes on drop.
- **Q3, R1 and R8 stay `Pending`** in `src/Quality.lex.tex` and `src/Security.lex.tex`: no module
  of `src/` changed, so no re-export of `crates/nfc-oracle` is needed.
- **Crates and licences.** `ed25519-dalek` 2 and `base64` 0.22 add 19 packages to
  `gateway/Cargo.lock` and move `rustix` from 1.1.5 to 1.1.6. `curve25519-dalek`, `ed25519-dalek` and
  `subtle` are BSD-3-Clause, which `deny.toml` does not allow. `cargo deny` does not read the
  gateway workspace (CI.md), so `deny.toml` is unchanged; if the gateway joins that gate, the
  licence is a decision to take then.

## Planted defects

Each defect was planted in `gateway/bindings/gateway-binding/src/signer.rs`, the command ran
(gateway workspace, offline, the registry's `CARGO_HOME`), and the file was restored with
`git checkout`; the unplanted tree passes the same command (19 passed).

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| the lengths of PAE swapped: `pae` writes `len(payload)` where `len(type)` goes and the reverse | `cargo test -p gateway-binding --test signing` | 16 passed, 3 failed: `pae_is_the_example_of_the_dsse_protocol_sg_02` finds `DSSEv1 11 http://example.com/HelloWorld 29 ...` where the example is `DSSEv1 29 ... 11 ...`; `the_envelope_is_dsse_over_an_in_toto_statement_sg_01` and `every_refusal_of_an_envelope_is_typed_and_reached_sg_04` fail because the signature is over another message | `git checkout gateway/bindings/gateway-binding/src/signer.rs` |
| the keyid taken from the seed: `from_seed` hashes the seed and not the public key | `cargo test -p gateway-binding --test signing` | 6 passed, 13 failed, among them `the_keyid_is_the_digest_of_the_public_key_not_of_the_seed_sg_01`, `a_key_file_of_64_hexadecimal_digits_is_the_seed_sg_06` (the key ID is `02d449a3...` where the digest of the public key is `10ba682c...`) and every validation, because the history entry no longer matches its key | `git checkout gateway/bindings/gateway-binding/src/signer.rs` |
| the digest of the subject not checked: the comparison in `validate` is `false && ...` | `cargo test -p gateway-binding --test signing` | 18 passed, 1 failed: `every_refusal_of_an_envelope_is_typed_and_reached_sg_04` panics `the envelope is refused: Object {"_type": ...` (an envelope for other bytes was accepted) | `git checkout gateway/bindings/gateway-binding/src/signer.rs` |
| a revoked key accepted: the revocation check in `check_signature` is `false && ...` | `cargo test -p gateway-binding --test signing` | 18 passed, 1 failed: `a_revoked_key_fails_and_the_other_key_does_not_sg_05` panics `the envelope is refused: Object {...` (the statement of a revoked key was returned) | `git checkout gateway/bindings/gateway-binding/src/signer.rs` |
| the history not appended on the first use: `sign` does not call `write_history` for a new key | `cargo test -p gateway-binding --test signing` | 7 passed, 12 failed: `a_revoked_key_signs_nothing_sg_05` panics `a revocation: UnknownKey`, and `the_first_use_of_a_key_adds_its_entry_and_later_uses_add_nothing_sg_05`, `a_history_that_cannot_be_written_gives_no_envelope_sg_07` and the validation tests fail | `git checkout gateway/bindings/gateway-binding/src/signer.rs` |

The root-workspace gate that holds every constructor of every public failure to a negative test
(`every_refusal_of_an_envelope_and_of_a_key_has_a_negative_test_sg_04`) has its own planted defect:

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| a constructor with no test: `EnvelopeError::Replayed` added to `signer.rs` | `cargo test -p repo-model --test signing` | 7 passed, 1 failed: `every_refusal_of_an_envelope_and_of_a_key_has_a_negative_test_sg_04` panics `no test reaches EnvelopeError::Replayed` | `git checkout gateway/bindings/gateway-binding/src/signer.rs` |

## Reproducing

```sh
gateway/registry.sh                                      # the registry from gateway/Cargo.lock
cd gateway
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline --test signing
cd .. && cargo test -p repo-model --test signing         # SG-01 to SG-08 against the rows
```
