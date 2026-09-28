#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
"""Mirror the wiserone.com quote pool into quotes/quotes.json and .csv.

The website's pool is canonical (docs/POLICIES.md#corpus-changes); this
crate ships a copy so `wiserone daily` selects the same quote. The weekly
corpus-sync workflow runs this script and opens a pull request when the
copy changes, instead of CI failing until someone mirrors it by hand.

Both files are regenerated from the site's `quotes` array, in its order
(the order is the rotation, so it is never re-sorted). Output is
deterministic: the same pool always produces byte-identical files.

Usage:
  python3 scripts/sync_corpus.py            # fetch the site, rewrite files
  python3 scripts/sync_corpus.py --check    # exit 1 if the files would change
  python3 scripts/sync_corpus.py --source pool.json
"""

from __future__ import annotations

import argparse
import csv
import io
import json
import sys
import urllib.request
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
URL = "https://wiserone.com/quotes.json"
JSON_PATH = ROOT / "quotes" / "quotes.json"
CSV_PATH = ROOT / "quotes" / "quotes.csv"
# Fields every quote must carry, and the CSV column order.
JSON_FIELDS = ["quote_text", "author", "date_added", "image_url", "id", "pillar"]
CSV_FIELDS = ["id", "pillar", "quote_text", "author", "date_added", "image_url"]


def fail(message: str) -> None:
    raise SystemExit(f"[corpus-sync] {message}")


def fetch(source: str | None) -> list[dict]:
    if source:
        data = json.loads(Path(source).read_text(encoding="utf-8"))
    else:
        request = urllib.request.Request(URL, headers={"User-Agent": "wiserone-corpus-sync"})
        with urllib.request.urlopen(request, timeout=30) as response:
            data = json.load(response)
    quotes = data.get("quotes")
    if not isinstance(quotes, list) or not quotes:
        fail("the site returned no quotes; refusing to write an empty corpus")
    for quote in quotes:
        missing = [field for field in JSON_FIELDS if field not in quote]
        if missing:
            fail(f"quote {quote.get('id')!r} lacks {', '.join(missing)}")
    return quotes


def render_json(quotes: list[dict]) -> str:
    # Each entry keeps the site's own key order, so an unchanged pool
    # rewrites to identical bytes.
    return json.dumps({"quotes": quotes}, ensure_ascii=False, indent=2) + "\n"


def render_csv(quotes: list[dict]) -> str:
    buffer = io.StringIO()
    writer = csv.writer(buffer, lineterminator="\n")
    writer.writerow(CSV_FIELDS)
    for quote in quotes:
        writer.writerow([quote[field] for field in CSV_FIELDS])
    return buffer.getvalue()


def main(argv: list[str]) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument("--check", action="store_true", help="report drift, write nothing")
    parser.add_argument("--source", help="read the pool from a file instead of the site")
    args = parser.parse_args(argv)
    quotes = fetch(args.source)
    targets = {JSON_PATH: render_json(quotes), CSV_PATH: render_csv(quotes)}
    changed = [path for path, text in targets.items()
               if not path.exists() or path.read_text(encoding="utf-8") != text]
    if args.check:
        print(f"[corpus-sync] {len(quotes)} quotes; "
              + (f"would change {', '.join(p.name for p in changed)}" if changed else "no drift"))
        return 1 if changed else 0
    for path in changed:
        path.write_text(targets[path], encoding="utf-8")
    print(f"[corpus-sync] {len(quotes)} quotes; "
          + (f"updated {', '.join(p.name for p in changed)}" if changed else "already current"))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
