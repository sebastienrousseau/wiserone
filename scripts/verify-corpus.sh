#!/usr/bin/env bash
set -euo pipefail

# Checks the corpus this crate ships against the pool wiserone.com
# publishes.
#
# Structural validation lives in tests/test_corpus.rs, which runs on
# every `cargo test`. This covers the one thing a test cannot: whether
# the corpus still matches the site. The two drifted invisibly once
# already, in the Swift app, because nothing compared them.
#
# A mismatch fails. An unreachable endpoint warns, so a flaky network
# does not redden an otherwise good build.

# Resolve the repo root from this script's own location rather than
# from `git rev-parse`, which yields an empty string — and so `cd ""` —
# when the script is invoked by absolute path from outside a checkout.
script_dir="$(cd "$(dirname "$0")" && pwd)"
cd "$script_dir/.."

CORPUS="${1:-quotes/quotes.json}"
UPSTREAM="${WISERONE_POOL_URL:-https://wiserone.com/quotes.json}"

tmp="$(mktemp)"
trap 'rm -f "$tmp"' EXIT INT TERM

if ! curl -fsSL --max-time 20 "$UPSTREAM" -o "$tmp" 2>/dev/null; then
  echo "WARNING: could not fetch $UPSTREAM; drift against the site is unverified."
  exit 0
fi

python3 - "$CORPUS" "$tmp" <<'PY'
import json, sys, pathlib

def load(p):
    q = json.loads(pathlib.Path(p).read_text())["quotes"]
    q.sort(key=lambda x: x.get("id", 1 << 30))
    return [(x.get("id"), x.get("quote_text")) for x in q]

local, upstream = load(sys.argv[1]), load(sys.argv[2])
if local == upstream:
    print(f"Corpus matches wiserone.com: {len(local)} quotes, no drift.")
    sys.exit(0)

print("Corpus has drifted from wiserone.com:")
print(f"  bundled {len(local)} quotes, site {len(upstream)}")
lt, ut = {t for _, t in local}, {t for _, t in upstream}
for t in list(ut - lt)[:5]:
    print(f"  + on the site, missing here: {t[:64]}")
for t in list(lt - ut)[:5]:
    print(f"  - bundled here, gone from the site: {t[:64]}")
if lt == ut:
    print("  same quotes, different order — `daily` will disagree with the site")
sys.exit(1)
PY
