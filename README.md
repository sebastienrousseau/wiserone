<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/clients/wiserone/v1/logos/wiserone.svg" alt="The Wiser One logo" width="128" />
</p>

<h1 align="center">wiserone</h1>

<p align="center">
  A command-line tool that renders a daily quote to HTML — the same
  quote <a href="https://wiserone.com">wiserone.com</a> is showing.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/wiserone/actions"><img src="https://img.shields.io/github/actions/workflow/status/sebastienrousseau/wiserone/ci.yml?style=for-the-badge&logo=github" alt="Build" /></a>
  <a href="https://crates.io/crates/wiserone"><img src="https://img.shields.io/crates/v/wiserone.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Crates.io" /></a>
  <a href="https://docs.rs/wiserone"><img src="https://img.shields.io/badge/docs.rs-wiserone-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs.rs" /></a>
  <a href="https://lib.rs/crates/wiserone"><img src="https://img.shields.io/badge/lib.rs-wiserone-orange.svg?style=for-the-badge" alt="lib.rs" /></a>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg?style=for-the-badge" alt="License" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — Cargo, source
- [Quick Start](#quick-start) — a quote on screen in two commands

**Reference**

- [Commands](#commands) — `daily`, `random`, `all`
- [How selection works](#how-selection-works) — why `daily` agrees with the website
- [The corpus](#the-corpus) — schema, JSON and CSV
- [Library usage](#library-usage) — using the crate directly
- [Output](#output) — what gets written, and where

**Operational**

- [Platform support](#platform-support) — tiers
- [Development](#development) — test, lint, coverage, corpus drift
- [Security](#security) — path handling guarantees
- [Documentation](#documentation) — all reference docs
- [License](#license)

---

## Install

| Channel | Install |
|---|---|
| Cargo (crates.io) | `cargo install wiserone --locked` |
| Cargo (from source) | `cargo install --locked --path .` |

Requires Rust **1.75.0** or later. See [`doc/POLICIES.md`](doc/POLICIES.md).

## Quick Start

```shell
git clone https://github.com/sebastienrousseau/wiserone
cd wiserone
cargo run -- daily ./quotes/quotes.json
```

That writes `docs/YYYY_MM_DD.html` and mirrors it to `docs/index.html`,
carrying the same quote [wiserone.com](https://wiserone.com) is showing
today.

## Commands

| Command | Selects | Writes |
|---|---|---|
| `wiserone daily <file>` | The quote of the day, matching the website | `docs/YYYY_MM_DD.html` + `docs/index.html` |
| `wiserone random <file>` | Any quote at random | `docs/YYYY_MM_DD.html` + `docs/index.html` |
| `wiserone all <file>` | Every quote in the corpus | `docs/quote-NNNN.html` per quote |

```shell
wiserone daily  ./quotes/quotes.json
wiserone random ./quotes/quotes.csv
wiserone all    ./quotes/quotes.json
```

## How selection works

The corpus is an ordered **pool**, not a calendar. `daily` computes:

```text
index = day_number % pool_length
```

`day_number` is days elapsed since 0001-01-01, in UTC — the value
Python's `date.toordinal()` returns, and the value
[wiserone.com](https://wiserone.com) rotates on. The pool is ordered by
`id`. Given the same corpus in the same order, the CLI and the website
show the same quote on the same day, and a test pins that against a
known date.

Two consequences worth knowing:

- **`date_added` selects nothing.** It records the day a line was
  written. It is not unique, and anything keyed on it will collide.
- **Order is load-bearing.** Reordering or renumbering shifts which
  quote every future day shows.

The reasoning is in [ADR 0001](doc/adr/0001-quote-pool-and-rotation.md).

## The corpus

```json
{
  "quotes": [
    {
      "id": 0,
      "pillar": "elimination",
      "quote_text": "Say no to a hundred good things.",
      "author": "The Wiser One",
      "date_added": "2024-02-17T06:06:06Z",
      "image_url": "https://cloudcdn.pro/stocks/images/example.webp"
    }
  ]
}
```

| Field | Meaning |
|---|---|
| `id` | Pool position — what `daily` indexes |
| `pillar` | Thematic block, e.g. `elimination`, `mortality` |
| `quote_text` | The line |
| `author` | Attribution |
| `date_added` | Provenance: when it was written |
| `image_url` | Banner image |

`quotes/quotes.csv` holds the same rows, same order, same columns. A
test asserts the two formats stay identical.

Only `.json` and `.csv` are accepted. Paths containing `..` are
rejected before the file is read.

## Library usage

```rust
use wiserone::quotes::{current_day_number, read_quotes_from_file};

let quotes = read_quotes_from_file("./quotes/quotes.json")?;
let today = quotes.select_daily_quote(current_day_number())?;
println!("{} — {}", today.quote_text, today.author);
```

Build a one-off quote with the macro, which defaults pool metadata:

```rust
use wiserone::wiserone;

let quote = wiserone! {
    quote_text: "Taste is knowing which good idea to throw away.",
    author: "The Wiser One",
    date_added: "2026-08-23T06:06:06Z",
    image_url: "https://example.com/banner.webp"
};
```

## Output

Pages are written to `./docs`, which is git-ignored — it is generated
output, not source. Generation needs `_layouts/quote.html`; a missing
template, or a directory in its place, is reported as an error rather
than a panic.

Each page's canonical URL is `https://wiserone.com/q/<slug>/`, the
website's canonical address for that quote.

## Platform support

| Tier | Platforms |
|---|---|
| Tier 1 🏆 | `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc` |
| Tier 2 🥈 | `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-musl` |

Tier 1 is built and tested on every push; Tier 2 is built.

## Development

```shell
cargo test                                    # 140 tests
cargo fmt --check                             # formatting
cargo clippy --all-targets --all-features     # lints (-D warnings in CI)
cargo tarpaulin --follow-exec --fail-under 92 # coverage
./scripts/verify-corpus.sh                    # corpus vs wiserone.com
```

The corpus lives in three repositories and the website's copy is
canonical. `verify-corpus.sh` fetches
[`wiserone.com/quotes.json`](https://wiserone.com/quotes.json) and fails
on any divergence in content or order. See
[`doc/TESTING.md`](doc/TESTING.md).

## Security

Two guarantees for a tool that writes files:

- **Output filenames** are validated before use: no `..`, no path
  separators, must end in `.html`, non-empty without the extension.
- **Input paths** are validated before being read: no `..`, and only
  `.json` or `.csv`.

Both rejection paths are covered by tests. Report vulnerabilities via
[`SECURITY.md`](.github/SECURITY.md).

## Documentation

| Document | Covers |
|---|---|
| [`doc/ARCHITECTURE.md`](doc/ARCHITECTURE.md) | Crate layout, the pool, selection, page generation |
| [`doc/USER-GUIDE.md`](doc/USER-GUIDE.md) | Commands, corpus format, troubleshooting |
| [`doc/TESTING.md`](doc/TESTING.md) | Suite layout, coverage policy, measurement traps |
| [`doc/POLICIES.md`](doc/POLICIES.md) | Versioning, MSRV, platforms, coverage, corpus changes |
| [`doc/adr/0001-quote-pool-and-rotation.md`](doc/adr/0001-quote-pool-and-rotation.md) | Why quotes are a pool, not a calendar |
| [`doc/adr/0002-testable-entry-points.md`](doc/adr/0002-testable-entry-points.md) | Why entry points come in pairs |
| [API docs](https://docs.rs/wiserone) | Generated reference |

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
