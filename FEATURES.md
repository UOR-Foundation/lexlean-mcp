# Features and journeys

Issue #18 asks for the 13 features and 6 journeys of the product as declarations that
a checker can hold against the rest of the model. This file says where each one comes
from, which parts the issues do not state and so wait for the owner, and which defects
were planted in the checks that guard them. `src/Features.lex.tex` is the model; this
file adds no claim of its own (R1), and FJ-10 holds the two in step.

## What is checked, and what is not

The declarations use the vocabulary of PrismPM#330: `Foundation.Arch::Feature` (a
statement, the calls that realize it, the acceptance rows that accept it) and
`Foundation.Arch::Journey` (an actor and ordered steps, each naming a feature). A call,
an acceptance row, a step and an actor are qualified `Module::declaration` names of
definitions in this repository's source graph.

- Kernel theorems in `src/Features.lex.tex`, all `decide`, restate the four rules of
  PrismPM for this product (every feature has a call, every feature has an acceptance
  row, every journey step names a declared feature, every journey actor is a declared
  stakeholder), the rule that a call or acceptance row names a definition, and the
  counts: 13 features and 6 journeys, numbered in order.
- `crates/model/tests/features.rs` (FJ-01 to FJ-10) reads the same source as text and
  applies the same rules independently, so a rule is not trusted to a single
  implementation.
- **Not done:** `prismpm check` has not validated the declarations. The acceptance line
  of #18 ("`prismpm check` validates all 13 features and 6 journeys") cannot run: the SDK
  image bound in `prismpm.lock` is PrismPM main `5faa29a8`, which predates PrismPM#330
  (merge `d11ace6f`) and does not know PP2014 to PP2018. It runs once the binding moves
  to an image that contains #330.

The vocabulary (`language/prism.arch/`, `src/Foundation/Arch.lex.tex`) is a byte-identical
copy of PrismPM main at `d11ace6f`: 143 files, whose manifest digest is the checksum of
the authority `A-PRISMPM-ARCH` (FJ-09). Bringing the copy forward widened some records
of `Foundation.Arch` (`Stakeholder`, `Concern`, `Component` gained `name` and `text`,
`StandardReference` gained `standardId` and `edition`), so `Architecture` and `Quality`
now state them. `architectureValid` ties each stakeholder, concern and component name
and text to the narrative lists that were already there, so the words have one source.
The ten component descriptions are new prose written from #5, #7 and the architecture
description; they are descriptive and no theorem depends on their wording.

## Declared items

Where a minimal named definition was needed because no definition existed for a tool or
an acceptance row, it is declared in `Features` as a `String` definition (the tool name,
or the acceptance criterion in the words of its issue). That is all they are: names the
rules can resolve. Their meaning lives in the issue that builds them.

| ID | Name | Provenance | Realized by | Accepted by |
| --- | --- | --- | --- | --- |
| F1 | Profiles | issue 8, title "(F1, F2, J5)" | `lexlean_profiles` | profile ID changes with any profile file; package of another language refused with LLC0103 |
| F2 | Vocabulary | issue 8, title "(F1, F2, J5)" | `lexlean_lexicon_search`, the lexicon resource template | an Atlas profile serves all 293 entries |
| F3 | Drafts | issue 9, title "(F3)" | `lexlean_draft_write`, `lexlean_draft_assemble`, `lexlean_draft_read` | AC-09 |
| F4 | Check | issue 10, title "(F4 to F7)", order of its Work list | `lexlean_check` | tool equals CLI; AC-07 check and build fixtures; AC-08; Q4 |
| F5 | Snapshot | issue 10, as F4 | `lexlean_snapshot` | tool equals CLI; Q4 |
| F6 | Format | issue 10, as F4 | `lexlean_format` | tool equals CLI; Q4 |
| F7 | Build | issue 10, as F4 | `lexlean_build` | tool equals CLI; AC-04; AC-07 check and build fixtures; Q4 |
| F8 | Verify | issue 11 | `lexlean_verify` | AC-05; AC-07 verify fixtures; AC-11; AC-12; AC-13; Q3 |
| F9 | Present | issue 12, title "(F9, F10, F12)" | `lexlean_present` | AC-06 |
| F10 | Attestation lookup | issue 12, title "(F9, F10, F12)"; needs owner for the acceptance row, see below | `lexlean_attestation` | attestation resolves by ID or file digest |
| F11 | Explain | issue 10 Work (`lexlean_explain`) and Q8 of the quality model ("every diagnostic code in a result resolves through F11") | `lexlean_explain` | tool equals CLI; Q8 |
| F12 | Export | issue 12, title "(F9, F10, F12)" | `lexlean_export` | renderings digest matches the attested build manifest |
| F13 | Lifecycle | needs owner: no issue gives F13 a name; it is the number left over once F1 to F12 are placed, and issue 13 is the issue left over | `store.get`, `store.put` (the host effects E1 and E2 of issue 7) | AC-14; AC-15; AC-16; AC-17 |

| ID | Name | Provenance | Actor | Steps |
| --- | --- | --- | --- | --- |
| J1 | Author and attest | needs owner: issue 12 fixes only the end ("J1 ends in an embedded resource", AC-06); the order is the order the tools are used in | Author agent | F1, F2, F3, F4, F8, F9 |
| J2 | Repair | needs owner: the order is the order the tools are used in | Author agent | F4, F11, F3, F4, F8 |
| J3 | Vocabulary gap | needs owner: the order is the order the tools are used in; AC-08 supplies the trigger (an atom outside the profile) | Author agent | F4, F2, F3 |
| J4 | Reader check | needs owner: issue 12 says a reader checks an exported bundle offline (J4); whether the reader also calls `lexlean_present` is not stated | Reader | F10, F12 |
| J5 | Provision a profile | needs owner: issue 8 puts provisioning (`lexlean init`, `lexlean lock`, the Lake workspace, review records) in J5, but those are command line work that no tool performs, so the only step a tool serves is F1 | Operator | F1 |
| J6 | Upgrade | needs owner: issue 13 and AC-14 give the content (new profile IDs, earlier records resolve), not the steps | Operator | F13, F1 |

The actor of each journey is the stakeholder of the architecture description whose
narrative fits it (Author agent drafts and repairs; Reader checks a rendering against its
attestation; Operator runs the server on a host). No issue assigns actors to journeys.

## Needs owner

These are the places where the declaration is a reading of the issues, not a statement in
them. Each stays as declared until the owner rules, and a ruling changes `Features` only.

1. **F13 as a whole.** Its identity (lifecycle), its statement and its calls. The issues
   never say what F13 is; #13 ("Store, persistence and lifecycle") is the one place
   left. Its calls are the host effects `store.get` and `store.put`, which are not tools.
2. **The step order of J1 to J6**, and in particular J4, J5 and J6, whose steps are one
   or two features each.
3. **The acceptance row of F10.** Issue 12 lists two acceptance criteria, AC-06 (F9) and
   the renderings digest (F12). The lookup row comes from its Work list; no criterion
   accepts F10. The security model measures it only as "every envelope F10/F12 return
   validates under the key history".
4. **Journey actors**, as above.
5. **Whether Q3, Q4 and Q8 belong in the acceptance rows.** Issues 10 and 11 list Q4 and
   Q3 among their acceptance criteria and the quality model fixes Q8 on F11, so they are
   declared as rows; they are measures of the quality model, and the owner may prefer to
   leave them there.
6. **The feature statements** are one-line readings of each issue's Work, not the owner's
   words.

F11 is not on this list: the quality model of this repository already states that
`lexlean_explain` is F11. The worked example in PrismPM called F11 a guess; here it is
confirmed by `Quality` (requirement Q8 and its evidence row).

## Planted defects

Each defect edits the data of `Features` and never a theorem. The command is
`lexlean verify src/Features.lex.tex` on a scratch copy of the repository with the defect
applied; the diagnostic is what it prints; restoring the line gives `verified`.

Every verify run below ends in `error[LLV7002]: Lean rejected LexLeanMcp.Features`,
"Tactic `decide` proved that the proposition ... = true is false", for the theorem named
and for `featuresModelHolds`, which restates all of them. The unplanted module verifies:
`verified 4 modules` (Foundation.Arch, Product, Architecture, Features), attestation
`8b2e3428...1968`. The last column is the test of `crates/model/tests/features.rs` that
fails on the same defect, run with `cargo test -p repo-model --test features`.

| Plant | Defect | Theorem rejected (`lexlean verify src/Features.lex.tex`) | Test rejected |
| --- | --- | --- | --- |
| P1 | F13 not declared (and J6 no longer steps to it) | `thirteenFeaturesInOrder` | FJ-01, FJ-07 |
| P2 | J6 not declared | `sixJourneysInOrder` | FJ-02 |
| P3 | F3 declared with no call | `everyFeatureHasACall` | FJ-03, FJ-07 |
| P4 | F9 declared with no acceptance row | `everyFeatureHasAnAcceptanceRow` | FJ-04, FJ-07 |
| P5 | J3 steps through `Features::featureF99Imaginary` | `everyJourneyStepNamesAFeature` | FJ-05 |
| P6 | J5 acted by `Architecture::stakeholderNobody` | `everyJourneyActorIsAStakeholder` | FJ-06 |
| P7 | the call `callExport` is not in the call table | `everyCallNamesADefinition` | FJ-07 |
| P10 | the acceptance row `acAtlasEntries` is not in the acceptance table | `everyAcceptanceRowNamesADefinition` | FJ-07 |
| P11 | the Reader is left out of the stakeholder table | `stakeholderRowsAreTheArchitectureStakeholders` and `everyJourneyActorIsAStakeholder` | FJ-06 |
| P8 | one byte appended to `language/prism.arch/entries/arch-feature.toml` | not a Lean claim | FJ-09 |
| P9 | the F13 row removed from this file | not a Lean claim | FJ-10 |

Restoring the planted edit gives back the committed bytes (checked with `cmp`) and
`verified`; the ten tests pass.

## Reproducing

```sh
lexlean fmt --check --all && lexlean lock --check
lexlean verify src/Features.lex.tex
cargo run -q -p xtask -- check-model
cargo test -p repo-model --test features
```
