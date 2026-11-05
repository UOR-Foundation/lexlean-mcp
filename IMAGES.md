# The images of the services

Each service of issue #17 is an image of its own, run apart from the others and exercised through
the gateway. Registered as IM-01 to IM-06.

This is what the deployment will start. It is not the deployment: PrismPM's `system-model/3` names
images by their digests and projects Compose from them, but its OCI release and its deployment of a
server application have not started upstream (PrismPM#317 lists them, after slices 2 and 3 of
PrismPM#216). So these definitions are this project's inputs to that model, and `deploy/images/run.sh`
is a test that stands where the projection will. When PrismPM's release arrives, it may replace them.

| ID | Requirement |
| --- | --- |
| IM-01 | Each service has an image of its own; each definition starts from a digest, has no RUN step and runs as user 65532; each image holds its own program alone. |
| IM-02 | The verifier image alone holds Lean, at the version of `gateway/lean-toolchain`, from the archive whose SHA-256 is recorded. |
| IM-03 | The services run apart, with no network but loopback where egress is denied; the key is in the signer alone; a module is checked, built, verified and its signed record found; a cancel leaves no process and no zombie. |
| IM-04 | The collector receives an alert from the telemetry file, and no text of a module. |
| IM-05 | A service that stops is a typed failure and not a hang, and a run is held to the count of its checks. |
| IM-06 | Each gate rejects its planted defect (below). |

## What each image holds

| Image | Holds | Network | Shares with the gateway |
| --- | --- | --- | --- |
| `gateway` | `gateway-host` | none but loopback | |
| `engine-host` | `gateway-engine-host` | none but loopback | its socket, the store |
| `signer` | `gateway-signer` | none but loopback | its socket, the store |
| `verifier` | `gateway-verifier-host`, `gateway-verifier`, Lean | none but loopback | its socket, the store |
| `collector` | the upstream collector at a digest, and `collector.yaml` | none but loopback | the telemetry directory |

The key is a file mounted read-only in the signer's container and nowhere else, so it is in no layer
and no other service can read it. Every container has a read-only root, a temporary `/tmp`, no
capabilities and `no-new-privileges`.

The programs are built on the runner with the gateway's locked dependencies, and the images are built
with no network and no RUN step, so an image holds what its definition lists and nothing a step could
fetch. Lean is unpacked before the build from the release archive, after `sha256sum --check` against
`deploy/images/lean-toolchain.sha256`. LexLean takes `lean` and `lake` from
`$ELAN_HOME/toolchains/leanprover--lean4---v<version>`, so no `elan` runs and no toolchain is fetched
when a verification runs.

The bases are multi-platform digests, and `ubuntu:24.04` is the OS of the runner, because a program
needs a C library at least as new as the one it was linked against.

## The run

`gateway / images` builds the programs and the images, then runs `deploy/images/run.sh`, whose last
line is `images: N checks passed`; `images_run` in `.github/lexlean-ci/lib.sh` holds N to the number of
`check` lines written there, so a run that stopped early or lost a check is not green.
`crates/model/tests/images.rs` reads the definitions as text in `just vv`, which has no programs to put
in an image.

- **The client shares the gateway's network namespace.** See "Not met".
- **A verification of the project takes about 2 s,** so a cancel or a stop can arrive after it ended.
  The cancel is retried until the message of the task says a child group ended, as
  `a_real_verification_is_cancelled_through_the_task_th_11` does. The stop is retried while the task
  ends verified, and any end but `verified` or `verifier-lost` fails.
- **A lost verifier is `status: completed` with `isError: true`,** `state: failed` and
  `{"failure":"verifier-lost"}`, as VP-05 says. The check reads the failure by its name.
- **The kernel keeps 15 bytes of a process's name,** so the verifier host and the program of a task are
  both `gateway-verifie`. What is left of a task is anything beyond one such process.
- **`--init` is needed, and a clean cancel alone does not show it.** Without it the verifier host is
  process 1, which does not reap a `lake` or `leanchecker` orphaned when its parent is killed. An early
  version of the run passed without `--init`, so a check now kills the program under a running Lean.
  A Compose service needs the same (`init: true`).

## Not met

- **A published port cannot reach the gateway**, as long as the gateway is on a bridge network. The
  model declares `address := "127.0.0.1"` (`gateway/src/Application.lex.tex`), and Docker forwards a
  published port to the container's interface. The test client shares the gateway's namespace for
  this reason. PrismPM#502 settles it in the projection: an `api` component runs on the host network
  and publishes no port. The deployment model (`DEPLOYMENT.md`) places the gateway so; this run does
  not use it yet.
- **No Compose projection and no deployment exist.** `-p 127.0.0.1` is unexercised, and AC-03 and
  AC-10 as deployed stay open. The system model `mcp-loopback` is not written.
- **R6 stays `Pending`.** The measurement names the deployment. This run shows that a verification
  succeeds with no network but loopback, which is the evidence it will rest on, but it is not that.
- **The store is shared whole** by the gateway, the engine host, the verifier host and the signer: no
  service has less than all of it.
- **`linux/amd64` only.** `lean-toolchain.sha256` records the one archive, and the job runs on one
  platform; the release asks for `linux/arm64` too for everything but verification.
- **Only an alert reaches the collector.** Metrics wait on `flush`, and the spans of requests and S2 on
  the per-request hook of PrismPM#429 (`OBSERVABILITY.md`).
- **The verifier image is about 4 GB,** nearly all of it Lean, and nothing here trims it.

## Planted defects

Each was applied alone on a copy of the tree, the named gate failed as shown, and the file was restored,
after which the run printed `images: 30 checks passed` and `cargo test -p repo-model --test images` passed.
"The run" is `images_run`, after `images_build` where the defect is in an image.

| Defect | Gate | Diagnostic |
| --- | --- | --- |
| a base named by a tag | `each_image_is_pinned_runs_nothing_and_is_not_root_im_01` | `gateway: the base is not named by a digest: FROM docker.io/library/ubuntu:24.04` |
| a RUN step | `each_image_is_pinned_runs_nothing_and_is_not_root_im_01` | `signer has a RUN step: RUN true` |
| a root user | `each_image_is_pinned_runs_nothing_and_is_not_root_im_01` | `engine-host: the user it runs as` |
| a recorded digest that is not the archive's | `deploy/images/build.sh` | `lean-4.32.1-linux.tar.zst: FAILED` |
| a toolchain of another version | `the_verifier_alone_holds_the_pinned_lean_im_02`, and `deploy/images/build.sh` | `the archive recorded is not of the version gateway/lean-toolchain names`, and `deploy/images/lean-toolchain.sha256 does not record lean-4.32.1-linux.tar.zst` |
| Lean in the gateway image | the run | `images: failed: the gateway holds no Lean` |
| the key mounted in the verifier | `the_services_run_apart_and_only_the_signer_has_the_key_im_03`, and the run | `verifier: the key is mounted in the signer alone`, and `images: failed: the verifier has no key` |
| a network given to the services | `the_services_run_apart_and_only_the_signer_has_the_key_im_03`, and the run | `a service runs without --network none`, and `images: failed: the engine host has no network but loopback` |
| no init to reap what a killed process leaves | `the_services_run_apart_and_only_the_signer_has_the_key_im_03`, and the run | `a service runs without --init`, and `still running: gateway-verifie lake` with `images: failed: a process killed under Lean leaves no process and no zombie`, or the same check of a cancel when its child is the one orphaned |
| a run that stops early | `images_run` | `the run of the images: no checks passed, expected 30` |
| the collector reading another file | the run | `images: failed: the alert of the signer reaches the collector` |
| a stopped service left running: the signer | the run | `lexlean_verify: expected the failure verifier-unavailable`, and `images: failed: with the signer stopped, verify is refused` |
