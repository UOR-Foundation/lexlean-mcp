# The signer of the verification records

Issue #11 asks that every verification record be signed: a DSSE envelope over an in-toto
statement whose subject is `attestation.json`, with the key history published. This file
records the signer, its key file, its key history and the validator (SG-01 to SG-08), and the
use the verify task makes of them (SG-09 to SG-13): the key is configured, a verification is
not started without it, and the record of a verification that ended `verified` is the signed
envelope. The verifier is still a thread of the gateway process, and Q3, R1 and R8 stay
`Pending` in the model (see "Not met").

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
| SG-08 | Each gate of the signer rejects its planted defect (below). |
| SG-09 | The key file is named by `LEXLEAN_MCP_SIGNING_KEY`; `verify.start` fails with `verifier-unavailable`, starts no task and copies nothing when there is no key or the file is missing, unreadable or malformed. |
| SG-10 | A verification that ended `verified` leaves the signed envelope, the `attestation.json` it signs byte for byte, and an envelope that validates against `keys/history`, with the IDs, the modules and the digests of the source files in its predicate. |
| SG-11 | `attestation.lookup` returns exactly that envelope, by attestation ID and by the digest of a source file. |
| SG-12 | A verification that cannot be signed, or whose `attestation.json` cannot be read, leaves no record; so does one that failed. |
| SG-13 | Each gate of the record rejects its planted defect (below). |

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

## The record

**Configuration.** `LEXLEAN_MCP_SIGNING_KEY` names the key file (an empty value is none). The
history is `keys/history` below the store. `verify.start` judges the request first: a project
that is not one is `project-refused` whatever the key, which is why the recorded exchanges of the
protocol replay with none. Then it loads the key, so a verification that could not be signed is
never begun and nothing is left in the `builds` area (the copy a draft makes is removed with the
request); a missing, unreadable or malformed key, and no key, are all `verifier-unavailable`,
the failure the model already declares for E7. An operator who fixes the key file does not restart
the gateway: the file is read at each start.

**Layout.** `verifications/<attestation ID>.json` is the envelope and
`verifications/<attestation ID>/attestation.json` is the file it signs, the `attestation.json` of
the verified directory. `index/by-file/<sha256>` names the attestation ID, as before. The subject is
written first, then the envelope, then the index entries, so a lookup never finds an envelope
without the file it signs. The envelope is signed after the engine has verified and the subject has
been read; a failure at any step ends the task `failed` with class 5 and leaves no envelope.

**Predicate.** `status` (`verified`), `attestation_id`, `build_id`, `source_id`, `semantic_id`,
`modules`, and `files`, the SHA-256 of each source file of the project in the order of their paths.
The compiler, the toolchain and the observed axioms are in `attestation.json`, which the subject
digest binds; they are not repeated in the predicate.

**Result.** The result of the task is the IDs and the modules as before, and `keyid`.

## Claim level

- Ed25519 is `ed25519-dalek` 2.2.0 and base64 is `base64` 0.22.1. SG-02 reproduces from RFC 8032
  and the DSSE protocol the two vectors it states; it does not prove the library correct.
- The keyid, the history and the validator are built here and shown by the tests only on the
  inputs they use.
- Ed25519 signatures are deterministic, so a signing failure cannot come from a weak random source
  and equal inputs give equal bytes (SG-02). The seed is not random here: the operator makes it.

## Not met

- **The key is in the gateway process.** The verification is still a thread of it, so the design's
  separation (the signer in the parent, a verifier child that never holds the key) arrives with the
  verifier process per task.
- **The cancel race.** The runner writes the record before the task's lock is taken again, so a
  task cancelled in the instant after the engine verified can leave a signed record and report
  `cancelled`. The record is of a verification that did end `verified`, but AC-12 ("leaves no record")
  is not shown, and Q3 stays `Pending`.
- **The observed axioms are not compared** with a committed record (AC-05), and no example other than
  the tiny fixture was verified and signed.
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

## Planted defects of the signed record

Planted and restored the same way, with `cargo test -p gateway-binding --test records` (7 passed on
the unplanted tree).

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| verify.start accepting a verification without a key: `verify_start` in `lib.rs` falls back to a built-in key when none loads | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `verify_start_without_a_usable_key_is_unavailable_and_starts_nothing_sg_09` finds `Ok("task-1")` where `Err(VerifierUnavailable)` is required, for "no key" | `git checkout gateway/bindings/gateway-binding/src/lib.rs` |
| the failure to sign ignored: `conclude` takes an empty envelope when `sign` fails and records it | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `a_history_that_cannot_be_written_leaves_no_record_sg_12` finds class 0 where 5 is required, and an envelope in the store | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| the envelope stored without its attestation.json: `record` does not write the subject | `cargo test -p gateway-binding --test records` | 5 passed, 2 failed: `a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10` and `the_lookup_returns_the_envelope_by_id_and_by_file_digest_sg_11` (`the subject: Os { code: 2, kind: NotFound ... }`) | `git checkout gateway/bindings/gateway-binding/src/attestation.rs` |
| the predicate without the digests of the source files: `files` is the empty list | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10` finds `Array []` where the digest of `Tiny.lex.tex` is required | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| the lookup answering the unsigned record again: it reads `<id>/attestation.json` and not `<id>.json` | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `the_lookup_returns_the_envelope_by_id_and_by_file_digest_sg_11` finds the bytes of attestation.json where the stored envelope is required | `git checkout gateway/bindings/gateway-binding/src/attestation.rs` |

## Reproducing

```sh
gateway/registry.sh                                      # the registry from gateway/Cargo.lock
cd gateway
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline --test signing
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline --test records
# a real verification, signed and validated (needs the Lean toolchain; the gateway job runs it by count)
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline -- --ignored
cd .. && cargo test -p repo-model --test signing         # SG-01 to SG-13 against the rows
```
