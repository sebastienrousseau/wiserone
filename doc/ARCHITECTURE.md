<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# Architecture

How `wiserone` is put together, and why the pieces are shaped the way
they are.

## Contents

- [The shape of the crate](#the-shape-of-the-crate)
- [The quote pool](#the-quote-pool)
- [Selection](#selection)
- [Page generation](#page-generation)
- [Where the corpus lives](#where-the-corpus-lives)
- [Module reference](#module-reference)

## The shape of the crate

`wiserone` is a small CLI with a library core. Everything that decides
*which quote* and *what HTML* is in the library; the binary is a thin
shim so the logic stays testable.

```text
main.rs        process entry; calls run()
  lib.rs       run() -> run_with(args)   logger, log file, dispatch
    cli.rs     argument parsing, the three subcommands
      quotes.rs  the pool: load, validate, select
      html.rs    template fill, filename validation, output
      sitemap.rs sitemap for the generated pages
```

The `run`/`run_with` and `run_cli`/`run_cli_from` pairs exist for one
reason: the outer function reads `std::env::args_os()` and, on a clap
error, calls `Error::exit`, which terminates the process. Neither is
usable from a test. The inner function takes an explicit argument list
and *returns* the error, so the whole pipeline can be driven in-process.

## The quote pool

`quotes/quotes.json` is an ordered pool. Each entry carries:

| Field | Meaning |
|---|---|
| `id` | Position in the pool. **This is what selection indexes.** |
| `pillar` | Thematic block, e.g. `elimination`, `mortality` |
| `quote_text` | The line itself |
| `author` | Attribution |
| `date_added` | The day the line was *written* — provenance only |
| `image_url` | Banner image |

`date_added` used to be the publication date, one quote per day. It no
longer is, and treating it as one is the single most likely way to
break this project — see
[ADR 0001](adr/0001-quote-pool-and-rotation.md).

`quotes/quotes.csv` carries the same rows in the same order, including
`id` and `pillar`, so the two formats are genuinely interchangeable. A
test asserts it.

## Selection

Two selectors, deliberately different:

```rust
quotes.select_random_quote()          // `random` — any quote
quotes.select_daily_quote(day_number) // `daily`  — the site's quote
```

`select_daily_quote` sorts the pool by `id` and indexes
`day_number % len`, using floored modulo so a pre-epoch ordinal wraps
instead of panicking. `current_day_number()` supplies the ordinal:
days elapsed since 0001-01-01 in the proleptic Gregorian calendar,
computed in UTC.

That is the same value Python's `date.toordinal()` returns, where
1970-01-01 is 719163 — and it is what
[wiserone.com](https://wiserone.com) rotates on. Given the same corpus
in the same order, the CLI and the website show the same quote on the
same day. `tests/test_corpus.rs` pins this against a known date.

## Page generation

`generate_html_file_in` fills `_layouts/quote.html` and writes into the
output directory. Two guards run before anything touches the disk:

- **Filename validation** rejects `..`, `/`, `\`, anything not ending
  in `.html`, and names that are empty once the extension is removed.
- **Template validation** requires `_layouts/quote.html` to exist *and*
  be a file.

The canonical URL written into each page is
`https://wiserone.com/q/<slug>/`, built by `slug()`. That is the site's
canonical address for a quote: every dated URL on wiserone.com points
its `rel=canonical` there. `slug()` is pinned against live URLs by
`test_slug_matches_published_urls`, so a drift in the site's slug rule
fails the build here rather than emitting a dead canonical.

`all` names its output `quote-NNNN.html` from the pool position. It
used to name from `date_added`, which silently overwrote pages once
two quotes shared a day.

## Where the corpus lives

The same corpus exists in three repositories:

| Repository | Path | Consumed by |
|---|---|---|
| `wiserone.github.io` | `_data/quotes/quotes.json` | the website; published at [`/quotes.json`](https://wiserone.com/quotes.json) |
| `wiserone` (this crate) | `quotes/quotes.json` | the CLI |
| `WiserOneApp` | `sources/resources/quotes.json` | the macOS menu-bar app |

The website's copy is canonical. `scripts/verify-corpus.sh` fetches it
and fails the build on any divergence in content or order, because a
silent divergence has already happened once — see
[ADR 0001](adr/0001-quote-pool-and-rotation.md).

## Module reference

| Module | Responsibility |
|---|---|
| `quotes` | Pool loading (JSON/CSV), validation, selection, `slug`, `current_day_number` |
| `cli` | Clap command definitions and dispatch for `random`, `daily`, `all` |
| `html` | Template filling, filename and template validation, file output |
| `sitemap` | Sitemap generation for produced pages |
| `ascii` | FIGlet banner for the CLI |
| `loggers` | `env_logger` initialisation |
| `macros` | Small convenience macros, with their tests in-file (see [TESTING.md](TESTING.md)) |
