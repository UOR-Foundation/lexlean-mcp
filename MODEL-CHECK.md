# `prismpm check` on the model

The development SDK image of `prismpm.lock` (PrismPM main `15f630aa`) checks this language-1.2 model.
`just vv` runs `prismpm check`, so a model it refuses fails the gate. This file records what the model
had to change, what the check rejects, and what it lets through. Registered as MC-01 to MC-03; the
tests are `crates/model/tests/model_check.rs`, which read the sources as text and hold the gate in
place outside the image.

## What is claimed

| ID | Claim |
| --- | --- |
| MC-01 | The root is a PrismPM project over the root model, and `just vv` runs `prismpm check`. |
| MC-02 | Each facet index is the position of its declaration in the byte order of the IDs, and each standard reference has the form of the standards profile. |
| MC-03 | Each rule the model relies on rejects its planted defect, and what the check does not reject is named below. |

## What the model had to change

- **The names of 123 declarations.** The check requires the `index` of a facet record to be its position
  in the byte order of `Module::name`. The model numbers its records in the order of their meaning, and
  those numbers are what the cross-references and the theorems use. So the indexes stay and the names
  move: `<kind><two digits of the index><rest>`, such as `risk05RequestTimeAcquisition`. Two digits,
  because in bytes `10` sorts before `2`. `Concern` already had this order, and `Feature` and `Journey`
  have no index. No index and no cross-reference changes.
- **The two standard references.** `standardId` is the profile's identifier (`ISO-42010-2022`,
  `ISO-25010-2023`), `standardIndex` its position in the profile (4 and 0) and `editionIndex` 0, because
  the profile cites one edition of each. `editionIs2023` now compares the identifier and the edition.
- **A root `prismpm.toml`**, which the check needs to run at all.

`src/Foundation/` is a vendored copy and did not change.

## Planted defects

Each was planted alone in a copy of the conformed model, inside the image of `prismpm.lock`, with the
command `prismpm check`, and the copy was restored; the unplanted model passes the same command.

| Defect | Diagnostic |
| --- | --- |
| an index out of order: the first activity has index 3 | `PP4004: Security::activity00RotateSigningKey must have canonical index 0` |
| a standard of another edition | `PP4004: Architecture::standardIso42010 must have edition 2022 for ISO-42010-2022, found 2021` |
| a standard outside the profile | `PP4004: Quality::standardIso25010 cites ISO-25010-2011, which is not in the standards profile` |
| a feature with no call | `PP2014: feature Features::featureF01Profiles is realized by no interface call` |
| a feature with no acceptance row | `PP2015: feature Features::featureF01Profiles is accepted by no acceptance row` |
| a call that names no declaration | `PP2018: feature Features::featureF01Profiles names call Features::callAbsent, which no declaration defines` |
| a journey step that is no feature | `PP2016: journey Features::journeyJ1AuthorAndAttest step 0 names Features::featureF99Absent, which is not a declared feature` |
| a journey actor that is no stakeholder | `PP2017: journey Features::journeyJ1AuthorAndAttest has actor Architecture::stakeholderAbsent, which is not a declared stakeholder` |
| a viewpoint link that is unresolved | `PP2001: viewpoint link is unresolved` |
| a risk link that is unresolved | `PP2005: risk link is unresolved` |
| a control link that is unresolved | `PP2006: control risk is unresolved` |
| a requirement link that is unresolved | `PP2007: subcharacteristic is unresolved` |
| a measure link that is unresolved | `PP2007: measure requirement is unresolved` |

## What the check does not reject

The check holds that links resolve. It does not hold that every record is covered, which is what the
acceptance lines of #5, #14 and #15 ask of it. A passing `prismpm check` therefore says less than those
lines do. Each defect below passes `prismpm check`, and was then run through `lexlean verify`, against
an unplanted control that verifies 18 modules.

| Defect | `prismpm check` | `lexlean verify` |
| --- | --- | --- |
| a concern framed by no viewpoint (#5) | passes | `LLV7002` in `Architecture`: `interestsFramed`, `architectureValid` |
| a risk with no control (#14) | passes | `LLV7002` in `Security` |
| a requirement with no measure (#15) | passes | `LLV7002` in `Quality`: `qualityChainHolds`, `qualityValid` |
| a view that names another viewpoint | passes | **passes**: nothing here rejects it |

So the coverage of the first three is held, by the kernel theorems of the model and in `lexlean verify`,
not by `prismpm check`. The last is held by nothing.

## The acceptance lines

| Issue | Line | State |
| --- | --- | --- |
| #18 | `prismpm check` validates all 13 features and 6 journeys | holds, on the development image: PP2014 to PP2018 each reject their defect |
| #5 | `prismpm check` validates the architecture description: every concern is framed by at least one viewpoint, and every view's components exist in the system model | not as written: the check resolves the links, the kernel holds the framing, and views here name no components |
| #14 | `prismpm check` validates the risk chain | links only; the kernel holds that each risk has a control |
| #15 | `prismpm check` validates each chain, and each measure runs in `just vv` | links only; the kernel holds that each requirement has a measure, Q5 stays `Pending`, and `lexlean_verify` on `uor-atlas` waits |

Whether those three lines should be reworded or the check strengthened is a decision for the lead:
PrismPM owns the check, and this repository's theorems already hold what it does not.

## Reproducing

```sh
just vv                                                  # inside the image of prismpm.lock, with prismpm check
prismpm check                                            # the same check alone
cargo test -p repo-model --test model_check              # MC-01 to MC-03 against the rows
lexlean verify                                           # the kernel theorems of the second table
```

## Not met

- **This is the development image.** No PrismPM SDK release holds these rules yet, so #4's release binding
  can still move them.
- **The gateway's model is not checked here.** `gateway/` has its own `prismpm.toml`, and CI runs
  `prismpm build` and `prismpm verify` there with the PrismPM of `.github/prismpm.rev`.
