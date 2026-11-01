# The signer of the verification records

Issue #11 asks that every verification record be signed: a DSSE envelope over an in-toto
statement whose subject is `attestation.json`, with the key history published. This file
records the signer, its key file, its key history and the validator (SG-01 to SG-08), and the
use the verify task makes of them (SG-09 to SG-13): the key is configured, a verification is
not started without it, and the record of a verification that ended `verified` is the signed
envelope. Each verification runs in a process of its own (VERIFIER.md), and Q3, R1 and R8 stay
`Pending` in the model (see "Not met"). The key is held by a program of its own, `gateway-signer`,
the first service issue #17 asks for (SG-16 to SG-19, "The service").

The code is the crate `gateway/bindings/gateway-signer`: `src/signer.rs`, `src/service.rs`,
`src/client.rs` and `src/bin/gateway-signer.rs`. Its tests are
`gateway/bindings/gateway-binding/tests/signing.rs`, `tests/records.rs` beside it, and
`gateway/bindings/gateway-signer/tests/service.rs`, and `crates/model/tests/signing.rs` holds them
against the rows, the failures and this file.

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
| SG-09 | The socket of the signer is named by `LEXLEAN_MCP_SIGNER`; `verify.start` fails with `verifier-unavailable`, starts no task and copies nothing when there is no signer, none answers, or the key file of the signer is missing, unreadable or malformed. |
| SG-10 | A verification that ended `verified` leaves the signed envelope, the `attestation.json` it signs byte for byte, and an envelope that validates against `keys/history`, with the IDs, the modules and the digests of the source files in its predicate. |
| SG-11 | `attestation.lookup` returns exactly that envelope, by attestation ID and by the digest of a source file. |
| SG-12 | A verification that cannot be signed, or whose `attestation.json` cannot be read, leaves no record; so does one that failed. |
| SG-13 | Each gate of the record rejects its planted defect (below). |
| SG-14 | A verified record keeps the evidence its `attestation.json` binds: the build manifest it names by digest, the canonical LaTeX of each module and the source files, each matching the digest the manifest gives. |
| SG-15 | A verification whose evidence cannot be kept whole leaves no record, as SG-12. |
| SG-16 | `gateway-signer` is the only holder of the key: it reads the file at each request and answers on a socket of mode 0600; its crate links no compiler and nothing generated; the host has no setting for a key and loads none. |
| SG-17 | An envelope through the socket equals the signer's, validates against the history the program wrote, and a key file put right or rotated is used at the next request. |
| SG-18 | Every failure of the service is typed and has a negative test, and a signer lost before a verification concludes leaves no record. |
| SG-19 | Each gate of the service rejects its planted defect (below). |

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

**Configuration.** `LEXLEAN_MCP_SIGNER` names the socket of the signer (an empty value is none). The
history is `keys/history` below the store. `verify.start` judges the request first: a project
that is not one is `project-refused` whatever the key, which is why the recorded exchanges of the
protocol replay with none. Then it asks the signer whether its key loads, so a verification that
could not be signed is never begun and nothing is left in the `builds` area (the copy a draft makes
is removed with the request); no signer, no answer, and a missing, unreadable or malformed key are
all `verifier-unavailable`, the failure the model already declares for E7.

**Layout.** `verifications/<attestation ID>.json` is the envelope and
`verifications/<attestation ID>/attestation.json` is the file it signs, the `attestation.json` of
the verified directory. Beside it, the record keeps the evidence that file binds (issue #12, so that
a rendering can be presented and exported after the verification's working copy is gone):
`build-manifest.json`, the manifest whose SHA-256 `attestation.json` names as `build_manifest`;
each output of kind `tex` the manifest names, at its path (`modules/<M>/<M>.tex`, the canonical
LaTeX of the module); and each input of kind `source`, at `sources/<path>`. Each is checked against
the digest the manifest gives before anything is written, so a reader can follow the digests from
the envelope down to a rendering and the source it came from (`src/evidence.rs`).
`index/by-file/<sha256>` names the attestation ID, as before. The subject is written first, then
the evidence, then the envelope, then the index entries, so a lookup never finds an envelope
without the files it binds. The envelope is signed after the engine has verified and the subject has
been read; a failure at any step ends the task `failed` with class 5 and leaves no envelope.

**Predicate.** `status` (`verified`), `attestation_id`, `build_id`, `source_id`, `semantic_id`,
`modules`, and `files`, the SHA-256 of each source file of the project in the order of their paths.
The compiler, the toolchain and the observed axioms are in `attestation.json`, which the subject
digest binds; they are not repeated in the predicate.

**Result.** The result of the task is the IDs and the modules as before, and `keyid`, read from the
envelope: a verification takes minutes, and the key may have rotated since it began.

## The service

`OWNER-VALUES.md` keeps the key "mounted read-only into the signer service only", and #17 deploys
that service. `gateway-signer` is its program. It takes `LEXLEAN_MCP_SIGNING_KEY` (the key file),
`LEXLEAN_MCP_SIGNER` (its socket) and `LEXLEAN_MCP_STORE` (whose `keys/history` it writes); without
one of them, or with its socket path taken by something that is not a socket, it exits with status 2.
It prints one line when it listens. It is a crate of its own so that the image that holds the key
holds no compiler and nothing generated. The transport is a Unix socket and not TCP so that the
service needs no network at all, which is what control K6 asks of its neighbours.

```text
client -> service   {"op":"key"}
service -> client   {"keyid":"<hex>"}
client -> service   {"op":"sign","subject":"<base64>","predicate":{...}}
service -> client   {"envelope":"<base64>"}
service -> client   {"refused":"<name>"}
```

One line each way and one connection for each request, answered one at a time, with `TIMEOUT` (30 s)
on each read and write. A request is at most `MAX_REQUEST_BYTES` (16 MiB). The names are
`history-unreadable`, `history-unwritable`, `history-malformed`, `history-unknown-key`,
`key-revoked`, `key-missing`, `key-unreadable`, `key-malformed`, `request-refused` and `unavailable`;
each comes back as its `SignError`, and an answer that is none of these is `SignError::Unavailable`.

The key file is read for each request, so the key is in memory only while it is used, and an
operator who puts the file right or rotates the key restarts nothing. The clock of a first signature
is the service's. A socket left by an earlier run is replaced; anything else at the path is left.

## Claim level

- Ed25519 is `ed25519-dalek` 2.2.0 and base64 is `base64` 0.22.1. SG-02 reproduces from RFC 8032
  and the DSSE protocol the two vectors it states; it does not prove the library correct.
- The keyid, the history and the validator are built here and shown by the tests only on the
  inputs they use.
- Ed25519 signatures are deterministic, so a signing failure cannot come from a weak random source
  and equal inputs give equal bytes (SG-02). The seed is not random here: the operator makes it.

## Not met

- **The service is a program, not yet a deployment.** There is no image and no Compose service
  (#17), so nothing yet shows the key mounted into this service only; that is what `signing.key_custody`
  asks, and until the deployment runs it is a property of the code and not of a running system.
- **Whoever reaches the socket can have anything signed.** The service takes the host's word for the
  subject. It keeps the key from leaving, which limits a compromise of the gateway to the time it
  lasts, and it does not stop that compromise signing. Mode 0600 keeps out other users; the same user
  is R10, the residual risk of `mcp-loopback`.
- **The mode is set just after the socket is made**, so it is open to the directory's reach for that
  moment. A deployment puts the socket in a directory only the two services share.
- **The observed axioms are not compared** with a committed record (AC-05), and no example other than
  the tiny fixture was verified and signed.
- **One verifier process per task**, the queue, the cancel race and the lock that Q3 needs, the
  tasks mapping and the pin move to MCP's tasks extension are afflom's design on #11 and are not
  here. AC-05 is not addressed (VERIFIER.md).
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

Each defect was planted in `gateway/bindings/gateway-signer/src/signer.rs`, the command ran
(gateway workspace, offline, the registry's `CARGO_HOME`), and the file was restored with
`git checkout`; the unplanted tree passes the same command (19 passed).

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| the lengths of PAE swapped: `pae` writes `len(payload)` where `len(type)` goes and the reverse | `cargo test -p gateway-binding --test signing` | 16 passed, 3 failed: `pae_is_the_example_of_the_dsse_protocol_sg_02` finds `DSSEv1 11 http://example.com/HelloWorld 29 ...` where the example is `DSSEv1 29 ... 11 ...`; `the_envelope_is_dsse_over_an_in_toto_statement_sg_01` and `every_refusal_of_an_envelope_is_typed_and_reached_sg_04` fail because the signature is over another message | `git checkout gateway/bindings/gateway-signer/src/signer.rs` |
| the keyid taken from the seed: `from_seed` hashes the seed and not the public key | `cargo test -p gateway-binding --test signing` | 6 passed, 13 failed, among them `the_keyid_is_the_digest_of_the_public_key_not_of_the_seed_sg_01`, `a_key_file_of_64_hexadecimal_digits_is_the_seed_sg_06` (the key ID is `02d449a3...` where the digest of the public key is `10ba682c...`) and every validation, because the history entry no longer matches its key | `git checkout gateway/bindings/gateway-signer/src/signer.rs` |
| the digest of the subject not checked: the comparison in `validate` is `false && ...` | `cargo test -p gateway-binding --test signing` | 18 passed, 1 failed: `every_refusal_of_an_envelope_is_typed_and_reached_sg_04` panics `the envelope is refused: Object {"_type": ...` (an envelope for other bytes was accepted) | `git checkout gateway/bindings/gateway-signer/src/signer.rs` |
| a revoked key accepted: the revocation check in `check_signature` is `false && ...` | `cargo test -p gateway-binding --test signing` | 18 passed, 1 failed: `a_revoked_key_fails_and_the_other_key_does_not_sg_05` panics `the envelope is refused: Object {...` (the statement of a revoked key was returned) | `git checkout gateway/bindings/gateway-signer/src/signer.rs` |
| the history not appended on the first use: `sign` does not call `write_history` for a new key | `cargo test -p gateway-binding --test signing` | 7 passed, 12 failed: `a_revoked_key_signs_nothing_sg_05` panics `a revocation: UnknownKey`, and `the_first_use_of_a_key_adds_its_entry_and_later_uses_add_nothing_sg_05`, `a_history_that_cannot_be_written_gives_no_envelope_sg_07` and the validation tests fail | `git checkout gateway/bindings/gateway-signer/src/signer.rs` |

The root-workspace gate that holds every constructor of every public failure to a negative test
(`every_refusal_of_an_envelope_and_of_a_key_has_a_negative_test_sg_04`) has its own planted defect:

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| a constructor with no test: `EnvelopeError::Replayed` added to `signer.rs` | `cargo test -p repo-model --test signing` | 7 passed, 1 failed: `every_refusal_of_an_envelope_and_of_a_key_has_a_negative_test_sg_04` panics `no test reaches EnvelopeError::Replayed` | `git checkout gateway/bindings/gateway-signer/src/signer.rs` |

## Planted defects of the signer service

Each defect was planted in a copy of `gateway/bindings/gateway-signer` built as a workspace of its own,
and the first four rows ran that crate's tests alone; the copy was then put back from the tree, which
is `git checkout` of the file, and rebuilt with `cargo clean -p gateway-signer`, because a restore that
keeps the file's old time leaves the planted program in place. The unplanted crate passes the same
command (14 passed). The fifth row ran in the root workspace.

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| the socket left open to other users: `from_mode(0o600)` is `0o666` in `bind` | `cargo test -p gateway-signer --test service` | 13 passed, 1 failed: `the_program_answers_on_a_socket_of_mode_0600_sg_16`, left 438, right 384 | `git checkout gateway/bindings/gateway-signer/src/service.rs` |
| a refusal name that comes back as another error: `refusal_named` answers `key-revoked` with `Unavailable` | `cargo test -p gateway-signer --test service` | 12 passed, 2 failed: `every_refusal_travels_by_its_name_sg_18` (left `Some(Unavailable)`, right `Some(KeyRevoked)`) and `a_revoked_key_is_refused_through_the_socket_sg_18` | `git checkout gateway/bindings/gateway-signer/src/service.rs` |
| the key cached at the start of the program: `signer` reads the file once and keeps the text | `cargo test -p gateway-signer --test service` | 11 passed, 3 failed: `a_key_file_put_right_or_rotated_is_used_at_the_next_request_sg_17`, `each_fault_of_the_key_file_comes_back_as_itself_sg_18` (left `Err(Key(Missing))`, right `Err(Key(Malformed))`) and `a_history_that_cannot_be_written_is_refused_through_the_socket_sg_18` | `git checkout gateway/bindings/gateway-signer/src/service.rs` |
| the bound on a request removed: `serve` reads without a limit and never refuses for length | `cargo test -p gateway-signer --test service` | 13 passed, 1 failed: `a_request_over_the_bound_is_refused_and_the_next_is_served_sg_18` finds an envelope where `Err(RequestRefused)` is required | `git checkout gateway/bindings/gateway-signer/src/service.rs` |
| the host loading the key itself: `verify_start` calls `Signer::load` before it asks the signer | `cargo test --locked -p repo-model --test signing` | `the_signer_is_a_program_of_its_own_and_the_host_holds_no_key_sg_16` fails: `gateway-binding/src/lib.rs holds Signer::load: the host holds no key` | `git checkout gateway/bindings/gateway-binding/src/lib.rs` |

## Planted defects of the signed record

Planted and restored the same way, with `cargo test -p gateway-binding --test records` (7 passed on
the unplanted tree when the first five rows were measured, 9 since SG-14 and SG-15 added two
tests; the counts in those five rows are as they were measured, before the two were added).

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| verify.start accepting a verification without a key: `verify_start` in `lib.rs` falls back to a built-in key when none loads | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `verify_start_without_a_usable_key_is_unavailable_and_starts_nothing_sg_09` finds `Ok("task-1")` where `Err(VerifierUnavailable)` is required, for "no key" | `git checkout gateway/bindings/gateway-binding/src/lib.rs` |
| the failure to sign ignored: `conclude` takes an empty envelope when `sign` fails and records it | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `a_history_that_cannot_be_written_leaves_no_record_sg_12` finds class 0 where 5 is required, and an envelope in the store | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| the envelope stored without its attestation.json: `record` does not write the subject | `cargo test -p gateway-binding --test records` | 5 passed, 2 failed: `a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10` and `the_lookup_returns_the_envelope_by_id_and_by_file_digest_sg_11` (`the subject: Os { code: 2, kind: NotFound ... }`) | `git checkout gateway/bindings/gateway-binding/src/attestation.rs` |
| the predicate without the digests of the source files: `files` is the empty list | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10` finds `Array []` where the digest of `Tiny.lex.tex` is required | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| the manifest kept without its digest checked: `gather` reads `build-manifest.json` without comparing it to the digest `attestation.json` names (`evidence.rs`) | `cargo test -p gateway-binding --test records` | 8 passed, 1 failed: `evidence_that_cannot_be_kept_whole_leaves_no_record_sg_15` finds class 0 for "a manifest that is not the one attestation.json names" | `git checkout -- gateway/bindings/gateway-binding/src/evidence.rs` |
| a rendering kept without its digest checked: `gather` reads each `tex` output without comparing it to the manifest (`evidence.rs`) | `cargo test -p gateway-binding --test records` | 8 passed, 1 failed: `evidence_that_cannot_be_kept_whole_leaves_no_record_sg_15` finds class 0 for "a rendering that is not the one the manifest names" | `git checkout -- gateway/bindings/gateway-binding/src/evidence.rs` |
| the evidence not written: `record` skips the files of the evidence (`attestation.rs`) | `cargo test -p gateway-binding --test records` | 7 passed, 2 failed: `a_verification_that_ended_verified_leaves_a_signed_envelope_sg_10` lists only the envelope and `attestation.json`, and `the_record_keeps_the_manifest_the_renderings_and_the_sources_sg_14` finds no `build-manifest.json` | `git checkout -- gateway/bindings/gateway-binding/src/attestation.rs` |
| evidence that cannot be gathered ignored: `conclude` records the verification with no evidence when `gather` refuses (`verifier.rs`) | `cargo test -p gateway-binding --test records` | 8 passed, 1 failed: `evidence_that_cannot_be_kept_whole_leaves_no_record_sg_15` finds class 0 for "no build manifest" | `git checkout -- gateway/bindings/gateway-binding/src/verifier.rs` |
| the lookup answering the unsigned record again: it reads `<id>/attestation.json` and not `<id>.json` | `cargo test -p gateway-binding --test records` | 6 passed, 1 failed: `the_lookup_returns_the_envelope_by_id_and_by_file_digest_sg_11` finds the bytes of attestation.json where the stored envelope is required | `git checkout gateway/bindings/gateway-binding/src/attestation.rs` |

## Reproducing

```sh
gateway/registry.sh                                      # the registry from gateway/Cargo.lock
cd gateway
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline --test signing
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline --test records
# a real verification, signed and validated (needs the Lean toolchain; the gateway job runs it by count)
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline -- --ignored
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-signer --locked --offline     # the program, over its socket
cd .. && cargo test -p repo-model --test signing         # SG-01 to SG-19 against the rows
```
