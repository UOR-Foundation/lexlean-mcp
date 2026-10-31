# The verifier process

Issue #11 asks for one verifier process per task, "so cancelling one task cannot end another".
This file records slice 3a: each verification runs in a process of its own, `gateway-verifier`,
and the host queues the tasks, cancels one without touching another, and cleans up after a
process it has lost. Registered as VP-01 to VP-06. The signer and the signed record are
SG-01 to SG-13 (SIGNING.md); the tasks-extension mapping is not here (see "Not met").

The code is `gateway/bindings/gateway-binding/src/{child,verifier,sweep}.rs` and
`src/bin/gateway-verifier.rs`. The tests are `tests/verifier_program.rs` (the real program),
`tests/verifier_tasks.rs` (the host, with a script that stands in for the program,
`tests/common/fake-verifier.sh`) and `crates/model/tests/verifier.rs`, which holds them against
the rows and this file.

## What is claimed

| ID | Claim |
| --- | --- |
| VP-01 | `gateway-verifier` speaks lines of JSON: it leads a session of its own and says so, reads one job line, prints one verdict (`verified`, `failed` with the class and diagnostics of the engine, or `cancelled` with its groups), ends its groups on `cancel` and on the end of its input, and answers a line that is no job with class 5. |
| VP-02 | Each task runs in a process that has the width and the memory of the owner values and nothing of the key; a program that is not a file is `verifier-unavailable`, after the request is judged. |
| VP-03 | At most `verify.concurrency` tasks run at once; the others wait, in order, and none fails for waiting; a verified task is signed and recorded by the host. |
| VP-04 | A cancel ends one task: a queued one never starts, a running one reports its groups and leaves no record, a verdict that races the cancel leaves no record, and a process that ignores the cancel is killed with its session. |
| VP-05 | A lost verifier fails its task as `verifier-lost` with no record, and everything left in its session is killed. |
| VP-06 | Each gate rejects its planted defect (below). |
| VP-07 | AC-07: the 32 `verify` fixtures give the command result of the CLI through `lexlean_verify` and `lexlean_verify_control`, with the class equal to its exit status, and leave no record. |

## The protocol

Lines of JSON. The host writes one job line and then, at most, `cancel`; the process writes its
events and one verdict.

```text
process -> host   {"event":"started","sid":<its process ID>}
host -> process   {"directory":"<project>","selection":{"kind":0|1|2,"files":["..."]}}
process -> host   {"event":"verified","attestation_id","build_id","source_id","semantic_id","modules","root"}
                  {"event":"failed","class":<ErrorClass>,"result":<diagnostics of the engine>}
                  {"event":"cancelled","ended":<groups>,"failed":<groups>}
host -> process   cancel
```

The end of the process's input before its verdict means the host is gone: it ends its groups and
exits with status 1, without a verdict. The host starts the process with `LEXLEAN_VERIFY_WIDTH`
and `LEXLEAN_VERIFY_MEMORY_KIB` from `verify.width` and `verify.memory_kib`, and without
`LEXLEAN_MCP_SIGNING_KEY`; the key is used only by the host, in `conclude`.

## The host

- **Slots.** `verify.concurrency` (default 1, `Config::with_verify_concurrency`) processes run
  at once; `start` returns `Running` or `Queued`.
- **One lock.** A verdict becomes a state, and a verified verdict becomes a record, in one place
  (`settle`), under the lock that `cancel` takes. A task being cancelled takes no record, whatever
  its process concluded: it is cancelled or recorded and never both. This closes the cancel race
  that SIGNING.md recorded as not met.
- **Cancel.** A queued task leaves the queue. A running one is sent `cancel` and the host waits for
  the verdict, for at most the grace (`CANCEL_GRACE`, 30 s); then it kills the process and its
  session and reports the processes it killed as groups ended. `groups_ended` and `groups_failed`
  are what the process reported.
- **Lost.** A process that ends without a verdict (exit, `SIGKILL`, the OOM killer) fails its task
  with class 5 and `{"failure":"verifier-lost","log":<the tail of its standard error>}`.
- **Where the verifier is.** `LEXLEAN_MCP_VERIFIER`, else `gateway-verifier` beside the running
  executable. Without a file there, `verify.start` is `verifier-unavailable`.

## Why the session

The design on #11 swept the leftovers of a lost verifier "by an environment marker". That cannot
work: LexLean starts every child of a verification (`lake`, `lean`) with `env_clear()`
(`verify/child.rs`), so a marker in the environment never reaches them, and `end_live_children`
documents `SIGKILL` of the host as the one case it does not cover. What the children do keep is
their session: the program calls `setsid` first, a session outlives its leader and is inherited by
every descendant, and `/proc/<pid>/stat` lists it. The host kills every process whose session is
the one the process said it leads, after the process is gone. `rustix` (already in the lock, for
`tempfile`) gives the two calls without `unsafe`; the sweep reads `/proc` and is Linux only, as are
the platforms.

A second finding came from the first test of it: a child that outlived its verifier also held the
verifier's output pipe open, so a host that waited for the end of the output waited for the child.
The host asks the process whether it is alive, and not the pipe.

## Not met

- **The signer is in the gateway process.** The verifier process no longer holds the key, but
  OWNER-VALUES and #17 plan a signer service of its own; the `Signer` module can move behind a
  process boundary and this slice does not do it.
- **`gateway-verifier` must be next to the host.** CI step 7k builds only `gateway-host`; the
  images of #17 must ship both programs. Without it `lexlean_verify` is `verifier-unavailable`,
  and the conformance suite does not call it.
- **AC-05 is not met.** Each example verifying, with its observed axioms equal to the committed
  records, is not tested. The mapping to MCP's tasks is in TASKHOST.md.
- **The queue is in memory.** A task of an earlier host is unknown after a restart; its process
  ends itself when the host's end of its input closes.
- **Q3, R1, R4 and R8 stay `Pending`** in the model; no module of `src/` changes. Only the tiny
  fixture was verified through the process, with Lean, and one real cancel was made.

## The verify fixtures

A failed verdict carries the result the CLI prints, `lexlean/command-result/1`, made by the function
the CLI calls (`command_result_json`); the other tools answer the same way. VP-07 holds it to the CLI
on all 32 fixtures (`gateway/tool-tests/tests/equality.rs`, ignored, with `LEXLEAN_SOURCES`). Each
fixture runs in its own conditions, with nothing global: the verifier program is a small script that
sets the variables of the fixture and starts the real `gateway-verifier`. The 12 fixtures that carry
a `toolchain/` get an elan home that overlays their programs on the pinned toolchain, as LexLean's
own harness does (`fake_elan_home`); `PATH` does not reach the programs the engine starts.

The job `gateway / verify fixtures` runs it (CI.md), not the gateway job. The whole test takes about
55 minutes on a development host, about two thirds of it in five
certificate fixtures (6 to 9 minutes each).

## Planted defects

Each defect was planted in `gateway/bindings/gateway-binding/src/verifier.rs`, the command ran
(gateway workspace, offline, the registry's `CARGO_HOME`), and the file was restored with
`git checkout`; the unplanted tree passes the same command (11 passed).

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| the key left in the environment of the process: the `env_remove` of `LEXLEAN_MCP_SIGNING_KEY` is gone | `cargo test -p gateway-binding --test verifier_tasks` | 10 passed, 1 failed: `the_process_gets_the_settings_and_nothing_of_the_key_vp_02` finds the variable in what the process saw | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| a record made for a task that is being cancelled: `settle` concludes a `verified` verdict in phase `Cancelling` too | `cargo test -p gateway-binding --test verifier_tasks` | 10 passed, 1 failed: `a_process_that_verifies_while_the_cancel_is_pending_leaves_no_record_vp_04` finds the task Verified (2) where Cancelled (4) is required | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| the session of a lost verifier not killed: `watch` does not call `kill_session` | `cargo test -p gateway-binding --test verifier_tasks` | 10 passed, 1 failed after 5 s: `the_session_of_a_lost_verifier_is_swept_vp_05` finds the child that outlived its verifier still alive | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| the slots ignored so that every task starts at once: `pump` runs while the queue is not empty | `cargo test -p gateway-binding --test verifier_tasks` | 8 passed, 3 failed: `one_slot_runs_one_task_at_a_time_and_the_others_wait_vp_03`, `two_slots_run_two_tasks_at_once_vp_03` and `a_queued_task_leaves_the_queue_and_its_process_is_never_started_vp_04` find the second task Running where Queued is required | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |
| a verdict that carries only the diagnostics: `verify_result` of `engine.rs` renders the diagnostics and not the command result | `cargo build -p gateway-binding`, then `cargo test -p gateway-tool-tests --test equality -- --ignored vp_07` | `the_verify_fixtures_give_the_result_of_the_cli_vp_07` reports "the result is {"diagnostics":...}, the CLI printed {"artifacts":[],"command":"verify",...}" (measured on `axiom-policy-excess`) | `git checkout -- gateway/bindings/gateway-binding/src/engine.rs` and build again |
| a class that is not the exit status: `from_error` of `child.rs` adds one to the class | `cargo build -p gateway-binding`, then `cargo test -p gateway-tool-tests --test equality -- --ignored vp_07` | `the_verify_fixtures_give_the_result_of_the_cli_vp_07` reports "the class is 2, the CLI exited 1" (measured on `axiom-policy-excess`) | `git checkout -- gateway/bindings/gateway-binding/src/child.rs` and build again |
| a process that ignores cancel left running: `cancel` neither kills the process nor its session | `cargo test -p gateway-binding --test verifier_tasks` | 10 passed, 1 failed after 10 s: `a_process_that_ignores_the_cancel_is_killed_after_the_grace_vp_04` finds the process still alive | `git checkout gateway/bindings/gateway-binding/src/verifier.rs` |

## Reproducing

```sh
gateway/registry.sh                                      # the registry from gateway/Cargo.lock
cd gateway
export CARGO_HOME=$PWD/registry/cargo-home
cargo test -p gateway-binding --locked --offline --test verifier_program --test verifier_tasks
# a real verification through the process, and a real cancel (need the Lean toolchain; the gateway job runs both by count)
cargo test -p gateway-binding --locked --offline -- --ignored
cd .. && cargo test -p repo-model --test verifier        # VP-01 to VP-06 against the rows
```
