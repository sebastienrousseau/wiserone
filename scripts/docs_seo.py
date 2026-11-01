#!/usr/bin/env python3
# SPDX-FileCopyrightText: 2024-2026 Sebastien Rousseau
# SPDX-License-Identifier: Apache-2.0 OR MIT
"""Make the published rustdoc tree crawlable without duplicates.

rustdoc pages carry no canonical link, and its search box rewrites the
URL to `<page>?search=<term>`. Search engines followed those URLs and
filed hundreds of them as "Duplicate without user-selected canonical";
items removed in a release kept being recrawled as 404s because nothing
listed the pages that do exist.

Run after `cargo doc`, on the directory the Pages workflow uploads:

* every HTML page gets `<link rel="canonical">` naming its clean URL;
* `robots.txt` disallows `?search=` URLs and the `/src/` source views,
  and names the sitemap;
* `sitemap.xml` lists every current item page (no source views, no
  rustdoc redirect stubs, no help or settings pages).

`--check` verifies a tree that has already been processed and exits 1 on
any gap, so CI can prove the deploy artefact is complete.

Usage: docs_seo.py [--check] DOC_DIR BASE_URL
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from urllib.parse import urljoin
from xml.sax.saxutils import escape

CANONICAL = '<link rel="canonical" href="{url}">'
REDIRECT = re.compile(
    r'http-equiv="refresh"\s+content="\d+;\s*url=([^"]+)"', re.IGNORECASE
)
SKIP_PAGES = {"help.html", "settings.html", "all.html"}
SKIP_DIRS = {"src", "static.files", "search.index", "trait.impl", "type.impl"}
ROBOTS = """User-agent: *
Disallow: /*?search=
Disallow: /*&search=
Disallow: /src/
Allow: /

Sitemap: {base}sitemap.xml
"""


def page_url(base: str, rel: Path) -> str:
    """Clean public URL for a page, `index.html` folded into its dir."""
    path = rel.as_posix()
    if path == "index.html" or path.endswith("/index.html"):
        path = path[: -len("index.html")]
    return base + path


def html_pages(doc_dir: Path) -> list[Path]:
    """Every HTML file under `doc_dir`, relative and sorted."""
    return sorted(p.relative_to(doc_dir) for p in doc_dir.rglob("*.html"))


def canonical_for(text: str, url: str) -> str:
    """The page's own URL, or its target if it is a meta-refresh stub.

    rustdoc writes such stubs for re-exported items; a stub declaring
    itself canonical would contradict its redirect.
    """
    match = REDIRECT.search(text)
    return urljoin(url, match.group(1)) if match else url


def in_sitemap(rel: Path, text: str) -> bool:
    """Whether a page is a current, indexable item page."""
    if rel.parts[0] in SKIP_DIRS or rel.name in SKIP_PAGES:
        return False
    return REDIRECT.search(text) is None


def add_canonical(text: str, url: str) -> str:
    """`text` with a canonical link at the top of `<head>`, once."""
    if 'rel="canonical"' in text:
        return text
    head = text.find("<head>")
    if head < 0:
        return text
    cut = head + len("<head>")
    return text[:cut] + CANONICAL.format(url=url) + text[cut:]


def render_sitemap(urls: list[str]) -> str:
    """A sitemaps.org document listing `urls`."""
    rows = "".join(f"<url><loc>{escape(u)}</loc></url>\n" for u in urls)
    return (
        '<?xml version="1.0" encoding="UTF-8"?>\n'
        '<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">\n'
        f"{rows}</urlset>\n"
    )


def process(doc_dir: Path, base: str) -> list[str]:
    """Add canonicals in place and return the sitemap URLs."""
    urls = []
    for rel in html_pages(doc_dir):
        path = doc_dir / rel
        text = path.read_text(encoding="utf-8")
        url = page_url(base, rel)
        updated = add_canonical(text, canonical_for(text, url))
        if updated != text:
            path.write_text(updated, encoding="utf-8")
        if in_sitemap(rel, text):
            urls.append(url)
    return urls


def write(doc_dir: Path, base: str) -> int:
    """Post-process `doc_dir` for publishing at `base`."""
    urls = process(doc_dir, base)
    (doc_dir / "sitemap.xml").write_text(render_sitemap(urls), encoding="utf-8")
    (doc_dir / "robots.txt").write_text(ROBOTS.format(base=base), encoding="utf-8")
    print(f"docs_seo: {len(urls)} pages in sitemap, robots.txt written")
    return 0


def check(doc_dir: Path, base: str) -> int:
    """Exit status 1 if any page or file the deploy needs is missing."""
    errors = []
    for rel in html_pages(doc_dir):
        text = (doc_dir / rel).read_text(encoding="utf-8")
        if 'rel="canonical"' not in text:
            errors.append(f"no canonical: {rel}")
    robots = doc_dir / "robots.txt"
    if not robots.is_file() or "Disallow: /*?search=" not in robots.read_text():
        errors.append("robots.txt missing or does not disallow ?search=")
    sitemap = doc_dir / "sitemap.xml"
    if not sitemap.is_file() or "/src/" in sitemap.read_text():
        errors.append("sitemap.xml missing or lists /src/ pages")
    for line in errors:
        print(f"docs_seo: {line}", file=sys.stderr)
    return 1 if errors else 0


def main(argv: list[str]) -> int:
    """Parse arguments and dispatch to write or check."""
    args = [a for a in argv if a != "--check"]
    if len(args) != 2:
        print(__doc__.strip().splitlines()[-1], file=sys.stderr)
        return 2
    doc_dir, base = Path(args[0]), args[1].rstrip("/") + "/"
    action = check if "--check" in argv else write
    return action(doc_dir, base)


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
