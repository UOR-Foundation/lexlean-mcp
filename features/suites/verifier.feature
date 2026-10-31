Feature: The verifier process

  Slice 3a of issue #11: each verification runs in a process of its own, and the host queues, cancels and cleans up after the processes. The signer and the signed record are SG-01 to SG-13; the tasks-extension mapping, the fixtures of AC-07 and the examples of AC-05 are not claimed here (VERIFIER.md, "Not met").

  @VP-01 @build
  Scenario: The verifier is a program that speaks lines of JSON: gateway-verifier becomes the leader of a session of its own and says so in a started event that names the session, reads one job line (the directory of the project and the selection) from its standard input, runs Engine::verify and prints one verdict --- verified with the IDs, the modules and the verified directory, failed with the class and the diagnostics of the engine, or cancelled with the groups it ended and those it could not end --- on a line of its standard output; a line cancel on its standard input ends every live child group of the process, the end of that input ends them and the process, and a job line that is not a job is a failed verdict of class 5.
    Given the program, a draft that does not verify, a line that is no job and a closed input
    When each is run
    Then the session is the process ID, the bad draft is failed with class 1 and its diagnostics, the bad line is failed with class 5 and a closed input ends the process with no verdict

  @VP-02 @build
  Scenario: Each task runs in a process of its own that does not hold the key: verify.start starts the program of the configuration (LEXLEAN_MCP_VERIFIER, else gateway-verifier beside the running executable) with the width and the memory of the owner values in LEXLEAN_VERIFY_WIDTH and LEXLEAN_VERIFY_MEMORY_KIB, without LEXLEAN_MCP_SIGNING_KEY in its environment and with nothing of the seed or of the path of the key file in its arguments, its environment or its input; when the configured program is not a file, verify.start fails with verifier-unavailable, starts no task and leaves nothing in the builds area.
    Given a host with a verifier program that records what it is given, and a host whose program is missing
    When a verification is started in each
    Then the program saw the two settings and nothing of the key, and the missing program is verifier-unavailable with nothing left in the builds area

  @VP-03 @build
  Scenario: The tasks run as many at a time as the slots of verify.concurrency allow and the others wait: with one slot the second and the third of three tasks are Queued while the first runs, none fails for waiting, they run in the order they were started and never two at once; with two slots two run at once; and a verification that the process reports verified is signed and recorded by the host with the facts the process printed, in the process of the host.
    Given tasks whose verifiers wait for a gate, with one slot and with two
    When three tasks are started and the gates are opened one at a time
    Then the waiting tasks are Queued, at most as many run as there are slots, all end Verified, and each has a signed record that validates

  @VP-04 @build
  Scenario: A cancel ends one task: a queued task leaves the queue and its process is never started; a running task is sent cancel, the answer carries the groups its process reported, the task ends Cancelled with no record and the other tasks are not touched; a process that reports verified while the cancel is pending leaves no record, because the record is made under the lock that the cancel takes; and a process that does not answer within the grace is killed, together with its session.
    Given a queued task, a running task, a process that verifies on cancel and a process that ignores cancel
    When each is cancelled
    Then each ends Cancelled with no record, the reported groups are returned, the other task still verifies and the process that ignored the cancel is gone

  @VP-05 @build
  Scenario: A verifier that is lost leaves nothing: a process that ends without a verdict, by exiting or by a signal, ends its task Failed with class 5 and the result {"failure":"verifier-lost"} with the tail of its standard error, no record is made, the other tasks are not touched, and every process left in its session --- children that outlived it --- is killed, because LexLean starts its own children with an empty environment, so no marker in the environment can find them.
    Given a process that exits, a process that kills itself and a process that leaves a child behind
    When each task is started beside a task that verifies
    Then each lost task is failed as verifier-lost with no record, the other task verifies and the orphan is gone

  @VP-06 @build
  Scenario: Each gate of the verifier process rejects its planted defect: the key left in the environment of the process, a record made for a task that is being cancelled, the session of a lost verifier not killed, the slots ignored so that every task starts at once, and a process that ignores cancel left running; VERIFIER.md records the command, the diagnostic and the restore of each.
    Given the host with one defect planted at a time
    When the tests of VP-02 to VP-05 run
    Then each defect makes the test that answers it fail, with the diagnostic VERIFIER.md records

  @VP-07 @build
  Scenario: The 32 verify fixtures of AC-07 give the command result of the CLI: for each negative fixture of class #11 in crates/negative-fixtures/register.toml, run in its own environment and, when it carries a toolchain directory, with an elan home that overlays those programs on the pinned toolchain, lexlean_verify and lexlean_verify_control end the task failed with the class equal to the exit status of lexlean verify, and a result equal to the lexlean/command-result/1 the CLI prints on the same bytes, and they leave no record.
    Given the 32 verify fixtures of LexLean at the pin, each in its own environment and toolchain
    When the CLI verifies one copy and the tool verifies another
    Then the class is the exit status, the result is the command result of the CLI, and no record is left
