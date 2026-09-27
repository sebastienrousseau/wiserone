#!/usr/bin/env bash
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
#
# Published-release audit (the "Published Release Audit" rule in
# ~/Code/AGENTS.md). Reads a release back from where it was published and
# never trusts the workflow's own result. It fails unless:
#
#   - the tag on GitHub is annotated, verified, says "wiserone v<VERSION>"
#     and targets --expect <sha> when given;
#   - the release is published (not a draft), titled "wiserone <VERSION>",
#     and its body has Highlights, What's Changed, Checksums and the
#     Full Changelog line;
#   - every asset downloaded from the release matches SHA256SUMS, and every
#     line of SHA256SUMS appears in the release body;
#   - crates.io serves <VERSION>, and a .crate asset, when attached, is
#     byte-identical to the one crates.io serves.
#
# Usage:
#   scripts/release_audit.sh <tag> [--expect <sha>]
set -euo pipefail
shopt -s inherit_errexit

PROJECT=wiserone
REPO=sebastienrousseau/wiserone
ROOT=$(cd "$(dirname "$0")/.." && pwd)

fail() { echo "[audit] FAIL: $*" >&2; exit 1; }
ok() { echo "[audit] ok: $*" >&2; }

audit_tag() {
  local tag=$1 expect=$2 target="" verified subject
  read -r target verified subject < <(
    gh api "repos/$REPO/git/ref/tags/$tag" -q 'select(.object.type=="tag") | .object.sha' \
      | xargs -I{} gh api "repos/$REPO/git/tags/{}" \
          -q '.object.sha + " " + (.verification.verified|tostring) + " " + (.message|split("\n")[0])') || true
  [ -n "${target:-}" ] || fail "$tag on GitHub is missing or not annotated"
  [ "$verified" = true ] || fail "$tag on GitHub does not verify"
  [ "$subject" = "$PROJECT $tag" ] || fail "$tag message is '$subject'"
  if [ -n "$expect" ] && [ "$target" != "$expect" ]; then
    fail "$tag on GitHub targets $target, expected $expect"
  fi
  ok "remote tag $tag: annotated, verified, '$subject', $target"
}

audit_body() {
  local tag=$1 body=$2 section
  for section in "## Highlights ⭐️" "## What's Changed" "## Checksums" "**Full Changelog**: "; do
    grep -qF -- "$section" <<<"$body" || fail "release body lacks '$section'"
  done
  ok "release body has every section"
}

audit_release() {
  local tag=$1 dir=$2 json
  json=$(gh release view "$tag" -R "$REPO" --json name,isDraft,body)
  [ "$(jq -r .isDraft <<<"$json")" = false ] || fail "$tag release is still a draft"
  [ "$(jq -r .name <<<"$json")" = "$PROJECT ${tag#v}" ] || fail "$tag release title is '$(jq -r .name <<<"$json")'"
  ok "release '$PROJECT ${tag#v}' is published"
  audit_body "$tag" "$(jq -r .body <<<"$json")"
  gh release download "$tag" -R "$REPO" -D "$dir" --clobber
  [ -s "$dir/SHA256SUMS" ] || fail "$tag has no SHA256SUMS asset"
  (cd "$dir" && shasum -a 256 -c --quiet SHA256SUMS) \
    || fail "an asset does not match SHA256SUMS"
  local line
  while IFS= read -r line; do
    grep -qF -- "$line" <<<"$(jq -r .body <<<"$json")" || fail "release body lacks checksum '$line'"
  done <"$dir/SHA256SUMS"
  ok "$(wc -l <"$dir/SHA256SUMS" | tr -d ' ') asset(s) match SHA256SUMS and the release body"
}

audit_crate() {
  local version=$1 dir=$2 agent="$PROJECT-audit ($REPO)"
  curl -sSf -A "$agent" -o /dev/null "https://crates.io/api/v1/crates/$PROJECT/$version" \
    || fail "crates.io does not serve $PROJECT $version"
  ok "crates.io serves $PROJECT $version"
  local asset="$dir/$PROJECT-$version.crate"
  [ -f "$asset" ] || return 0
  curl -sSfL -A "$agent" -o "$dir/registry.crate" \
    "https://static.crates.io/crates/$PROJECT/$PROJECT-$version.crate"
  cmp -s "$asset" "$dir/registry.crate" || fail "the .crate asset differs from crates.io's"
  ok "the .crate asset is byte-identical to crates.io's"
}

main() {
  local tag=${1:-} expect=""
  [[ "$tag" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]] || fail "usage: $0 <vX.Y.Z> [--expect <sha>]"
  shift
  [ "${1:-}" = --expect ] && expect=$(git -C "$ROOT" rev-parse "$2^{commit}")
  WORK=$(mktemp -d)
  trap 'rm -rf "$WORK"' EXIT
  local dir=$WORK
  audit_tag "$tag" "$expect"
  audit_release "$tag" "$dir"
  audit_crate "${tag#v}" "$dir"
  echo "[audit] PASS: $tag"
}

main "$@"
