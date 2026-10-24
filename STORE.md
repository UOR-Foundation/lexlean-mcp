# Store, persistence and lifecycle

Issue #13 asks for the store layout `lexlean-mcp/store/1`, the backup of what
cannot be recomputed, the drift checks, and the rollout, rollback, upgrade and
retirement of profiles, with four acceptance criteria (AC-14 to AC-17). The real
store, its persistence and the signed records need the verify task and signing
(#11), which do not exist. This file records the slice that stands without them:
`src/Store.lex.tex`, registered as ST-01 to ST-09. Its acceptance criteria are
theorems over a record-resolution model; the issue's own acceptance stays open.

## What is claimed

Every claim below is a theorem of `src/Store.lex.tex`, checked by
`lexlean verify` with an exact axiom policy, and held to the issue's words by
`cargo test -p repo-model --test store`.

| ID | Claim |
| --- | --- |
| ST-01 | The layout is a closed type of seven directories with the wire names `profiles`, `drafts`, `builds`, `verifications`, `index`, `exports` and `keys`, distinct, all listed, parsed back; an unknown name is refused. |
| ST-02 | The backup set is `verifications`, `keys` and the drafts references. Every item that cannot be recomputed and does not expire is in it; nothing recomputable is in it; the `Archive` type has exactly those three fields. |
| ST-03 | Retention is a total function of the owner values `store.draft_retention` (30 days) and `store.attestation_retention` (forever): an attestation never expires, a draft is kept at 30 days and expired at 31. |
| ST-04 | The drift check maps a stored profile record, the `lexlean lock --check` result, the recomputed profile ID and the recomputed toolchain hashes to `NoDrift`, `LockDrift`, `IdDrift` or `ToolchainDrift`. The order is lock, then ID, then toolchain; the eight inputs of the decision are tabulated; an `Identity` refusal (`DuplicatePath`, `DuplicateReview`, `FieldTooLong`) is `IdDrift`. |
| ST-05 | The lifecycle is a machine over the served revision and profile set, the previous release, the retired profiles and the records. Rollout, rollback, upgrade and retirement never change a record; rollback restores the previous revision and served set; an upgrade serves the new profile; a retired profile is not served. |
| ST-06 | AC-14: `aPreimageDeterminesItsToolchain` recovers the toolchain field from a profile preimage, from `Identity`'s round trip `profileRT`, so a new revision gives another preimage; after an upgrade the earlier record resolves and still validates. |
| ST-07 | AC-15 and AC-17: after a rollback the previous revision and its profiles are served and a record written after the rollout still resolves; a rollback resolves every record that resolved before it. |
| ST-08 | AC-16: after a restore from the backup every backed-up record resolves as before and the keys and draft references come back; re-verifying a restored record gives the attestation ID it is stored under. |
| ST-09 | Each law rejects its planted defect (below). |

## Claim level

- Injectivity is of the preimage. `Identity` proves a left inverse of the
  profile and draft preimages (`profileRT`, `draftRT`); `Store` uses it to
  recover the toolchain field. Neither claims SHA-256 collision resistance, and
  this model does not either: the profile IDs of two revisions differ because
  their preimages differ and SHA-256 is assumed to separate them, which nothing
  here proves.
- The model leaves attestation signing and the DSSE envelope abstract until
  #11. An attestation ID is `attestationId`: the framed list of the profile ID
  and the verification inputs (draft ID, toolchain, host). That makes "the same
  inputs give the same ID" true because the ID is a function of its inputs, and
  nothing more. Whether the signer is deterministic is #11's claim.
- Profile and attestation identifiers are lists of octets here, compared by
  `sameOctets` (the `if`-based `Identity.octetsEqual` does not rewrite under
  `simp only`). The concrete cases use short octet strings, not digests; the
  digest-valued drift cases use all-zero and one-bit digests.
- AC-15 and AC-17 hold in the model because no lifecycle event writes or
  removes a record; records leave the store only through retention, which never
  applies to an attestation. That is the design the model states, and the
  theorems are its consequences, not a measurement of a deployed system.

## Not met

The following wait for #11 and the executable halves of the product and are not
claimed by ST-01 to ST-09:

- the filesystem layout on disk and its permissions;
- persistence, that is, writing, reading and syncing records;
- an actual backup and restore of a store;
- the signed attestation format and the key history (#11);
- drift checks run against real profiles (`lexlean lock --check`, a recomputed
  profile ID, toolchain hashes), rollout and rollback of a real release.

## Planted defects

Each defect was planted in a copy of the tree, `lexlean verify src/Store.lex.tex`
run (LexLean `release/v0.4` at `9796067`), and the copy
discarded. The unplanted tree passes the same command (baseline below).

| Defect | Law | Planted | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| P1 | `attestationsNeverExpire` | `retentionOf` maps `Kind.Attestation` to `OwnerValues::storeDraftRetention`, so an attestation expires after 30 days | `lexlean verify src/Store.lex.tex` exits 1: `error[LLV7002]: Lean rejected LexLeanMcp.Store: Tactic rfl failed ... isExpired Kind.Attestation ageDays` is not definitionally equal to `false` (`src/Store.lex.tex:392`); `anAttestationIsKeptAfterTenYears` fails too | the line restored; `lexlean verify` accepts the tree |
| P2 | `idDriftIsReportedBeforeToolchainDrift` | `driftStage` tests `hashesMatched` before `idMatched`, so a changed toolchain hides a changed ID | `lexlean verify src/Store.lex.tex` exits 1: `error[LLV7002]: ... Tactic rfl failed ... driftStage LockResult.Clean false hashesMatched` is not definitionally equal to `Verdict.IdDrift` (`src/Store.lex.tex:520`); 6 errors, among them `theDecisionTableIsExact` | the line restored; `lexlean verify` accepts the tree |
| P3 | `rollbackWithKeepsRecords` | `rollbackWith` rebuilds the machine with `records := (nil Verification)`, so a rollback drops every record | `lexlean verify src/Store.lex.tex` exits 1: `error[LLV7002]: ... Tactic rfl failed ... (rollbackWith m (Prior.Prior q)).records` is not definitionally equal to `m.records` (`src/Store.lex.tex:853`); 3 errors | the line restored; `lexlean verify` accepts the tree |
| P4 | `restoreReturnsTheKeys` | `restore` sets `keys := (nil (List Nat))`, so a restored store has no key history | `lexlean verify src/Store.lex.tex` exits 1: `error[LLV7002]: ... Tactic rfl failed ... (restore (backup d)).keys` is not definitionally equal to `d.keys` (`src/Store.lex.tex:1139`); 2 errors | the line restored; `lexlean verify` accepts the tree |

Baseline: `lexlean verify src/Store.lex.tex` on the unplanted tree: `verified 5 modules; attestation 5ee76e509683c39ef3347d2587071270720c04fd7e270b6f4b1538b4d96426cc`, exit 0. The CI job `lexlean / verify` runs the whole project.
