<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

<p align="center">
  <img src="https://cloudcdn.pro/clients/wiserone/v1/logos/wiserone.svg" alt="wiserone logo" width="128" />
</p>

<h1 align="center">wiserone</h1>

<p align="center">
  A command-line tool and Rust library that renders a daily quote to HTML — the same
  quote <a href="https://wiserone.com">wiserone.com</a> is showing.
</p>

<p align="center">
  <a href="https://github.com/sebastienrousseau/wiserone/actions/workflows/ci.yml"><img src="https://github.com/sebastienrousseau/wiserone/actions/workflows/ci.yml/badge.svg" alt="Build" /></a>
  <a href="https://crates.io/crates/wiserone"><img src="https://img.shields.io/crates/v/wiserone.svg?style=for-the-badge&color=fc8d62&logo=rust" alt="Registry" /></a>
  <a href="https://docs.rs/wiserone"><img src="https://img.shields.io/docsrs/wiserone?style=for-the-badge&labelColor=555555&logo=docs.rs" alt="Docs" /></a>
  <a href="https://scorecard.dev/viewer/?uri=github.com/sebastienrousseau/wiserone"><img src="https://img.shields.io/ossf-scorecard/github.com/sebastienrousseau/wiserone?style=for-the-badge&label=OpenSSF%20Scorecard&logo=openssf" alt="OpenSSF Scorecard" /></a>
  <a href="LICENSE-MIT"><img src="https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg?style=for-the-badge" alt="License: Apache-2.0 OR MIT" /></a>
  <a href="https://github.com/sebastienrousseau/wiserone/blob/main/docs/POLICIES.md"><img src="https://img.shields.io/badge/rust-1.88.0%2B-93450a.svg?style=for-the-badge&logo=rust" alt="Minimum toolchain: Rust 1.88.0" /></a>
</p>

---

## Contents

**Getting started**

- [Install](#install) — crates.io, pre-built binaries, source
- [Requirements](#requirements) — toolchain floor, platforms
- [Quick Start](#quick-start) — today's quote as a page in one command

**The wiserone ecosystem**

- [The wiserone ecosystem](#the-wiserone-ecosystem) — the crate, the corpus, wiserone.com

**Library reference**

- [Capabilities at a glance](#capabilities-at-a-glance) — the current surface by theme
- [Ecosystem comparison](#ecosystem-comparison) — why there is no matrix
- [Benchmarks](#benchmarks) — headline numbers; full table at [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md)
- [Features](#features) — commands, selection, the corpus, output
- [Configuration](#configuration) — inputs and fixed paths
- [Examples](#examples) — runnable example index

**Operational**

- [When not to use wiserone](#when-not-to-use-wiserone) — limitations
- [Development](#development) — make targets, CI gates
- [Security](#security) — guarantees and compliance
- [Documentation](#documentation) — all reference docs
- [Stability guarantees](#stability-guarantees) — SemVer axis, output stability, minimum toolchain discipline
- [License](#license)

---

## Install

### As a Rust library

```toml
[dependencies]
wiserone = "0.0.8"
```

### As a command-line tool

```bash
cargo install wiserone --locked
```

Pre-built binaries for 20 targets are attached to every
[GitHub release](https://github.com/sebastienrousseau/wiserone/releases),
with a `SHA256SUMS` file, a CycloneDX SBOM, and manpages and shell
completions for bash, zsh, fish, PowerShell and elvish. Each asset carries
a signed build-provenance attestation:

```bash
gh attestation verify wiserone-v0.0.8-x86_64-unknown-linux-gnu.tar.gz -R sebastienrousseau/wiserone
```

From source: `cargo install --locked --path .`

---

## Requirements

- **Rust 1.88.0** or later (the MSRV, built and tested in CI; see
  [`docs/POLICIES.md`](docs/POLICIES.md)).
- Generating pages needs the template `_layouts/quote.html` and a corpus
  file, both in this repository, relative to the working directory.

| Tier | Platforms |
| :--- | :--- |
| Tier 1 | `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc` |
| Tier 2 | `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-musl` |

Every tier is built for each release. CI tests on every push on Linux
(`x86_64`) and macOS (`aarch64`); the other targets are built, not tested.

---

## Quick Start

```bash
git clone https://github.com/sebastienrousseau/wiserone
cd wiserone
cargo run -- daily ./quotes/quotes.json
```

That writes `docs/YYYY_MM_DD.html` and mirrors it to `docs/index.html`,
carrying the same quote [wiserone.com](https://wiserone.com) is showing
today.

---

## The wiserone ecosystem

The quotes are one pool shared by this crate and the website. The
website's copy is canonical; this repository mirrors it and CI fails when
the two diverge.

| Component | Purpose | Use case |
| :--- | :--- | :--- |
| `wiserone` crate | CLI and library: load, select and render quotes | Generate quote pages, or pick today's quote in Rust code |
| [wiserone.com](https://wiserone.com) | The site, and the canonical corpus at [`quotes.json`](https://wiserone.com/quotes.json) | Read the daily quote |
| `quotes/quotes.json`, `quotes/quotes.csv` | The mirrored corpus, 136 quotes | Input to every command |
| `scripts/verify-corpus.sh` | Compares the mirror with the website | Catch drift before it ships |

---

## Capabilities at a glance

| Area | Capability | Status |
| :--- | :--- | :--- |
| Selection | Quote of the day, identical to wiserone.com | Stable |
| Selection | Random quote; every quote | Stable |
| Input | JSON and CSV corpora, checked identical by a test | Stable |
| Output | One HTML page per quote, `index.html`, `sitemap.xml` | Stable |
| Library | `read_quotes_from_file`, `select_daily_quote`, `wiserone!` | Stable |
| Distribution | 20 pre-built targets, manpages, completions, SBOM, provenance | Since 0.0.8 |

---

## Ecosystem comparison

Not applicable. wiserone renders one corpus to the pages of one website;
it is not a general static-site or quote library, and no other crate
reproduces wiserone.com's rotation, so there is nothing like-for-like to
compare against. A comparison with general static-site generators would
measure a different job.

---

## Benchmarks

On an Apple A18 Pro with rustc 1.98.0 and the 136-quote corpus,
`wiserone daily` takes **127 ms ± 26 ms** end to end (50 runs), and
`wiserone all` writes all 136 pages in **1.2 s to 1.9 s**.

| Scenario | Result | Environment |
| :--- | ---: | :--- |
| `daily`, end to end | 127.3 ms ± 25.7 ms | A18 Pro, release build, hyperfine |
| `all`, 136 pages | 1.2–1.9 s | A18 Pro, release build, 3 runs |
| Parse 1,000 quotes, JSON | 1.03 ms | A18 Pro, criterion median |
| Render one page from the template | 30.6 µs | A18 Pro, criterion median |

See [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md) for methodology and full results.

---

## Features

### Commands

| Command | Selects | Writes |
| :--- | :--- | :--- |
| `wiserone daily <file>` | The quote of the day, matching the website | `docs/YYYY_MM_DD.html` + `docs/index.html` |
| `wiserone random <file>` | Any quote at random | `docs/YYYY_MM_DD.html` + `docs/index.html` |
| `wiserone all <file>` | Every quote in the corpus | `docs/quote-NNNN.html` per quote |

Every command also rewrites `docs/sitemap.xml`. `man wiserone` and shell
completions come from the release archives or `make assets`.

### How selection works

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

- **`date_added` selects nothing.** It records the day a line was
  written. It is not unique, and anything keyed on it will collide.
- **Order is load-bearing.** Reordering or renumbering shifts which
  quote every future day shows.

The reasoning is in [ADR 0001](docs/adr/0001-quote-pool-and-rotation.md).

### The corpus

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
| :--- | :--- |
| `id` | Pool position — what `daily` indexes |
| `pillar` | Thematic block, e.g. `elimination`, `mortality` |
| `quote_text` | The line |
| `author` | Attribution |
| `date_added` | Provenance: when it was written |
| `image_url` | Banner image |

`quotes/quotes.csv` holds the same rows, same order, same columns.

### Output

Pages are written to `./docs`; the generated pages, sitemap and logs are
git-ignored. Each page's canonical URL is `https://wiserone.com/q/<slug>/`,
the website's canonical address for that quote. A missing template, or a
directory in its place, is reported as an error rather than a panic.

---

## Configuration

There is no configuration file and no environment variable. A run is
configured entirely by its command and corpus path:

| Input | Values |
| :--- | :--- |
| Command | `daily`, `random` or `all` |
| Corpus | a `.json` or `.csv` path; `..` is rejected |

These are fixed, relative to the working directory: the template
`_layouts/quote.html`, the output directory `./docs`, the log at
`./docs/logs/wiserone.log`, and the site URL `https://wiserone.com/` used
for canonical links and the sitemap.

---

## Examples

| Example | Shows | Run |
| :--- | :--- | :--- |
| [`examples/example.rs`](examples/example.rs) | Loading the corpus and generating pages through the library | `cargo run --example example` |
| [`examples/gen_assets.rs`](examples/gen_assets.rs) | Generating manpages and completions from the CLI definition | `cargo run --example gen_assets -- target/assets` |

Selecting today's quote from Rust:

```rust
use wiserone::quotes::{current_day_number, read_quotes_from_file};

let quotes = read_quotes_from_file("./quotes/quotes.json")?;
let today = quotes.select_daily_quote(current_day_number())?;
println!("{} — {}", today.quote_text, today.author);
```

Building a one-off quote with the macro, which defaults pool metadata:

```rust
use wiserone::wiserone;

let quote = wiserone! {
    quote_text: "Taste is knowing which good idea to throw away.",
    author: "The Wiser One",
    date_added: "2026-08-23T06:06:06Z",
    image_url: "https://example.com/banner.webp"
};
```

---

## When not to use wiserone

- **You need your own paths.** The template, output directory and site
  URL are fixed; there are no flags to change them.
- **You have a large corpus and use `all`.** Its cost grows faster than
  the number of quotes, because each page re-lists and re-logs everything
  already written. At 136 quotes it takes under two seconds.
- **You want a calendar.** Quotes rotate through a pool by day number;
  a quote cannot be pinned to a date.
- **You want a general static-site generator.** wiserone renders one
  template for one site.

---

## Development

```bash
make help            # every target
cargo test           # 140 tests
make lint            # clippy, zero warnings
make coverage        # tarpaulin, 92% floor
make complexity      # per-function complexity ceilings
make assets          # manpages and completions
./scripts/verify-corpus.sh
```

[`DEVELOPMENT.md`](DEVELOPMENT.md) maps every CI gate to the command that
reproduces it, and describes the test layout and the release process.

---

## Security

- **Output filenames** are validated before use: no `..`, no path
  separators, must end in `.html`, non-empty without the extension.
- **Input paths** are validated before being read: no `..`, and only
  `.json` or `.csv`.
- **Supply chain:** `cargo audit` and `cargo deny` run on every change;
  CodeQL scans the Rust, Python and workflow code; secret scanning and
  push protection are on; OpenSSF Scorecard reports the repository's
  posture. Releases are cut from signed tags and ship checksums, an SBOM
  and provenance attestations.

Both validation paths are covered by tests.

Report vulnerabilities according to [`SECURITY.md`](.github/SECURITY.md).

---

## Documentation

| Document | Covers |
| :--- | :--- |
| [`DEVELOPMENT.md`](DEVELOPMENT.md) | Toolchain, CI gates, tests, releases |
| [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) | Crate layout, the pool, selection, page generation |
| [`docs/USER-GUIDE.md`](docs/USER-GUIDE.md) | Commands, corpus format, troubleshooting |
| [`docs/TESTING.md`](docs/TESTING.md) | Suite layout, coverage policy, measurement traps |
| [`docs/POLICIES.md`](docs/POLICIES.md) | Versioning, MSRV, platforms, coverage, corpus changes |
| [`docs/BENCHMARKS.md`](docs/BENCHMARKS.md) | Measurements and how to reproduce them |
| [`docs/releases/`](docs/releases/) | Highlights of every release |
| [`docs/adr/0001-quote-pool-and-rotation.md`](docs/adr/0001-quote-pool-and-rotation.md) | Why quotes are a pool, not a calendar |
| [`docs/adr/0002-testable-entry-points.md`](docs/adr/0002-testable-entry-points.md) | Why entry points come in pairs |
| [API docs](https://docs.rs/wiserone) | Generated reference |

---

## Stability guarantees

- **SemVer axis.** The crate is pre-1.0 and versions advance by 0.0.1, so
  any release may break the public API. Breaking changes are named in the
  release's Highlights in [`docs/releases/`](docs/releases/).
- **Output stability.** What the tool produces is part of its contract:
  which quote `daily` selects on a given day, page file names, and the
  canonical URL scheme. A change to any of them is treated as breaking
  even when no API signature moves, and is called out the same way.
- **Minimum toolchain.** The MSRV is 1.88.0, built and tested in CI.
  Raising it is a breaking change, recorded in the release Highlights and
  in [`docs/POLICIES.md`](docs/POLICIES.md).

---

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT license](LICENSE-MIT) at your option.

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in this crate by you, as defined in the
Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
