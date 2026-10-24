Feature: Features and journeys

  Issue #18 declares the 13 features and 6 journeys of the product with the vocabulary of PrismPM#330, so that each feature is realized by a call and accepted by an acceptance row and each journey step names a declared feature. The kernel theorems of src/Features.lex.tex check the declarations; these scenarios check them once more from the source text, and hold the vocabulary copy and the provenance table.

  @FJ-01 @build
  Scenario: Thirteen features, F1 to F13, are declared as Foundation.Arch::Feature definitions in src/Features.lex.tex, numbered 1 to 13 in order, and the kernel theorem thirteenFeaturesInOrder holds.
    Given src/Features.lex.tex
    When its Foundation.Arch::Feature definitions, the table featureRows and the theorems are read
    Then there are 13 features whose statements begin F1 to F13, the table lists them in order, and the theorem is proved by decide

  @FJ-02 @build
  Scenario: Six journeys, J1 to J6, are declared as Foundation.Arch::Journey definitions in src/Features.lex.tex, numbered 1 to 6 in order, and the kernel theorem sixJourneysInOrder holds.
    Given src/Features.lex.tex
    When its Foundation.Arch::Journey definitions, the table journeyRows and the theorems are read
    Then there are 6 journeys named J1 to J6, the table lists them in order, and the theorem is proved by decide

  @FJ-03 @build
  Scenario: Every feature names at least one interface call, the rule of PP2014, and the kernel theorem everyFeatureHasACall holds.
    Given the feature definitions of src/Features.lex.tex
    When the calls field of each is read
    Then no feature has an empty calls list and the theorem is proved by decide

  @FJ-04 @build
  Scenario: Every feature names at least one acceptance row, the rule of PP2015, and the kernel theorem everyFeatureHasAnAcceptanceRow holds.
    Given the feature definitions of src/Features.lex.tex
    When the acceptance field of each is read
    Then no feature has an empty acceptance list and the theorem is proved by decide

  @FJ-05 @build
  Scenario: Every step of every journey names a declared feature, the rule of PP2016, and the kernel theorem everyJourneyStepNamesAFeature holds.
    Given the journey and feature definitions of src/Features.lex.tex
    When each step of each journey is resolved
    Then every step is the qualified name of a feature definition and the theorem is proved by decide

  @FJ-06 @build
  Scenario: Every journey actor is a stakeholder declared in src/Architecture.lex.tex, the rule of PP2017, and the kernel theorems everyJourneyActorIsAStakeholder and stakeholderRowsAreTheArchitectureStakeholders hold.
    Given the journey definitions of src/Features.lex.tex and the stakeholders of src/Architecture.lex.tex
    When the actor of each journey is resolved
    Then every actor is the qualified name of a stakeholder definition, the table of stakeholders is the architecture's, and the theorems are proved by decide

  @FJ-07 @build
  Scenario: Every call and every acceptance row of a feature names a definition of src/Features.lex.tex, the rule of PP2018, every such definition is used by a feature, and the kernel theorems everyCallNamesADefinition, everyAcceptanceRowNamesADefinition and featuresModelHolds hold.
    Given the feature definitions and the String definitions of src/Features.lex.tex
    When each call and each acceptance row is resolved and each String definition is looked up
    Then every one names a definition, every definition is exactly one call or acceptance row used by some feature, and the theorems are proved by decide

  @FJ-08 @some-true
  Scenario: PrismPM main at d11ace6f, the merge of PrismPM#330, declares Foundation.Arch::Feature with a statement, calls and acceptance, and Foundation.Arch::Journey with an actor and steps, and its prism.arch lexicon names them with the entries arch-feature, arch-journey and holo-normalized-features; prismpm check refuses a feature with no call (PP2014), a feature with no acceptance row (PP2015), a journey step naming no declared feature (PP2016), a journey actor that is no declared stakeholder (PP2017), and a malformed declaration or a call or acceptance row that names no definition (PP2018).
    Given the Foundation.Arch module and the prism.arch entries of this repository
    When they are read for the Feature and Journey structures and the three entries
    Then the structures have the fields PrismPM#330 defines and the entries denote them

  @FJ-09 @build
  Scenario: The prism.arch lexicon and Foundation.Arch of this repository are, byte for byte, the 143 files of PrismPM d11ace6f: the SHA-256 over their path-sorted manifest equals the checksum of the authority A-PRISMPM-ARCH.
    Given the 143 files of language/prism.arch and src/Foundation/Arch.lex.tex and the authority A-PRISMPM-ARCH
    When the manifest of their SHA-256 digests is hashed
    Then the digest equals the checksum of the authority, which names PrismPM d11ace6f

  @FJ-10 @build
  Scenario: FEATURES.md states one row for each of F1 to F13 and J1 to J6, in order, and every row names where it comes from: an issue of this repository, or needs owner when no issue states it.
    Given FEATURES.md and the declarations of src/Features.lex.tex
    When the provenance table is compared with the declared features and journeys
    Then every feature and journey has one row with a provenance cell that names an issue or needs owner
