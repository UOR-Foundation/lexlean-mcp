Feature: The executed measures of the quality model

  Q2 asks that each in-scope function of LexLean's public API reach exactly one tool, and Q8 that every diagnostic code resolve through lexlean_explain. The tools do not exist yet (#9 to #12), so what is measured here is what stands without them: the public functions and the diagnostic registry of the pinned LexLean, recomputed from its vendored files, and the API-to-tool map and the explained codes checked against them. A measure is Executed in src/Quality.lex.tex only when the registered rows whose tests pass observe it.

  @QM-01 @some-true
  Scenario: LexLean release/v0.4 at 9796067 declares 33 public functions in the files of its stable API surface: 7 methods of Engine, 8 functions of lexlean::sdk, 11 of lexlean::api::production with the methods of its request, result and report, 5 free functions of lexlean::api and the 2 lifecycle functions of lexlean::verify; the vendored copies of crates/lexlean/src/api.rs, api/production.rs, sdk.rs and verify/lifeline.rs are the files of that commit, each with the git blob id the table records.
    Given the cited authority A-LEXLEAN-API, .github/lexlean.rev and the vendored surface files
    When the blob ids, the checksum and the public functions are recomputed from the bytes
    Then each file is the pinned blob, the manifest is the authority's checksum, the pin is the one lexlean.rev names, and the count of public functions is the one the statement gives

  @QM-02 @build
  Scenario: The API-to-tool map has one row for each of the 33 public functions the vendored sources declare, with the signature they declare, and no other row; the source declares every function SPEC 24.1, 24.6, 24.7 and 24.8 declare, and Engine and lexlean::sdk have exactly the methods and functions SPEC lists.
    Given map.toml and the vendored surface files
    When the public functions are derived again from the source text and compared with the rows
    Then the rows and the derived functions are the same set with the same signatures, and the functions SPEC 24 lists are all derived

  @QM-03 @build
  Scenario: Q2 is measured: every one of the 33 public functions reaches exactly one tool or states a reason no tool does, every tool named is one of the 14 that issues 8 to 12 define, and the 9 functions that reach a tool are the 9 in-scope functions the model of Q2 names (Engine check, snapshot, format, build and verify; sdk assemble, write_module, read_module and declaration_spans).
    Given map.toml and src/Quality.lex.tex
    When each row is read for its tool or its reason, and the in-scope functions are read from the description of Q2 in the model
    Then no function is unmapped, no row names two outcomes, every tool is defined by an issue, and the functions that reach a tool are the in-scope ones

  @QM-04 @some-true
  Scenario: LexLean release/v0.4 at 9796067 registers 86 diagnostic codes in model/errors.toml, each with a class, an exit status, a title and a statement, and generates ERRORS.md from the registry, one entry per code, which lexlean explain prints; the vendored copies are the files of that commit.
    Given the cited authority A-LEXLEAN-ERRORS and the vendored registry and entries
    When the blob ids and the checksum are recomputed from the bytes and the registry is counted
    Then each file is the pinned blob, the manifest is the authority's checksum, and the registry holds the 86 codes the statement gives

  @QM-05 @build
  Scenario: Every one of the 86 registered codes is explained: its row has a well formed code in its range, a class with its exit status, a title and a statement, rows are sorted and unique, and ERRORS.md holds one entry for each code, no other, equal to the text explain prints.
    Given the vendored registry and ERRORS.md
    When each row is checked and each entry is rendered from its row and compared
    Then every row is complete, the entries are the rows rendered, and none is missing or extra

  @QM-06 @build
  Scenario: Every diagnostic code that the 297 negative fixtures of AC-07 return is a registered code with an entry in ERRORS.md, so a code in a result resolves through explain.
    Given the fixture register and the vendored registry and entries
    When the codes of the command results of the fixtures are collected and looked up
    Then each returned code is registered and has an entry

  @QM-07 @build
  Scenario: The measures of Q2 and Q8 are Executed in src/Quality.lex.tex by QM-01, QM-02 and QM-03 (Q2) and by QM-04, QM-05 and QM-06 (Q8), whose tests pass; Q1 and Q3 to Q7 stay Pending, each naming the issue that executes it, the clause of each executed measure that waits for the tools stays in pendingClauses, and the counts the model states (9 functions, 86 codes) are those of the pinned files.
    Given the register, the test names of the workspace, src/Quality.lex.tex and the vendored files
    When the rows that observe the API-to-tool map and the explained codes and the statuses of the measures are read
    Then each cited row is registered with a test, Quality records Q2 and Q8 as its only Executed measures, the clauses that wait for the tools are recorded, and the same checks reject a measure flipped back to unexecuted, a cited row that is missing and a count that disagrees

  @QM-08 @build
  Scenario: Each gate of Q2 and Q8 rejects its planted defect: a function removed from the map, a tool name misspelled, a function with no tool and no reason, a function both mapped and not exposed, a function added or re-signed upstream, a vendored byte altered or removed, a pin moved without a new import, a code returned that the registry lacks, a code with no statement, an entry removed from ERRORS.md or altered, an exit status that disagrees with its class, rows out of order, Q2 or Q8 flipped back to Pending, a measure flipped to Executed without rows, a cited row missing and a count that disagrees; QUALITY.md records the command, the diagnostic and the restore of each.
    Given the map, the vendored files and the model, and a copy of each with one defect planted
    When every gate reads the copy
    Then each gate reports the defect it exists to catch, and reports nothing on the unplanted files
