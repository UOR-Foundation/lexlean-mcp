Feature: The engine host

  The second service of issue #17: the four Engine calls in a program of their own, reached through a socket, with no call left in the host.

  @EH-01 @build
  Scenario: The engine host is a program of its own and the only maker of the four Engine calls: gateway-engine-host answers on the Unix socket LEXLEAN_MCP_ENGINE with mode 0600 for the store LEXLEAN_MCP_STORE, the crate that holds it links no generated package, no host and no signer, and the host makes those calls through that socket alone, so a host without one answers engine-unavailable.
    Given the program gateway-engine-host with a store, and a host configured with no engine host
    When the program is asked for a check and the host is asked for each Engine effect
    Then the program answers on a socket of mode 0600, and the host answers engine-unavailable

  @EH-02 @build
  Scenario: An answer through the socket is the answer of the Engine: for check, snapshot, format, format with check_only and build, and for each selection, the class and the bytes of the result equal those of the Engine on an equal project, and a refusal of the Engine is an answer with its class.
    Given the fixture project twice below a store, one served through the socket and one beside it
    When each call and each selection is made on both
    Then the class and the result bytes are equal, and a spoiled configuration is an answer of class 2

  @EH-03 @build
  Scenario: Every failure of the engine host is typed and has a negative test: each refusal and each call travels by its name, a directory that is not below the store after links are followed is refused, a request that is not one or is over MAX_REQUEST_BYTES is refused and the next is served, no answer at the socket, a service lost before it answers and an answer that is not one are Unavailable, a host whose engine host has gone answers engine-unavailable and judges the project first, and each constructor of StartError stops the program.
    Given directories outside the store, requests that are not ones, sockets that answer nonsense or nothing, and the program without each of its settings
    When each is asked
    Then each gives its own typed failure, the next request is served, and the program exits with status 2

  @EH-04 @build
  Scenario: Each gate of the engine host rejects its planted defect: the socket left open to other users, a directory outside the store admitted, a result changed on its way through the socket, a refusal name that comes back as another, the bound on a request removed, a class that is not an answer accepted, and the host making an Engine call itself; ENGINE-HOST.md records the command, the diagnostic and the restore of each.
    Given the engine host with one defect planted at a time
    When the tests of EH-01 to EH-03 run
    Then each defect makes the test that answers it fail, with the diagnostic ENGINE-HOST.md records
