# Owner-set values

Issue #19 lists the values that no source determines. Each one is bound in the
model (`src/Product.lex.tex`, `src/OwnerValues.lex.tex`, `src/Security.lex.tex`)
to the proposal below, so that no declaration depends on an unset value. The
owner overrides a value by changing its row and its binding in one commit;
`cargo test -p repo-model` (OV-01 to OV-03) fails when the two disagree.

This file is project-owned evidence. The rows are the single statement of why
each value is what it is; the model is the single statement of what it is.

## How to read a row

| Column | Meaning |
| --- | --- |
| Value | the key; the first column of a row is a key, never prose |
| Proposed | the value bound in the model |
| Evidence or constraint | the source the value follows from, or the reason no source decides it |
| Decides | `owner` (afflom, as product owner), `sponsor` (Ilya, UOR Foundation), or both |
| Status | `evidenced` when a constraint fixes the value or a range around it; `needs owner` when it is a policy choice and the proposal is the conservative default |
| If it moves | what else has to change |

## Identity and signing

| Value | Proposed | Evidence or constraint | Decides | Status | If it moves |
| --- | --- | --- | --- | --- | --- |
| `version` | `0.1.0` | The release tracker (#20) and its milestone are named v0.1.0, and the workspace package version in `Cargo.toml` is 0.1.0. | owner | evidenced | `Cargo.toml` follows (OV-03 compares them); the server identity string and the release tag follow. |
| `signing.algorithm` | `Ed25519` | #11 signs a DSSE envelope over an in-toto statement; DSSE and in-toto both allow Ed25519 (RFC 8032). LexLean leaves signing outside the language and signs nothing itself (SPEC 22.9: release automation may sign the completed file). Ed25519 signatures are deterministic, so a signing failure cannot come from a weak random source. | owner | evidenced | The signer, the key history format and every envelope validator change; envelopes already written stay valid under their recorded key. |
| `signing.key_custody` | `file secret mounted read-only into the signer service only` | A2 (the signing key) is held by the signer and by nothing else (Security A2), and K8 states the key is held by reference. #17 deploys the signer as its own service, so a file secret granted to that one service keeps the key out of the gateway, the store and the engine-host. A hardware module or a key service would add custody the loopback release does not need. | owner and sponsor | needs owner | A stronger custody changes #17 (the Compose projection) and the key-compromise control K8; R8 may move to a lower likelihood. |
| `signing.key_rotation_days` | `365` | NIST SP 800-57 Part 1 Revision 5, Table 1, gives a private signature key an originator usage period of one to three years. This takes the short end. K8 publishes a key history with revocations, so earlier envelopes still validate after a rotation (AC-14, AC-16). | owner and sponsor | needs owner | A shorter period means more entries in the key history and a rotation to operate; a longer one raises the weight of R8. |
| `request_state_key.custody` | `file secret mounted read-only into the protocol core service only` | A6 (the `requestState` integrity key) is held by the protocol core. UOR-Foundation/MCP#10 names the key as the secret reference `request-state-key`; a file secret mounted into the one service that checks the MAC is the same mechanism as for the signing key. | owner and sponsor | needs owner | Changes #17 and control K9. |
| `request_state_key.rotation_days` | `90` | A `requestState` lives for one input-required round trip, so a state older than the period is already useless. NIST SP 800-57 Part 1 Revision 5, Table 1, allows a symmetric authentication key up to two years; 90 days is the conservative default. A rotation invalidates the states in flight, and the client repeats the round. Whether the core also accepts the previous key for one round is MCP#10's decision. | owner and sponsor | needs owner | A shorter period means more rounds repeated; a longer one raises the weight of R9. |

## Risk likelihoods

`Foundation.Sec.Likelihood` is closed over `low`, `medium` and `high`. The
rubric is one rule, applied to each risk with its control not counted:

- `high`: it occurs in ordinary use, with no adversary.
- `medium`: it needs an adversary who can put content in front of the agent or the browser, or run code as the same user.
- `low`: it needs the defeat of a primary control or access to a held secret.

Impact is unchanged from the model (critical: R1, R2, R3, R5, R8; major: the
rest). The proposals are the conservative reading of the rubric; every row is a
judgment of the owner's, so every row needs the owner.

| Value | Proposed | Evidence or constraint | Decides | Status | If it moves |
| --- | --- | --- | --- | --- | --- |
| `risk.R1` | `low` | Forging an attestation needs the signing key or write access to the store and the signer, behind K1. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R2` | `medium` | A misstating lexicon entry needs a contributor error or intent; 293 Atlas entries and the lexicons of the examples are authored by people, and K2 binds a review record into the profile ID. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R3` | `medium` | Any web page in the user's browser can try the loopback port; DNS rebinding is a known attack on local servers, which is why the MCP transport specification requires `Origin` validation. It needs a page the user visits, so it is not ordinary use. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R4` | `high` | Exhaustion needs no adversary: a LexLean release build of repo-conformance reached about 12 GB resident, `check` of the Atlas example measured 4.3 GB, and an agent loop calls tools without a human pause. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R5` | `medium` | Draft paths come from an agent that returned content can steer (R7), so a hostile path needs an adversary but no access. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R6` | `low` | Network acquisition needs a `lock` effect that the host does not grant, and egress is denied for the engine-host and the verifier (K6). | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R7` | `medium` | Returned text steers the agent only if a third party authored instructions into a lexicon package or a module, which needs a contributor, and K7 returns module text in data fields only. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R8` | `low` | The key sits in one service's secret, never in the store or the gateway, and compromise needs access to the signer host. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R9` | `low` | Forging `requestState` needs a client that is hostile and knows the key, or breaks the MAC. | owner and sponsor | needs owner | Changes the risk matrix only. |
| `risk.R10` | `medium` | Any local process of the same user can call a loopback server that has no authorization; this release has none, so R10 stays a recorded residual risk until the core has resource-server authorization. | owner and sponsor | needs owner | Changes the risk matrix only. |

## Parameters

| Value | Proposed | Evidence or constraint | Decides | Status | If it moves |
| --- | --- | --- | --- | --- | --- |
| `verify.width` | `1` | LexLean SPEC 22.11: the width is an integer from 1 to 64; 1 is the default, called the conservative profile. A width above 1 is for a host whose memory envelope is known, and the published artifacts are byte for byte the same at every width. | owner | evidenced | Peak memory of one task scales with the width; recheck `verify.memory_kib`. |
| `verify.memory_kib` | `12582912` | LexLean SPEC 9.4 gives its own normative gate a budget of 12 GiB (12582912 KiB) for the summed expected peaks of concurrent proof processes on a 16 GB runner with a 4 GiB swap file. The budget gates the start of a process and never kills one (SPEC 22.11), so the container limit of #17 is a separate, higher number. | sponsor | evidenced | The host needs `verify.concurrency` times this much memory (OV-03 checks it against 16 GiB). |
| `verify.concurrency` | `1` | One verifier process per task (#11). The budget above is nearly the whole of a 16 GiB host, and two tasks would need 24 GiB. AC-13 (one task more than the concurrency waits, and none fails) is shown with one task running and one waiting. | sponsor | evidenced | Raise it only with a host that holds the multiple of `verify.memory_kib`; OV-03 fails first. |
| `rate.tool_calls` | `100` | UOR-Foundation/MCP TOOLS.md: the limit counts `tools/call` requests per client in a window, and the fixture's limit is 100. That is the only value MCP exercises. One principal on loopback gains nothing from a higher number; the limit is a brake on a runaway agent loop (R4). | owner | needs owner | The deployed core refuses call 101 in a window with code 429. |
| `rate.window_ms` | `60000` | TOOLS.md: the host owns the window, the client identity and the increment, so the limit has no meaning without a window. #19 does not list this value; it is added because `rate.tool_calls` needs it. | owner | needs owner | The gateway's counter resets on this period. |
| `draft.max_bytes` | `8388608` | Lower bound: the `.lex.tex` sources of the largest example other than the Atlas, `production-coverage`, total 5413010 bytes at LexLean 9796067. Upper bound: `max_total_source_bytes = 67108864` in `lexlean.toml` counts the profile and the draft together; the 86 modules of the Atlas are 24841413 bytes, so the Atlas plus 8 MiB stays under it. The count is content bytes only, as LexLean's `max_total_source_bytes` counts source bytes. | owner | evidenced | Above 64 MiB less the largest profile, LexLean refuses first with `LLS8002`; below 5413010 the largest example cannot be drafted. OV-03 checks both. |
| `draft.max_files` | `256` | The largest example other than the Atlas, `production-coverage`, has 148 modules; the next power of two is 256. The Atlas profile carries its 86 modules itself, not in the draft. | owner | evidenced | Below 148 the largest example cannot be drafted (OV-03). |
| `store.draft_retention` | `30 days` | #13: only verification records, signatures and the key history cannot be recomputed. A draft is an input the agent can submit again, and the backup holds the drafts reference only. Disk is bounded by `draft.max_bytes` per draft. | owner | needs owner | Drafts older than this since their last change are removed by the store lifecycle (#13). |
| `store.attestation_retention` | `forever` | #13 and AC-14 to AC-17: after an upgrade, a rollback or a restore every earlier record must still resolve, and a verification record cannot be recomputed. No expiry satisfies that; deletion is an operator act. | owner | evidenced | A finite period breaks AC-14 to AC-17 for records older than it. |
| `cache.ttl_ms` | `60000` | The MCP 2026-07-28 caching utility: `ttlMs` is a freshness hint like `Cache-Control: max-age`, required on `server/discover`, `tools/list`, `prompts/list`, `resources/list`, `resources/templates/list` and `resources/read`; a client may fetch earlier. A rollout or rollback (AC-14, AC-15) changes the profiles served, and the value bounds how long a client may keep the old answer. The scope is fixed by #8 (`public` for the lexicon template), not an owner value. | owner | needs owner | Longer means fewer requests and a longer stale window after a rollout. |
| `listen.port` | `5395` | The MCP transport specification says only to bind 127.0.0.1 and validate `Origin`; it assigns no port. 5395 is unassigned in the IANA service name and port number registry (checked 2026-10-23), is below the Linux ephemeral range that starts at 32768, and spells LEXL on a telephone keypad. | owner | needs owner | The Compose projection publishes `127.0.0.1:<port>` (#17) and the oracle clients use it (AC-03, AC-10). |

## Service level objectives

One per service level indicator of #16, plus the signing alert. The count rows
are zero-tolerance, the same all-or-none form as the thresholds of the quality
model. The form of a value is `threshold unit / window s`.

| Value | Proposed | Evidence or constraint | Decides | Status | If it moves |
| --- | --- | --- | --- | --- | --- |
| `slo.S1` | `0 count / 300 s` | Tool outcome by `ErrorClass`. LexLean maps the classes to exit codes 1 (language), 2 (configuration), 3 (environment), 4 (security or limit) and 70 (internal). Classes 1 and 4 are the answers to a bad draft and an over-limit one; classes 2, 3 and 70 are faults of the deployment, so #16 alerts on them. | owner | evidenced | Any fault in the window raises the alert. |
| `slo.S2` | `1 percent / 300 s` | Protocol errors by code, as a share of requests. A conformant client causes none except for the errors it provokes on purpose; no source gives the share, so this is the conservative default. | owner | needs owner | A higher share hides a failing client. |
| `slo.S3` | `5000 milliseconds / 3600 s` | Check latency at the 95th percentile, profiles other than the Atlas. Measured at LexLean 9796067: `check` took 0.15 to 0.51 seconds on the examples other than the Atlas, so this is about ten times the slowest. The Atlas took 13.5 seconds and 4.3 GB and is outside this objective. | owner | needs owner | Tightening it to the measured time would alert on every cold start. |
| `slo.S4` | `300000 milliseconds / 86400 s` | Verify duration at the 95th percentile, profiles other than the Atlas. It equals `child_timeout_ms = 300000` in `lexlean.toml`: a verification that needs longer than one child may take is a timeout. The Atlas carries `child_timeout_ms = 10800000` in its own project and is outside this objective. | owner | needs owner | Moves with `child_timeout_ms`. |
| `slo.S5` | `600000 milliseconds / 86400 s` | Verifier queue wait at the 95th percentile. With `verify.concurrency` at 1 a task waits for the tasks ahead; twice S4 allows one running task and one queued before an alert. | owner | needs owner | Raising the concurrency lowers the waits. |
| `slo.S6` | `0 count / 300 s` | Child group ends that failed. SPEC 24.6 reports `Ended`, `AlreadyGone` or `Failed` for every group; AC-12 requires `Ended`. | owner | evidenced | Any failed end raises the alert. |
| `slo.S7` | `0 count / 3600 s` | Profiles that drifted. A drift is a stale lock, a changed profile ID or a changed toolchain hash (#13); each one is a defect, not a rate. | owner | evidenced | Any drift raises the alert. |
| `slo.S8` | `0 count / 300 s` | Signing failures. Every verification record is signed (#11) and Q3 forbids a record for a verification that did not end `verified`, so a signing failure is a verified task that cannot be recorded. | owner | evidenced | Any failure raises the alert. |

## Profile set

The proposal is one profile for each project in `examples/` of LexLean at the
pinned revision (`.github/lexlean.rev`, 9796067; v0.4.0 once it is tagged). AC-05
requires that each example verifies through the product, and #8 requires one
language per profile, so a profile is an example's language, packages and
sources. The languages are 1.2 (11 profiles), 1.0 (5) and 1.1 (1). The packages
are what each example's own `lexlean.toml` declares: `lexlean.std.nat`, the
example's title lexicon, and for `helpdesk` the source packages `std`,
`std-models` and `std-reasoning`; the `uor-atlas` profile serves
`lexlean.uor.atlas@1.0.0`. The packages are the examples' facts, not owner
choices, so only the set and the languages are bound.

The `uor-atlas` profile costs the most. Its project sets `child_timeout_ms` to
three hours, its `check` takes 13.5 seconds and 4.3 GB, and LexLean shards its
verification across jobs. Serving it is the owner's decision.

| Value | Proposed | Evidence or constraint | Decides | Status | If it moves |
| --- | --- | --- | --- | --- | --- |
| `profile.collections` | `1.2` | Example `collections`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.fold-laws` | `1.2` | Example `fold-laws`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.helpdesk` | `1.2` | Example `helpdesk`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.higher-order` | `1.2` | Example `higher-order`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.language-1.2` | `1.2` | Example `language-1.2`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.list-induction` | `1.0` | Example `list-induction`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.models` | `1.2` | Example `models`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.nat-add-zero` | `1.0` | Example `nat-add-zero`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.peano-arithmetic` | `1.0` | Example `peano-arithmetic`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.production` | `1.2` | Example `production`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.production-coverage` | `1.2` | Example `production-coverage`; it also sets the lower bound of `draft.max_bytes` and `draft.max_files`. | owner | needs owner | Without it the draft limits could shrink; OV-03 holds the constants. |
| `profile.propositional-logic` | `1.0` | Example `propositional-logic`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.reasoning` | `1.2` | Example `reasoning`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.recursion` | `1.2` | Example `recursion`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.recursive-data` | `1.2` | Example `recursive-data`. | owner | needs owner | The profile is not served; AC-05 covers one example fewer. |
| `profile.semantic-1.1` | `1.1` | Example `semantic-1.1`; the only language 1.1 example. | owner | needs owner | Language 1.1 is not served at all. |
| `profile.uor-atlas` | `1.0` | Example `uor-atlas`; #8 requires an Atlas profile that serves all 293 entries. Cost as above. | owner and sponsor | needs owner | The Atlas profile, its lexicon and AC for 293 entries leave v0.1.0. |

## Gates

| ID | What it holds |
| --- | --- |
| OV-01 | No source applies the `Unbound` constructor of `OwnerValue`; the version is bound; the ten risks carry a `Foundation.Sec.Likelihood`. |
| OV-02 | This table and the model state the same value for every key, in both directions, and every row has its five other cells. |
| OV-03 | Each value respects the constraint it follows from (the `lexlean.toml` limits, the example sizes, the host memory, the width range, the port range, the workspace version). |

`lexlean verify` also checks the model side: `ownerValuesAreBound`,
`versionIsBound`, `likelihoodsAreAsBound` and the range theorems of
`OwnerValues` fail when a binding leaves its range.

## Planted defects

Each defect was planted in a copy of the tree, the failing command run, and the file restored.
The restored tree passes `cargo test -p repo-model --test owner_values` (3 passed).

| Defect | Failing command | Diagnostic |
| --- | --- | --- |
| `draftMaxFiles` set to 255 in the model only | `cargo test -p repo-model --test owner_values` | OV-02: `draft.max_files: the model binds 255 and OWNER-VALUES.md proposes 256` |
| the Unbound constructor applied to `signingAlgorithm` | the same | OV-01: `src/OwnerValues.lex.tex applies the Unbound constructor of OwnerValue: an owner value is unset (#19)`; OV-02 fails too |
| `draft.max_bytes` raised to 67108864 in the model and the table | the same | OV-03 fails alone: the draft and the Atlas modules pass `max_total_source_bytes` |
