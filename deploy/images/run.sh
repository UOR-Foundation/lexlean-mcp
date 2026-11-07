#!/usr/bin/env bash
# The services run apart, from their images (IM-03, IM-04, IM-05). This is a test and not the
# deployment: `docker run` stands where the Compose projection of PrismPM will (IMAGES.md).
#
# Usage: deploy/images/run.sh, after deploy/images/build.sh.
# The last line is `images: N checks passed`; `images_run` holds N to the checks written here.
set -euo pipefail

here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
# Only for the client of this test; no image of a service holds it.
client_image=docker.io/library/python:3.13-slim@sha256:ae14d4ce8771606477a81446e7ef429edd6762305fc50c84507c1041fa8285d1
prefix=lexlean-images-$$
work=$(mktemp -d)
passed=0

as_root() {
  docker run --rm --user 0 --network none --volume "$work:/work" --entrypoint "$1" \
    lexlean-mcp/gateway:local "${@:2}"
}

cleanup() {
  local status=$?
  if ((status != 0)); then
    for name in signer engine-host verifier gateway collector; do
      echo "--- the log of $name" >&2
      docker logs "$prefix-$name" 2>&1 | tail -n 20 >&2 || true
    done
  fi
  docker rm --force "$prefix-client" "$prefix-signer" "$prefix-engine-host" "$prefix-verifier" \
    "$prefix-gateway" "$prefix-collector" >/dev/null 2>&1 || true
  as_root rm -rf /work/store /work/sockets /work/key /work/telemetry 2>/dev/null || true
  rm -rf "$work"
  exit "$status"
}
trap cleanup EXIT

check() {
  local name=$1
  shift
  if ! "$@"; then
    echo "images: failed: $name" >&2
    return 1
  fi
  passed=$((passed + 1))
  echo "ok $name"
}

# The init Docker supplies reaps what a killed process leaves: no service is written to be process 1,
# and a Lean child that the sweep ended must not stay a zombie.
service() {
  local name=$1 image=$2
  shift 2
  docker run --detach --name "$prefix-$name" --init --network none --read-only --tmpfs /tmp \
    --cap-drop ALL --security-opt no-new-privileges "$@" "lexlean-mcp/$image:local" >/dev/null
}

# One container for every call: a verification can end before a new one would have started.
client() {
  docker exec "$prefix-client" python3 -I /exercise.py "$@"
}

# The readiness command the deployment names for a service, run in its container (DP-05, DP-06).
ready() {
  local name=$1
  shift
  docker exec "$prefix-$name" "/usr/local/bin/$1" "${@:2}"
}

in_container() {
  docker exec "$prefix-$1" sh -c "$2"
}

wait_for_socket() {
  local attempt
  for attempt in $(seq 1 100); do
    [[ -S $1 ]] && return 0
    sleep 0.1
  done
  echo "no socket at $1" >&2
  return 1
}

# The names the processes of a container are running under, but for its init and this shell.
programs_of() {
  in_container "$1" 'for f in /proc/[0-9]*/comm; do cat "$f" 2>/dev/null; done' |
    grep -v -E '^(docker-init|sh|cat)$' || true
}

# The verifier host is killed under a task. A verification can win that race, so an attempt that
# ended verified starts the host again and tries again; any end but those two is a defect.
a_task_fails_when_the_verifier_host_goes() {
  local attempt task ended
  for attempt in $(seq 1 8); do
    task=$(client start)
    docker kill "$prefix-verifier" >/dev/null
    ended=$(client settled "$task")
    if [[ $ended == verifier-lost ]]; then
      return 0
    fi
    if [[ $ended != verified ]]; then
      echo "the task of a lost service ended $ended" >&2
      return 1
    fi
    start_verifier_again "$((attempt + 1))"
  done
  echo "the verification ended first in every attempt" >&2
  return 1
}

# The socket of the last run is still there until the host replaces it, so what is awaited is the
# line the host writes when it listens: the log of a container runs across its restarts.
start_verifier_again() {
  local times=$1 attempt
  docker start "$prefix-verifier" >/dev/null
  for attempt in $(seq 1 100); do
    if (($(docker logs "$prefix-verifier" 2>&1 | grep -c listening) >= times)); then
      return 0
    fi
    sleep 0.1
  done
  echo "the verifier host did not listen again" >&2
  return 1
}

# The process of a task, killed while Lean runs under it: Lean then has no parent but process 1, and
# only that can reap it once the sweep has ended it. A cancel cannot show this, because the program
# waits for its children. An attempt whose verification ended first tries again.
a_killed_process_leaves_nothing() {
  local attempt task poll pid ended
  for attempt in $(seq 1 8); do
    task=$(client start)
    pid=
    for poll in $(seq 1 100); do
      pid=$(in_container verifier 'for d in /proc/[0-9]*; do
        if grep -q -E "^(lean|lake)$" "$d/comm" 2>/dev/null; then
          awk "/^PPid:/ { print \$2 }" "$d/status" 2>/dev/null || sed -n "s/^PPid:\t//p" "$d/status"
          break
        fi
      done' 2>/dev/null || true)
      [[ -n $pid ]] && break
      sleep 0.02
    done
    if [[ -n $pid ]]; then
      in_container verifier "kill -9 $pid" 2>/dev/null || true
    fi
    ended=$(client settled "$task")
    if [[ $ended == verifier-lost ]]; then
      no_verifier_process && no_zombie
      return
    fi
    if [[ $ended != verified ]]; then
      echo "the task of a killed process ended $ended" >&2
      return 1
    fi
  done
  echo "the verification ended first in every attempt" >&2
  return 1
}

only_loopback() {
  [[ $(in_container "$1" "tail -n +3 /proc/net/dev | cut -d: -f1 | tr -d ' '") == lo ]]
}

no_lean() {
  in_container "$1" 'test ! -e /opt/elan && ! command -v lean && ! command -v lake' >/dev/null
}

no_key() {
  in_container "$1" 'test ! -e /run/signing/signing.key' &&
    ! docker inspect --format '{{json .Config.Env}}' "$prefix-$1" | grep -q LEXLEAN_MCP_SIGNING_KEY_FILE
}

runs_as_65532() {
  [[ $(in_container "$1" 'id -u') == 65532 ]]
}

holds_only() {
  local name=$1 expected=$2 found
  found=$(in_container "$name" 'ls /usr/local/bin' | paste -sd' ')
  [[ $found == "$expected" ]] || { echo "$name holds: $found" >&2; return 1; }
}

# The kernel keeps 15 bytes of a name, so the host and the program of a task are both
# gateway-verifie: what is left of a task is anything beyond the one host.
no_verifier_process() {
  local attempt
  for attempt in $(seq 1 40); do
    if [[ $(programs_of verifier) == gateway-verifie ]]; then
      return 0
    fi
    sleep 0.5
  done
  echo "still running: $(programs_of verifier | paste -sd' ')" >&2
  return 1
}

no_zombie() {
  ! in_container verifier 'cat /proc/[0-9]*/stat 2>/dev/null' | grep -q -E '^[0-9]+ \([^)]*\) Z '
}

alert_reached_the_collector() {
  local attempt
  for attempt in $(seq 1 60); do
    if as_root cat /work/telemetry/received.jsonl 2>/dev/null | grep -q '"alert"'; then
      return 0
    fi
    sleep 0.5
  done
  echo "the collector received no alert" >&2
  return 1
}

no_module_text_reached_the_collector() {
  local received
  received=$(as_root cat /work/telemetry/received.jsonl)
  [[ -n $received ]] && ! grep -q -F "$module_text" <<<"$received"
}

mkdir -p "$work"/store/drafts "$work"/sockets/signer "$work"/sockets/engine "$work"/sockets/verifier \
  "$work"/key "$work"/telemetry
cp -a "$root/gateway/vectors/fixture/tiny" "$work/store/drafts/dir"
head -c 32 /dev/urandom | od -An -tx1 | tr -d ' \n' >"$work/key/signing.key"
as_root chown -R 65532:65532 /work/store /work/sockets /work/key /work/telemetry
as_root chmod 0400 /work/key/signing.key

# A line of the module of the project: the words that must not leave in a record (OB-04).
module_text=$(grep -rh -m1 -E 'name:' "$root/gateway/vectors/fixture/tiny/src" | head -n 1)

service signer signer --volume "$work/store:/store" --volume "$work/sockets/signer:/sockets" \
  --volume "$work/key:/run/signing:ro" --env LEXLEAN_MCP_STORE=/store \
  --env LEXLEAN_MCP_SIGNER=/sockets/signer.sock --env LEXLEAN_MCP_SIGNING_KEY_FILE=/run/signing/signing.key
service engine-host engine-host --volume "$work/store:/store" --volume "$work/sockets/engine:/sockets" \
  --env LEXLEAN_MCP_STORE=/store --env LEXLEAN_MCP_ENGINE=/sockets/engine.sock
service verifier verifier --volume "$work/store:/store" --volume "$work/sockets/verifier:/sockets" \
  --env LEXLEAN_MCP_STORE=/store --env LEXLEAN_MCP_VERIFIER_HOST=/sockets/verifier.sock
wait_for_socket "$work/sockets/signer/signer.sock"
wait_for_socket "$work/sockets/engine/engine.sock"
wait_for_socket "$work/sockets/verifier/verifier.sock"
service gateway gateway --volume "$work/store:/store" --volume "$work/telemetry:/telemetry" \
  --volume "$work/sockets/signer:/sockets/signer" --volume "$work/sockets/engine:/sockets/engine" \
  --volume "$work/sockets/verifier:/sockets/verifier" --env LEXLEAN_MCP_STORE=/store \
  --env LEXLEAN_MCP_TELEMETRY=/telemetry/otlp.jsonl --env LEXLEAN_MCP_SIGNER=/sockets/signer/signer.sock \
  --env LEXLEAN_MCP_ENGINE=/sockets/engine/engine.sock \
  --env LEXLEAN_MCP_VERIFIER_HOST=/sockets/verifier/verifier.sock
service collector collector --volume "$work/telemetry:/telemetry"
docker run --detach --name "$prefix-client" --network "container:$prefix-gateway" --read-only \
  --volume "$here/exercise.py:/exercise.py:ro" --entrypoint sleep "$client_image" infinity >/dev/null

check "the gateway answers from its image" client ready
check "the gateway is ready by its readiness command" ready gateway gateway-ready
check "the signer is ready by its readiness command" ready signer gateway-signer ready
check "the engine host is ready by its readiness command" ready engine-host gateway-engine-host ready
check "the verifier host is ready by its readiness command" ready verifier gateway-verifier-host ready
check "the gateway runs as user 65532" runs_as_65532 gateway
check "the engine host runs as user 65532" runs_as_65532 engine-host
check "the signer runs as user 65532" runs_as_65532 signer
check "the verifier runs as user 65532" runs_as_65532 verifier
check "the engine host has no network but loopback" only_loopback engine-host
check "the verifier has no network but loopback" only_loopback verifier
check "the signer has no network but loopback" only_loopback signer
check "the gateway holds no Lean" no_lean gateway
check "the engine host holds no Lean" no_lean engine-host
check "the signer holds no Lean" no_lean signer
check "the gateway holds its program and its readiness command alone" holds_only gateway "gateway-host gateway-ready"
check "the engine host holds its program alone" holds_only engine-host gateway-engine-host
check "the signer holds its program alone" holds_only signer gateway-signer
check "the verifier holds its two programs alone" holds_only verifier "gateway-verifier gateway-verifier-host"
check "the gateway has no key" no_key gateway
check "the engine host has no key" no_key engine-host
check "the verifier has no key" no_key verifier

check "the Engine calls answer with no Lean in their image" client engine
check "a module is verified, and its signed record is found" client verified

check "a cancel ends a Lean child of its task" client cancelled
check "a cancel leaves no process of the task" no_verifier_process
check "a cancel leaves no zombie" no_zombie
check "a process killed under Lean leaves no process and no zombie" a_killed_process_leaves_nothing

check "a task whose verifier host stops ends as verifier-lost" a_task_fails_when_the_verifier_host_goes
check "with the verifier host stopped, verify is refused" client verify-refused

docker stop --time 2 "$prefix-engine-host" >/dev/null
check "with the engine host stopped, an Engine call is refused" client engine-refused

start_verifier_again "$(($(docker logs "$prefix-verifier" 2>&1 | grep -c listening) + 1))"
docker stop --time 2 "$prefix-signer" >/dev/null
check "with the signer stopped, verify is refused" client verify-refused
check "the alert of the signer reaches the collector" alert_reached_the_collector
check "no module text reaches the collector" no_module_text_reached_the_collector

echo "images: $passed checks passed"
