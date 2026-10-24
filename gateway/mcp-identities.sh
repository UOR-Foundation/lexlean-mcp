#!/usr/bin/env bash
# AC-03 restated for the gateway (lexlean-mcp#7, GW-11). The gateway's core cannot carry the
# digest of MCP's fixture core, because it is another program over the same modules. What
# can be held to the pin is the modules themselves:
#   1. every file laid out under gateway/mcp/src and gateway/language/mcp.core equals, byte
#      for byte, the file of UOR-Foundation/MCP at .github/mcp.rev, and no file is missing
#      or extra;
#   2. the lexicon package mcp.core has, in gateway/lexlean.lock, the tree hash that MCP's
#      lexlean.lock records for it at the pin.
# MCP's own lock is made by another LexLean commit than this repository's, and the module
# prefix differs (UorMcp, LexLeanMcpGateway), so the semantic ID of a module is not
# comparable across the two; the source and the lexicon tree are.
#
# usage: mcp-identities.sh [MCP_CHECKOUT]   (a checkout at the pin; fetched when absent)
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$here/../.github/mcp.rev")
test -n "$rev" || { echo "mcp-identities: .github/mcp.rev has no 40-digit rev" >&2; exit 1; }

checkout=${1:-}
if [ -z "$checkout" ]; then
  scratch=$(mktemp -d)
  trap 'rm -rf "$scratch"' EXIT
  git -C "$scratch" init -q
  git -C "$scratch" remote add origin https://github.com/UOR-Foundation/MCP
  git -C "$scratch" fetch -q --depth 1 origin "$rev"
  git -C "$scratch" checkout -q FETCH_HEAD
  checkout=$scratch
fi
test "$(git -C "$checkout" rev-parse HEAD)" = "$rev" || { echo "mcp-identities: $checkout is not at $rev" >&2; exit 1; }

status=0
diff -r "$checkout/src" "$here/mcp/src" || status=1
diff -r "$checkout/language/mcp.core" "$here/language/mcp.core" || status=1
if ((status != 0)); then
  echo "mcp-identities: the laid-out MCP files differ from the pin $rev" >&2
  exit 1
fi
echo "mcp-identities: src and language/mcp.core equal the files of MCP $rev"

# The tree hash of a lexicon package in a lock: the `tree_sha256` of its [[package]] block.
tree_hash() {
  awk -v id="$2" '
    /^\[\[package\]\]/ { inblock = 0 }
    $0 == "id = \"" id "\"" { inblock = 1 }
    inblock && /^tree_sha256 = / { gsub(/"/, "", $3); print $3; exit }
  ' "$1"
}
theirs=$(tree_hash "$checkout/lexlean.lock" mcp.core)
ours=$(tree_hash "$here/lexlean.lock" mcp.core)
if [[ -z $theirs || $theirs != "$ours" ]]; then
  echo "mcp-identities: mcp.core has tree hash '$ours' here and '$theirs' in MCP's lexlean.lock at $rev" >&2
  exit 1
fi
echo "mcp-identities: mcp.core has the tree hash $ours in both locks"
