Feature: Generated gateway

  Issue #7 generates the host adapter and the gateway binary from the model: a PrismPM server application over one LexLean production root that holds the MCP protocol core and the tool layer, a binding crate that implements the ten host effects, and the acceptance vectors that exercise each effect.

  @GW-01 @build
  Scenario: The gateway declares the ten effects of src/Effects.lex.tex, in order, followed by one frame effect: the host protocol of gateway/src/Application.lex.tex lists the names E1 to E10 that Effects.lex.tex declares at the tags 0 to 9 and then rate.calls at the tag 10, which no core can request, and a test holds the first ten equal to the model and names the eleventh.
    Given gateway/src/Application.lex.tex and src/Effects.lex.tex
    When the effect names and their positions are read from both
    Then the two lists are equal, ten long, with no name twice

  @GW-02 @build
  Scenario: Each effect of the gateway has a typed request, a typed result and a closed list of failures in PrismPM's field kinds, as GATEWAY.md tabulates them: an engine error travels in the result's class and result fields, a host fault is one of the declared failures, and no field or failure is declared that the table does not name.
    Given the host protocol of gateway/src/Application.lex.tex and the table of GATEWAY.md
    When the request, result and failures of each effect are read from both
    Then they are equal, every kind is one of U32, U64, Flag, Bytes and Text, and the bounds follow the owner values

  @GW-03 @build
  Scenario: The core of the gateway is one LexLean production root, LexLeanMcpGateway.Gateway.gatewayRun, over the sources of UOR-Foundation/MCP at the commit of .github/mcp.rev (the provider effects of MCP#39) and the tool layer, whose functions take the arguments and the effect outcomes of the turn and return a typed outcome: gateway/lexlean.toml has those two source roots, the root declares only the allocation and overflow effects, and the committed export request names the root and its package.
    Given gateway/lexlean.toml, gateway/src/Gateway.lex.tex, gateway/generated/request.json and .github/mcp.rev
    When the project, the root and the request are read
    Then the project has the two roots, the root is a production root with no recursion effect, and the pin is a full commit

  @GW-04 @build
  Scenario: Every effect is exercised by an acceptance vector of the gateway: the transcripts of the declaration request each of E1 to E10, each from a vector whose tool call is one of gateway/vectors/exchanges.json, and the vectors that request no effect are the invalid, unknown and listing calls.
    Given the transcripts of gateway/src/Application.lex.tex and gateway/vectors/exchanges.json
    When the effects of every transcript step are collected
    Then the set is exactly the ten effects, and each vector is in the file that generated it

  @GW-05 @build
  Scenario: No effect outside E1 to E10 can be encoded by the core: the model's decoder refuses the tag of the frame effect and every other tag past the set, the generated tables mark exactly the tags 0 to 9 as the core's (the Request sum decodes eleven tags and refuses every other with an undeclared effect), and negative tests of the binding crate assert the tag 10 is not the core's and the first and the largest tag past the eleven are refused.
    Given the binding crate tests of gateway/bindings/gateway-binding
    When the tests that name the closed set are listed
    Then the closed-set tests exist, each is run by the gateway job with a count check and none is ignored without that reason

  @GW-06 @build
  Scenario: Tool names and MCP method names occur nowhere in host adapter source (Q6): the names of the tools of gateway/src and the methods of the MCP sources at the pin do not occur in gateway/bindings, and, in the gateway job, not in the generated host-api and host crates either, except the one method the application declares as the counted value of its frame section, which the generated serving surface carries.
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

  @GW-10 @build
  Scenario: The rate counter is a frame effect and not a constant: the declaration has the effect rate.calls (request: counted, result: calls), the serving frame section tool_calls takes the field calls of its result, the adapter and not the core requests it once for every request that reaches the core, EFFECTS.md records that it is outside the ten effects the core can encode, and a test holds that the model's decoder refuses its tag.
    Given the host protocol of gateway/src/Application.lex.tex, EFFECTS.md and the decoder of src/Effects.lex.tex
    When the frame effect, its section and the tag 10 are read
    Then the section takes its field from the effect, the effect is the eleventh and the decoder refuses its tag

  @GW-11 @build
  Scenario: The MCP modules laid out under gateway/mcp are those of the pin: every laid-out source file and lexicon file equals the file of UOR-Foundation/MCP at .github/mcp.rev byte for byte, and the lexicon package mcp.core has in gateway/lexlean.lock the tree hash that MCP's lexlean.lock at the pin records for it, which restates AC-03 for a gateway that cannot carry the fixture core's digest.
    Given gateway/mcp-identities.sh, the two lexlean.lock files and the MCP tree at the pin
    When the laid-out files and the lock entry of mcp.core are compared with the pin
    Then every file is equal and the tree hashes of mcp.core are equal

  @GW-12 @build
  Scenario: The gateway passes the pinned server conformance suite of UOR-Foundation/MCP with its own exact waiting list (AC-03 part b): the gateway job serves the built gateway-host on loopback and runs the server-suite.mjs and the schema-oracle.mjs of the MCP checkout at .github/mcp.rev (reused, never copied) over it, against the suite commit that MCP pins; gateway/waiting.json names every scenario that does not pass, each with the lexlean-mcp issue or pull request that would deliver it, and the job passes only when the failing scenarios equal that list (an unlisted failure, a stale entry and an entry without an issue each fail it); the lifecycle, transport, header, DNS-rebinding, caching and schema scenarios are not listed and must pass, and no fixture tool is added to the catalogue; GATEWAY-CONFORMANCE.md records the run per scenario with the reason for every waiting entry and the planted defects; a test that needs node or the installed suite is ignored in the workspace gate and the job runs it with --ignored and a count check (the pattern of MCP CF-08).
    Given gateway/waiting.json, the gateway job of .github/workflows/lexlean.yml and GATEWAY-CONFORMANCE.md
    When the waiting list, the job steps and the record are read, and the verdict of the MCP oracle is run over the list
    Then the job serves the gateway and runs MCP's oracle at the pin with the list, every entry names an issue or a pull request, a stale entry and an unlisted failure are rejected, and the ignored tests run with a count check

  @GW-13 @build
  Scenario: The adapter refuses a foreign Origin by itself, independently of the core's own check: over a real socket the generated served adapter answers each origin of a pinned foreign corpus with 403 and the body OriginForbidden, which is the adapter's word and not the core's, and the core is never called; a request with an allowed origin reaches the core; the declaration's origins are exactly http and https over localhost, 127.0.0.1 and [::1] with any port; and GATEWAY.md records the planted widened allow-list, the failing command and the diagnostic.
    Given the generated served adapter of gateway-host, the real core wrapped in a call counter, and the origins of the declaration
    When each foreign origin of the corpus, an allowed origin and the declared list are put to the adapter over a real socket
    Then each foreign origin is answered 403 OriginForbidden without a call of the core, the allowed one reaches the core, and the declared list is exactly the six loopback origins
