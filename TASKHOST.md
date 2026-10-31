# The task host

Issue #11, slice 4. The gateway serves `lexlean_verify` as an MCP task: a client that declares
the tasks extension (`io.modelcontextprotocol/tasks`) is answered with a task handle, and polls
or cancels it with `tasks/get` and `tasks/cancel`. A client that does not declare it is answered
as before, with the verifier's task name and state in the result of the call.

## What is claimed

| Row | Claim |
| --- | --- |
| `TH-01` | The declaration lists the four entries the core needs, at the tags 11 to 14, with the maximums of MCP's own conformance declaration, and the frame carries five sections after the body. |
| `TH-02` | `clock.now` answers a counter that never goes back and the same instant as ISO 8601 text in UTC. |
| `TH-03` | `task.id` answers 32 lowercase hexadecimal digits from the system's random source. |
| `TH-04` | `task.table` answers what the last `task.store` gave, byte for byte. |
| `TH-05` | The production core serves the recorded vectors at the pin of MCP with the new frame; an exchange that touches no task never asks for `task.store`. |
| `TH-06` | Each gate rejects its planted defect (below). |
| `TH-07` | `lexlean_verify` by a client that declares the tasks extension creates a task named by the verifier's task name; any other client is answered as before. |
| `TH-08` | `tasks/get` settles a working task from `verify.control`; a terminal task is answered from the table without asking the verifier. |
| `TH-09` | `tasks/cancel` cancels a working task through `verify.control` and stores it as cancelled; asked again it is answered from the table. |

## The entries

| Tag | Effect | Kind | Request | Result |
| --- | --- | --- | --- | --- |
| 10 | `rate.calls` | frame | `counted` | `calls` |
| 11 | `clock.now` | frame | none | `ms`, `iso` |
| 12 | `task.id` | frame | none | `id` |
| 13 | `task.table` | frame | none | `table` |
| 14 | `task.store` | core | `table` | none |

The core keeps no state. The adapter asks for the three frame effects once for each request that
reaches the core and puts the answers in the frame, in the order `tool_calls`, `now_ms`, `now`,
`fresh_task_id`, `task_table`. When a request changes the table, the core asks the host for
`task.store` with the whole new table (the tag is the `taskStoreTag` of `src/Gateway.lex.tex`),
and answers once the host has kept it. The host keeps the table in memory.

## The mapping

`lexlean_verify` with a declaring client: the core asks `verify.start` (as before). When the host
has started the verification, the task takes the verifier's task name as its handle, with a
time to live of 24 hours and a poll interval of 5000 ms, and the core stores it as `working`.

`tasks/get` and `tasks/cancel` on a task the table holds as `working` ask `verify.control` once
(status or cancel) and settle the entry from the answer: `queued` and `running` leave it
working, `verified` and `failed` complete it with the result of the tool, `cancelled` cancels it,
and a verifier that no longer knows the task fails it. A task that is already terminal is
answered from the table without asking the verifier. A handle the table does not hold (never made, expired, or
lost by a restart) is the protocol's unknown task.

MCP's `TaskServe` fixes the time to live and the poll interval (60 s and 1 s) and takes its
task from a scripted plan. The gateway therefore builds its entries in `src/TaskTools.lex.tex`
and uses `TaskServe` only to find, read and cancel a terminal entry.

## The vectors

Three vectors exercise the tasks: `tasks/get` of a task the verifier lost (the only transcript of
`task.store`), of a task the table lacks, and by a client that does not declare the extension.
The first is not served (`NOT_SERVED` of `gateway/vectors/generate.py`): its response carries the
time and the table its frame gives, and a served program asks its own host for both, so its
answer there is the unknown task. `prismpm verify` replays its frame through the binding, where
the bytes are the modeled ones; the served pass still covers the frame effects with every other
vector.

## Planted defects

A gate is evidence only after a planted defect shows that it can fail (AGENTS.md). Each defect below was applied to the committed file named, the command was run, and the file was restored. The unplanted tree passes every one of these commands. The first three commands run in `gateway/` against the local registry (`CARGO_HOME=gateway/registry/cargo-home`, `--locked --offline`); the fourth runs at the root.

| Defect | Command | Gate | Diagnostic | Restore |
| --- | --- | --- | --- | --- |
| the text of the time disagreeing with its counter: `Clock::at` renders the start of the host, not the start plus the counter (`src/taskhost.rs` of gateway-binding) | `cargo test -p gateway-binding --test taskhost` | `the_counter_never_goes_back_and_the_text_agrees_with_it_th_02` | the test fails: "the text agrees with the counter over the interval", left 0, right 1 | `git checkout -- gateway/bindings/gateway-binding/src/taskhost.rs`: the command passes again |
| an identifier made from the counter: `fresh_id` fills its last eight octets with the clock in milliseconds and returns (`src/taskhost.rs`) | `cargo test -p gateway-binding --test taskhost` | `an_identifier_is_32_lowercase_hexadecimal_digits_and_never_repeats_th_03`, `hosts_do_not_share_identifiers_and_none_is_made_from_the_clock_th_03` | the first fails: "0000000000000000000001a18885f75f came twice"; the second fails: "begins with the counter" | `git checkout -- gateway/bindings/gateway-binding/src/taskhost.rs`: the command passes again |
| a store that appends to the table instead of replacing it: `task_store` extends `self.tasks` (`src/lib.rs` of gateway-binding) | `cargo test -p gateway-binding --test taskhost` | `a_store_replaces_the_whole_table_and_an_empty_store_empties_it_th_04`, `the_table_is_empty_until_one_is_stored_and_then_it_is_what_was_stored_th_04`, `bytes_of_every_value_and_the_largest_table_come_back_unchanged_th_04` | all three fail on a table that is the earlier table followed by the later (for the second store of `[4]` after `[1,2,3]`: left `[1,2,3][4]`, right `[4]`) | `git checkout -- gateway/bindings/gateway-binding/src/lib.rs`: the command passes again |
| a provider that names the wrong tag for the store: `taskStoreTag := 13` in `gatewayInfo` (`gateway/src/Gateway.lex.tex`) | `cargo test -p repo-model --test taskhost` | `the_declaration_lists_the_entries_the_core_needs_at_the_tags_th_01` | the test fails: "taskStoreTag is the tag of task.store" | `git checkout -- gateway/src/Gateway.lex.tex`: the command passes again || a store that keeps nothing: `task_store` drops the table (`src/lib.rs` of gateway-binding) | `cargo test -p gateway-tool-tests --test taskhost` | the tests of TH-07 to TH-09 that read or poll a task: 8 of 11 (`th_07`, `th_08`, `th_09`) | each fails on the effects or the table: a created task is not in the table (`left: Some(0)`, `right: Some(1)`), and a poll or cancel asks no `verify.control` (`left: []`, `right: ["verify.control", "task.store"]`); the other three do not touch the table and pass | `git checkout -- gateway/bindings/gateway-binding/src/lib.rs`: the command passes again |

The mapping itself (`gateway/src/TaskTools.lex.tex`) has no planted defect recorded: a change there takes a `prismpm build` (about an hour) to apply. The tests of TH-07 to TH-09 take the verifier behind the host as their oracle, and `Verdict` (`gateway/tool-tests/tests/taskhost.rs`) makes it say verified, failed or lost.

## Not met

- The table is in the memory of the process. A restart empties it; the handles it held are then
  unknown tasks (invalid params), though the verifier's records may still exist.
- MCP's core asks for `task.store` after every answer about a task, changed or not. Two such
  requests at the same time each store the table they read, so the later store can drop the
  earlier one's change. The adapter serves one request at a time per connection, not across
  connections.
- `tasks/update` is the protocol's: the verifier never asks for input, so it only answers
  unknown task or acknowledges.
- A status notification is streamed only with the answer to a poll that changed the task. The
  gateway keeps no connection to push on, so a task that ends between polls is announced by the
  next poll, not when it ends.
- The nine `tasks-*` scenarios of `gateway/waiting.json` (filed under #11) were measured before
  the tasks were served, and their reasons still say the extension is missing. They also call
  fixture tools of the suite (`slow_compute`, `greet`, `confirm_delete`, `failing_job`,
  `test_tool_with_task`) that the catalogue does not have (decision recorded in #45), so they
  are expected to keep failing. This slice did not run the suite: step 7n of CI is the
  measurement, and a scenario that now passes must leave the list.
