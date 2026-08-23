<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->

# User Guide

## Contents

- [Install](#install)
- [Commands](#commands)
- [The corpus file](#the-corpus-file)
- [Output](#output)
- [Using your own quotes](#using-your-own-quotes)
- [Library use](#library-use)
- [Troubleshooting](#troubleshooting)

## Install

```shell
cargo install wiserone --locked
```

Or from source:

```shell
git clone https://github.com/sebastienrousseau/wiserone
cd wiserone
cargo install --locked --path .
```

## Commands

| Command | Selects | Writes |
|---|---|---|
| `wiserone daily <file>` | The quote of the day, matching [wiserone.com](https://wiserone.com) | `docs/YYYY_MM_DD.html` + `docs/index.html` |
| `wiserone random <file>` | Any quote at random | `docs/YYYY_MM_DD.html` + `docs/index.html` |
| `wiserone all <file>` | Every quote | `docs/quote-NNNN.html` per quote |

```shell
wiserone daily ./quotes/quotes.json
wiserone random ./quotes/quotes.csv
wiserone all ./quotes/quotes.json
```

`daily` and the website agree because both select
`pool[ordinal % len]` on the same UTC day number, where the pool is
ordered by `id`. Change the order, or drop the ids, and they diverge.

## The corpus file

JSON:

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

CSV, same columns in the same order:

```text
id,pillar,quote_text,author,date_added,image_url
```

`id` is the pool position and drives `daily`. `date_added` is
provenance — the day the line was written — and selects nothing. A file
without ids still loads; those entries sort last and `daily` will not
agree with the website.

Only `.json` and `.csv` are accepted, and paths containing `..` are
rejected.

## Output

Pages are written to `./docs`, which is git-ignored: it is generated
output, not source. Each run also mirrors the page to `docs/index.html`
and refreshes `docs/sitemap.xml`.

Generated pages need `_layouts/quote.html` to exist. If it is missing,
or is a directory, the command fails with a clear error rather than a
panic.

## Using your own quotes

Nothing is hardcoded to The Wiser One's corpus — point any command at
your own file. If you want `daily` to be stable for your readers, give
every quote a contiguous `id` starting at zero and do not reorder them;
the rotation indexes by position, so inserting at the front shifts
every subsequent day.

## Library use

```rust
use wiserone::quotes::{current_day_number, read_quotes_from_file};

let quotes = read_quotes_from_file("./quotes/quotes.json")?;
let today = quotes.select_daily_quote(current_day_number())?;
println!("{} — {}", today.quote_text, today.author);
```

## Troubleshooting

| Symptom | Cause |
|---|---|
| `Only .json and .csv files are supported` | Wrong extension; the check runs before the file is read |
| `Path contains directory traversal sequence` | The path contains `..` |
| `Template path is not a file` | `_layouts/quote.html` is missing or is a directory |
| `daily` disagrees with the website | The corpus has drifted, or lost its ids — run `./scripts/verify-corpus.sh` |
| `all` produced fewer files than quotes | An old corpus without ids, falling back to date-derived filenames |
