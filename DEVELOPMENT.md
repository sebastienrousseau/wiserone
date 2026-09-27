<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Developing wiserone

The single entry point for working on this repository: toolchain setup,
how to reproduce every CI gate locally, how the tests are laid out, and how
a release is cut.

## Toolchain

| Tool | Version | Used for |
| :--- | :--- | :--- |
| Rust | 1.88.0 minimum (MSRV), stable for development | building and testing |
| `rustfmt`, `clippy` | with the toolchain | formatting and lints |
| `cargo-tarpaulin` | latest | coverage |
| `cargo-audit`, `cargo-deny` | latest | advisories, licences, bans |
| `rust-code-analysis-cli` | 0.0.25 | complexity ceilings |
| Python | 3.9+ | release and complexity scripts |
| `shellcheck` | any | release scripts |

```bash
rustup component add rustfmt clippy
make install-tools                                  # tarpaulin, audit, deny
cargo install rust-code-analysis-cli --version 0.0.25 --locked
```

## Reproducing CI locally

Each row is a CI job; the command is what it runs.

| Gate | Command |
| :--- | :--- |
| Formatting | `cargo fmt --all -- --check` |
| Lints | `cargo clippy --workspace --all-targets --all-features --no-deps -- -D warnings` |
| Tests | `cargo test --workspace --all-features` |
| MSRV | `cargo +1.88.0 test --locked` |
| Coverage (92% floor) | `make coverage` |
| Advisories and licences | `cargo audit` and `cargo deny check` |
| Complexity ceilings | `make complexity` |
| Release notes format | `python3 scripts/release_notes.py --check` |
| Release scripts | `shellcheck scripts/release_*.sh` |
| README template | `python3 scripts/validate_readme.py` |
| Corpus matches wiserone.com | `./scripts/verify-corpus.sh` |
| Spelling, Markdown, internal links | `codespell`, `npx markdownlint-cli2`, `lychee --offline --include-fragments './**/*.md'` |
| Fuzz seed replay | `cd fuzz && cargo +nightly fuzz run parse_corpus corpus/parse_corpus seeds/parse_corpus -- -runs=0` |
| Benchmarks compile and run | `cargo bench --workspace --all-features` |

`make ci-local` runs the Rust gates in sequence. `make help` lists every
target. External links are checked nightly rather than on each PR, so a
flaky third-party host cannot block a merge.

`cargo-semver-checks` is deliberately not a gate yet: Cargo treats every
0.0.x bump as a major version, so it runs no checks against 0.0.x
releases. It becomes useful from 0.1.0.

## Fuzzing

`fuzz/` holds a `cargo-fuzz` target, `parse_corpus`, that feeds arbitrary
bytes to the corpus parser, daily selection (including negative and
extreme day numbers) and `slug`, and asserts the slug rule. Seeds live in
`fuzz/seeds/parse_corpus/`; add any crashing input there so every CI run
replays it. `fuzz/corpus/` is the fuzzer's working set and is not
committed. Fuzzing needs a nightly toolchain:

```bash
cargo install cargo-fuzz --locked
cd fuzz && cargo +nightly fuzz run parse_corpus corpus/parse_corpus seeds/parse_corpus
```

## Complexity ceilings

Every function must stay within cyclomatic 10, cognitive 15, Halstead
difficulty 30 and 60 lines, and every file within 500 lines.
`scripts/complexity_check.py` measures them with `rust-code-analysis-cli`.
A function already over a ceiling is recorded in `complexity-baseline.json`
and may not get worse; the baseline is empty today and may only shrink.

## Tests

`tests/` holds one integration suite per module (`test_cli.rs`,
`test_html.rs`, `test_quotes.rs`, `test_sitemap.rs` and so on), plus
`test_corpus.rs`, which pins the JSON and CSV corpora to each other, and
`test_docs_not_touched.rs`, which stops any test from clearing the
project's own `docs/` directory. Tests that write pages use
`generate_html_file_in` and `generate_sitemap_file_in` with a scratch
directory. See [`docs/TESTING.md`](docs/TESTING.md) for coverage policy
and measurement traps.

## Generated files

Nothing below is committed; each is generated on demand.

| Output | Command |
| :--- | :--- |
| Manpages and shell completions | `make assets` (writes `target/assets/`) |
| SBOM | `cargo cyclonedx --format json --spec-version 1.5` |
| API docs | `cargo doc --no-deps` |
| The quote site | `cargo run -- daily ./quotes/quotes.json` (writes `docs/*.html`) |

`make install` builds the release binary and installs it with its
manpages and bash, zsh and fish completions under `PREFIX` (default
`/usr/local`), staged under `DESTDIR` when set; `make uninstall` removes
them.

## Releases

Work for the next version happens on `feat/v<next>`, with the version in
`Cargo.toml` already bumped. A release is cut by pushing a signed,
annotated tag whose message is `wiserone v<VERSION>`:

1. Write `docs/releases/v<VERSION>.md`: two to four Highlights bullets.
2. Merge the release branch into `main`.
3. Tag the merge commit with `git tag -s -m "wiserone v<VERSION>"` and
   run `scripts/release_preflight.sh v<VERSION> --expect <sha>`.
4. Push the tag. `.github/workflows/release.yml` runs the same preflight,
   builds 20 targets, generates the manpages, completions and SBOM,
   attests provenance, publishes the release and the crate, and audits
   the published result with `scripts/release_audit.sh`.

A manual run of the release workflow with `publish=false` builds
everything and publishes nothing; use it to test release changes.
