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

if ((failures != 0)); then
  printf '%s case(s) of gate do not hold\n' "$failures"
  exit 1
fi
printf 'gate: every case holds\n'
