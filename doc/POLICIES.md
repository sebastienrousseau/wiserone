<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Policies

## Contents

- [Versioning](#versioning)
- [MSRV](#msrv)
- [Platform support](#platform-support)
- [Coverage](#coverage)
- [Corpus changes](#corpus-changes)
- [Security](#security)

## Versioning

[Semantic Versioning](https://semver.org). The crate is pre-1.0, so
breaking changes may land in a minor release; they are called out in
`CHANGELOG.md`.

Adding a field to the public `Quote` struct is a breaking change for
code that constructs one with a struct literal. The `wiserone!` macro
defaults new fields, so macro call sites are unaffected — prefer the
macro.

## MSRV

| Crate | MSRV | Why |
|---|---|---|
| `wiserone` | **1.75.0** | The lowest toolchain the crate builds and tests on. Raising it is a breaking change and needs a `CHANGELOG.md` entry. |

## Platform support

| Tier | Platforms |
|---|---|
| Tier 1 | `x86_64-unknown-linux-gnu`, `x86_64-apple-darwin`, `aarch64-apple-darwin`, `x86_64-pc-windows-msvc` |
| Tier 2 | `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-musl` |

Tier 1 is built and tested on every push. Tier 2 is built.

## Coverage

The floor is **92%**, enforced by `cargo tarpaulin --fail-under 92` in
CI, with `--follow-exec` so coverage inside spawned binaries counts.

The floor is a ratchet. Raise it when coverage genuinely improves;
never lower it to turn a red build green. If a change cannot clear the
floor, the answer is a test, not a smaller number.

Some lines are structurally unreachable — assertion failure branches,
`macro_rules!` headers, continuation lines of multi-line macro
arguments. They are documented in [TESTING.md](TESTING.md) rather than
excluded, so the gap stays visible. Genuinely dead code is deleted, not
excluded: see the removal of `read_quotes_from_file`'s catch-all arm.

## Corpus changes

The corpus in `quotes/` is a mirror. The canonical copy is published by
the website at
[`wiserone.com/quotes.json`](https://wiserone.com/quotes.json).

- Change the website's pool first, then mirror here.
- Never reorder or renumber ids to "tidy" the file. `id` is the
  rotation index; shifting it changes which quote every future day
  shows, in this crate and in the app, silently.
- `scripts/verify-corpus.sh` runs in CI and fails on divergence.

## Security

Reported via [`.github/SECURITY.md`](../.github/SECURITY.md). Two guarantees relevant to a tool that
writes files:

- Every output filename is validated before use: no `..`, no path
  separators, must end in `.html`, must be non-empty without the
  extension.
- Every input path is validated before it is read: no `..`, and only
  `.json` or `.csv`.

Both guards have tests covering their rejection branches, which is the
half most likely to be left untested.
