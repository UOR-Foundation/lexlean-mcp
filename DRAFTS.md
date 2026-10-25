# The draft tools (F3, issue #9)

`lexlean_draft_write` stores the files of a draft under the name that their content and their
profile give it, and `lexlean_draft_read` reads them back. An Engine tool (`lexlean_check`,
`lexlean_snapshot`, `lexlean_format`, `lexlean_build`, `lexlean_verify`) then takes that name as
its `project`. The rows are `DR-01` to `DR-11` (suite `drafts`); the model is
`src/Identity.lex.tex` (the identity) and `src/Admission.lex.tex` (path admission), both already
verified, and the core is `gateway/src/Drafts.lex.tex`. The tests are
`cargo test -p repo-model --test drafts` (the register half, from the root workspace) and, in
the gateway workspace, `cargo test -p gateway-tool-tests --test drafts` (the behaviour half:
every call goes through the production core with the real host) and
`cargo test -p gateway-binding --test materialize` (the host's materialization).

## The tools

| Tool | Input | Result | Effects |
| --- | --- | --- | --- |
| `lexlean_draft_write` | `profile` (the name of a provisioned profile), `files` (`[{path, content}]`) | `{draft, id, profile, files, bytes}`; the text is `draft <profile>/<id>` | `lexicon.read` of the record of the profile, then `store.put` |
| `lexlean_draft_read` | `draft` (`<profile>/<id>`), `path` (optional) | with a `path`: the text of that file and `{path, bytes}`; without: the paths one to a line and `{files: [{path, bytes}], bytes}` | `store.get` |

Both input schemas are closed (`additionalProperties: false`), and a call that breaks one is
`Invalid arguments:` and asks for no effect (`DR-09`). The set of effects is the model's ten
(E1 to E10, `src/Effects.lex.tex`): the draft tools add none.

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
`PathUnknown`. A handle that names a key the store refuses is the host failure `key-refused`.

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

A refusal is an error result whose structured content is `{"refusal": <name>}`; a host failure
is one whose structured content is `{"failure": <name>}`.

## Not met

The issue (#9) names three tools. Two are built, and the third is not.

- **`lexlean_draft_assemble`**, and the structured side of **`lexlean_draft_read`**, call four
  functions of `lexlean::sdk`: `assemble` and `write_module` (build the text of a module from
  a header and its data form) and `read_module` and `declaration_spans` (read a module as data
  and as the spans of its declarations). None of the ten effects reaches them: `engine.*` runs
  one of five commands on a project and returns the bytes the CLI prints, and `store.*` and
  `lexicon.read` move bytes. A tool for them cannot be written in the core, whatever the host
  does. The set of effects is closed (`GW-05`: no effect outside E1 to E10 can be encoded), so
  this is a decision for the lead, not a gap in the draft tools.
  *Proposal:* two optional fields on the effects that already move a draft's bytes. A `view`
  on E1 `store.get` (`0` the bytes, `1` the module as the data of `sdk::read_module`, `2` its
  declaration spans of `sdk::declaration_spans`) would give `lexlean_draft_read` its structured
  read, and a `form` on E2 `store.put` (the data form of a module, with the header, which the
  host renders with `sdk::assemble` and `sdk::write_module` before it stores the text) would give
  `lexlean_draft_assemble` its write. The cost is one request or result field each, in
  `src/Effects.lex.tex` and its copy `gateway/src/Effects.lex.tex`, and the rows `GW-01`, `GW-02`
  and `GW-05` that pin the fields. The alternative is an eleventh effect (`sdk.read`,
  `sdk.assemble`), which also changes the order of the effects and the generated host crate.
- `Q2` of the quality model keeps its clause ("each in-scope function is reached by a tool that
  exists and runs") pending for the four `sdk` functions for the same reason.

## Planted defects (`DR-11`)

Each defect was applied to the working tree, the command was run, and the file was restored
(`git checkout`, then the command again).

| Id | Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
