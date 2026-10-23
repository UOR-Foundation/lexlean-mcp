Feature: Owner-set values

  Issue #19 lists the values that no source determines. The model binds each one to a proposal that OWNER-VALUES.md states with its evidence, so that no declaration depends on an unset value and the owner overrides a value in one place.

  @OV-01 @build
  Scenario: No model declaration depends on an unset owner value: the product version, the signing key and request-state-key custody and rotation, every parameter, the retention, the cache time to live, the listen port, the service level objectives and the profile set are bound in src/Product.lex.tex and src/OwnerValues.lex.tex, the ten risk likelihoods are Foundation.Sec.Likelihood constructors in src/Security.lex.tex, and no source applies the Unbound constructor of OwnerValue.
    Given the sources under src/ and the list of values issue #19 names
    When the sources are read for the Unbound constructor, the version, the ten risks and the proposal table
    Then no source applies Unbound, the version is bound, each of the ten risks has a likelihood and the table has a row for every value the issue names

  @OV-02 @build
  Scenario: OWNER-VALUES.md and the model state the same value for every owner-set value, in both directions: every row of the proposal table is bound in the model with the same value, every value the model binds has a row, and every row names its evidence, its decider, its status and what changes if it moves.
    Given OWNER-VALUES.md and the definitions of src/OwnerValues.lex.tex, src/Product.lex.tex and src/Security.lex.tex
    When each row is compared with the definition its key names and each definition with the rows
    Then every row equals its definition, every value definition has a row and no cell of a row is empty

  @OV-03 @build
  Scenario: Each bound value respects the constraint it follows from: draft.max_bytes and draft.max_files hold the largest example and stay within the limits of lexlean.toml, verify.concurrency times verify.memory_kib fits a 16 GiB host, verify.width is within the range of LexLean SPEC 22.11, listen.port is a registered-range port below the ephemeral range, and the product version equals the workspace package version.
    Given the bound values, lexlean.toml, Cargo.toml and the sizes of the largest example at the pinned LexLean revision
    When each value is held against the constraint it follows from
    Then every constraint holds
