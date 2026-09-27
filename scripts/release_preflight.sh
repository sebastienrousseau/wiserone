#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Blocking release preflight (the "Release Preflight Is Blocking" rule in
# ~/Code/AGENTS.md). Run it before pushing a release tag, and release.yml
# runs it again before anything is built or published. It fails unless:
#
#   - the tag is an annotated tag, cryptographically signed, whose first
#     message line is exactly "wiserone v<VERSION>";
#   - the tag targets the intended commit (--expect <sha>);
#   - Cargo.toml and the Cargo.lock root package at that commit carry
#     <VERSION>, and any README install snippet and CITATION.cff name it;
#   - docs/releases/v<VERSION>.md holds Highlights in the release format;
#   - for an unpublished version, the crates.io dry-run archive contains
#     the manifest, README and licences, and builds.
#
# Signatures are checked with `git verify-tag` locally, or through the
# GitHub API with --remote (CI has no allowed-signers file; GitHub checks
# the tag against the signing keys registered on the account).
#
# Usage:
#   scripts/release_preflight.sh <tag> [--expect <sha>] [--remote]
set -euo pipefail
shopt -s inherit_errexit

PROJECT=wiserone
REPO=sebastienrousseau/wiserone
ROOT=$(cd "$(dirname "$0")/.." && pwd)

fail() { echo "[preflight] FAIL: $*" >&2; exit 1; }
ok() { echo "[preflight] ok: $*" >&2; }

# Prints "<target sha> <verified> <first message line>" from the local tag.
tag_facts_local() {
  local tag=$1
  [ "$(git -C "$ROOT" cat-file -t "refs/tags/$tag" 2>/dev/null)" = tag ] \
    || fail "$tag is missing or not an annotated tag"
  local verified=false
  git -C "$ROOT" verify-tag "$tag" >/dev/null 2>&1 && verified=true
  printf '%s %s %s\n' "$(git -C "$ROOT" rev-parse "$tag^{commit}")" "$verified" \
    "$(git -C "$ROOT" for-each-ref "refs/tags/$tag" --format='%(contents:subject)')"
}

# Same facts, read from the tag object GitHub holds.
tag_facts_remote() {
  local tag=$1 ref
  ref=$(gh api "repos/$REPO/git/ref/tags/$tag" -q '.object.type + " " + .object.sha') \
    || fail "$tag is not on GitHub"
  [ "${ref%% *}" = tag ] || fail "$tag on GitHub is not an annotated tag"
  gh api "repos/$REPO/git/tags/${ref#* }" \
    -q '.object.sha + " " + (.verification.verified|tostring) + " " + (.message|split("\n")[0])'
}

check_tag() {
  local tag=$1 expect=$2 mode=$3 facts target verified subject
  facts=$("tag_facts_$mode" "$tag")
  read -r target verified subject <<<"$facts"
  [ "$verified" = true ] || fail "$tag is not signed, or its signature does not verify"
  ok "$tag is annotated and signed"
  [ "$subject" = "$PROJECT $tag" ] || fail "$tag message is '$subject', expected '$PROJECT $tag'"
  ok "message '$subject'"
  if [ -n "$expect" ]; then
    [ "$target" = "$(git -C "$ROOT" rev-parse "$expect^{commit}")" ] \
      || fail "$tag targets $target, expected $expect"
    ok "targets $target"
  fi
  echo "$target"
}

check_versions() {
  local commit=$1 version=$2 manifest lock
  manifest=$(git -C "$ROOT" show "$commit:Cargo.toml" | sed -n 's/^version = "\(.*\)"/\1/p' | head -1)
  [ "$manifest" = "$version" ] || fail "Cargo.toml at $commit says $manifest, not $version"
  lock=$(git -C "$ROOT" show "$commit:Cargo.lock" \
    | awk -v n="$PROJECT" '$0=="name = \"" n "\""{getline; gsub(/version = |"/,""); print; exit}')
  [ "$lock" = "$version" ] || fail "Cargo.lock root package at $commit says $lock, not $version"
  local stale
  stale=$(git -C "$ROOT" show "$commit:README.md" \
    | grep -oE "$PROJECT = \"[0-9]+\.[0-9]+\.[0-9]+\"" | grep -v "\"$version\"" || true)
  [ -z "$stale" ] || fail "README at $commit has a stale install snippet: $stale"
  local cff
  cff=$(git -C "$ROOT" show "$commit:CITATION.cff" 2>/dev/null | sed -n 's/^version: //p')
  [ -z "$cff" ] || [ "$cff" = "$version" ] || fail "CITATION.cff at $commit says $cff, not $version"
  ok "Cargo.toml, Cargo.lock, README and CITATION.cff carry $version"
}

published() {
  [ "$(curl -sS -o /dev/null -w '%{http_code}' -A "$PROJECT-preflight ($REPO)" \
    "https://crates.io/api/v1/crates/$PROJECT/$1")" = 200 ]
}

package_check() {
  local list
  list=$(cd "$1" && cargo package --locked --list --allow-dirty)
  for f in Cargo.toml Cargo.toml.orig Cargo.lock README.md LICENSE-APACHE LICENSE-MIT; do
    grep -qx "$f" <<<"$list" || fail "the crates.io archive lacks $f"
  done
  (cd "$1" && cargo publish --dry-run --locked --quiet) || fail "cargo publish --dry-run failed"
}

# Packages the tag's own tree, so the archive checked is the one that
# would be uploaded, not whatever is in the working tree.
check_package() {
  local commit=$1 version=$2 dir list
  if published "$version"; then
    ok "$PROJECT $version is already on crates.io; archive check skipped"
    return
  fi
  dir=$(mktemp -d)/src
  git -C "$ROOT" worktree add --detach -q "$dir" "$commit"
  local status=0
  package_check "$dir" || status=$?
  git -C "$ROOT" worktree remove --force "$dir"
  [ "$status" = 0 ] || exit "$status"
  ok "crates.io dry-run archive holds the manifest, README and licences, and builds"
}

main() {
  local tag=${1:-} expect="" mode=local
  [[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "usage: $0 <vX.Y.Z> [--expect <sha>] [--remote]"
  shift
  while [ $# -gt 0 ]; do
    case $1 in
      --expect) expect=$2; shift 2 ;;
      --remote) mode=remote; shift ;;
      *) fail "unknown argument $1" ;;
    esac
  done
  local commit
  commit=$(check_tag "$tag" "$expect" "$mode")
  check_versions "$commit" "${tag#v}"
  python3 "$ROOT/scripts/release_notes.py" --check >/dev/null
  python3 - "$ROOT" "$tag" <<'PY' || fail "docs/releases/$tag.md is missing or malformed"
import sys; sys.path.insert(0, sys.argv[1] + "/scripts")
import release_notes; release_notes.highlights(sys.argv[2])
PY
  ok "docs/releases/$tag.md holds the Highlights"
  check_package "$commit" "${tag#v}"
  echo "[preflight] PASS: $tag"
}

main "$@"
