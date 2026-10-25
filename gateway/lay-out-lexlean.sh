#!/usr/bin/env bash
# Lay out the parts of LexLean that the tests of the tools (gateway/tool-tests, issue #10) compare
# the tools with, at the commit of gateway/mcp/lexlean.rev: the negative fixtures, the examples
# and the source packages the examples import. Nothing here is committed: the projects are
# LexLean's, and the fixture register of crates/negative-fixtures records their digests and trees
# at the root project's pin, not at this one (FIXTURES.md, TOOLS.md).
#
# usage: lay-out-lexlean.sh DESTINATION
#   DESTINATION   an absent or empty directory; the tests read it as LEXLEAN_SOURCES
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
dest=${1:?usage: lay-out-lexlean.sh DESTINATION}
rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$here/mcp/lexlean.rev")
test -n "$rev" || { echo "lay-out-lexlean: $here/mcp/lexlean.rev has no 40-digit rev (run gateway/lay-out-mcp.sh)" >&2; exit 1; }

if [ -e "$dest" ] && [ -n "$(ls -A "$dest")" ]; then
  echo "lay-out-lexlean: $dest is not empty" >&2
  exit 1
fi
mkdir -p "$dest"
git -C "$dest" init -q
git -C "$dest" remote add origin https://github.com/UOR-Foundation/LexLean
git -C "$dest" sparse-checkout set tests/negative examples stdlib std-models std-reasoning lexlean-workspace.toml
git -C "$dest" fetch -q --depth 1 origin "$rev"
git -C "$dest" checkout -q FETCH_HEAD
head=$(git -C "$dest" rev-parse HEAD)
if [ "$head" != "$rev" ]; then
  echo "lay-out-lexlean: $dest is at $head, the pin is $rev" >&2
  exit 1
fi
for path in tests/negative examples stdlib; do
  test -d "$dest/$path" || { echo "lay-out-lexlean: $path is absent at $rev" >&2; exit 1; }
done
echo "lay-out-lexlean: LexLean $rev is in $dest ($(ls "$dest/tests/negative" | wc -l) fixtures, $(ls "$dest/examples" | wc -l) examples)"
