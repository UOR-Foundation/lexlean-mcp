#!/bin/sh
# A stand-in for gateway-verifier that speaks its protocol and does what the file `fake-behavior`
# of the project says, so that the host can be tested without Lean. The first word of the
# behavior is the mode; the second, when there is one, is a gate: a file the mode waits for.
#
#   verified [GATE]   wait for the gate, then print a verified verdict for a made-up directory
#   failed            print a failed verdict of class 1
#   wait-cancel       wait for `cancel`, then print cancelled with 2 groups ended and 1 not
#   verified-on-cancel  wait for `cancel`, then print verified (the verdict that raced the cancel)
#   hang              ignore everything and never end
#   die               write "boom" to the standard error and exit without a verdict
#   kill-self         kill itself with SIGKILL
#   orphan            leave a child in the session and exit without a verdict

# A session of its own, as the real program makes: setsid does not fork for a process that is not a
# group leader, so the process ID stays and is the session.
[ -n "$FAKE_SESSION" ] || exec env FAKE_SESSION=1 setsid "$0" "$@"

printf '{"event":"started","sid":%s}\n' "$$"
IFS= read -r job || exit 1
dir=$(printf '%s' "$job" | sed 's/.*"directory":"\([^"]*\)".*/\1/')
{
  echo "argv: $*"
  echo "job: $job"
  env
} > "$dir/seen"
echo "$$" > "$dir/pid"

behavior=$(cat "$dir/fake-behavior" 2>/dev/null || echo verified)
# shellcheck disable=SC2086
set -- $behavior
mode=$1
gate=$2

verified() {
  id=$(printf '%s' "$dir" | sha256sum | cut -c1-64)
  mkdir -p "$dir/verified/modules/Fake"
  # The evidence a real verification leaves (SIGNING.md, SG-14): a rendering, a manifest that names
  # it and the first source file of the project, and an attestation.json that names the manifest.
  printf '%s\n' '\documentclass{article}' > "$dir/verified/modules/Fake/Fake.tex"
  tex=$(sha256sum < "$dir/verified/modules/Fake/Fake.tex" | cut -c1-64)
  src=$(cd "$dir" && find src -name '*.lex.tex' 2>/dev/null | sort | head -n 1)
  srcsum=$(sha256sum < "$dir/$src" | cut -c1-64)
  printf '{"inputs":[{"kind":"source","path":"%s","sha256":"%s"}],"outputs":[{"kind":"tex","path":"modules/Fake/Fake.tex","sha256":"%s"}]}' \
    "$src" "$srcsum" "$tex" > "$dir/verified/build-manifest.json"
  manifest=$(sha256sum < "$dir/verified/build-manifest.json" | cut -c1-64)
  printf '{"attestation_id":"%s","build_manifest":{"sha256":"%s"},"toolchain":{"lean":"fake"}}' "$id" "$manifest" > "$dir/verified/attestation.json"
  printf '{"event":"verified","attestation_id":"%s","build_id":"%s","source_id":"%s","semantic_id":"%s","modules":["Tiny"],"root":"%s/verified"}\n' \
    "$id" "$id" "$id" "$id" "$dir"
}

case "$mode" in
  verified)
    while [ -n "$gate" ] && [ ! -e "$gate" ]; do sleep 0.05; done
    verified
    ;;
  failed)
    printf '{"event":"failed","class":1,"result":{"diagnostics":[]}}\n'
    ;;
  wait-cancel)
    IFS= read -r line
    [ "$line" = cancel ] && printf '{"event":"cancelled","ended":2,"failed":1}\n'
    ;;
  verified-on-cancel)
    IFS= read -r line
    [ "$line" = cancel ] && verified
    ;;
  hang)
    while :; do sleep 1; done
    ;;
  die)
    echo boom >&2
    exit 3
    ;;
  kill-self)
    kill -9 "$$"
    ;;
  orphan)
    sleep 300 &
    echo "$!" > "$dir/orphan.pid"
    exit 4
    ;;
esac
