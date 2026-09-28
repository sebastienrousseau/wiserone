<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Benchmarks

Numbers for wiserone 0.0.8, measured on 2026-09-27, with `all`
remeasured for 0.0.10 on 2026-09-28. They describe one
machine and are a baseline for spotting regressions, not a promise.

## Environment

| | |
| :--- | :--- |
| Machine | Apple A18 Pro, macOS |
| Toolchain | rustc 1.98.0, `--release` |
| Corpus | `quotes/quotes.json`, 136 quotes |

## End to end

Wall-clock time of the release binary, run in a scratch directory holding
`_layouts/` and `quotes/`.

| Command | Result | Method |
| :--- | ---: | :--- |
| `wiserone daily quotes/quotes.json` | 127.3 ms ± 25.7 ms | `hyperfine -N --warmup 5 --runs 50` |
| `wiserone all quotes/quotes.json` (136 pages) | 0.10 s to 0.14 s | five runs, empty output directory each time |
| `wiserone all`, 500 pages (synthetic) | 0.32 s | one run |
| `wiserone all`, 1,000 pages (synthetic) | 1.47 s | one run |

`all` scales linearly since 0.0.10. It used to refresh the log, index and
sitemap after every page, rescanning the whole output directory each
time, which made it quadratic: the same release builds measured

| Pages | 0.0.9 | 0.0.10 |
| ---: | ---: | ---: |
| 136 | 0.84 s | 0.12 s |
| 500 | 8.85 s | 0.32 s |
| 1,000 | 36.65 s | 1.47 s |

The generated pages, index, sitemap and logged events are identical
between the two.

## Library operations

`benches/performance_suite.rs`, run with
`cargo bench --bench performance_suite -- --warm-up-time 1 --measurement-time 3`.
Values are criterion's median, with its confidence interval in brackets.

| Scenario | Result |
| :--- | ---: |
| Parse 10 quotes, JSON | 175.8 µs [154.1, 203.1] |
| Parse 100 quotes, JSON | 251.4 µs [239.1, 264.6] |
| Parse 1,000 quotes, JSON | 1.03 ms [0.95, 1.12] |
| Parse 1,000 quotes, CSV | 1.02 ms [0.94, 1.12] |
| Random selection, 1,000 quotes | 482.6 µs [435.4, 535.9] |
| All quotes sorted, 1,000 quotes | 88.7 µs [78.2, 100.4] |
| Template processing, one page | 30.6 µs [26.2, 36.3] |
| Write 100 KiB | 571.0 µs [495.4, 676.1] |
| Read 100 KiB | 151.0 µs [143.6, 159.0] |

Before 0.0.8 this suite never ran: it had no `[[bench]]` entry, so Cargo
built it with the default test harness, which ignores `criterion_main!`.

## Reproducing

```bash
cargo bench --bench performance_suite
cargo bench --bench benchmark            # the macro micro-benchmarks
cargo build --release
hyperfine -N --warmup 5 'target/release/wiserone daily quotes/quotes.json'
```
