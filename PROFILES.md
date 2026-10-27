# Profiles and vocabulary

Issue #8 asks for the profile identity, provisioning (J5), the language rule,
the Atlas profiles, the tools `lexlean_profiles` and `lexlean_lexicon_search`,
the resource template `lexlean://profiles/{profile}/lexicon/{package}@{version}`
and completion of its arguments. This file records both halves. The model side is
`src/Profiles.lex.tex`, registered as PF-01 to PF-09. The gateway side is the
code that binds the model into the gateway of #7, registered as VO-01 to VO-16
(CONFORMANCE.md): the host's identity, the operator's provisioning command, the
two tools, the resource template, completion and the capabilities that carry them.

## What is claimed

Every claim below is a theorem of `src/Profiles.lex.tex`, checked by
`lexlean verify` with an exact axiom policy, and held to the issue's words and
to the pinned LexLean by `cargo test -p repo-model --test profiles`.

| ID | Claim |
| --- | --- |
| PF-01 | The profile identity is `Identity.profileId` over the profile files; the domain tag is the 22 octets of `lexlean-mcp-profile-v1`, which `Identity` frames first, before the sorted rows, the review records and the toolchain. |
| PF-02 | A path is under a source root when it is the root, one `/` and more: whole components. `src/Main.lex.tex` and `src/deep/Other.lex.tex` are under `src`; `srcs/Main.lex.tex`, `src` itself and `lexlean.toml` are not. |
| PF-03 | Changing the bytes of `lexlean.toml`, `lexlean.lock`, a file of a local lexicon package or an `artifact_source` file, renaming one, adding one or removing one changes the profile preimage; so do a file under `srcs/` and a file named `src`. Two digests are computed as well, on the smallest project: changed bytes and a renamed path. |
| PF-04 | A file under a source root is not a profile file: `profileFiles` drops it, and for every other file list, review list and toolchain the profile ID with the file equals the ID without it. |
| PF-05 | Four reorderings of the four profile files, one of them with a source file added, give the preimage of the original order and, for one of them, the same ID; a repeated path is refused with `DuplicatePath`. |
| PF-06 | One language per profile: a package of the profile's language is admitted, a package of another language is refused with `LLC0103`, for the nine pairs of 1.0, 1.1 and 1.2; the first refusal of a package list is the answer. |
| PF-07 | Reproduced from the pinned LexLean: `lexlean.uor.atlas@1.0.0` is language 1.0 with 293 entry files, distinct ids, each a document denotation into the module `Atlas`; `examples/uor-atlas` holds 86 core modules (80 at the top, 6 under `Prelude/`) that include `Atlas`. The model's ids and module names equal the committed copy. |
| PF-08 | An Atlas profile serves all 293 entries: all denote `Atlas`, `Atlas` is among the 86 modules, 293 are served; with no `Atlas` module none is served, an entry into a missing module is not, and the owner values put `uor-atlas` at language 1.0, so a language 1.2 package is refused with `LLC0103`. |
| PF-09 | Each law rejects its planted defect (below). |

## Claim level

- PF-03 and PF-05 are claims about the preimage, the framed bytes that
  `Identity` hashes, plus two digests. A different preimage gives a different
  ID because SHA-256 is assumed to separate them; nothing here proves that.
  The digests are computed on a one-file project because the kernel evaluation
  of SHA-256 costs about 2.4 GB for two IDs of the four-file project.
- The profile files are given to the model as rows. Which files of a real
  directory are profile files (`lexlean.toml`, `lexlean.lock`, the files of
  local lexicon packages, the `artifact_source` files) is the provisioning of
  J5, not modelled here; the fixtures name one file of each kind.
- The issue also puts the review records and the toolchain identity into the
  ID. `Identity` frames both and sorts the reviews; this slice passes fixed
  values and does not model what a review record is.
- PF-07 is `some-true`: the facts are the pinned LexLean's
  (`A-LEXLEAN-ATLAS`) and the committed copy under `crates/model/tests/atlas`
  is checked against the authority's checksum. The model repeats them as data
  and the test compares the 293 ids and the 86 module names in order.
- "Served" means that the module an entry denotes is among the modules of the
  profile's source roots. The real gateway serving them is #7's.
- `src/Security.lex.tex` risk 1 ("no profile served without a review record
  per package digest") was Pending in this slice, because no review record
  existed in it. The gateway side executes it (`SM-02`, below).

## The gateway side (VO-01 to VO-16)

What is served, and how it is held to the model:

- **Identity (VO-01, VO-02).** The profile ID is the SHA-256 of `Identity.encProfile`: the
  frame `domain` (`lexlean-mcp-profile-v1`), the `files` list of (path, bytes) rows sorted
  bytewise, the `reviews` list sorted bytewise and the frame `toolchain`. The rows are
  `lexlean.toml`, the lockfile, every file of each path lexicon package (all of its files, so a
  renamed, added or removed file counts) and the `artifact_source` files; a file under a source
  root is not a row. The toolchain bytes are `compiler_semantics <hex>\nlean_toolchain <str>\n`,
  read from the header of the lock. The encoder is Rust (`identity.rs`) and is held to the
  model three ways: it reproduces the model's own vectors (`profilePreimageVector`,
  `profileIdVector`, `profileDigestVector`, `changedFileProfileId`,
  `profileFramingReviewsVector`) read from `src/Identity.lex.tex` at test time,
  `gateway/vectors/generate.py profile-id` recomputes the ID of a real project independently,
  and a byte of each kind of profile file changes the ID.
- **Language (VO-03, VO-04).** `gateway/src/ProfileLanguage.lex.tex` holds `PackageCheck`,
  `PackageDecl`, `languageMismatchCode`, `checkPackage` and `checkPackages` byte for byte as the
  model declares them (the model carries the three definitions as `executable: true`, and a test
  compares the text of each declaration). A package of another language is refused with
  `LLC0103` at `install` (the compiler's own refusal, carried through), by the catalogue (the
  profile is listed under `refused`), by `lexlean_lexicon_search` and by `resources/read`.
- **Provisioning (VO-05 to VO-08).** `gateway-provision` is the operator's command and no
  effect: `init`, `lock` (`--allow-network` only for the Git-sourced packages of a lock, and
  accepted by `lock` alone), `identify` and `install`. `install` lays the project out as locked
  into `profiles/<name>`, with the record `profile` (line 1 the ID in hex, then the roots), the
  derived `vocabulary` and the `reviews`, by one rename; it refuses with a typed `Refusal` and
  leaves the store as it was. A profile is one project directory; a project with source packages
  is refused, because their files lie outside the bundle. The catalogue is the store key
  `profiles` of the area `index`.
- **Review records (VO-06).** A record is four lines (`lexlean-mcp-review-v1`,
  `package ID@VERSION`, `tree_sha256 HEX`, `reviewer TEXT`). A locked package of kind `path` or
  `git` needs a record that names its ID, version and tree digest; a builtin package needs none.
  The bytes of every record enter the ID.
- **Tools, template, completion (VO-09 to VO-12).** `lexlean_profiles` asks one `store.get` of
  the catalogue; `lexlean_lexicon_search` asks one `lexicon.read` (effect E9, the profile's
  `vocabulary` entry); `resources/templates/list`, `resources/list` and `resources/read` serve
  the template with `cacheScope: public` and the owner's TTL; `completion/complete` completes
  `profile` and `package` (at most 100 values). No effect was added: the set stays E1 to E10.
- **Capabilities (VO-13).** The gateway declares `tools`, `resources` and `completions`; it does
  not declare `prompts` or `tasks`, because it serves neither.
- **Atlas (VO-14).** `examples/uor-atlas` of the pinned LexLean installs as a profile of
  language 1.0 with its 86 core modules in its source root, and its 293 entries are listed,
  searched and read through the tools and the template. The tests need the sources of LexLean and
  run in the gateway job with `--ignored`.

The decisions that the issue leaves open, and what was chosen, are in the comment on #8.

## Planted defects

Each defect was planted in a copy of the tree, `lexlean lock` and
`lexlean verify src/Profiles.lex.tex` run (LexLean `release/v0.4` at
`9796067`), and the copy discarded. The unplanted tree passes the same command
(baseline below).

| Defect | Law | Planted | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| P1 | `rootsMatchWholeComponents` | `underRoot` returns `true` after any prefix, instead of testing the separator, so `srcs/Main.lex.tex` is under `src` | `lexlean verify src/Profiles.lex.tex` exits 1: `error[LLV7002]: Lean rejected LexLeanMcp.Profiles: Tactic rfl failed` on `underRoot` of the `srcs/` path (`src/Profiles.lex.tex:685`); 2 errors, the other is `aLookalikeDirectoryCounts` | the line restored; `lexlean verify` accepts the tree |
| P2 | `sourceFilesAreNotProfileFiles` | `profileFiles` keeps the file in both branches, so a source file is a profile file | `lexlean verify src/Profiles.lex.tex` exits 1: `error[LLV7002]: ... Tactic rfl failed ... profileFiles exampleRoots withSources` is not definitionally equal to `profileFiles exampleRoots baseFiles` (`src/Profiles.lex.tex:693`); 5 errors, among them `aSourceFileNeverChangesTheId` | the line restored; `lexlean verify` accepts the tree |
| P3 | `anEntryIntoAMissingModuleIsNotServed` | `servedCount` adds 1 for an entry whose module is not served, so every entry counts | `lexlean verify src/Profiles.lex.tex` exits 1: `error[LLV7002]: ... Tactic decide proved that the proposition servedCount atlasModules (unknownEntry :: atlasEntries) = 293 is false` (`src/Profiles.lex.tex:947`); 3 errors | the line restored; `lexlean verify` accepts the tree |
| P4 | `theAtlasProfileRefusesALanguageOnePackage` | `checkPackage` answers `Admitted` for a package of another language | `lexlean verify src/Profiles.lex.tex` exits 1: `error[LLV7002]: ... Tactic decide proved that the proposition refusedWith languageMismatchCode (checkPackage atlasLanguage "1.2") = true is false` (`src/Profiles.lex.tex:905`); 4 errors, among them `languageTable` | the line restored; `lexlean verify` accepts the tree |

Baseline: `lexlean verify src/Profiles.lex.tex` on the unplanted tree:
`verified 5 modules; attestation 476737ae7d8ad1d704fee5e585993cba9128d73f123f1aa63ed3a3676443dd27`, exit 0. The CI job `lexlean / verify` runs the whole project.
