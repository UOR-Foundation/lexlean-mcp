# The draft tools (F3, issue #9)

`lexlean_draft_write` stores the files of a draft under the name that their content and their
profile give it, `lexlean_draft_read` reads them back (as text, or as the data and the spans of a
module), and `lexlean_draft_assemble` builds a module from its data into a new draft. An Engine
tool (`lexlean_check`, `lexlean_snapshot`, `lexlean_format`, `lexlean_build`, `lexlean_verify`)
then takes that name as its `project`. The rows are `DR-01` to `DR-16` (suite `drafts`); the
model is `src/Identity.lex.tex` (the identity) and `src/Admission.lex.tex` (path admission), both
already verified, and the core is `gateway/src/Drafts.lex.tex`. The tests are
`cargo test -p repo-model --test drafts` (the register half, from the root workspace) and, in
the gateway workspace, `cargo test -p gateway-tool-tests --test drafts` (the behaviour half:
every call goes through the production core with the real host),
`cargo test -p gateway-binding --test materialize` (the host's materialization) and
`cargo test -p gateway-binding --test views` (the `view` and the `form` of the host).

## The tools

| Tool | Input | Result | Effects |
| --- | --- | --- | --- |
| `lexlean_draft_write` | `profile` (the name of a provisioned profile), `files` (`[{path, content}]`) | `{draft, id, profile, files, bytes}`; the text is `draft <profile>/<id>` | `lexicon.read` of the record of the profile, then `store.put` |
| `lexlean_draft_read` | `draft` (`<profile>/<id>`), `path` (optional), `view` (optional: `text`, `data` or `spans`) | with a `path` and no `view` (or `text`): the text of that file and `{path, bytes}`; without a `path`: the paths one to a line and `{files: [{path, bytes}], bytes}`; with `data` or `spans` (a `path` is required): the JSON of the view as text and `{path, view, bytes, data}` or `{path, view, bytes, spans}` | `store.get` (with the `view` 0, 1 or 2) |
| `lexlean_draft_assemble` | `profile`, `path`, `data` (a JSON object, the data form of a module); optional `draft` (the base, a handle), `kind` (`semantic`, `tooling` or `core`) and `header` (`{name, glossaries, imports, title}`, all four required), the last two together | `{draft, id, profile, path, form, files, bytes}`; the text is `draft <profile>/<id>`; `form` is `assemble` or `rewrite` | `lexicon.read`, then `store.get` of the base when there is one, then `store.put` of the rendered module, then `store.put` of the draft |

All three input schemas are closed (`additionalProperties: false`), and a call that breaks one is
`Invalid arguments:` and asks for no effect (`DR-09`). The set of effects is the model's ten
(E1 to E10, `src/Effects.lex.tex`): the draft tools add none. The three tools use two of them in a
way that issue #9 needed fields for, and the fields are the next two sections.

## The `view` of `store.get` and the `form` of `store.put` (`DR-12`, `DR-13`)

`lexlean::sdk` has four functions that read and write modules, and none of the ten effects
reached them. E1 and E2 therefore gained one closed field each (EFFECTS.md, "The closed fields";
`GATEWAY.md`), and the host, not the core, calls the sdk:

- `view` of E1 `store.get`: `0` the bytes as stored, `1` the module as the data of
  `sdk::read_module`, `2` the byte ranges of its declarations as `sdk::declaration_spans` gives
  them (`null` for a module written as data). The answer has a `class`: `0` accepted, `1` refused
  (the content is the message of the sdk), `2` the draft has no such path. A `path` in the request
  makes the key the handle of a draft, whose identity the host checks before it takes the file;
  without a `path` the view is of the bytes at the key.
- `form` of E2 `store.put`: `0` stores the content as given, `1` stores the text that
  `sdk::assemble` renders for `{kind, header, data}` (the content is that JSON), `2` stores the
  text that `sdk::write_module` renders for `{text, data}`, which keeps the form of `text`
  (notation or data). The answer has `length`, a `class` and the `rendered` bytes. A refusal of the
  sdk is the class `1` with its message, and nothing is stored.

`lexlean_draft_read` with `view` `data` or `spans` asks one `store.get` with the path and the
view; with no `view` it asks the view `0`, and the text it returns is the bytes. The core never
parses a module: it takes the JSON of the answer with `JsonRead.readJson` for the structured
result, and reads a `class` outside the closed set as `OutcomeMalformed`.

`lexlean_draft_assemble` is the one place a draft is composed from data. With a base `draft`, it
asks for the profile record (`lexicon.read`), reads the base (`store.get`, view `0`, checked
against the identity in its handle), admits the path (the same admission as a write, with an empty
placeholder, before any store effect), and asks `store.put` with the form `1` (`kind` and
`header` given) or `2` (neither given; the file at `path` must exist in the base, and its text is
the `text` that the sdk keeps the form of). The rendered module is stored once under the key
`<profile>/modules/<SHA-256 of the request>`, which is not a draft handle, so no tool lists it as a
draft, and the core puts the same bytes as the file `path` of the new draft, with the form `0`:
the draft is named by the identity over the bytes the sdk rendered, exactly as a write names its
draft. The other files of the base are kept. Without a base, the draft has one file and `kind` and
`header` are required. The effect count is four with a base and three without; the exchange
budget is therefore four (GATEWAY.md).

## Prose documents (`DR-14`)

`lexlean::sdk` handles semantic, tooling and core modules only, and a prose document (text of a
`.lex.tex` file that has no module) is stored and read as text: `lexlean_draft_write` stores with
the form `0`, and `lexlean_draft_read` without a `view` asks the view `0`, so neither reaches the
sdk. `kind` has no value for a prose document, so `lexlean_draft_assemble` cannot be asked to
make one. A prose document asked for the `data` or `spans` view is the refusal `ModuleRefused`
with the message of the sdk, an error result, and nothing else changes.

## Identity (`DR-01`, `DR-02`)

The identity of a draft is `Identity.draftId`, the SHA-256 of the preimage `encDraft` of
`src/Identity.lex.tex`: the frame `domain` holding `lexlean-mcp-draft-v1`, the frame `profile`
holding the 32 octets of the profile ID, then the head `sources`, the 64-bit count `2n`, and for
each row, in the order of the octets of the path, a frame `source` with the path and a frame
`source` with the bytes. The core encodes the preimage itself, in LexLean (`Drafts.preimage`) and
hashes it with `Sha256`; `draftPreimageVector` and `draftIdVector` of the model are the oracle
(`the_core_reproduces_the_models_draft_identity_dr_01`), and `gateway/vectors/generate.py`
computes the handle of the vector draft in Python, independently of the core. The identity
covers the profile ID, so the same files under another profile are another draft, and every
path and byte, so a renamed file or one changed byte is another draft. The handle of a draft is
`<profile name>/<identity in 64 lower case hexadecimal digits>`; a draft written twice is the
same key.

## Admission (`DR-03`, `DR-04`)

The core admits the files with the exported admission, not with a check of its own:
`gateway/src/DraftAdmission.lex.tex` is `src/Admission.lex.tex` with its module name and its
title changed (the test requires every other line to be equal), on `src/Nfc.lex.tex`,
`src/NfcTables.lex.tex` and the tables in `gateway/artifacts` (byte for byte). `admitDraft`
refuses, with the constructor of `Refusal` named for it, a path that is not UTF-8, is not NFC,
is absolute, has an empty, `.` or `..` component, is outside the source roots of the profile or
is not a `.lex.tex` file, and a draft with more than `draft.max_files` 256 files, more than
`draft.max_bytes` 8388608 bytes of content (the paths are not counted) or a repeated path. The
refusal is the verdict of the model, in its order (`..` first). It comes after the one effect
that the call needs to know the roots, `lexicon.read`, and before `store.put`: no refused draft
is stored, and no Engine tool runs on a refused path.

## The profile record (`DR-06`)

The source roots are those of the profile, and the profile is provisioned by #8. This is the
contract with it: the entry `profile` of the profile in the `profiles` area (the lexicon entry
of that name) is text of lines separated by `\n`. The first line is the profile ID, 64 lower
case hexadecimal digits. Every following non-empty line is one source root, a relative path of
whole components (`src`, `src/lib`). At least one root is required. A profile that is not
provisioned is the host failure `profile-unknown` (`lexicon.read` fails). A profile without that
entry, or with an entry that does not have that form, is refused as `ProfileRecordMalformed`,
with no `store.put`. The entry is not a file of the bundle: the host does not copy it into a
project.

## Storage and reading (`DR-05`)

The preimage is stored whole, as the content of the one key `<profile>/<identity>` of the
`drafts` area (area 1): one effect, so a write is `lexicon.read` and `store.put` within the
budget of two effects of an exchange. A store limit of `draft.max_bytes` of content alone would
refuse a draft at the limit, because the preimage also holds the paths and 36 octets of framing
for each file; the host therefore allows `draft.max_bytes` and 1 MiB (`DRAFT_BLOB_HEADROOM`), and
the core, not the store, holds a draft to `draft.max_bytes`. A read asks `store.get` for the key
and checks, before it answers, that the SHA-256 of the bytes is the identity of the handle and
that they decode as a preimage: an unknown draft or a handle that is not
`<profile>/<64 lower case hexadecimal digits>` is `DraftUnknown`; bytes that were changed, that
do not decode, or that have trailing octets are `DraftCorrupt`; a path the draft lacks is
`PathUnknown`. A handle that names a key the store refuses is the host failure `key-refused`. A read of the
structured views checks the identity in the host, which answers the host failure `unreadable` for
a draft whose bytes are not the draft their name says.

## Materialization (`DR-07`, `DR-08`)

An Engine tool whose `project` is the handle of a draft runs on a directory that the host makes
for the call (`gateway-binding/src/materialize.rs`): it reads the draft, checks it as a read
does, requires that the profile ID of the preimage is that of the record of the profile, and
**copies** the files of the profile bundle (`profiles/<profile>/`, except the record) and then
the files of the draft, each into a regular file of its own, into a new directory of the
`builds` area. Nothing in that directory is a link, so LexLean (SPEC 25.1) sees none, and a
link is never created. The directory belongs to the call and is removed when the call ends;
`lexlean_verify` keeps its directory, because the verification outlives the call (`builds` is a
cache that the lifecycle of the store, `ST-`, can drop). Two calls on one draft are two
directories and never see each other's files. A draft or a profile that is absent, a draft that
was changed, a draft of another profile than its record, and a path that the host would not
write below a project are the host failure `project-refused`.

A bundle that holds a symbolic link, anywhere, to a file, a directory, a pipe or nothing at all,
is refused before the Engine is called, as `project-refused`: the host inspects each entry with
`symlink_metadata`, refuses an entry that is a link or neither a directory nor a file, and opens
each file it copies with `O_NOFOLLOW`, so a link that appears between the inspection and the
copy is refused as well. A link is never followed, and no byte is read through one. This is the
symlink clause of control K5, which `src/Security.lex.tex` recorded as pending until drafts were
materialized; `DR-08` executes it (SECURITY.md).

## Refusals (`DR-10`)

| Refusal or failure | Where it is raised | Test (`gateway-tool-tests`, `tests/drafts.rs`) |
| --- | --- | --- |
| `DotDotComponent`, `NotLexTex`, `NotNfc`, `AbsolutePath`, `EmptyComponent`, `DotComponent`, `OutsideRoots`, `TooManyFiles`, `TooManyBytes`, `DuplicatePath` | `lexlean_draft_write`, by `admitDraft` | `<constructor>_is_a_typed_refusal_dr_10` for each |
| `NotUtf8` | the reader of JSON, before any tool runs: the path of a call is JSON text, which cannot hold octets that are not UTF-8 or a lone surrogate. The constructor is the verdict of `admitDraft` on octets, and it is observed there by `DL-03` and `NF-09`. | `not_utf8_is_refused_by_the_json_reader_before_the_tool_is_reached_dr_10` |
| `ProfileRecordMalformed` | `lexlean_draft_write` | `profile_record_malformed_is_a_typed_refusal_dr_10` |
| `DraftUnknown`, `PathUnknown`, `DraftCorrupt` | `lexlean_draft_read` | `draft_unknown_`, `path_unknown_`, `draft_corrupt_is_a_typed_refusal_dr_10` and two tests of bytes that are not a preimage |
| `profile-unknown` | the host, for `lexicon.read` | `profile_unknown_is_a_typed_host_failure_dr_10` |
| `key-refused` | the host, for `store.get` and `store.put` | `a_key_the_store_refuses_is_a_typed_host_failure_dr_10` |
| `project-refused` | the host, for an Engine tool on a handle | `project_refused_is_a_typed_host_failure_dr_10` and the tests of `DR-07` and `DR-08` |
| `ModuleRefused` | `lexlean_draft_assemble` and the structured `lexlean_draft_read`, when the sdk refuses (class `1`); the structured content is `{"refusal": "ModuleRefused", "message": <the message of the sdk>}` | `module_refused_is_a_typed_refusal_dr_15`, `a_prose_document_asked_for_the_view_data_is_module_refused_dr_14` |
| `PathUnknown` (a rewrite or a structured read) | `lexlean_draft_assemble` without `kind` and `header`, for a path the base lacks; `lexlean_draft_read` with a `view`, for a path the draft lacks (class `2`) | `path_unknown_for_a_rewrite_is_a_typed_refusal_dr_15` |
| `ProfileMismatch` | `lexlean_draft_assemble`, when the profile of the base handle is not `profile`; no effect is asked | `profile_mismatch_is_a_typed_refusal_dr_15` |
| `OutcomeMalformed` | `lexlean_draft_assemble` and the structured `lexlean_draft_read`, for a `class` outside the closed set or an answer that is not the layout of the effect | `a_class_outside_the_closed_set_is_a_malformed_outcome_dr_15` |
| `view-unknown`, `form-unknown` | the host, for a `view` or a `form` outside the closed fields | `an_unknown_view_is_a_typed_failure_dr_15`, `an_unknown_form_is_a_typed_failure_dr_15` (gateway-binding, `tests/views.rs`, which also name them `_gw_05`) |

A refusal is an error result whose structured content is `{"refusal": <name>}`; a host failure
is one whose structured content is `{"failure": <name>}`.

## Not met

Nothing of issue #9 is open: the three tools, the identity, the admission, the materialization
and the two acceptance criteria are built and tested. Two limits of the draft tools are stated
here, not hidden:

- `sdk::assemble`, `sdk::write_module`, `sdk::read_module` and `sdk::declaration_spans` are
  called by the host (`gateway-binding/src/modules.rs`), because the core is a pure function of
  the outcomes of its effects. What the core owns is the choice of the view and of the form, the
  closed set of their values, the identity of the draft that is made of the rendered bytes, and the
  refusals; the rendering of a module is the sdk's, and `an_assembled_module_is_what_sdk_assemble_returns_dr_13`
  and its two siblings hold the host to the sdk's own result on the same input.
- The rendered module is stored once under `<profile>/modules/<SHA-256 of the request>` as well as
  in the draft, because `store.put` with a `form` stores what it renders. The `drafts` area has
  those entries beside the drafts of a profile, and the lifecycle of the store (`ST-`) is where
  a cap on them belongs.

## Planted defects (`DR-11`)

Each defect was applied to the working tree, the command was run, and the file was restored
(`git checkout`, then the command again). D1 to D4 are those of the first draft tools; D5 to D7
are those of the `view`, the `form` and the prose documents.

| Id | Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| D1 | a `..` path admitted: in `gateway/src/DraftAdmission.lex.tex` the `dotdot` branch of the component stage answers `Admitted` | `cargo test -p repo-model --test drafts` | `the_admission_of_the_core_is_the_exported_one_dr_03` fails: `the copy differs from src/Admission.lex.tex in its module name and its title only`, left `[0, 6, 147]`, right `[0, 6]`. The pinned Lean rejects the generated module too (`lexlean build`, then `lean` on `DraftAdmission.lean`): `error: Tactic rfl failed` for `refusesDotDotInTheMiddle` and `refusesLeadingDotDot`, which state that a path with `..` in the middle and one with a leading `..` are `Refused DotDotComponent` (`lexlean verify` is CI's) | `git checkout gateway/src/DraftAdmission.lex.tex` (the model test passes again) |
| D2 | a symbolic link followed in a profile bundle: `copy_bundle` inspects with `fs::metadata` and `copy_file` opens with `fs::File::open`, where `materialize.rs` has `symlink_metadata` and `open_for_reading` (`O_NOFOLLOW`) | `cargo test -p gateway-binding --test materialize` (gateway workspace, offline), and `cargo test -p repo-model --test drafts` | four tests of `DR-08` fail with the link materialized: `a_link_to_a_file_is_refused_dr_08` finds `Ok(".../builds/<id>-<pid>-<n>")` where `Err(Link)` is expected, and so do `a_link_to_a_directory_`, `a_link_below_a_subdirectory_` (11 passed, 4 failed), and `a_dangling_link_` finds `Err(Io)`; the model test `a_symbolic_link_in_a_bundle_is_refused_dr_08` fails: `the host inspects an entry without following it and refuses a link` | the saved `materialize.rs` (equal by `git diff`); 15 passed |
| D3 | a draft identity that ignores the profile: `(frameOf "profile" profile)` replaced by the frame of an empty profile in `gateway/src/Drafts.lex.tex` | `cargo test -p repo-model --test drafts` | `the_identity_depends_on_the_profile_and_on_every_row_dr_02` fails: `the preimage holds the profile ID: a draft identity that ignores the profile fails here`. Built into the core, `generate.py record` also stops: `the core named the draft ..., the generator ...` (the generator computes the handle in Python) | `git checkout gateway/src/Drafts.lex.tex` |
| D4 | a `..` component admitted by the host's last check: `path_is_safe` of `materialize.rs` without `".."` | `cargo test -p gateway-binding --test materialize` | `a_draft_with_a_path_that_leaves_the_project_is_refused_dr_07` fails: for `../x.lex.tex` the left side is `Ok(".../builds/<id>-<pid>-<n>")` where `Err(Draft)` is expected (14 passed, 1 failed) | the saved `materialize.rs`; 15 passed |
