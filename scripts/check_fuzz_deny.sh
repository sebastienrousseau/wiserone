#!/usr/bin/env bash
# cargo-deny for the fuzz crate. fuzz/deny.toml is ../deny.toml plus a
# fuzz-only tail (the libfuzzer-sys NCSA exception); fail if the shared
# part has drifted, then run the check. --locked also fails a stale
# fuzz/Cargo.lock, which a root version bump leaves behind.
set -euo pipefail
cd "$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

marker='# --- fuzz only'
grep -q "^$marker" fuzz/deny.toml || { echo "fuzz/deny.toml lacks the '$marker' line" >&2; exit 1; }
shared=$(sed "/^$marker/,\$d" fuzz/deny.toml)
if [ "$shared" != "$(cat deny.toml)" ]; then
  echo "fuzz/deny.toml has drifted from deny.toml above the '$marker' line:" >&2
  diff <(printf '%s\n' "$shared") deny.toml >&2 || true
  exit 1
fi
cd fuzz
cargo deny --locked check
