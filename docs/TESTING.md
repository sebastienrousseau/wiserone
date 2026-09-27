<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Testing

What the suite covers, how to run it, and the two measurement traps
that made earlier coverage numbers misleading.

## Contents

- [Running the suite](#running-the-suite)
- [What each file covers](#what-each-file-covers)
- [Coverage](#coverage)
- [Two measurement traps](#two-measurement-traps)
- [What is deliberately not covered](#what-is-deliberately-not-covered)
- [Corpus drift](#corpus-drift)

## Running the suite

```shell
cargo test                     # 140 tests
cargo fmt --check              # formatting gate
cargo clippy --all-targets --all-features   # lint gate, -D warnings in CI
cargo tarpaulin --follow-exec --fail-under 92   # coverage gate
./scripts/verify-corpus.sh     # corpus vs wiserone.com
```

## What each file covers

| File | Covers |
|---|---|
| `tests/test_corpus.rs` | The shipped corpus: parses, ≥100 entries, ids contiguous from zero, no duplicate text, no slug collisions, JSON ≡ CSV, daily selection deterministic and wrapping, slugs matching live URLs |
| `tests/test_coverage_gaps.rs` | `run_with` end to end, the logger, every `QuoteError` variant and conversion, filename and template validation including path traversal, legacy CSV without ids |
| `tests/test_cli.rs` | Command dispatch for `random`, `daily` and `all`, including that same-day quotes produce distinct pages |
| `tests/test_html.rs` | Template filling and output |
| `tests/test_macros.rs` | Macro behaviour from a caller's perspective |
| `src/macros.rs` | Macro behaviour from inside the defining crate — see below |
| `tests/test_main.rs` | The compiled binary: `--help`, exit codes, cwd independence |

## Coverage

Measured with `cargo tarpaulin`. The floor is **92**, and CI enforces
it. Local macOS runs report **96.50%**; Linux CI reports slightly less
for reasons that are the tool's, not the tests'.

Both numbers are line coverage over the same code. The floor is a
ratchet: raise it when coverage genuinely improves, never lower it to
turn a red build green.

## Two measurement traps

Coverage sat at 60.64% for a long time behind a comment asserting the
shortfall was structural — that several modules were "reached only
through the compiled binary… so none of these lines can ever be
counted". That was wrong twice over, and both errors are worth knowing
because they recur.

**A macro's body is attributed to its expansion site.** The forty tests
in `tests/test_macros.rs` exercise every macro thoroughly, and credited
all of it to that integration target, leaving `src/macros.rs` reading
0/11. The definitions looked untested while being well tested. The fix
is a `#[cfg(test)] mod tests` *inside* `src/macros.rs`, so the
expansions land in the file that defines them.

A side effect: clippy then lints the expanded code, and lints belonging
to a macro body cannot be silenced with `#[allow]` at the call site —
the diagnostic points at the definition's span. Fix the macro instead.

**tarpaulin does not follow child processes by default.** It drives the
test binary under ptrace; tests that spawn the compiled binary get no
credit for what runs inside it. `--follow-exec` fixes that and is set in
CI. The "can never be counted" claim was a missing flag, not a law.

## What is deliberately not covered

Some lines cannot be entered by a passing test, and chasing them would
mean deforming the code:

- **`assert!` failure branches.** A passing assertion never enters its
  failure arm.
- **Multi-line `macro_log!` and `println!` arguments.** Continuation
  lines attribute to no executed region on some platforms.
- **`macro_rules!` header lines.**

These are instrumentation artefacts. The one genuine piece of dead code
found this way — a catch-all arm in `read_quotes_from_file` that
`validate_file_path` had already made unreachable — was deleted rather
than excluded.

## Corpus drift

No test can tell whether the shipped corpus still matches the website,
because the answer lives on the network. `scripts/verify-corpus.sh`
fetches [`wiserone.com/quotes.json`](https://wiserone.com/quotes.json)
and fails on any divergence in content or order. An unreachable
endpoint warns rather than failing, so a flaky network does not redden
an otherwise good build.

This gate exists because a divergence has already shipped undetected in
a sibling repository — see
[ADR 0001](adr/0001-quote-pool-and-rotation.md).
