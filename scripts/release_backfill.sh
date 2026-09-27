#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Publish or reformat the GitHub release for a tag that predates the
# current release workflow (v0.0.1 to v0.0.5).
#
# Those versions were uploaded to crates.io before tag-driven releases, so
# no binaries were built for them. Rebuilding 2024 sources with today's
# toolchain would not reproduce what users installed; the artefact they did
# install is the .crate on crates.io. This script attaches exactly that
# file, after checking its SHA-256 against the checksum crates.io records,
# with a SHA256SUMS, and composes the notes with scripts/release_notes.py.
#
# The tag must already be pushed and pass scripts/release_preflight.sh.
#
# Usage:
#   scripts/release_backfill.sh <tag> [--latest]
set -euo pipefail
shopt -s inherit_errexit

PROJECT=wiserone
REPO=sebastienrousseau/wiserone
ROOT=$(cd "$(dirname "$0")/.." && pwd)

fail() { echo "[backfill] FAIL: $*" >&2; exit 1; }

sha256() { shasum -a 256 "$1" | cut -d' ' -f1; }

fetch_crate() {
  local version=$1 dir=$2 agent="$PROJECT-backfill ($REPO)" expected actual
  expected=$(curl -sSf -A "$agent" "https://crates.io/api/v1/crates/$PROJECT/$version" \
    | jq -r .version.checksum) || fail "crates.io does not serve $PROJECT $version"
  curl -sSfL -A "$agent" -o "$dir/$PROJECT-$version.crate" \
    "https://static.crates.io/crates/$PROJECT/$PROJECT-$version.crate"
  actual=$(sha256 "$dir/$PROJECT-$version.crate")
  [ "$actual" = "$expected" ] || fail "downloaded crate $actual != crates.io checksum $expected"
  (cd "$dir" && printf '%s  ./%s\n' "$actual" "$PROJECT-$version.crate" >SHA256SUMS)
}

publish() {
  local tag=$1 dir=$2 latest=$3 title
  title=$(python3 "$ROOT/scripts/release_notes.py" --tag "$tag" --title)
  python3 "$ROOT/scripts/release_notes.py" --tag "$tag" --sums "$dir/SHA256SUMS" --out "$dir/notes.md"
  if gh release view "$tag" -R "$REPO" >/dev/null 2>&1; then
    gh release edit "$tag" -R "$REPO" --title "$title" --notes-file "$dir/notes.md" \
      --draft=false --latest="$latest"
    gh release upload "$tag" -R "$REPO" --clobber "$dir/$PROJECT-${tag#v}.crate" "$dir/SHA256SUMS"
  else
    gh release create "$tag" -R "$REPO" --verify-tag --title "$title" --notes-file "$dir/notes.md" \
      --latest="$latest" "$dir/$PROJECT-${tag#v}.crate" "$dir/SHA256SUMS"
  fi
}

main() {
  local tag=${1:-} latest=false
  [[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "usage: $0 <vX.Y.Z> [--latest]"
  [ "${2:-}" = --latest ] && latest=true
  WORK=$(mktemp -d)
  trap 'rm -rf "$WORK"' EXIT
  local dir=$WORK
  fetch_crate "${tag#v}" "$dir"
  publish "$tag" "$dir" "$latest"
  echo "[backfill] $tag: published $(python3 "$ROOT/scripts/release_notes.py" --tag "$tag" --title)"
}

main "$@"
