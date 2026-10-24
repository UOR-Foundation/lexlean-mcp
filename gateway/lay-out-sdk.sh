#!/usr/bin/env bash
# Lay out the lexicon package `prism.arch` of UOR-Foundation/PrismPM at the commit of
# .github/prismpm.rev as gateway/language/prism.arch, the whole package and nothing else.
# It is not committed (gateway/.gitignore): the gateway reads the commit the pin names, as it
# reads the sources of MCP (lay-out-mcp.sh), so there is no hand-made copy to drift from the
# SDK's tree. The two Foundation modules the gateway imports are whole files, committed, and
# held byte for byte to the same pin by sdk_copies_agree (.github/lexlean-ci/lib.sh).
#
# usage: lay-out-sdk.sh [PRISMPM_CHECKOUT]
#   PRISMPM_CHECKOUT  a checkout of UOR-Foundation/PrismPM at the pinned commit (with
#                     language/prism.arch). Without it the commit is fetched, sparsely, into a
#                     temporary directory.
set -euo pipefail

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
pin_file=$here/../.github/prismpm.rev
rev=$(sed -n 's/^rev = \([0-9a-f]\{40\}\)$/\1/p' "$pin_file")
test -n "$rev" || { echo "lay-out-sdk: $pin_file has no 40-digit rev" >&2; exit 1; }

checkout=${1:-}
if [ -z "$checkout" ]; then
  scratch=$(mktemp -d)
  trap 'rm -rf "$scratch"' EXIT
  git -C "$scratch" init -q
  git -C "$scratch" remote add origin https://github.com/UOR-Foundation/PrismPM
  git -C "$scratch" sparse-checkout set language/prism.arch
  git -C "$scratch" fetch -q --depth 1 --filter=blob:none origin "$rev"
  git -C "$scratch" checkout -q FETCH_HEAD
  checkout=$scratch
fi
head=$(git -C "$checkout" rev-parse HEAD)
if [ "$head" != "$rev" ]; then
  echo "lay-out-sdk: $checkout is at $head, the pin is $rev" >&2
  exit 1
fi
if [ -n "$(git -C "$checkout" status --porcelain --untracked-files=no -- language/prism.arch)" ]; then
  echo "lay-out-sdk: $checkout has changes in language/prism.arch" >&2
  exit 1
fi
test -f "$checkout/language/prism.arch/lexicon.toml" || { echo "lay-out-sdk: $checkout has no language/prism.arch" >&2; exit 1; }

rm -rf "$here/language/prism.arch"
mkdir -p "$here/language"
cp -r "$checkout/language/prism.arch" "$here/language/prism.arch"
echo "lay-out-sdk: language/prism.arch of PrismPM $rev is in $here/language/prism.arch"
