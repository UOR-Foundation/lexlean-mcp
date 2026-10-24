Feature: Store, persistence and lifecycle

  Issue #13 asks for the store layout, the backup set, the drift checks and the rollout, rollback, upgrade and retirement of profiles. The real store needs the verify task and signing (#11), so this suite holds the model of them: the layout, the backup set, retention, the drift check, the lifecycle machine and AC-14 to AC-17 as theorems over a record-resolution model. `lexlean verify` checks the kernel side.

  @ST-01 @build
  Scenario: The store layout of lexlean-mcp/store/1 is a closed type of seven directories, profiles, drafts, builds, verifications, index, exports and keys, in that order: each has one distinct wire name, every directory is listed, every wire name parses back to its directory and an unknown name is refused.
    Given the store layout of src/Store.lex.tex and the seven directory names of issue #13
    When the Dir type, its wire names and the theorems about them are read
    Then the seven names are the issue's, in its order, and the model proves them distinct, complete and parsed back

  @ST-02 @build
  Scenario: The backup set is exactly verifications, keys and the drafts references: the predicate is membership of that set, every item that cannot be recomputed and does not expire is in it, nothing recomputable is in it, and the archive has exactly those three fields.
    Given the backup predicate and the Archive type of src/Store.lex.tex and the backup list of issue #13
    When the predicate, the set, the coverage laws and the archive fields are read
    Then the set is the issue's three items, the laws hold for every item and the archive has exactly those fields

  @ST-03 @build
  Scenario: Retention is a total function of the bound owner values: an attestation record never expires at any age, a draft record expires after store.draft_retention (30 days) and not before, and both durations are the values bound in src/OwnerValues.lex.tex.
    Given the retention functions of src/Store.lex.tex and the bound values of src/OwnerValues.lex.tex
    When the functions and the bound values are read
    Then an attestation never expires, a draft expires after 30 days and the model reads both from the owner values

  @ST-04 @build
  Scenario: The drift check is a total function from the stored profile record, the lock check result, the recomputed profile ID and the recomputed toolchain hashes to one of NoDrift, LockDrift, IdDrift and ToolchainDrift, deciding in that order of precedence: lock, then ID, then toolchain; all eight inputs of the decision are tabulated, and a recomputation that Identity refuses is IdDrift.
    Given the drift check of src/Store.lex.tex and the order the issue lists: lexlean lock --check, the recomputed profile ID, the toolchain hashes
    When the decision, its table and its theorems are read
    Then the table equals the one computed here from that order and every refused recomputation is IdDrift

  @ST-05 @build
  Scenario: The lifecycle is a state machine over the served revision, the served profile set, the previous release, the retired profiles and the records: rollout, rollback, upgrade and retirement keep every record, rollback restores the previous revision and served set, an upgrade serves the new profile and a retired profile is not served.
    Given the machine of src/Store.lex.tex
    When the events, the transition function and the transition theorems are read
    Then no event other than a write changes the records, and the four transition laws are theorems

  @ST-06 @build
  Scenario: AC-14 (model): the profile preimage determines its toolchain field for every input, so a new LexLean revision gives another preimage, and after an upgrade the earlier records still resolve and still validate; this is a claim about preimages, not about SHA-256.
    Given the recovery theorem over Identity.profileRT and the upgrade case of src/Store.lex.tex
    When the theorems and the concrete upgrade case are read
    Then the toolchain is recovered from the preimage, two revisions give two preimages and the earlier record resolves and validates

  @ST-07 @build
  Scenario: AC-15 and AC-17 (model): returning to the previous revision serves its profiles and its revision, a record written after the rollout still resolves after the rollback, and a release rollback resolves every record that resolved before it.
    Given the rollback theorems and the rollback case of src/Store.lex.tex
    When the theorems and the concrete rollback case are read
    Then the previous revision and profiles are served and both records resolve

  @ST-08 @build
  Scenario: AC-16 (model): after restore from a backup every backed-up record resolves as before, the keys and the draft references are restored, and re-verifying a restored record yields the attestation ID it is stored under because the ID is a function of the profile and the inputs; the signed attestation format stays abstract until #11.
    Given the backup and restore functions and the restore case of src/Store.lex.tex
    When the theorems and the concrete restore case are read
    Then every backed-up record resolves, the keys come back and every restored record validates

  @ST-09 @build
  Scenario: Each law of the store model rejects its planted defect: an attestation that expires, a drift check that reports ToolchainDrift before IdDrift, a rollback that drops a later record and a restore that omits keys each fail lexlean verify; STORE.md records the command, the diagnostic and the restore of each.
    Given STORE.md and the laws of src/Store.lex.tex
    When the planted-defect table is read
    Then four defects are recorded with a command, a diagnostic and a restore, each against a law that exists
