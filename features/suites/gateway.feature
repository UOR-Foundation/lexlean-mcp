Feature: Generated gateway

  Issue #7 generates the host adapter and the gateway binary from the model: a PrismPM server application over one LexLean production root that holds the MCP protocol core and the tool layer, a binding crate that implements the ten host effects, and the acceptance vectors that exercise each effect.

  @GW-01 @build
  Scenario: The gateway declares the ten effects of src/Effects.lex.tex and no other: the host protocol of gateway/src/Application.lex.tex lists, in order, the names E1 to E10 that Effects.lex.tex declares, so that the tag of an effect is its position 0 to 9, and a test holds the two lists equal.
    Given gateway/src/Application.lex.tex and src/Effects.lex.tex
    When the effect names and their positions are read from both
    Then the two lists are equal, ten long, with no name twice

  @GW-02 @build
  Scenario: Each effect of the gateway has a typed request, a typed result and a closed list of failures in PrismPM's field kinds, as GATEWAY.md tabulates them: an engine error travels in the result's class and result fields, a host fault is one of the declared failures, and no field or failure is declared that the table does not name.
    Given the host protocol of gateway/src/Application.lex.tex and the table of GATEWAY.md
    When the request, result and failures of each effect are read from both
    Then they are equal, every kind is one of U32, U64, Flag, Bytes and Text, and the bounds follow the owner values

  @GW-03 @build
  Scenario: The core of the gateway is one LexLean production root, LexLeanMcpGateway.Gateway.gatewayRun, over the sources of UOR-Foundation/MCP at the commit of .github/mcp.rev and the tool layer: gateway/lexlean.toml has those two source roots, the root declares only the allocation and overflow effects, and the committed export request names the root and its package.
    Given gateway/lexlean.toml, gateway/src/Gateway.lex.tex, gateway/generated/request.json and .github/mcp.rev
    When the project, the root and the request are read
    Then the project has the two roots, the root is a production root with no recursion effect, and the pin is a full commit

  @GW-04 @build
  Scenario: Every effect is exercised by an acceptance vector of the gateway: the transcripts of the declaration request each of E1 to E10, each from a vector whose tool call is one of gateway/vectors/exchanges.json, and the vectors that request no effect are the invalid, unknown and listing calls.
    Given the transcripts of gateway/src/Application.lex.tex and gateway/vectors/exchanges.json
    When the effects of every transcript step are collected
    Then the set is exactly the ten effects, and each vector is in the file that generated it

  @GW-05 @build
  Scenario: No effect outside E1 to E10 can be encoded: the generated Request sum has ten variants and its decoder refuses every other tag with EffectUndeclared, which a negative test of the binding crate asserts for the first tag past the set and for the largest tag.
    Given the binding crate tests of gateway/bindings/gateway-binding
    When the tests that name the closed set are listed
    Then the closed-set tests exist, each is run by the gateway job with a count check and none is ignored without that reason

  @GW-06 @build
  Scenario: Tool names and MCP method names occur nowhere in host adapter source (Q6): the names of the tools of gateway/src and the methods of the MCP sources at the pin do not occur in gateway/bindings, and, in the gateway job, not in the generated host-api and host crates either.
    Given the tool names of gateway/src, the method names of gateway/mcp/src and the host adapter source
    When every name is searched for in every file of the adapter source
    Then none is found, and a planted tool name in an adapter file is found

  @GW-07 @build
  Scenario: The binding implements each of the ten host effects with a real effect and holds no dispatch: the store on the filesystem with refused keys, the four Engine calls, the verifier process of a task, the lexicon and the attestation reads, each with a test of its success and of each failure it declares.
    Given gateway/bindings/gateway-binding
    When the effect methods and the tests are listed
    Then each effect has a method and a test, and the binding has no match over a tag

  @GW-08 @build
  Scenario: prismpm build and prismpm verify accept the gateway: the build produces the core and the two host packages, the core package equals the committed export, and verify replays every vector through the binding and exercises all ten effects; GATEWAY.md records each planted defect, the failing command and the diagnostic.
    Given the gateway job of .github/workflows/lexlean.yml and GATEWAY.md
    When the job steps and the planted defects are read
    Then the job builds, compares the core and verifies, and each defect names a command and a diagnostic

  @GW-09 @build
  Scenario: The gateway builds without a published crate: gateway/registry.sh assembles a directory source for the gateway workspace from the pinned commits with cargo package, records the SHA-256 of each package it makes and compares it with a fresh package, no manifest has a path or Git dependency, deny.toml admits crates.io only, and CI.md states the release step that removes the replacement.
    Given gateway/registry.sh, gateway/Cargo.toml, deny.toml and CI.md
    When the script, the manifests and the document are read
    Then the script checks the digests, the manifests have only registry dependencies and CI.md names the release step

