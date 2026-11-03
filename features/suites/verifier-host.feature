Feature: The verifier host

  The third service of issue #17: the process of each verification is started, ended and swept where the verifier runs, because a process ID and a session mean something only in their own container.

  @VS-01 @build
  Scenario: The process of each task is started where the verifier runs: gateway-verifier-host answers on the Unix socket LEXLEAN_MCP_VERIFIER_HOST with mode 0600 for the store LEXLEAN_MCP_STORE, starts LEXLEAN_MCP_VERIFIER or gateway-verifier beside it, says whether it has a program to start, its crate links no compiler, no generated package, no host and no signer, and the host starts no process, kills none and sweeps none of its own.
    Given the program gateway-verifier-host with a store and a program, and the source of the host
    When it is asked whether it is ready and given a task
    Then it answers on a socket of mode 0600, and the host has no process of its own to start or kill

  @VS-02 @build
  Scenario: One connection is one task in one process of its own: the process gets the job, the width and the memory it is asked for and nothing of the key, each line it writes reaches the host as it wrote it, ended is the last line, after the process has gone, with the tail of its standard error, and a cancel of one task does not touch another.
    Given the service on a socket and a stand-in for the program
    When tasks run through it
    Then each process has what it was given, its lines arrive unchanged and ended comes last

  @VS-03 @build
  Scenario: The service ends what it started: a cancel reaches the process and its answer comes back, kill ends a process that ignores everything, the end of the connection ends the input of the process, a process that ignores that or does not end after its verdict is killed after the grace, what a lost process leaves in its session is killed, and a child that holds the pipe does not hold the end of the task.
    Given processes that answer, hang, linger, die and leave children behind
    When the host cancels, kills or goes away
    Then each process and its session are gone, and ended still arrives

  @VS-04 @build
  Scenario: Every failure of the verifier host is typed and has a negative test: each refusal travels by its name, a directory that is not below the store after links are followed starts no process, a program that cannot start is refused, a request that is not one or is over MAX_REQUEST_BYTES is refused and the next is served, no answer at the socket and an answer that is not one are Lost, a service lost during a task ends its events without ended and each line already buffered still arrives, and each constructor of StartError stops the program.
    Given requests, directories, programs and sockets spoiled in each way
    When each is asked
    Then each gives its own typed failure and the next request is served

  @VS-05 @build
  Scenario: Each gate of the verifier host rejects its planted defect: the socket left open to other users, a directory outside the store admitted, the key left in the environment of the process, kill ignored, the session not swept, a process kept after the host has gone or after its verdict, a refusal name that comes back as another, the bound on a request removed, the first lines of a task lost in a buffer, and the host starting a process itself; VERIFIER.md records the command, the diagnostic and the restore of each.
    Given the verifier host with one defect planted at a time
    When the tests of VS-01 to VS-04 run
    Then each defect makes the test that answers it fail, with the diagnostic VERIFIER.md records
