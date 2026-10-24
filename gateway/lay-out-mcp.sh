#!/usr/bin/env bash
# Lay out the sources of UOR-Foundation/MCP at the commit of .github/mcp.rev beside the
# gateway's own: gateway/mcp/src (the LexLean modules) and gateway/language/mcp.core (the
# lexicon they use). Neither is committed (gateway/.gitignore): the gateway reads the
# commit the pin names, so the protocol core has one source, the MCP repository (R1).
#
# usage: lay-out-mcp.sh [MCP_CHECKOUT]
#   MCP_CHECKOUT  a checkout of UOR-Foundation/MCP; it must be at the pinned commit.
#                 Without it the commit is fetched into a temporary directory.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
pin_file=$here/../.github/mcp.rev
rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$pin_file")
test -n "$rev" || { echo "lay-out-mcp: $pin_file has no 40-digit rev" >&2; exit 1; }

checkout=${1:-}
scratch=
if [ -z "$checkout" ]; then
  scratch=$(mktemp -d)
  trap 'rm -rf "$scratch"' EXIT
  git -C "$scratch" init -q
  git -C "$scratch" remote add origin https://github.com/UOR-Foundation/MCP
  git -C "$scratch" fetch -q --depth 1 origin "$rev"
  git -C "$scratch" checkout -q FETCH_HEAD
  checkout=$scratch
fi
head=$(git -C "$checkout" rev-parse HEAD)
if [ "$head" != "$rev" ]; then
  echo "lay-out-mcp: $checkout is at $head, the pin is $rev" >&2
  exit 1
fi
if [ -n "$(git -C "$checkout" status --porcelain --untracked-files=no -- src language/mcp.core)" ]; then
  echo "lay-out-mcp: $checkout has changes in src or language/mcp.core" >&2
  exit 1
fi

rm -rf "$here/mcp" "$here/language/mcp.core"
mkdir -p "$here/mcp"
cp -r "$checkout/src" "$here/mcp/src"
cp -r "$checkout/language/mcp.core" "$here/language/mcp.core"
echo "lay-out-mcp: the sources of MCP $rev are in $here/mcp and $here/language/mcp.core"
