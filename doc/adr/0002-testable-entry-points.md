<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# ADR 0002 — Entry points come in pairs

**Status:** Accepted · **Date:** 2026-08-23

## Context

`run()` and `run_cli()` were untestable, and coverage recorded them as
zero for a long time behind a comment claiming that was structural.

Two separate reasons, both fixable:

- They read `std::env::args_os()`. Under a test harness those are the
  harness's own arguments, so parsing fails for reasons unrelated to
  the CLI.
- On a clap error they call `Error::exit`, which terminates the
  process — including a test process.

## Decision

Each entry point is a pair:

| Outer | Inner |
|---|---|
| `run()` | `run_with(args)` |
| `run_cli()` | `run_cli_from(args)` |

The outer function owns the two untestable behaviours: reading process
arguments, and converting a clap error into `Error::exit` so `--help`
prints to stdout and exits zero. The inner function takes an explicit
argument list and returns errors.

## Consequences

Command dispatch, page generation and the sitemap write are all
exercised in-process. `lib.rs` went from 0/15 covered lines to 19/21,
`cli.rs` from 0/47 to 48/57.

The split must be respected. Routing `run()` straight through
`run_cli_from` once dropped the `Error::exit` handling, and
`wiserone --help` began printing to stderr and exiting non-zero. Two
pre-existing tests caught it.

`clap::Error::exit` must never be called from a function a test calls.
