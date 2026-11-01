# Helpers for .github/workflows/lexlean.yml. Sourced by its steps, never run.
# Every step runs its command through `gate`, so a failure always leaves its
# last lines where `gh api repos/<owner>/<repo>/check-runs/<id>/annotations`
# can read them: the job log itself cannot be downloaded from every sandbox.

# Lines of output kept in a failure annotation.
GATE_TAIL_LINES=${GATE_TAIL_LINES:-40}

# Escape text for the value of an `::error::` workflow command.
_annotation_data() {
  local text=$1
  text=${text//'%'/'%25'}
  text=${text//$'\r'/'%0D'}
  text=${text//$'\n'/'%0A'}
  printf '%s' "$text"
}

# Escape text for a property (`title=`) of a workflow command.
_annotation_property() {
  local text
  text=$(_annotation_data "$1")
  text=${text//':'/'%3A'}
  text=${text//','/'%2C'}
  printf '%s' "$text"
}

# annotate LEVEL TITLE LINES LOGFILE: one annotation holding the last LINES
# lines of LOGFILE, without terminal colour, each line cut to 400 characters.
annotate() {
  local level=$1 title=$2 lines=$3 log=$4 tail_text
  tail_text=$(tail -n "$lines" "$log" |
    sed -e 's/\x1b\[[0-9;]*[A-Za-z]//g' | cut -c1-400)
  printf '::%s title=%s::%s\n' "$level" \
    "$(_annotation_property "$title")" \
    "$(_annotation_data "$tail_text")"
}

# gate LABEL COMMAND...: run COMMAND and show its output. When it fails, emit an
# error annotation of its last $GATE_TAIL_LINES lines and return its exit status.
# When it passes, emit a notice of its last line and its duration, so a run that
# executed nothing cannot look like a run that passed. COMMAND runs in a
# subshell, so it passes anything to later steps through $GITHUB_PATH or
# $GITHUB_OUTPUT. COMMAND runs with errexit on, so a failing command inside a
# helper function fails the gate, and the caller's errexit is restored after it,
# so a failed gate in a step of several gates fails the step
# (.github/lexlean-ci/test-gate.sh).
gate() {
  local label=$1 log status had_errexit=0 started=$SECONDS
  shift
  log=$(mktemp "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/gate.XXXXXX")
  [[ $- == *e* ]] && had_errexit=1
  set +e
  (
    set -e
    "$@"
  ) 2>&1 | tee "$log"
  status=${PIPESTATUS[0]}
  ((had_errexit)) && set -e
  if ((status != 0)); then
    annotate error "$label failed (exit $status)" "$GATE_TAIL_LINES" "$log"
  else
    annotate notice "$label passed in $((SECONDS - started))s" 1 "$log"
  fi
  rm -f "$log"
  return "$status"
}

# lexlean_rev: the revision .github/lexlean.rev names, or an error. The gateway job sets
# LEXLEAN_REV_FILE to gateway/mcp/lexlean.rev: the gateway is compiled by the LexLean commit
# that MCP names at .github/mcp.rev (AC-03 a), not by the root project's.
lexlean_rev() {
  local file=${LEXLEAN_REV_FILE:-.github/lexlean.rev} matches
  matches=$(grep -c '^rev = ' "$file") || true
  if [[ $matches != 1 ]]; then
    echo "$file must hold exactly one 'rev = ' line, found $matches" >&2
    return 1
  fi
  local rev
  rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$file")
  if [[ -z $rev ]]; then
    echo "$file: rev must be a 40-digit lowercase commit hash" >&2
    return 1
  fi
  printf '%s\n' "$rev"
}

# The tool roots the workflow caches.
LEXLEAN_ROOT=${LEXLEAN_ROOT:-$HOME/.lexlean-bin}

# install_lexlean: build `lexlean` at the pinned revision into $LEXLEAN_ROOT,
# unless that root already holds exactly that revision.
install_lexlean() {
  local rev
  rev=$(lexlean_rev)
  if ! lexlean_installed_at "$rev"; then
    rm -rf "$LEXLEAN_ROOT"
    cargo install --git https://github.com/UOR-Foundation/LexLean \
      --rev "$rev" --locked --root "$LEXLEAN_ROOT" lexlean
  fi
  lexlean_installed_at "$rev"
  echo "$LEXLEAN_ROOT/bin" >>"${GITHUB_PATH:-/dev/null}"
  "$LEXLEAN_ROOT/bin/lexlean" --version
}

# lexlean_installed_at REV: cargo's own record says $LEXLEAN_ROOT was built from
# exactly REV, so a cache that was restored for another key cannot pass.
lexlean_installed_at() {
  grep -qF "UOR-Foundation/LexLean?rev=$1#$1)" "$LEXLEAN_ROOT/.crates.toml" 2>/dev/null &&
    test -x "$LEXLEAN_ROOT/bin/lexlean"
}

# install_elan: the pinned elan release (ELAN_VERSION, ELAN_SHA256), checked
# against its SHA-256 before it is unpacked or run.
install_elan() {
  local dir
  dir=$(mktemp -d)
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
    --output "$dir/elan.tar.gz" \
    "https://github.com/leanprover/elan/releases/download/${ELAN_VERSION}/elan-x86_64-unknown-linux-gnu.tar.gz"
  echo "${ELAN_SHA256}  $dir/elan.tar.gz" | sha256sum --check --strict -
  tar -xzf "$dir/elan.tar.gz" -C "$dir"
  "$dir/elan-init" -y --no-modify-path --default-toolchain none
  echo "$HOME/.elan/bin" >>"${GITHUB_PATH:-/dev/null}"
  "$HOME/.elan/bin/elan" --version
}

# install_lean: the toolchain `lean-toolchain` names, then proof that the
# `lean` elan resolves in this directory is that version.
install_lean() {
  local toolchain version
  toolchain=$(<lean-toolchain)
  version=${toolchain##*:v}
  "$HOME/.elan/bin/elan" toolchain install "$toolchain"
  "$HOME/.elan/bin/lean" --version | tee /dev/stderr | grep -qF "version $version,"
  "$HOME/.elan/bin/lake" --version
}

# install_cargo_deny: the pinned cargo-deny release (CARGO_DENY_VERSION,
# CARGO_DENY_SHA256), checked against its SHA-256 before it is unpacked.
install_cargo_deny() {
  local dir name
  dir=$(mktemp -d)
  name=cargo-deny-${CARGO_DENY_VERSION}-x86_64-unknown-linux-musl
  curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
    --output "$dir/deny.tar.gz" \
    "https://github.com/EmbarkStudios/cargo-deny/releases/download/${CARGO_DENY_VERSION}/${name}.tar.gz"
  echo "${CARGO_DENY_SHA256}  $dir/deny.tar.gz" | sha256sum --check --strict -
  tar -xzf "$dir/deny.tar.gz" -C "$dir"
  install -D -m 0755 "$dir/$name/cargo-deny" "$HOME/.cargo/bin/cargo-deny"
  cargo deny --version
}

# install_rust: the toolchain rust-toolchain.toml pins, and nothing else.
install_rust() {
  rustup toolchain install
  rustup show active-toolchain
  rustc --version
  cargo --version
}

# output_pin: publish the pinned revision as the step output `rev`, the key of
# the cache that holds the compiler built from it.
output_pin() {
  local rev
  rev=$(lexlean_rev)
  echo "rev=$rev" >>"${GITHUB_OUTPUT:?}"
}

# GATEWAY_INPUTS: every path the gateway job reads, as extended regular expressions over
# repository paths. A pull request that changes none of them skips the job; a push to main
# always runs it, so an input missing here is still caught on main.
GATEWAY_INPUTS=(
  '^gateway/'
  '^\.github/(lexlean-ci/|mcp\.rev$|prismpm\.rev$|workflows/lexlean\.yml$)'
  '^crates/model/(Cargo\.toml$|src/|tests/gateway(_conformance)?\.rs$)'
  '^(Cargo\.toml|Cargo\.lock|rust-toolchain\.toml|lean-toolchain|prismpm\.lock|deny\.toml)$'
  '^src/(Effects|OwnerValues)\.lex\.tex$'
  '^(CI|EFFECTS|GATEWAY|GATEWAY-CONFORMANCE)\.md$'
)

# VERIFY_FIXTURES_INPUTS: what can change the answer of the job that runs the verify fixtures
# (VP-07): the verifier, the engine and the program, the tests, the LexLean commit they compare
# with, and the job itself. `gateway/mcp/lexlean.rev` is laid out from `.github/mcp.rev` and is not
# committed, so the commit is reached through the pin.
VERIFY_FIXTURES_INPUTS=(
  '^gateway/bindings/gateway-binding/src/(verifier|engine|child)\.rs$'
  '^gateway/tool-tests/'
  '^gateway/mcp/lexlean\.rev$'
  '^\.github/(mcp\.rev$|lexlean-ci/lib\.sh$|workflows/lexlean\.yml$)'
)

# scope_of NAME PATTERN...: publish `NAME=true` unless BASE names a pull request's base and no file
# changed since it matches a pattern.
scope_of() {
  local name=$1 changed pattern reach=false
  shift
  if [[ -z ${BASE:-} ]]; then
    reach=true
  else
    changed=$(git diff --name-only "$BASE"...HEAD)
    for pattern in "$@"; do
      if grep -Eq "$pattern" <<<"$changed"; then
        echo "$name reached by: $(grep -E "$pattern" <<<"$changed" | head -n 1)"
        reach=true
        break
      fi
    done
  fi
  echo "$name=$reach"
  echo "$name=$reach" >>"${GITHUB_OUTPUT:?}"
}

# output_gateway_scope: whether the gateway job and the job of the verify fixtures run.
output_gateway_scope() {
  scope_of gateway "${GATEWAY_INPUTS[@]}"
  scope_of verify_fixtures "${VERIFY_FIXTURES_INPUTS[@]}"
}

# output_matrix: publish the jobs that need `lexlean` as the step output
# `matrix`: one that runs the project gates, and one for every production
# request the repository commits, found here and never listed twice.
output_matrix() {
  local found matrix
  found=$(find crates -type f -path '*/generated/request.json' | LC_ALL=C sort)
  if [[ -z $found ]]; then
    echo "no crates/*/generated/request.json: nothing binds a package to its sources" >&2
    return 1
  fi
  matrix=$(printf '%s\n' "$found" | jq -R . | jq -sc '
    {include: ([{kind: "verify", label: "verify", request: ""}] +
      map({kind: "export", label: ("export " + (split("/")[1])), request: .}))}')
  echo "$matrix"
  echo "matrix=$matrix" >>"${GITHUB_OUTPUT:?}"
}

# compiler_semantics_agrees: the language-1.2 compiler-semantics ID of the
# binary built from .github/lexlean.rev is the one lexlean.lock records.
# `lexlean --version` prints the language-1.0 ID, which a language-1.2 lock
# does not record, so the binary is asked the question the lock asks: it locks
# a copy of the committed tree, and the copy's ID is compared with the original.
compiler_semantics_agrees() {
  local scratch recorded built language
  scratch=$(mktemp -d)
  git ls-files -z | xargs -0 cp --parents -t "$scratch"
  language=$(sed -n 's/^language = "\(.*\)"$/\1/p' lexlean.lock)
  recorded=$(sed -n 's/^compiler_semantics = "\(.*\)"$/\1/p' lexlean.lock)
  (cd "$scratch" && lexlean --color never lock >/dev/null)
  built=$(sed -n 's/^compiler_semantics = "\(.*\)"$/\1/p' "$scratch/lexlean.lock")
  rm -rf "$scratch"
  echo "language $language"
  echo "lexlean.lock records compiler-semantics $recorded"
  echo "$(lexlean_rev) computes     compiler-semantics $built"
  if [[ -z $recorded || $recorded != "$built" ]]; then
    echo "The LexLean revision in .github/lexlean.rev is not the compiler that made lexlean.lock." >&2
    echo "Move the revision and the lock together: build that revision and run 'lexlean lock'." >&2
    return 1
  fi
}

# export_matches REQUEST: export REQUEST with `lexlean production build` and
# require that the packages it publishes equal, byte for byte, the packages
# committed beside it, in both directions.
export_matches() {
  local request=$1 dir exported committed name status=0
  dir=$(dirname "$request")
  rm -rf .lexlean/production
  lexlean --color never production build --request "$request"
  exported=$(find .lexlean/production -mindepth 4 -maxdepth 4 -type d \
    -path '*/packages/rust-std/*' | LC_ALL=C sort)
  if [[ -z $exported ]]; then
    echo "$request: the build published no rust-std package" >&2
    return 1
  fi
  committed=$(find "$dir" -mindepth 1 -maxdepth 1 -type d | LC_ALL=C sort)
  # The same packages, by name: neither a committed package the request no
  # longer exports nor an export nobody committed escapes the comparison.
  if [[ $(xargs -r -n1 basename <<<"$exported") != "$(xargs -r -n1 basename <<<"$committed")" ]]; then
    echo "$request: the export and $dir hold different packages" >&2
    echo "exported:  $(xargs -r -n1 basename <<<"$exported" | paste -sd' ')" >&2
    echo "committed: $(xargs -r -n1 basename <<<"$committed" | paste -sd' ')" >&2
    return 1
  fi
  while IFS= read -r package; do
    name=$(basename "$package")
    if ! diff -rq "$package" "$dir/$name" >/dev/null; then
      status=1
      echo "== $dir/$name differs from the export of $request =="
      diff -ru "$dir/$name" "$package" | cut -c1-200 | head -n 20 || true
      echo "== differing files =="
      diff -rq "$dir/$name" "$package" | sed -e "s#^#$request: #" || true
    else
      echo "$dir/$name equals the export of $request"
    fi
  done <<<"$exported"
  return "$status"
}

# prismpm_rev: the revision .github/prismpm.rev names, or an error.
prismpm_rev() {
  local file=.github/prismpm.rev matches rev
  matches=$(grep -c '^rev = ' "$file") || true
  if [[ $matches != 1 ]]; then
    echo "$file must hold exactly one 'rev = ' line, found $matches" >&2
    return 1
  fi
  rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$file")
  if [[ -z $rev ]]; then
    echo "$file: rev must be a 40-digit lowercase commit hash" >&2
    return 1
  fi
  printf '%s\n' "$rev"
}

# The root the workflow caches the SDK binary in.
PRISMPM_ROOT=${PRISMPM_ROOT:-$HOME/.prismpm-bin}

# output_prismpm_pin: the pinned PrismPM revision as the step output `rev` and the
# LexLean revision it is built with as `lexlean`, the key of the cache that holds the
# binary built from them.
output_prismpm_pin() {
  local rev lexlean
  rev=$(prismpm_rev)
  lexlean=$(lexlean_rev)
  echo "rev=$rev" >>"${GITHUB_OUTPUT:?}"
  echo "lexlean=$lexlean" >>"${GITHUB_OUTPUT:?}"
}

# fetch_rev URL REV DEST: the tree of exactly commit REV of URL in DEST.
fetch_rev() {
  local url=$1 rev=$2 dest=$3
  git init -q "$dest"
  git -C "$dest" remote add origin "$url"
  git -C "$dest" fetch -q --depth 1 origin "$rev"
  git -C "$dest" checkout -q FETCH_HEAD
  test "$(git -C "$dest" rev-parse HEAD)" = "$rev"
}

# prismpm_installed_at REV LEXLEAN: the record `install_prismpm` wrote says
# $PRISMPM_ROOT was built from exactly these two revisions, so a cache restored for
# another key cannot pass.
prismpm_installed_at() {
  test -x "$PRISMPM_ROOT/bin/prismpm" &&
    test "$(cat "$PRISMPM_ROOT/pins" 2>/dev/null)" == "$(printf 'prismpm %s\nlexlean %s' "$1" "$2")"
}

# install_prismpm: `prismpm` at the pinned PrismPM revision, built with the LexLean
# revision of .github/lexlean.rev in place of the LexLean it vendors, unless
# $PRISMPM_ROOT already holds exactly that pair. One pin drives both tools (R1): the
# compiler that certifies the core is the compiler lexlean.lock was made with.
#
# Nothing in PrismPM changes. The build is a copy of the PrismPM tree at the revision
# whose vendor/lexlean is the package `cargo package` makes of the LexLean tree at
# the revision, so Cargo.lock is the one PrismPM committed (--locked). The profile is
# debug: PrismPM's release profile (thin LTO, one codegen unit) is killed for memory
# on the 7 GB runners and on the machines this was developed on, and `prismpm build`
# spends its time in the LexLean checker, which the profile of the binary does not
# change.
install_prismpm() {
  local rev lexlean scratch
  rev=$(prismpm_rev)
  lexlean=$(lexlean_rev)
  if ! prismpm_installed_at "$rev" "$lexlean"; then
    rm -rf "$PRISMPM_ROOT"
    scratch=$(mktemp -d)
    fetch_rev https://github.com/UOR-Foundation/LexLean "$lexlean" "$scratch/lexlean"
    fetch_rev https://github.com/UOR-Foundation/PrismPM "$rev" "$scratch/prismpm"
    (cd "$scratch/lexlean" && cargo package -p lexlean --no-verify --allow-dirty)
    tar -xzf "$scratch"/lexlean/target/package/lexlean-*.crate -C "$scratch"
    rm -rf "$scratch/prismpm/vendor/lexlean"
    mv "$scratch"/lexlean-*/ "$scratch/prismpm/vendor/lexlean"
    (cd "$scratch/prismpm" && cargo install --path crates/prismpm --locked --debug --root "$PRISMPM_ROOT")
    printf 'prismpm %s\nlexlean %s' "$rev" "$lexlean" >"$PRISMPM_ROOT/pins"
    rm -rf "$scratch"
  fi
  prismpm_installed_at "$rev" "$lexlean"
  echo "$PRISMPM_ROOT/bin" >>"${GITHUB_PATH:-/dev/null}"
  "$PRISMPM_ROOT/bin/prismpm" --version
}

# sdk_copies_agree: the files the gateway copies from the PrismPM standard library and
# lexicon, `Foundation.Bytes` and `Foundation.Server.V1.Model`, are byte for byte the ones at
# the pinned revision, and so is the whole `prism.arch` lexicon package that
# gateway/lay-out-sdk.sh laid out from it (no subset is kept).
sdk_copies_agree() {
  local rev scratch status=0 file
  rev=$(prismpm_rev)
  scratch=$(mktemp -d)
  git -C "$scratch" init -q
  git -C "$scratch" remote add origin https://github.com/UOR-Foundation/PrismPM
  git -C "$scratch" sparse-checkout set stdlib/src/Foundation language/prism.arch
  git -C "$scratch" fetch -q --depth 1 --filter=blob:none origin "$rev"
  git -C "$scratch" checkout -q FETCH_HEAD
  for file in gateway/src/Foundation/Bytes.lex.tex gateway/src/Foundation/Server/V1/Model.lex.tex; do
    if cmp -s "$file" "$scratch/stdlib/src/${file#gateway/src/}"; then
      echo "$file equals the standard library's"
    else
      echo "$file differs from stdlib/src/${file#gateway/src/} at $rev" >&2
      status=1
    fi
  done
  if diff -rq gateway/language/prism.arch "$scratch/language/prism.arch"; then
    echo "gateway/language/prism.arch equals the SDK's"
  else
    echo "gateway/language/prism.arch differs from the SDK's at $rev" >&2
    status=1
  fi
  rm -rf "$scratch"
  return "$status"
}

# gateway_registry: assemble the local registry of the gateway workspace from the pins
# (gateway/registry.sh) and publish the CARGO_HOME that replaces crates-io with it as the
# step output `cargo_home`.
gateway_registry() {
  local log
  log=$(mktemp "${RUNNER_TEMP:-${TMPDIR:-/tmp}}/registry.XXXXXX")
  bash gateway/registry.sh | tee "$log"
  echo "cargo_home=$PWD/gateway/registry/cargo-home" >>"${GITHUB_OUTPUT:-/dev/null}"
  rm -f "$log"
}

# ---- the gateway job -----------------------------------------------------------------
# gateway/ is its own Cargo workspace, built against the registry that gateway/registry.sh
# assembles from the pins (CI.md, "The gateway job"). Every function below runs from the
# root of the repository and leaves it there.

# The CARGO_HOME of the gateway workspace: a directory source in place of crates-io.
gateway_cargo_home() {
  printf '%s\n' "$PWD/gateway/registry/cargo-home"
}

# gateway_lexlean ARGS...: `lexlean` in the gateway project.
gateway_lexlean() {
  (cd gateway && lexlean --color never "$@")
}

# gateway_fmt_check: every module of gateway/src that is the repository's own is canonical.
# `lexlean fmt --check` checks the whole closure, and the copies of PrismPM's standard
# library under gateway/src/Foundation are PrismPM's bytes (sdk_copies_agree holds them to
# the pin), which its formatter writes a little differently from the pinned LexLean's. So
# the project is formatted in a scratch copy and each module that is not a copy is compared.
gateway_fmt_check() {
  local scratch status=0 file
  scratch=$(mktemp -d)
  cp -r gateway/. "$scratch/"
  rm -rf "$scratch/registry" "$scratch/.lexlean" "$scratch/.prism"
  (cd "$scratch" && lexlean --color never fmt)
  while IFS= read -r file; do
    if cmp -s "gateway/$file" "$scratch/$file"; then
      echo "gateway/$file is canonical"
    else
      echo "gateway/$file is not canonical; run lexlean fmt in gateway/ and restore src/Foundation" >&2
      status=1
    fi
  done < <(cd gateway && find src -name '*.lex.tex' -not -path 'src/Foundation/*' | sort)
  rm -rf "$scratch"
  return "$status"
}

# gateway_cargo ARGS...: `cargo` in the gateway workspace, offline, against the registry.
gateway_cargo() {
  (cd gateway && CARGO_HOME=$(cd .. && gateway_cargo_home) cargo "$@")
}

# gateway_build: `prismpm build` in the gateway project, then lay the generated packages out
# and require the core to be the committed export, byte for byte, in both directions.
gateway_build() {
  local output id
  output=$(cd gateway && prismpm build) || { printf '%s\n' "$output"; return 1; }
  printf '%s\n' "$output"
  id=$(printf '%s\n' "$output" | sed -n 's/^build published: \([0-9a-f]\{64\}\)$/\1/p')
  if [[ -z $id ]]; then
    echo "prismpm build did not print 'build published: <id>'" >&2
    return 1
  fi
  gateway/lay-out.sh "gateway/.prism/build/$id"
  if diff -r gateway/lexlean_mcp_gateway_core gateway/generated/lexlean_mcp_gateway_core; then
    echo "gateway/generated/lexlean_mcp_gateway_core is the export of the sources"
  else
    echo "gateway/generated/lexlean_mcp_gateway_core differs from the export of the sources" >&2
    return 1
  fi
}

# gateway_ignored_tests PACKAGE EXPECTED [SKIP]: run the ignored tests of PACKAGE, but for the test
# SKIP that another job runs, and require that exactly EXPECTED of them ran and passed (an ignored
# test that is filtered out proves nothing).
gateway_ignored_tests() {
  local package=$1 expected=$2 skip=${3:-} log
  log=$(mktemp)
  gateway_cargo test -p "$package" --locked --offline -- --ignored ${skip:+--skip "$skip"} 2>&1 | tee "$log"
  local passed
  passed=$(sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' "$log" | paste -sd+ | bc)
  rm -f "$log"
  if [[ ${passed:-0} != "$expected" ]]; then
    echo "$package: $passed ignored tests passed, expected $expected" >&2
    return 1
  fi
}

# gateway_verify_fixtures: the verify fixtures (VP-07), by name, and require that the test ran: a
# name that matches nothing passes with no test.
gateway_verify_fixtures() {
  local log passed
  log=$(mktemp)
  gateway_cargo test -p gateway-tool-tests --test equality --locked --offline -- \
    --ignored --exact the_verify_fixtures_give_the_result_of_the_cli_vp_07 2>&1 | tee "$log"
  passed=$(sed -n 's/^test result: ok\. \([0-9]*\) passed.*/\1/p' "$log")
  rm -f "$log"
  if [[ $passed != 1 ]]; then
    echo "the verify fixtures: ${passed:-no} tests passed, expected 1" >&2
    return 1
  fi
}

# free_runner_disk: remove toolchains the hosted runner preinstalls and no job here uses, so
# the SDK image fits beside the builds of the gateway job (main ran out of space pulling it).
free_runner_disk() {
  sudo rm -rf /usr/share/dotnet /usr/local/lib/android /opt/ghc /opt/hostedtoolcache/CodeQL
  df -h /
}

# gateway_verify: `prismpm verify` in the gateway project, on a store that starts empty.
# gateway_verify: `prismpm verify` of the gateway, run inside the locked SDK image (the
# `sdk_image` digest of prismpm.lock), as bootstrap.yml runs `just vv`. `prismpm verify` takes
# its executables (lake, lean, rustc, rustfmt, cargo, timeout) from the SDK's signed inventory
# when it runs there, and stops with PP5008 on a bare runner, whose toolchain is not the pinned
# one. The binary that runs is the one install_prismpm built (PrismPM at .github/prismpm.rev with
# LexLean at MCP's pin), mounted read-only and called by its path, which is not on the SDK's
# PATH; the workspace is mounted at the same path, so the registry's directory source
# (gateway/registry/cargo-home/config.toml) resolves inside the container.
gateway_verify() {
  local image store work
  image=$(sed -n 's/.*"sdk_image":"\([^"]*\)".*/\1/p' prismpm.lock)
  if [[ ! $image =~ ^[a-z0-9.-]+(:[0-9]{1,5})?/[a-z0-9./_-]+@sha256:[0-9a-f]{64}$ ]]; then
    echo "prismpm.lock does not select an immutable SDK image" >&2
    return 64
  fi
  store=$(mktemp -d)
  work=$(mktemp -d)
  # The vectors of the draft tools name the profile `tiny`, which #8 provisions in a real store:
  # the replay starts from a store that holds the profiles the vectors were recorded with.
  # The operator's command is the one that provisions: it is built first, and the generator
  # calls it (gateway/target/debug/gateway-provision) for each profile of the vectors.
  gateway_cargo build -p gateway-binding --bin gateway-provision --locked --offline || return 1
  python3 -I gateway/vectors/generate.py provision "$store" || return 1
  docker run --rm \
    --user "$(id -u):$(id -g)" \
    --volume "$PWD:$PWD" \
    --volume "$PRISMPM_ROOT:$PRISMPM_ROOT:ro" \
    --volume "$store:$store" \
    --volume "$work:$work" \
    --workdir "$PWD/gateway" \
    --env HOME="$work" \
    --env TMPDIR="$work" \
    --env CARGO_HOME="$PWD/gateway/registry/cargo-home" \
    --env CARGO_NET_OFFLINE=true \
    --env LEXLEAN_MCP_STORE="$store" \
    "$image" "$PRISMPM_ROOT/bin/prismpm" verify
}

# ---- the conformance run of the gateway (AC-03 b, GW-12) -----------------------------
# The gateway is held to the server suite that UOR-Foundation/MCP pins, with MCP's own
# oracle: the checkout of MCP at .github/mcp.rev (the commit gateway/lay-out-mcp.sh lays the
# sources out from) supplies conformance/oracle and the specification schema, and nothing of
# it is copied here. The verdict is the oracle's: the job passes only when the failing
# scenarios are exactly the ones gateway/waiting.json names.

# The directory the conformance run keeps its checkout, the suite and its reports in.
gateway_conformance_dir() {
  printf '%s\n' "${GATEWAY_CONFORMANCE_DIR:-${RUNNER_TEMP:-${TMPDIR:-/tmp}}/gateway-conformance}"
}

# gateway_conformance_checkout: the MCP checkout at the pin, fetched once and refused
# unless its HEAD is the commit .github/mcp.rev names. Prints its path.
gateway_conformance_checkout() {
  local dir rev head
  dir=$(gateway_conformance_dir)/mcp
  rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' .github/mcp.rev)
  if [[ -z $rev ]]; then
    echo ".github/mcp.rev has no 40-digit rev" >&2
    return 1
  fi
  if [[ ! -d $dir/.git ]]; then
    fetch_rev https://github.com/UOR-Foundation/MCP "$rev" "$dir" >&2
  fi
  head=$(git -C "$dir" rev-parse HEAD)
  if [[ $head != "$rev" ]]; then
    echo "the MCP checkout $dir is at $head, the pin is $rev" >&2
    return 1
  fi
  for oracle in server-suite.mjs schema-oracle.mjs verdict.mjs recorder.mjs fetch-suite.sh; do
    test -f "$dir/conformance/oracle/$oracle" || { echo "MCP at $rev has no conformance/oracle/$oracle" >&2; return 1; }
  done
  printf '%s\n' "$dir"
}

# gateway_conformance_install: the checkout and the pinned suite, installed by MCP's own
# fetch-suite.sh (the exact commit, `npm ci`). MCP_CONFORMANCE_SUITE_ROOT and
# GATEWAY_MCP_CHECKOUT name them for later steps.
gateway_conformance_install() {
  local mcp suite
  mcp=$(gateway_conformance_checkout)
  suite=$(gateway_conformance_dir)/suite
  if [[ ! -f $suite/dist/index.js ]]; then
    rm -rf "$suite"
    bash "$mcp/conformance/oracle/fetch-suite.sh" "$suite"
  fi
  {
    echo "MCP_CONFORMANCE_SUITE_ROOT=$suite"
    echo "GATEWAY_MCP_CHECKOUT=$mcp"
  } >>"${GITHUB_ENV:-/dev/null}"
}

# gateway_conformance_build: the served program. `gateway-host` is the generated host of
# `prismpm build` (gateway/lay-out.sh) linked with the binding, built in the gateway
# workspace against the registry.
gateway_conformance_build() {
  gateway_cargo build --release --locked --offline -p gateway-host
  test -x gateway/target/release/gateway-host
}

# gateway_conformance_suite: MCP's server-suite.mjs over the served gateway, with the
# gateway's waiting list. Exit 1 while the failing scenarios are not exactly the list; the
# last lines name the scenarios and the issue each waits on.
gateway_conformance_suite() {
  local mcp out
  mcp=$(gateway_conformance_checkout)
  out=$(gateway_conformance_dir)/out
  mkdir -p "$out"
  node "$mcp/conformance/oracle/server-suite.mjs" \
    --suite-root "${MCP_CONFORMANCE_SUITE_ROOT:?the installed suite (gateway_conformance_install)}" \
    --out "$out" --waiting gateway/waiting.json \
    "$PWD/gateway/target/release/gateway-host" serve
}

# gateway_conformance_schema: every message of the recorded transcript against the
# 2026-07-28 schema MCP vendors at the pin.
gateway_conformance_schema() {
  local mcp schema
  mcp=$(gateway_conformance_checkout)
  schema=$(ls "$mcp"/crates/lifecycle-oracle/suite/mcp-spec-*/schema/2026-07-28/schema.json)
  node "$mcp/conformance/oracle/schema-oracle.mjs" \
    --suite-root "${MCP_CONFORMANCE_SUITE_ROOT:?the installed suite (gateway_conformance_install)}" \
    "$schema" "$(gateway_conformance_dir)/out/transcript.json"
}

# gateway_conformance_summary: the per-scenario table of the run, to the step summary and
# the log. A waiting scenario is a failure here, as in the oracle's report.
gateway_conformance_summary() {
  local report
  report="$(gateway_conformance_dir)/out/report.json"
  test -f "$report" || { echo "no report: the suite did not run" >&2; return 1; }
  node -e '
    const r = JSON.parse(require("fs").readFileSync(process.argv[1], "utf8"));
    const lines = ["| kind | scenario | status | checks passed | other |", "|---|---|---|---|---|"];
    for (const s of r.scenarios) lines.push(`| ${s.kind} | ${s.scenario} | ${s.status} | ${s.passed}/${s.checks} | ${Object.entries(s.other).map(([k, v]) => k + " " + v).join(", ")} |`);
    lines.push("", `required ${r.required.passing}/${r.required.scenarios} pass, not scored ${r.not_scored.passing}/${r.not_scored.scenarios} pass`);
    lines.push("", `${r.failing.length} scenarios FAIL (valid: ${r.valid}); the job gate, failing = gateway/waiting.json exactly: ${r.matches_waiting}. Waiting scenarios are failures, never passes.`);
    for (const [label, names] of [["failing and not named in waiting.json", r.unexplained], ["named as waiting but not failing", r.stale_waiting], ["skipped by the suite, not passing", r.skipped_by_the_suite]]) if (names.length) lines.push("", `${label}: ${names.join(", ")}`);
    for (const [label, names] of Object.entries(r.waits)) lines.push("", `${label}: ${names.join(", ")}`);
    console.log(lines.join("\n"));
  ' "$report" | tee -a "${GITHUB_STEP_SUMMARY:-/dev/null}"
}

# gateway_conformance_ignored: the tests of crates/model/tests/gateway_conformance.rs that
# need node and the installed suite. The workspace gate (`cargo test --workspace`, and
# `just vv` in the locked SDK image, which has neither) ignores them; this runs exactly
# those with --ignored and is green only when as many passed as the source ignores, so a run
# that selected none or skipped some is not green (the pattern of MCP's run_oracle_tests).
gateway_conformance_ignored() {
  local file=crates/model/tests/gateway_conformance.rs log expected passed status=0
  : "${MCP_CONFORMANCE_SUITE_ROOT:?the installed suite (gateway_conformance_install)}"
  : "${GATEWAY_MCP_CHECKOUT:?the MCP checkout at the pin (gateway_conformance_install)}"
  expected=$(grep -c '^#\[ignore' "$file" || true)
  if ((expected == 0)); then
    echo "$file ignores no test; the oracle tests are not where the job expects them" >&2
    return 1
  fi
  log=$(mktemp)
  cargo test --locked -p repo-model --test gateway_conformance -- --ignored >"$log" 2>&1 || status=$?
  cat "$log"
  if ((status != 0)); then
    rm -f "$log"
    return "$status"
  fi
  passed=$(sed -n 's/^test result: ok\. \([0-9][0-9]*\) passed; 0 failed.*/\1/p' "$log")
  rm -f "$log"
  if [[ $passed != "$expected" ]]; then
    echo "${passed:-0} passed of the ignored tests, expected $expected" >&2
    return 1
  fi
  echo "the $expected ignored gateway conformance tests ran and passed"
}

# check_node: the node on the runner satisfies the engines of the pinned conformance suite
# (^20.19.0 || >=22.12.0), which also runs MCP's oracle scripts.
check_node() {
  local version major minor
  version=$(node --version)
  version=${version#v}
  major=${version%%.*}
  minor=${version#*.}
  minor=${minor%%.*}
  echo "node $version"
  if ((major > 22 || (major == 22 && minor >= 12) || (major == 20 && minor >= 19))); then
    return 0
  fi
  echo "node $version does not satisfy ^20.19.0 || >=22.12.0" >&2
  return 1
}
