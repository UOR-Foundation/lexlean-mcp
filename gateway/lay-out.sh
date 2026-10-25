#!/usr/bin/env bash
# Lay the packages `prismpm build` generated beside the binding crate: the core package and
# the two host packages, from the build's `server/` directory into the consumer workspace
# (gateway/Cargo.toml). None of the three is committed except the core, whose committed
# export (gateway/generated) the gateway job compares with the laid-out one.
#
# usage: lay-out.sh BUILD_DIRECTORY
#   BUILD_DIRECTORY   gateway/.prism/build/<id>, the id `prismpm build` prints
set -euo pipefail

build=${1:?usage: lay-out.sh BUILD_DIRECTORY}
here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
server="$build/server"
test -d "$server" || { echo "lay-out: $server is not a server build" >&2; exit 1; }

for package in lexlean_mcp_gateway_core gateway-host-api gateway-host; do
  test -d "$server/$package" || { echo "lay-out: $server/$package is absent" >&2; exit 1; }
  rm -rf "${here:?}/$package"
  cp -r "$server/$package" "$here/$package"
done
echo "lay-out: the generated packages of $build are in $here"
