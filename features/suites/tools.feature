Feature: Check, snapshot, format and build agree with the CLI

  Issue #10 asks that the tools of F4 to F7 and F11 add no meaning to LexLean: their result is the CLI's for the same project bytes (and, for lexlean_explain, the same code). The tools reach the Engine through the effects E3 to E6 of the gateway; the core turns the answer into a tool result; the tests run the production core with the real host beside the lexlean binary of the same commit.

  @TL-01 @build
  Scenario: The tools of F4 to F7 declare one schema each: lexlean_check, lexlean_snapshot, lexlean_format and lexlean_build take project (a non-empty string), files (strings) and all (a boolean), lexlean_format also check_only, and no other member; each declares an outputSchema, the lexlean/command-result/1 object for check, format and build and an object with a spec for snapshot; tools/list through the production core shows exactly these, and a call whose arguments break the schema, or that sets all together with files, is a protocol error and never a tool result.
    Given gateway/src/GatewayTools.lex.tex and the tools/list answer of the production core
    When the schemas of the four tools are read from the source and from the answer
    Then they are equal, the members are exactly the ones named, every tool declares an outputSchema, and a call that breaks a schema is refused as invalid arguments

  @TL-02 @build
  Scenario: An answer of the Engine is a tool result and never a protocol error: class 0 gives a result without isError whose structuredContent is the canonical command result, classes 1, 2 and 4 give isError true with the command result of the failure, the text block is the same JSON, a host failure (project-refused, engine-unavailable) gives isError true with its failure name, and bytes that are not a command result or whose exit_code differs from the class are a typed internal refusal of the core.
    Given the production core, the binding and a draft project
    When each class of answer, each host failure and each malformed outcome is made to reach the tool layer
    Then the result is the one the statement gives, and the malformed outcomes are refused with the typed internal error

  @TL-03 @build
  Scenario: AC-07: for every check fixture that issue #10 puts in F4 and for the build fixture of F7 in tests/negative of the LexLean commit of gateway/mcp/lexlean.rev, lexlean_check and lexlean_build give what lexlean --diagnostic-format json check and build give for the same project: the same exit code and class, the same diagnostics, the same bytes in the text block, and the same files in the project afterwards; the test runs the CLI and the tool, and the fixtures it runs are the F4 and F7 rows of the register plus the names the register lacks.
    Given the check and build fixtures of tests/negative at the pinned LexLean commit, the lexlean binary built at that commit, and the register
    When the CLI and the tool are run on a copy of each project
    Then the exit code, the diagnostics, the text and the files agree for every fixture, and the count is the register's plus the names it lacks

  @TL-04 @build
  Scenario: lexlean_snapshot on the same fixtures agrees with the CLI: a refused project gives the command result that lexlean check gives with command snapshot, an accepted one gives a semantic snapshot whose source_id and semantic_id are those lexlean check prints, and the bytes of the result are the canonical bytes of the Engine's snapshot.
    Given the same fixtures and lexlean check
    When lexlean_snapshot is called on each
    Then a refused project gives the check result with command snapshot, and an accepted one a snapshot with the identities the CLI prints

  @TL-05 @build
  Scenario: lexlean_format agrees with lexlean fmt: on the same fixtures check_only true gives the result of lexlean fmt --check and writes nothing, and check_only false gives the result of lexlean fmt and leaves the project with the same files, byte for byte.
    Given the same fixtures and lexlean fmt
    When lexlean_format is called with check_only true and false on each
    Then the results agree with lexlean fmt --check and lexlean fmt, and the files afterwards are equal

  @TL-06 @build
  Scenario: AC-04: each project of examples/ of the LexLean commit of gateway/mcp/lexlean.rev, with its configuration, lock, local packages and artifact sources, builds through lexlean_build to the tree of its expected/ directory byte for byte (the process-bound files by presence), and the build is the one lexlean build makes.
    Given the examples of the pinned LexLean commit and their expected trees
    When each is installed as a draft project and built through lexlean_build
    Then the build tree equals the expected tree and the tree lexlean build makes

  @TL-07 @build
  Scenario: AC-08: a draft with an atom outside the profile gives, through lexlean_check, an isError result with the diagnostic LLL1004 and its span (path, byte range, line and column), the same as the CLI gives.
    Given a draft with a word outside the profile
    When lexlean_check is called on it
    Then the result is an error with LLL1004 and a primary span equal to the CLI's

  @TL-08 @build
  Scenario: Q4: lexlean_check, lexlean_snapshot, lexlean_format and lexlean_build start no child process: the host process accounting (the waited-for children of /proc/self/stat and the live children of every thread) does not move across a call of each, in a test binary in which nothing else spawns, and the same instrument sees a child that a control spawns.
    Given the process accounting of the test process and a control that spawns a child
    When each of the four tools is called
    Then the accounting does not move across a tool and moves across the control

  @TL-09 @build
  Scenario: Each gate of the tools rejects its planted defect: a class mapped to the wrong exit code, a diagnostic dropped from a result, and a format whose bytes differ from the CLI's; TOOLS.md records the command, the diagnostic and the restore of each.
    Given TOOLS.md and the planted defects
    When the defects are read
    Then each names a command, a diagnostic and a restore, and the three kinds the statement names are among them

  @TL-10 @build
  Scenario: lexlean_explain (F11) is declared with one input, code (a string, any string), no other member, and an outputSchema that is the lexlean/command-result/1 object with command explain; tools/list through the production core shows it; a call to it asks the host for no effect, because an explanation is a pure function of the code that is compiled into the core, so src/Effects.lex.tex and the set E1 to E10 do not change; and a call that breaks the schema is refused by the core as an Invalid arguments error result before the tool runs.
    Given gateway/src/GatewayTools.lex.tex, src/Effects.lex.tex and the tools/list answer of the production core
    When the schema of lexlean_explain is read from the source and from the answer and the tool is called through the real host
    Then they are equal, the call asks for no effect, the effect set is unchanged, and a call that breaks the schema is refused as invalid arguments

  @TL-11 @build
  Scenario: The explanations compiled into the core are the rendering of the registry: gateway/src/Explain.lex.tex is what gateway/explain/generate.py renders from model/errors.toml of the LexLean commit of gateway/mcp/lexlean.rev, with one entry for every registered code and none other, the gateway job fails when the committed module is not that rendering, and the tool layer holds no explanation text of its own.
    Given gateway/src/Explain.lex.tex, gateway/explain/generate.py and model/errors.toml of the pinned LexLean commit
    When the module is rendered again from the registry
    Then it equals the committed module, holds one entry per registered code, and the tool layer holds no explanation text

  @TL-12 @build
  Scenario: For every code that the registry of the LexLean commit of gateway/mcp/lexlean.rev registers, lexlean_explain gives what lexlean --diagnostic-format json explain gives: a result without isError whose text is the CLI's standard output byte for byte and whose structuredContent is its parse; and for every string the registry does not register (the empty string, a code in another case, quotes, backslashes, control characters, non-ASCII, a code that begins with a dash, which the CLI reads after --) it gives the CLI's refusal: isError true, exit_code 2, the diagnostic LLC0001 that quotes the string, the same bytes; the codes are read from the registry and not typed, and their count is asserted.
    Given the registry and the lexlean binary built at the pinned LexLean commit
    When the CLI and lexlean_explain are run on every registered code and on a corpus of strings the registry does not register
    Then the text, the structured content, the error flag and the exit code agree for every one, and the count of codes is the registry's

  @TL-13 @build
  Scenario: Q8, the running clause: every diagnostic code that appears in the result of lexlean_check or lexlean_build on the fixtures of TL-03 resolves through lexlean_explain to the entry the CLI prints for it, and the test counts the distinct codes it resolved.
    Given the check and build fixtures of tests/negative at the pinned LexLean commit
    When the codes in the results of lexlean_check and lexlean_build are explained through lexlean_explain
    Then every distinct code resolves to the entry the CLI prints for it

  @TL-14 @build
  Scenario: Each gate of lexlean_explain rejects its planted defect: a wrong byte in an explanation, an unregistered code answered as registered, and an unknown code quoted with an escape that differs from the CLI's; TOOLS.md records the command, the diagnostic and the restore of each.
    Given TOOLS.md and the planted defects of lexlean_explain
    When the defects are read
    Then each names a command, a diagnostic and a restore, and the three kinds the statement names are among them

  @TL-15 @build
  Scenario: Q2, the running clause: each of the nine in-scope functions of crates/quality-map/map.toml is reached by a tool that tools/list of the production core shows and that runs, lexlean_check, lexlean_snapshot, lexlean_format, lexlean_build and lexlean_verify for the Engine calls, and lexlean_draft_assemble and lexlean_draft_read for sdk::assemble, sdk::write_module, sdk::read_module and sdk::declaration_spans, whose results equal what those functions give on the same text.
    Given the map of crates/quality-map/map.toml and the gateway tools
    When tools/list is read and each tool of an in-scope function is called
    Then every tool is listed and runs, and the four results of the sdk functions equal the functions' own

  @TL-16 @build
  Scenario: Every result of a tool that lists an output schema validates against it (R7): lexlean_check, lexlean_snapshot, lexlean_format, lexlean_build and lexlean_explain are called for a success, a refused selection (class 2) and a project over a limit (class 4), the structured content of each result is judged by a validator of the test that is independent of the core, a tool that lists an output schema and has no call fails the test, and the validator rejects a required member removed, an integer given as a string, a constant that differs, a member the schema does not list, a value that is not an array and a diagnostic without its message.
    Given the gateway with the tiny project and a project over a limit
    When tools/list is read and each tool that lists an output schema is called for a success and for two refusals
    Then the structured content of every result validates against the schema listed for its tool, the set of tools that list one is the set the test has calls for, and each planted defect in a result is rejected
