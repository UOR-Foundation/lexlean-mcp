Feature: The AC-07 fixture register

  Issue #10 asks that 273 of LexLean's negative fixtures give the CLI's command result through the tools of F4 to F7 and #11. The register is the checked table those tools start from: the fixtures of the pinned LexLean commit, each classified once, with the expected command result and the digests that bind it to the pinned tree. Nothing here runs a LexLean command.

  @FX-01 @some-true
  Scenario: LexLean v0.4.0 at `e0b74b8` (commit e0b74b81e6f49c83596a957d232c8383a5db1a25) holds in tests/negative the git tree f5a43991dec0c604c8647c5dd050f183a16832a6 with 298 fixtures, of which 256 run the command check, 1 build, 1 migrate, 32 verify, 4 lock and 4 production.
    Given the cited authority A-LEXLEAN-NEGATIVE and .github/lexlean.rev
    When the pinned commit, the pinned tree and the commands of the rows are read
    Then the commit is the one lexlean.rev pins, the trees of the rows make the pinned tree, and the counts by command are the ones the statement gives

  @FX-02 @some-true
  Scenario: The expected result of each fixture, expected/command.json, is the canonical command result (lexlean/command-result/1) that the LexLean CLI of that commit gives for the project of the fixture, as the check-fixtures gate of LexLean decides; this repository records the results and runs no command.
    Given the vendored expected/command.json of every fixture and the ledger claim AUTH-08
    When each is read as a command result
    Then each carries the command-result and diagnostic schema tags, its success agrees with its exit status, and the claim cites A-LEXLEAN-NEGATIVE

  @FX-03 @build
  Scenario: The files vendored under crates/negative-fixtures/lexlean are the case.toml and the expected files of the pinned tests/negative: each has the SHA-256 the register records, the git trees recomputed from the vendored files and the recorded trees of the project directories are the recorded trees of the fixtures and the pinned tree of tests/negative, and the checksum of the authority covers every vendored file.
    Given the vendored files, the register and the authority checksum
    When the digests and the git trees are recomputed from the bytes
    Then every digest and every tree equals the recorded one, the root is the pinned tree, and the manifest checksum is the authority's

  @FX-04 @build
  Scenario: Every row of the register is what the vendored bytes give: the command of case.toml, the exit status and the diagnostic codes of the canonical command result, and the digests of both files, with the exit status of case.toml equal to that of the result.
    Given the register and the vendored case.toml and command.json of every fixture
    When each row is derived again from the bytes
    Then the derived row equals the registered row

  @FX-05 @build
  Scenario: Every fixture of the pinned tree is classified exactly once: no name is listed twice, the register and the vendored directories name the same fixtures, and a fixture is F4 when it runs check, F7 when it runs build, #11 when it runs verify, and excluded when issue #10 leaves it out.
    Given the register and the vendored directories
    When the names and the classes are read
    Then no name repeats, the names are the vendored ones, and each class is the one the command and the exclusions give

  @FX-06 @build
  Scenario: The register has 298 fixtures: 252 for F4, 1 for F7, 32 for #11 and 13 excluded, so 285 reach a tool; every sum is recomputed from the rows, so a hand edit of a row or of a sum fails.
    Given the register
    When the fixtures are counted by class
    Then the sums are the ones the statement gives and the classes add up to the total

  @FX-07 @build
  Scenario: The excluded fixtures are the twelve that issue #10 names, each with the reason it gives, and no other: 2 for CLI argument parsing, 2 for symlinks, 4 for setup and 4 for being outside the product.
    Given the register and the exclusion list of issue #10
    When the excluded rows are compared with the list
    Then each named fixture is excluded with its reason and no other fixture is excluded

  @FX-08 @build
  Scenario: Each gate of the register rejects its planted defect: a fixture reclassified, a digest altered, a vendored byte altered or removed, an exclusion without a reason, an exclusion the issue does not make, a name listed twice and a fixture missing; FIXTURES.md records the command, the diagnostic and the restore of each.
    Given the register and the vendored files, and a copy of each with one defect planted
    When every gate reads the copy
    Then the gate named for the defect reports it, and the unplanted register passes every gate
