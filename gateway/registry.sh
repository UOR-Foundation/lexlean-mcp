#!/usr/bin/env bash
# The local registry of the gateway workspace (gateway/Cargo.toml), assembled from the
# pins and nothing else. Development links crates that are not published yet, so the
# gateway is its own Cargo workspace with its own Cargo.lock (the root workspace and the
# SDK's offline `just vv` stay free of them) and its CI job builds it against a
# directory source that replaces crates-io for that workspace only. No manifest has a
# Git or path dependency on an unpublished crate, deny.toml is unchanged, and no `.crate`
# file is committed: this script makes them from the pins.
#
# What it makes, under gateway/registry (not committed, .gitignore):
#   <name>-<version>/                the unpacked package of every crate Cargo.lock names,
#                                    each with the `.cargo-checksum.json` a directory source
#                                    reads, whose "package" is the checksum Cargo.lock records
#   cargo-home/config.toml           the replacement of crates-io by that directory
#
# Where each package comes from:
#   lexlean   `cargo package` of UOR-Foundation/LexLean at .github/lexlean.rev. Its SHA-256
#             is recorded in gateway/registry.expected and checked against the package made
#             now, and against the checksum Cargo.lock records for it, so a moved pin or a
#             different archive stops the build instead of changing it.
#   the rest  the crates.io closure of that package, downloaded from static.crates.io and
#             checked against the checksum in Cargo.lock.
#
# Usage:
#   gateway/registry.sh            build the registry; print the CARGO_HOME to use
#   gateway/registry.sh --lock     resolve gateway/Cargo.lock once (a temporary [patch] makes
#                                  lexlean resolvable; the lock then names it as a registry
#                                  package with its recorded checksum)
#   gateway/registry.sh --record   write gateway/registry.expected from a fresh package
#
# Release step (CI.md): at the release tag the crates are on crates.io, this directory
# source is deleted, and the same versions resolve from crates.io with the same lock.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
repo=$(cd "$here/.." && pwd)
registry=${GATEWAY_REGISTRY:-$here/registry}
expected_file=$here/registry.expected
mode=${1:-build}

# The crates that are not on crates.io yet, each at the revision of its pin file.
# name  pin file                package in the repository
pinned_crates=("lexlean|$repo/.github/lexlean.rev|https://github.com/UOR-Foundation/LexLean|lexlean")

pin_of() {
  local file=$1 rev
  rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$file")
  test -n "$rev" || { echo "registry: $file has no 40-digit 'rev = ' line" >&2; return 1; }
  printf '%s\n' "$rev"
}

# fetch_rev URL REV DEST: the tree of exactly REV, shallow.
fetch_rev() {
  local url=$1 rev=$2 dest=$3
  mkdir -p "$dest"
  git -C "$dest" init -q
  git -C "$dest" remote add origin "$url"
  git -C "$dest" fetch -q --depth 1 origin "$rev"
  git -C "$dest" checkout -q FETCH_HEAD
  test "$(git -C "$dest" rev-parse HEAD)" = "$rev"
}

# package_pinned NAME SCRATCH: make the .crate of one pinned crate, print its path.
package_pinned() {
  local name=$1 scratch=$2 entry pin url package rev
  for entry in "${pinned_crates[@]}"; do
    IFS='|' read -r n pin url package <<<"$entry"
    [[ $n == "$name" ]] || continue
    rev=$(pin_of "$pin")
    fetch_rev "$url" "$rev" "$scratch/$name-src"
    (cd "$scratch/$name-src" && cargo package -p "$package" --no-verify --allow-dirty >&2)
    ls "$scratch/$name-src"/target/package/"$name"-*.crate
    return 0
  done
  echo "registry: $name is not a pinned crate" >&2
  return 1
}

is_pinned() {
  local entry n
  for entry in "${pinned_crates[@]}"; do
    n=${entry%%|*}
    [[ $n == "$1" ]] && return 0
  done
  return 1
}

# expected_sha NAME: the recorded SHA-256 of the pinned crate's package.
expected_sha() {
  awk -v name="$1" '$1 == name { print $3 }' "$expected_file"
}

# lock_packages: `name version source checksum` for every registry package of Cargo.lock.
lock_packages() {
  awk '
    /^\[\[package\]\]/ { if (name != "") print name, version, source, checksum; name = version = source = checksum = "-" ; next }
    /^name = /    { gsub(/"/, "", $3); name = $3 }
    /^version = / { gsub(/"/, "", $3); version = $3 }
    /^source = /  { gsub(/"/, "", $3); source = $3 }
    /^checksum = / { gsub(/"/, "", $3); checksum = $3 }
    END { if (name != "") print name, version, source, checksum }
  ' "$here/Cargo.lock"
}

write_unpacked() {
  local crate=$1 name=$2 version=$3 sha=$4
  rm -rf "${registry:?}/$name-$version"
  mkdir -p "$registry"
  tar -xzf "$crate" -C "$registry"
  test -d "$registry/$name-$version"
  printf '{"files":{},"package":"%s"}\n' "$sha" >"$registry/$name-$version/.cargo-checksum.json"
}

case $mode in
  --record)
    scratch=$(mktemp -d)
    trap 'rm -rf "$scratch"' EXIT
    : >"$expected_file.new"
    for entry in "${pinned_crates[@]}"; do
      name=${entry%%|*}
      crate=$(package_pinned "$name" "$scratch")
      version=$(basename "$crate" .crate)
      version=${version#"$name"-}
      printf '%s %s %s\n' "$name" "$version" "$(sha256sum "$crate" | cut -d' ' -f1)" >>"$expected_file.new"
    done
    mv "$expected_file.new" "$expected_file"
    cat "$expected_file"
    ;;

  --lock)
    scratch=$(mktemp -d)
    trap 'rm -rf "$scratch"' EXIT
    patch=""
    for entry in "${pinned_crates[@]}"; do
      name=${entry%%|*}
      crate=$(package_pinned "$name" "$scratch")
      sha=$(sha256sum "$crate" | cut -d' ' -f1)
      test "$sha" = "$(expected_sha "$name")" || { echo "registry: the package of $name is $sha, not the expected $(expected_sha "$name")" >&2; exit 1; }
      mkdir -p "$scratch/unpacked"
      tar -xzf "$crate" -C "$scratch/unpacked"
      patch+="$name = { path = \"$(echo "$scratch"/unpacked/"$name"-*)\" }"$'\n'
    done
    # A copy of the workspace with a temporary [patch.crates-io]; the committed manifests
    # carry none.
    cp -r "$here" "$scratch/workspace"
    rm -rf "$scratch/workspace/registry" "$scratch/workspace/Cargo.lock"
    printf '\n[patch.crates-io]\n%s' "$patch" >>"$scratch/workspace/Cargo.toml"
    (cd "$scratch/workspace" && cargo generate-lockfile)
    # Name each pinned crate as a registry package with its recorded checksum.
    for entry in "${pinned_crates[@]}"; do
      name=${entry%%|*}
      sha=$(expected_sha "$name")
      awk -v name="$name" -v sha="$sha" '
        $0 == "name = \"" name "\"" { inblock = 1 }
        inblock && /^version = / {
          print
          print "source = \"registry+https://github.com/rust-lang/crates.io-index\""
          print "checksum = \"" sha "\""
          inblock = 0
          next
        }
        { print }
      ' "$scratch/workspace/Cargo.lock" >"$scratch/Cargo.lock.next"
      mv "$scratch/Cargo.lock.next" "$scratch/workspace/Cargo.lock"
    done
    cp "$scratch/workspace/Cargo.lock" "$here/Cargo.lock"
    echo "registry: wrote $here/Cargo.lock"
    ;;

  build)
    test -f "$here/Cargo.lock" || { echo "registry: $here/Cargo.lock is absent; run $0 --lock" >&2; exit 1; }
    scratch=$(mktemp -d)
    trap 'rm -rf "$scratch"' EXIT
    rm -rf "$registry"
    mkdir -p "$registry"
    declare -A made=()
    # The pinned crates first: a fresh package, checked against the record and the lock.
    for entry in "${pinned_crates[@]}"; do
      name=${entry%%|*}
      crate=$(package_pinned "$name" "$scratch")
      version=$(basename "$crate" .crate)
      version=${version#"$name"-}
      sha=$(sha256sum "$crate" | cut -d' ' -f1)
      want=$(expected_sha "$name")
      if [[ $sha != "$want" ]]; then
        echo "registry: the package of $name $version is $sha but gateway/registry.expected records $want" >&2
        exit 1
      fi
      locked=$(lock_packages | awk -v n="$name" -v v="$version" '$1 == n && $2 == v { print $4 }')
      if [[ $locked != "$sha" ]]; then
        echo "registry: Cargo.lock records $locked for $name $version, the package is $sha" >&2
        exit 1
      fi
      write_unpacked "$crate" "$name" "$version" "$sha"
      made[$name]=1
      echo "registry: $name $version $sha (packaged from its pin)"
    done
    # The crates.io closure, each file checked against the checksum of Cargo.lock.
    while read -r name version source checksum; do
      [[ $source == registry+* ]] || continue
      [[ -n ${made[$name]:-} ]] && continue
      crate=$scratch/$name-$version.crate
      curl --proto '=https' --tlsv1.2 --fail --silent --show-error --location \
        --output "$crate" "https://static.crates.io/crates/$name/$name-$version.crate"
      got=$(sha256sum "$crate" | cut -d' ' -f1)
      if [[ $got != "$checksum" ]]; then
        echo "registry: $name $version is $got, Cargo.lock records $checksum" >&2
        exit 1
      fi
      write_unpacked "$crate" "$name" "$version" "$checksum"
    done < <(lock_packages)
    mkdir -p "$registry/cargo-home"
    cat >"$registry/cargo-home/config.toml" <<EOF
# Written by gateway/registry.sh. A workspace built with this CARGO_HOME takes every
# crates-io package from the directory below, and from nowhere else.
[source.crates-io]
replace-with = "gateway-local"

[source.gateway-local]
directory = "$registry"
EOF
    echo "registry: $(find "$registry" -maxdepth 1 -mindepth 1 -type d -name '*-[0-9]*' | wc -l) packages under $registry"
    echo "registry: CARGO_HOME=$registry/cargo-home"
    ;;

  *)
    echo "usage: registry.sh [--lock|--record]" >&2
    exit 2
    ;;
esac
