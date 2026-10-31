# The present and export tools

Issue #12, F9 and F12. `lexlean_present` returns the attested canonical LaTeX of a verification
as an embedded resource, with the attestation ID and the envelope as structured content.
`lexlean_export` writes the bundle of a verification below the `exports` area of the store, which
a reader checks offline (J4). Both read the record that a verification leaves (SG-14, SG-15):
nothing is typeset or retyped between the attestation and the reader.

## What is claimed

| Row | Claim |
| --- | --- |
| `PT-01` | By attestation ID the tool returns one embedded resource whose text is the rendering in the store byte for byte; the structured content is the ID, the path of the rendering and the envelope as JSON, which validates against `keys/history` and the stored `attestation.json`. |
| `PT-02` | AC-06: for the record of `nat-add-zero` the embedded resource equals the canonical LaTeX LexLean wrote for the example, and its digest is the one the build manifest names. |
| `PT-03` | By digest of a source file the answer is the same; `module` selects one rendering of a record that has several; an empty `module` is the only rendering of a record that has one; otherwise it is `module-unknown`. |
| `PT-04` | A miss is `found: false`; a malformed value is `value-refused`; an empty value or module is refused by the core; a present writes nothing. |
| `PT-05` | Only what is attested is presented: a rendering or a manifest that differs from the digest that names it, or a rendering that is missing, is a fault of the store and gives no bytes. |
| `PT-06` | Each gate rejects its planted defect (below). |
| `EX-01` | The export writes `exports/<ID>/` with `attestation.json`, `envelope.json`, `keys.json`, `build-manifest.json`, each rendering at its path and each source at `sources/<path>`, byte for byte as the record holds them. |
| `EX-02` | A reader checks the bundle from its files alone (J4). |
| `EX-03` | The renderings digest of a bundle matches the attested build manifest. |
| `EX-04` | A miss or a refusal writes nothing, and a record that is not whole leaves no bundle. |
| `EX-05` | An export is repeatable; bundles are apart. |
| `EX-06` | Each gate rejects its planted defect (below). |

## What the tools return

Both take `by` (`id` or `digest`) and `value`, as `lexlean_attestation` does. `lexlean_present` takes
`module` too: a path the build manifest gives to a rendering (`modules/<Prefix>/<Module>.tex`).

`lexlean_present` answers a record that exists with one content block of type `resource`: the
canonical LaTeX as text, at the URI `lexlean://attestation/<attestation ID>/<path>`, with the type
`application/x-tex`. `structuredContent` is `{"found": true, "attestationId", "module": <path>,
"envelope": <the envelope, a JSON document>}`. A record that does not exist is `{"found": false}`
with the text `not found`; a refusal of the value is a tool error with `{"failure":
"value-refused"}`, and of the module `{"failure": "module-unknown"}`.

`lexlean_export` answers with the key of the bundle as its text, `exports/<attestation ID>`, and
`{"found": true, "attestationId", "bundle"}`; misses and refusals are those of `lexlean_present`.

Both ask the host once, for `attestation.lookup`, which gained a `want` (0 the envelope, 1 the
canonical LaTeX of a module, 2 the bundle) and a `module`, and answers with the ID, the path and the
bytes of the rendering, and the key of the bundle besides the envelope. The view is chosen by the
tool (the core), never by the client.

## The bundle and its check offline

| File | Content |
| --- | --- |
| `attestation.json` | the file the envelope signs |
| `envelope.json` | the signed envelope |
| `keys.json` | the key history of the store: the public keys |
| `build-manifest.json` | the manifest whose SHA-256 `attestation.json` names |
| `<path of a rendering>` | the canonical LaTeX of a module, at the path the manifest gives it |
| `sources/<path>` | a source file the verification covered |

A reader needs no store and no network. The envelope validates against `keys.json` and
`attestation.json` (DSSE with Ed25519, `SIGNING.md`); the SHA-256 of `build-manifest.json` is
`build_manifest.sha256` of `attestation.json`; the SHA-256 of each rendering and of each source is
the digest of its row of the manifest. `check` in `gateway/tool-tests/tests/present.rs` does exactly
this from the files of a bundle copied out of the store, and fails when any one file is altered
(EX-02).

The host reads and checks everything before it writes the first file, so a record that is not
whole leaves no bundle; a bundle is written in place, so exporting again writes the same bytes.

## Planted defects

A gate is evidence only after a planted defect shows that it can fail (AGENTS.md). Each defect below was applied to `gateway/bindings/gateway-binding/src/presentation.rs`, the command was run in `gateway/` (`CARGO_HOME=gateway/registry/cargo-home`, `--locked --offline`), and the file restored. The unplanted tree passes the command (10 passed).

| Defect | Command | Gate | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| the rendering read from the wrong file: `present` reads `build-manifest.json` in place of the rendering | `cargo test -p gateway-tool-tests --test present` | `the_tool_presents_the_attested_rendering_as_an_embedded_resource_pt_01`, `j1_for_nat_add_zero_ends_in_the_expected_canonical_latex_pt_02`, `by_digest_and_by_module_the_tool_presents_the_chosen_rendering_pt_03`, `only_what_is_attested_is_presented_pt_05`, `the_renderings_digest_of_a_bundle_matches_the_attested_manifest_ex_03` | 5 fail: the resource text is the manifest, not the rendering in the store | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| the digest of the rendering not checked: `read_bound` accepts any bytes | `cargo test -p gateway-tool-tests --test present` | `only_what_is_attested_is_presented_pt_05`, `a_miss_or_a_refusal_writes_nothing_and_a_record_that_is_not_whole_leaves_no_bundle_ex_04` | both fail: an altered rendering is presented, or exported, where a host fault was expected | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| the module not selected: a named module returns the first rendering | `cargo test -p gateway-tool-tests --test present` | `by_digest_and_by_module_the_tool_presents_the_chosen_rendering_pt_03`, `the_renderings_digest_of_a_bundle_matches_the_attested_manifest_ex_03` | both fail: the second module of a record presents the first one's bytes | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| an empty module on several renderings takes the first | `cargo test -p gateway-tool-tests --test present` | `by_digest_and_by_module_the_tool_presents_the_chosen_rendering_pt_03` | fails: `module-unknown` was expected for two renderings and none named | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| a present that writes: `index/by-file/last-present` is written | `cargo test -p gateway-tool-tests --test present` | `a_miss_is_not_found_a_bad_value_is_refused_and_nothing_is_written_pt_04` | fails: "a present writes nothing", the store having the extra file | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| a rendering left out of the bundle | `cargo test -p gateway-tool-tests --test present` | `the_tool_writes_the_bundle_of_the_record_ex_01`, `a_reader_checks_the_bundle_offline_ex_02`, `the_renderings_digest_of_a_bundle_matches_the_attested_manifest_ex_03`, `an_export_is_repeatable_and_a_bundle_is_apart_from_another_ex_05`, `a_miss_or_a_refusal_writes_nothing_and_a_record_that_is_not_whole_leaves_no_bundle_ex_04` | 5 fail: the bundle lacks the rendering the manifest names | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| the key history left out of the bundle | `cargo test -p gateway-tool-tests --test present` | `the_tool_writes_the_bundle_of_the_record_ex_01`, `a_reader_checks_the_bundle_offline_ex_02`, `the_renderings_digest_of_a_bundle_matches_the_attested_manifest_ex_03`, `an_export_is_repeatable_and_a_bundle_is_apart_from_another_ex_05` | 4 fail: `keys.json` is missing from the bundle | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| the envelope replaced by attestation.json in the bundle | `cargo test -p gateway-tool-tests --test present` | `the_tool_writes_the_bundle_of_the_record_ex_01`, `a_reader_checks_the_bundle_offline_ex_02` | both fail: `envelope.json` is not the envelope in the store, and the offline check fails | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |
| the evidence not checked before the first write: `attestation.json` is written before the history is read | `cargo test -p gateway-tool-tests --test present` | `a_miss_or_a_refusal_writes_nothing_and_a_record_that_is_not_whole_leaves_no_bundle_ex_04` | fails: the bundle was written for a record without a key history | `git checkout -- gateway/bindings/gateway-binding/src/presentation.rs` (the file is new: restore the committed copy): the command passes again (10 passed) |

## Not met

- A record that has several renderings needs `module`: `lexlean_present` returns one rendering and
  an empty `module` on several is `module-unknown`. The tool does not list the paths.
- The bundle does not carry a program that checks it; the check is the list above, and its code is
  the test `check`.
- The export is a directory of the store on the host. Making it reach a client that has no access to
  the host's disk is not claimed.
- AC-06 is run on the canonical LaTeX that LexLean wrote for `nat-add-zero`
  (`gateway/tool-tests/fixture/nat-add-zero/`, copied from `examples/nat-add-zero/expected/build` of
  the LexLean at the pin), recorded with `conclude` as the verifier does. The real verification of
  the tiny project, which needs the Lean toolchain, is presented and exported at the end of the
  ignored test `a_real_verification_runs_as_a_task_to_completed_th_11`.
- Neither tool lists the attestations of a store.
