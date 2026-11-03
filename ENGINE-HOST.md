# The engine host

Issue #17 deploys `gateway`, `engine-host`, `verifier`, `signer` and the telemetry collector as
services of their own. This file records the second of them, after the signer (SIGNING.md): the four
Engine calls, E3 to E6, move out of the gateway into a program, `gateway-engine-host`. Registered as
EH-01 to EH-04. It is a program, not yet a deployment (see "Not met").

The code is `gateway/bindings/gateway-engine-host` and the call in `Native::engine_call`
(`gateway/bindings/gateway-binding/src/lib.rs`). The tests are
`gateway/bindings/gateway-engine-host/tests/service.rs` (the program and the service, over a real
socket, with the real Engine), `gateway/bindings/gateway-binding/tests/effects.rs` (the host) and
`crates/model/tests/engine_host.rs`, which holds them against the rows and this file.

## What is claimed

| ID | Claim |
| --- | --- |
| EH-01 | `gateway-engine-host` is the only maker of the four calls: it answers on a socket of mode 0600 for one store, its crate links nothing generated, no host and no signer, and the host makes the calls through that socket alone. A host without one answers `engine-unavailable`. |
| EH-02 | An answer through the socket equals the answer of the Engine, class and bytes, for each call and each selection, and a refusal of the Engine is an answer with its class. |
| EH-03 | Every failure of the service is typed and has a negative test. |
| EH-04 | Each gate rejects its planted defect (below). |

## The protocol

One line of JSON in, one line of JSON out, one connection for each request.

```text
client -> service   {"directory":"<project>","call":"check"|"snapshot"|"format"|"format-check"|"build",
                     "selection":{"kind":0|1|2,"files":["..."]}|{"invalid":"<message>"}}
service -> client   {"class":<0|1|2|4>,"result":"<base64>"}
service -> client   {"refused":"request-refused"|"directory-refused"|"engine-unavailable"}
```

The socket is `LEXLEAN_MCP_ENGINE`, for the program and for the host, and the program serves the store
`LEXLEAN_MCP_STORE`. **Behaviour change for anyone running the host:** without a running
`gateway-engine-host` named by `LEXLEAN_MCP_ENGINE`, each Engine effect is `engine-unavailable`, after
the project is judged. No in-process fallback remains (AGENTS.md, Required workflow, step 3).

## Decisions

- **The gateway still makes the directory, and the service runs on a directory it is named.** Judging
  a project and materializing a draft stay where `verify.start` needs them too, so this slice moves
  the calls and nothing else. Both services therefore mount the store.
- **A directory outside the store is refused after links are followed.** The host has judged the
  project already. This is the second check, for a client of the socket that is not the host.
- **Every refusal reaches the host as `engine-unavailable`.** The host judged the request before it
  asked, so a refusal is a host and a service that do not agree, and no answer of the effect fits it
  better. The name travels so that a test can see which one it was.
- **Requests are answered one at a time**, as the host asks them: `Native` makes one effect at a time.
- **Only the sending has a time limit.** The call takes as long as the limits of LexLean let it, as it
  did in the host, and a lost service ends the connection.
- **The stack of a call is the one it had in the process of the core**, 512 MiB, on a thread of its
  own, so a panic of the compiler is `engine-unavailable` and the next request is served. No test
  drives that path: nothing makes the real compiler panic on demand.
- **The telemetry stays in the host**: the outcome and the latency are observed around the call, and
  the drift of a profile where the project is materialized (OBSERVABILITY.md).
- **The replay of the vectors reaches the program.** `gateway_verify` and `generate.py record` run it
  beside the program under test, so the vectors of E3 to E6 hold through the socket (CI.md).

## Planted defects

Each defect was planted in a copy of `gateway/bindings/gateway-engine-host` built as a workspace of its
own, with `lexlean` patched to the commit of `gateway/mcp/lexlean.rev`, and the file was restored from
the tree; the unplanted crate passes the same commands. The last row was planted in a copy of the
repository.

| Defect | Command | Diagnostic | Restore |
| --- | --- | --- | --- |
| the socket left open to other users: `bind` does not set the mode | `cargo test --test service` | `the_program_answers_on_a_socket_of_mode_0600_eh_01`: `left: 493`, `right: 384` (mode 0755, not 0600) | the file from the tree; 13 passed |
| a directory outside the store admitted: `admits` joins its two conditions with `\|\|` | the same | `a_directory_outside_the_store_is_refused_eh_03`: `left` an answer of class 0, `right` `{"refused":"directory-refused"}` | the same |
| a result changed on its way through the socket: `answered` encodes the result without its last byte | the same | `each_call_through_the_socket_is_the_answer_of_the_engine_eh_02`: the two `Answer` values differ | the same |
| a refusal name that comes back as another: `directory-refused` names `Refusal::Request` | the same | `every_refusal_and_call_travels_by_its_name_eh_03`: `left: Some(Request)`, `right: Some(Directory)` | the same |
| the bound on a request removed: `serve` compares the line with twice `MAX_REQUEST_BYTES` | the same | `a_request_over_the_bound_is_refused_and_the_next_is_served_eh_03`: `left` an answer of class 0, `right` `{"refused":"request-refused"}` | the same |
| a class that is not an answer accepted: `answer_of` also admits 3 and 5 | the same | `an_answer_that_is_not_one_is_unavailable_eh_03`: `left: Ok(Answer { class: 3, result: [] })`, `right: Err(Unavailable)` | the same |
| the host making an Engine call itself: `engine_call` in `gateway-binding/src/lib.rs` calls `gateway_engine_host::engine::run` in place of `client::call` | `cargo test -p repo-model --test engine_host` | `the_engine_host_is_a_program_of_its_own_and_the_host_makes_no_call_eh_01`: `the host calls the engine host` | the file from the tree; 5 passed |

## Reproducing

```sh
gateway/registry.sh                                      # the registry from gateway/Cargo.lock
cd gateway
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-engine-host --locked --offline
CARGO_HOME=$PWD/registry/cargo-home cargo test -p gateway-binding --locked --offline --test effects
cd .. && cargo test -p repo-model --test engine_host     # EH-01 to EH-04 against the rows
```

## Not met

- **An image exists and a Compose service does not** (`IMAGES.md`): a test runs this service with no
  network but loopback, and R6 stays `Pending`, because its measurement names the deployment.
- **The gateway still links `lexlean`.** The views of `store.get` and `store.put`, provisioning and the
  program `gateway-verifier` use it, so moving E3 to E6 does not take the compiler out of the process
  that faces the client.
- **E1, E2 and materialization are still in the gateway.** The architecture gives the engine host the
  store as well; here both services mount it, and they must run as one user, because the socket is
  mode 0600 and the files of a draft are the gateway's.
- **Whoever reaches the socket can run the Engine on any project in the store.** Other users are kept
  out by the mode; the same user is R10, the residual risk of `mcp-loopback`.
- **The mode is set just after the socket is made**, so it is open to the directory's reach for that
  moment. A deployment puts the socket in a directory only the two services share.
- **`prismpm replay-acceptance`** of a release has to be given a running engine host, as it has to be
  given a provisioned store. No job runs that replay yet.
