Feature: The task host of the gateway

  Slice 4 of issue #11: the gateway moves to the MCP sources that serve tasks, and its host answers the entries that the protocol core asks for beside the rate counter --- the time, a fresh identifier, and the table of tasks --- and keeps the table the core hands it. A client that declares the tasks extension is answered lexlean_verify with a task, which it polls with tasks/get and cancels with tasks/cancel (TASKHOST.md, "The mapping").

  @TH-01 @build
  Scenario: The gateway declares the entries that the protocol core needs to serve tasks at the pin of MCP: after the ten effects of src/Effects.lex.tex and the frame effect rate.calls at the tag 10, the declaration lists the frame effects clock.now (result ms and iso), task.id (result id) and task.table (result table) at the tags 11 to 13, and the core effect task.store (request table, no result) at the tag 14, which the provider information of the core names as taskStoreTag; the serving frame sections tool_calls, now_ms, now, fresh_task_id and task_table take the fields calls, ms, iso, id and table of those effects, in the order that the frame of the core reads them, and the declaration states the maximums of MCP's own conformance declaration for them.
    Given gateway/vectors/generate.py, gateway/src/Gateway.lex.tex and the declaration it renders
    When the effects, the frame sections and the provider information are read
    Then the tags are 11 to 14, the sections are the five of the frame in order, and taskStoreTag is the tag of task.store

  @TH-02 @build
  Scenario: clock.now answers a millisecond counter that never goes back and the same instant as ISO 8601 text in UTC with three digits of milliseconds: the text of 0 is 1970-01-01T00:00:00.000Z, of 1700000000123 is 2023-11-14T22:13:20.123Z, of 951782400000 is 2000-02-29T00:00:00.000Z and of 1709164799999 is 2024-02-28T23:59:59.999Z, and the text of a later answer is the text of the start of the host plus the difference of the counters.
    Given a host asked for the time twice, and the text of four known instants
    When clock.now is answered
    Then the counter did not go back, the texts of the known instants are exact and the two answers agree on the interval between them

  @TH-03 @build
  Scenario: task.id answers 32 lowercase hexadecimal digits, a different value at every call and at every host, made from the operating system's source of randomness and not from the clock or from a counter, so that no client can guess the identifier of a task it did not create.
    Given a thousand identifiers from one host and a hundred from each of ten hosts
    When task.id is answered for each
    Then each has the 32 digits, no two are equal, and none is a function of the counter of the clock

  @TH-04 @build
  Scenario: task.table answers the bytes that the last task.store gave and nothing before any: a store replaces the whole table, the host hands back exactly what it was given byte for byte, bytes of every value and a table of the 524296 bytes that the declaration allows included, and a store of an empty table empties it.
    Given stores of an empty table, of bytes of every value, of the largest table and of an empty table again
    When task.table is asked after each store
    Then each answer is the last table stored, byte for byte

  @TH-05 @build
  Scenario: The production core serves the vectors at the pin of MCP with the new frame: every request of gateway/vectors/exchanges.json is answered with the response recorded for it by the core of this commit, the adapter asks for the three frame effects of the time, the identifier and the table once for each request that reaches the core, and an exchange that creates or touches no task asks the core for no task.store.
    Given the production core with the real host, a listing and a tool call that creates no task
    When each is served
    Then the response is the usual one, the effects asked by the core are the usual ones and none of them is task.store

  @TH-06 @build
  Scenario: Each gate of the task host rejects its planted defect: the text of the time disagreeing with its counter, an identifier made from the counter, a store that appends to the table instead of replacing it, and a provider that names the wrong tag for the store; TASKHOST.md records the command, the diagnostic and the restore of each.
    Given the host and the declaration with one defect planted at a time
    When the tests of TH-01 to TH-04 run
    Then each defect makes the test that answers it fail, with the diagnostic TASKHOST.md records

  @TH-07 @build
  Scenario: lexlean_verify called by a client that declares the tasks extension creates a task: the core asks verify.start as before, and when the host has started the verification the result is a task handle whose taskId is the task name the verifier gave, in the status working, with a time to live of 86400000 ms and a poll interval of 5000 ms, and the core asks task.store with a table that holds the task as working; a client that does not declare the extension is answered as before, with the task name in the result of the call and no task.store, and a verification that the host refuses to start creates no task.
    Given a gateway whose store holds a project, a client that declares the tasks extension and one that does not, and a project the host refuses
    When lexlean_verify is called by each
    Then the declaring client is given a working task named by the verifier with the stated time to live and poll interval and the table holds it, the other client is answered as before with no task.store, and the refused verification makes no task

  @TH-08 @build
  Scenario: tasks/get on a task the table holds as working asks verify.control once for its status and settles the entry from the answer: queued and running leave it working with no notification, verified and failed complete it with the result of the tool, cancelled cancels it, and a verifier that no longer knows the task fails it with the code -32603; each answer about a task is followed by task.store with the table, and a notification of the new status is streamed before the response only when the task changed; a task that is already terminal is answered from the table without asking verify.control, and a task the table does not hold is the protocol's unknown task.
    Given a task in the table and a verifier that runs, has verified, has failed, or no longer knows the task
    When the client polls it with tasks/get
    Then the entry is working, completed with the result of the tool, or failed with -32603 as the verifier says, a terminal task is answered without asking the verifier, and a handle the table lacks is the unknown task

  @TH-09 @build
  Scenario: tasks/cancel on a working task asks verify.control to cancel it, answers with the empty acknowledgement, streams a notification of the status cancelled and stores the task as cancelled; asked again, or on a task that is already terminal, it is answered from the table without asking verify.control, and on a task the table does not hold it is the protocol's unknown task.
    Given a working task in the table
    When the client cancels it with tasks/cancel, and cancels it again
    Then the verifier cancels it once, the table holds it as cancelled, one notification of that status is streamed, the second cancel asks nothing, and a handle the table lacks is the unknown task
