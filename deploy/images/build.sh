#!/usr/bin/env bash
# The images of the services (IM-01, IM-02), from the programs in RELEASE: the programs are built
# with the gateway's locked dependencies, so no step here resolves or compiles anything. Each image
# is built with no network and no RUN step, so nothing can reach it but what is listed here.
#
# Usage: deploy/images/build.sh RELEASE
# LEAN_ARCHIVE_DIR keeps the archive between runs; it is checked whichever way it arrived.
set -euo pipefail

release=${1:?usage: build.sh RELEASE}
here=$(cd "$(dirname "$0")" && pwd)
root=$(cd "$here/../.." && pwd)
work=$(mktemp -d)
trap 'rm -rf "$work"' EXIT

version=$(sed -n 's|^leanprover/lean4:v\(.*\)$|\1|p' "$root/gateway/lean-toolchain")
archive=lean-$version-linux.tar.zst
if ! grep -q "  $archive\$" "$here/lean-toolchain.sha256"; then
  echo "deploy/images/lean-toolchain.sha256 does not record $archive, the version gateway/lean-toolchain names" >&2
  exit 1
fi

cache=${LEAN_ARCHIVE_DIR:-$work/archive}
mkdir -p "$cache"
if [[ ! -f $cache/$archive ]]; then
  curl --fail --silent --show-error --location --retry 3 --output "$cache/$archive" \
    "https://github.com/leanprover/lean4/releases/download/v$version/$archive"
fi
(cd "$cache" && sha256sum --check --strict "$here/lean-toolchain.sha256")

build() {
  local name=$1
  local context=$work/$name
  shift
  mkdir -p "$context"
  for file in "$@"; do
    cp -a "$file" "$context/"
  done
  docker build --quiet --network none --tag "lexlean-mcp/$name:local" \
    --file "$here/$name.Containerfile" "$context"
}

toolchain=$work/toolchain
mkdir -p "$toolchain"
tar --zstd --extract --strip-components=1 --file "$cache/$archive" --directory "$toolchain"

build gateway "$release/gateway-host"
build engine-host "$release/gateway-engine-host"
build signer "$release/gateway-signer"
build verifier "$release/gateway-verifier-host" "$release/gateway-verifier" "$toolchain"
build collector "$here/collector.yaml"
