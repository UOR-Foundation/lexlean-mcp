#!/usr/bin/env bash
# Self-test of `gate` in lib.sh, run by the `prepare` job before any gate is
# trusted. Each case runs in a fresh `bash -eo pipefail` (the shell GitHub
# Actions gives a `run:` step) and states the exit status that step must have.
# Exit 0 when every case holds; otherwise the first case that does not hold is
# named and the exit status is 1.
set -euo pipefail

lib=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/lib.sh
failures=0

# expect STATUS NAME SCRIPT: SCRIPT, run after sourcing lib.sh in a step shell,
# exits with STATUS.
expect() {
  local want=$1 name=$2 script=$3 got=0
  bash -eo pipefail -c ". '$lib' >/dev/null; $script" >/dev/null 2>&1 || got=$?
  if [[ $got == "$want" ]]; then
    printf 'ok    %s (exit %s)\n' "$name" "$got"
  else
    printf 'FAIL  %s: exit %s, expected %s\n' "$name" "$got" "$want"
    failures=$((failures + 1))
  fi
}

expect 0 "a passing gate passes" 'gate "true" true'
expect 3 "a failing gate returns its status" 'gate "three" sh -c "exit 3"'
# A failed gate followed by a passing one: the step fails at the first gate.
expect 3 "a failed gate fails a step of several gates" \
  'gate "three" sh -c "exit 3"; gate "true" true'
# A helper whose middle command fails, and whose last command passes.
expect 1 "a failing command inside a helper fails the gate" \
  'helper() { false; true; }; gate "helper" helper'
# errexit stays on after a passing gate, so a later bare failure fails the step.
expect 1 "errexit is restored after a passing gate" \
  'gate "true" true; false; true'
# A gate whose command fails in a pipeline fails.
expect 1 "a failing pipeline inside a helper fails the gate" \
  'helper() { false | cat; true; }; gate "helper" helper'

# The generated packages that pass between jobs: each case makes a repository with them laid out,
# packs them, and removes them again, as the job that unpacks starts without them.
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
packed() {
  local project=$scratch/$1 package
  mkdir -p "$project/gateway/generated/lexlean_mcp_gateway_core/src"
  echo manifest >"$project/gateway/generated/lexlean_mcp_gateway_core/Cargo.toml"
  echo core >"$project/gateway/generated/lexlean_mcp_gateway_core/src/lib.rs"
  for package in lexlean_mcp_gateway_core gateway-host-api gateway-host; do
    mkdir -p "$project/gateway/$package/src"
    echo manifest >"$project/gateway/$package/Cargo.toml"
    echo core >"$project/gateway/$package/src/lib.rs"
  done
  install -m 0755 /dev/null "$project/gateway/gateway-host/run"
  git -C "$project" init -q
  git -C "$project" -c user.name=t -c user.email=t@t commit -q --allow-empty -m one
  (
    cd "$project"
    . "$lib" >/dev/null
    RUNNER_TEMP=$project/temp gateway_build_pack >/dev/null
    rm -rf gateway/lexlean_mcp_gateway_core gateway/gateway-host-api gateway/gateway-host
  )
  printf "cd '%s'; export RUNNER_TEMP='%s/temp'; " "$project" "$project"
}
expect 0 "the packages come back with their file modes" \
  "$(packed modes)"'gate "unpack" gateway_build_unpack; test -x gateway/gateway-host/run; test -f gateway/gateway-host-api/Cargo.toml'
expect 1 "the build of another revision is refused" \
  "$(packed revision)"'git -c user.name=t -c user.email=t@t commit -q --allow-empty -m two; gate "unpack" gateway_build_unpack'
expect 1 "a build without one of the packages is refused" \
  "$(packed missing)"'mkdir w; tar -C w -xf "$RUNNER_TEMP"/gateway-build/*.tar; tar -C w -cf "$RUNNER_TEMP"/gateway-build/"$(git rev-parse HEAD)".tar lexlean_mcp_gateway_core gateway-host-api; gate "unpack" gateway_build_unpack'
expect 1 "a core that is not the committed export is refused" \
  "$(packed core)"'echo edit >>gateway/generated/lexlean_mcp_gateway_core/src/lib.rs; gate "unpack" gateway_build_unpack'

if ((failures != 0)); then
  printf '%s case(s) of gate do not hold\n' "$failures"
  exit 1
fi
printf 'gate: every case holds\n'
