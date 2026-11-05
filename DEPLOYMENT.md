# The loopback deployment

The services of issue #17 as one PrismPM host-effect system model, `mcp-loopback`
(`src/Deployment.lex.tex`), from which PrismPM's `prismpm/system-model/3` projects the Compose
project. Registered as DP-01 to DP-04.

This is step 1 of the plan in #17: the model and its proofs. The projection, the run from it and
the acceptance criteria as deployed are not here (see "Not met").

| ID | Requirement |
| --- | --- |
| DP-01 | `Production.Core` and `Production.HostEffectSystem` are PrismPM's files at `ecb594bd`, byte for byte. |
| DP-02 | The gateway alone is an `api`, on the host network, with the one listener on `127.0.0.1` at `listen.port`; every other service is on the internal network `backend` alone; the key is the signer's alone; the model is valid for every build that names its five images, and only for such a build. |
| DP-03 | The application digest is the SHA-256 of `gateway/src/Application.lex.tex`. |
| DP-04 | The declared effects are the gateway's 15 host effects, in order. |
| DP-05 | `gateway-signer ready`, `gateway-engine-host ready` and `gateway-verifier-host ready` ask their service whether it can serve and exit 0 only on its yes; otherwise 1 with the reason, and 64 for any other argument. |
| DP-06 | `gateway-ready` asks the gateway at the address and port of the generated surface and exits 0 only on an HTTP/1.1 status line; the gateway image holds it. |
| DP-07 | The signer reads its key from `LEXLEAN_MCP_SIGNING_KEY_FILE`, the variable the projection sets for the secret, and no source names the variable without `_FILE`. |

## Why the gateway is on the host network

The generated gateway listens on `127.0.0.1` only (PrismPM's PP2019), and a port published from a
bridge network reaches the container's interface, never its loopback. IMAGES.md recorded that
contradiction. PrismPM#502 (merged at `ecb594bd`) settles it in the projection, not in the serving
address: an `api` component runs with `network_mode: host`, declares no network and publishes no
port, so the `127.0.0.1` it binds is the host's loopback. `gateway/src/Application.lex.tex` does
not change.

The consequence is that the gateway shares the host's network stack and has egress. The engine host,
signer, verifier and collector are on `backend`, which is internal.

## The model

| Component | Kind | Image | Networks | Volumes | Secret |
| --- | --- | --- | --- | --- | --- |
| `telemetry-collector` | `telemetry-collector` | `lexlean-mcp-collector` | `backend` | `telemetry` | |
| `verifier` | `worker` | `lexlean-mcp-verifier` | `backend` | `store`, `verifier-socket` | |
| `signer` | `worker` | `lexlean-mcp-signer` | `backend` | `store`, `signer-socket` | `lexlean-mcp-signing-key` |
| `engine-host` | `worker` | `lexlean-mcp-engine-host` | `backend` | `store`, `engine-socket` | |
| `gateway` | `api` | `lexlean-mcp-gateway` | none (host) | all five | |

- **The verifier is a `worker`, not a `job`.** It serves tasks for the life of the deployment. The
  projection makes a dependency on a `job` wait for it to exit (`service_completed_successfully`).
- **The services talk through files, not the network.** Each socket and the telemetry file is on a
  volume, so no service needs a route to another and `backend` can be internal.
- **The environment comes from the parameters.** The projection names a variable after the
  parameter id, upper-cased with hyphens as underscores, so `lexlean-mcp-store` is
  `LEXLEAN_MCP_STORE=/store`. A socket path is one parameter, read by both ends.
- **The verifier's resources are the owner values:** `verify.width` times `verify.concurrency`
  CPUs, and `verify.memory_kib` times `verify.concurrency` of memory.
- **The images are the only input a build gives.** `Images` holds their five digests. They are
  build outputs, not source, so `deploymentIsValidForEveryNamedBuild` states validity for every
  `Images`: `validateManifest` is true exactly when each of the five is a non-empty string. The proof
  is by reflexivity: the kernel evaluates every part of the check that does not read a digest
  (`onlyTheImagesAreOpen`), and what remains is the five non-empty checks.
- **The application digest is a constant.** `simp only` cannot prove that a string equals itself
  without a lemma LexLean does not give, and the manifest repeats the digest, so a digest that
  varied would block the proof. The gateway's application is source, so it is bound here:
  `sha256:` and the SHA-256 of `gateway/src/Application.lex.tex`. PrismPM does not yet put a server
  application in its model document (#216), so there is no digest of PrismPM's to bind instead.
  DP-03 fails whenever the declaration changes, and the constant moves with it.

## Readiness

The projection gives every service but a `job` a healthcheck from its readiness command, and a
service that depends on another waits for it to be healthy. So each readiness command answers one
question about its own service:

| Command | Ready when |
| --- | --- |
| `gateway-signer ready` | the signer at `LEXLEAN_MCP_SIGNER` can load its key (its `key` request) |
| `gateway-engine-host ready` | the engine host at `LEXLEAN_MCP_ENGINE` has its store (`{"op":"ready"}`, new) |
| `gateway-verifier-host ready` | the verifier host at `LEXLEAN_MCP_VERIFIER_HOST` has its verifier program |
| `gateway-ready` | the gateway answers an HTTP/1.1 status line at `surface::ADDRESS` and `surface::PORT` |

The generated `gateway-host` has no such mode and is not this repository's to change, so
`gateway-ready` is a program of its own, in the gateway image. It sends a `GET`, which the adapter
refuses before the core runs, so a probe calls no tool and asks no effect. It reads the address and
port from the generated host, so it restates neither.

## Not a PrismPM release root yet

`prismpm check` reads a definition named `systemModel`, or `systemModel` and a suffix, as a release
root. A release root must be a record of literals, lists, records and `Option` only, with no
parameter and no call (`PP2101`). So `deploymentModelFor` is a function of the five images, not a
release root, and it is named so that the check does not read it as one. The release root PrismPM
projects, a closed `systemModel` with the digests of one build written out and its
`…SystemReleaseReady` theorem by `decide`, is generated from it in step 2. With no root, `prismpm
check` checks the rest of the model as before.

## The format gate

PrismPM's formatter writes `loopbackAddress` on five lines and the pinned LexLean on one, so
`lexlean fmt --check` refuses the byte copy. Step 4a of the `lexlean` workflow therefore runs
`root_fmt_check`, which formats a scratch copy and compares every module but those under
`src/Production`, as `gateway_fmt_check` already does for the gateway's copies.

## Not met

- **No projection and no run.** Nothing here calls PrismPM's projection, and `deploy/images/run.sh`
  still stands where the Compose run will. That is step 2 and step 3 of #17.
- **The collector's readiness checks its configuration**, `otelcol-contrib validate`, not that it
  runs. The upstream image has no command that asks a running collector.
- **The images are not published.** The model names `ghcr.io/uor-foundation/lexlean-mcp-*`, and
  nothing pushes them (decision 3 on #17).
- **Two PrismPM pins.** The gateway is built by PrismPM at `706fb746`, which does not contain #502.
  The profile copied here is from `ecb594bd`. Which PrismPM runs the projection is decision 1 on #17.
- **R6 for the gateway.** On the host network the gateway has egress. `Security` is not changed
  here (decision 2 on #17).
- **Linux only.** Host networking means something else on Docker Desktop for macOS and Windows.

## Planted defects

Each was applied alone to a copy of the tree, the gates were run, and the copy was removed. The
unplanted tree gives `verified 6 modules` for `lexlean verify src/Deployment.lex.tex` (LexLean
v0.4.0) and `4 passed` for `cargo test -p repo-model --test deployment`; `prismpm check` in the SDK image of
`prismpm.lock` passes.

| Defect | `lexlean verify src/Deployment.lex.tex` | `cargo test -p repo-model --test deployment` |
| --- | --- | --- |
| the gateway on `backend` | `LLV7002` at `onlyTheImagesAreOpen`: `rfl` failed | DP-02: `the gateway declares no network` |
| the listener on `0.0.0.0` | `LLV7002` at `onlyTheImagesAreOpen`: `rfl` failed | DP-02: the listener's `bindAddress` |
| the signing key also in the gateway | `LLV7002` at `theSigningKeyIsTheSignersAlone`: `decide` refutes it | DP-02: `gateway holds no secret` |
| `imagesNamed` without the gateway image | `LLV7002` at `deploymentIsValidForEveryNamedBuild`: unsolved goals | DP-02: `imagesNamed requires the gateway image to be named` |
| `task.store` dropped from the effects | `LLV7002` at `theEffectsAreTheGatewaysHostProtocol`: `decide` refutes it | DP-04: the frame and task effects differ |
| `loopbackAddress` of the copy set to `0.0.0.0` | `LLV7002` in `Production.HostEffectSystem`: `decide` refutes its own refusal theorems | DP-01: the checksum is not the authority's, and DP-02: `PrismPM's loopback address is 127.0.0.1` |
| one byte added to `gateway/src/Application.lex.tex` | passes: the gateway is not in this closure | DP-03: `gatewayApplicationDigest is not "sha256:2dc4be89…"` |
| `deploymentModelFor` named `systemModelFor` | passes | `prismpm check` (SDK `0519fe6e`): `PP2101: system model is not a closed generated data value`, exit 3 |
| `gateway-signer ready` exits 0 whatever the answer | | `cargo test -p gateway-signer --test ready`: 3 of 5 fail, among them `a_signer_whose_key_is_gone_is_not_ready_and_says_why_dp_05` |
| the engine host answers ready without its store | | `cargo test -p gateway-engine-host --test ready`: `an_engine_host_without_its_store_is_not_ready_and_says_why_dp_05` |
| the probe takes any `HTTP/1.` version | | `probe.rs`: `an_answer_that_is_not_http_is_not_ready_dp_06` |
| the signer back on `LEXLEAN_MCP_SIGNING_KEY` | | DP-07: `gateway/bindings/gateway-signer/src/service.rs does not name LEXLEAN_MCP_SIGNING_KEY_FILE` |
| the gateway's readiness command `gateway-host` | passes | DP-06: the readiness of `gateway` is not `gateway-ready` |
| two lines added to `src/Deployment.lex.tex` | | `root_fmt_check`: `src/Deployment.lex.tex is not canonical`, exit 1 |

The application defect passes `lexlean verify`, which is why DP-03 is a test: the gateway is
another project, and only a test reads both.
