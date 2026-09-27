<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# ADR 0001 — Quotes are a pool, not a calendar

**Status:** Accepted · **Date:** 2026-08-23

## Context

The corpus began as a calendar: one quote per date, keyed by
`date_added`, with the website publishing whichever quote carried
today's date. It held 1,033 entries.

Only 138 of those had been written and reviewed by hand. The rest were
machine-generated backfill in a register the project had abandoned, so
a reader landing on a random day had roughly an 87% chance of seeing
superseded work. The calibrated corpus existed but was, in practice,
unpublished.

Deleting the backfill left ~140 quotes to cover an unbounded number of
days. A calendar cannot do that. It had already failed in the other
direction: when the queue ran out, the website sat on 25 February 2024
for months, because no date matched today.

## Decision

Quotes are an ordered pool. Selection is by position:

```text
index = day_number % pool_length
```

where `day_number` is days since 0001-01-01 in UTC — the value Python's
`date.toordinal()` returns, which is what the website already used.

`id` is the pool position and the only thing selection reads.
`date_added` is retained as provenance: the day a line was written. It
selects nothing.

## Consequences

**The front page cannot run dry.** Every date maps to a quote, so the
25-February-2024 failure cannot recur.

**Dates repeat.** Each quote surfaces roughly every `pool_length` days.
At ~140 quotes that is about four and a half months, which is why the
website gates on pool depth.

**`date_added` is no longer unique.** Anything deriving a filename or a
key from it silently collides. This bit immediately: the `all` command
produced 130 files from 136 quotes because six shared a date. Output is
now named from `id`.

**Order is load-bearing.** Reordering the pool changes which quote every
future day shows. In the macOS app, a repository that sorted by
`date_added` rather than `id` shipped a corpus shuffled relative to the
website — the app and the site agreed on the index and disagreed on the
quote, through a fully green test suite, because nothing compared them.

**Three copies must agree.** The website, this crate and the app each
ship the corpus. The website now publishes the canonical pool at
`/quotes.json`, and both consumers verify against it in CI.

## Alternatives considered

**Keep writing until the calendar refills.** At the rate quotes survive
review this was several hundred more, and we had already established
the voice does not carry that many lines without dilution.

**Shrink the archive to the calibrated dates only.** Would have 404'd
1,033 live, indexed URLs.

**Random selection.** Not reproducible: two readers on the same day, or
one reader reloading, would see different quotes, and the CLI could not
agree with the site.
