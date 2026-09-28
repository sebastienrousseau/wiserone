<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Security Assurance Case

Why wiserone's security requirements are met, and where they are not.
It covers the `wiserone` binary and library as of 0.0.9.

## Security requirements

What a user can rely on:

1. **Writes stay inside the output directory.** A run creates or
   overwrites files only under `./docs` (pages, `sitemap.xml`,
   `logs/wiserone.log`).
2. **Only the named corpus is read**, and only if it is a `.json` or
   `.csv` file.
3. **Corpus text cannot inject markup** into a generated page.
4. **Bad input fails cleanly:** a malformed corpus, a missing template or
   a bad path returns an error; it does not panic or corrupt output.
5. **No network access and no secrets.** The tool makes no network
   requests and handles no credentials.

## Threat model

| Actor | Can control | Goal it must not reach |
| :--- | :--- | :--- |
| Author of a corpus file | Every field in the JSON or CSV | Write outside `./docs`; inject script into pages |
| Person running the CLI | Command and corpus path | Read or write arbitrary files through the tool |
| Viewer of generated pages | Nothing | (Protected by requirement 3) |
| Attacker on the supply chain | A dependency, a CI action, a release asset | Ship code the maintainer did not build |

## Trust boundaries

```text
 command line ──► path validation ──► corpus file ──► serde parse ──► HTML escape ──┐
                                                                                    ▼
 _layouts/quote.html (trusted, fixed path) ──────────────────────────────► page ──► ./docs/<validated name>
```

- **Command-line arguments** are parsed by clap into a closed set of
  commands; anything else is a usage error.
- **The corpus path** is checked before it is opened: `..` is rejected,
  and only `.json` and `.csv` are accepted (`src/quotes.rs`).
- **Corpus contents** are parsed by `serde_json` or `csv` into typed
  fields. Every field placed into a page is HTML-escaped (`&`, `<`, `>`,
  `"`) before substitution (`src/html.rs`).
- **The template** `_layouts/quote.html` is trusted: it is part of the
  repository and read from a fixed path, never from user input.
- **Output file names** are validated before use: no `..`, no path
  separators, must end in `.html`, non-empty without the extension
  (`src/html.rs`). The output directory is a fixed constant.

## Secure design principles applied

- **Least privilege:** no network, no elevated permissions, writes only
  to one fixed directory. CI tokens default to read-only; release jobs
  request only the permissions they use.
- **Fail-safe defaults:** input paths and output names are allow-listed
  (extensions, characters), not deny-listed; anything else is rejected.
- **Complete mediation:** every write goes through the filename check and
  every read through the path check; there is no bypass.
- **Economy of mechanism:** one binary, no plugins, no configuration
  file, no runtime code loading.
- **Memory safety:** safe Rust only; the crate is `#![forbid(unsafe_code)]`.

## Common weaknesses countered

| Weakness | Countermeasure | Evidence |
| :--- | :--- | :--- |
| CWE-22 path traversal | `..` rejected in corpus paths and output names; separators rejected in names | `test_path_traversal_is_rejected`, `test_filename_validation_rejects_traversal_and_bad_names` (`tests/test_coverage_gaps.rs`) |
| CWE-79 cross-site scripting | Corpus fields HTML-escaped before insertion | `test_quote_fields_are_html_escaped` |
| CWE-119/787 memory corruption | Safe Rust, `forbid(unsafe_code)` | `src/lib.rs` |
| CWE-248 uncaught panic on input | Errors returned for bad input; fuzzed | `fuzz/fuzz_targets/parse_corpus.rs` |
| CWE-798 hard-coded credentials | None in the code; secret scanning with push protection | GitHub secret scanning |
| CWE-1395 vulnerable dependency | `cargo audit` and `cargo deny` on every change; Dependabot | `.github/workflows/ci.yml` |

## Supply chain

- Every GitHub Action is pinned to a commit SHA; workflow tokens are
  read-only by default.
- Releases are cut only from signed, annotated tags, after a blocking
  preflight; crates.io publishing uses OIDC trusted publishing, with no
  stored token.
- Each release ships `SHA256SUMS`, a CycloneDX SBOM, and a signed SLSA
  build-provenance attestation, verifiable with
  `gh attestation verify <asset> -R sebastienrousseau/wiserone`.

## Verification

CodeQL (Rust, Python, Actions), clippy with warnings as errors, 142 tests
with a 92% line-coverage floor, a fuzz target replayed on every change,
and OpenSSF Scorecard. See [`../DEVELOPMENT.md`](../DEVELOPMENT.md).

## Known limitations

- **The template is trusted.** A modified `_layouts/quote.html` can put
  anything in a page; it is part of the repository, not user input.
- **No input size limits.** A very large corpus is read into memory in
  full; the tool is meant for corpora of hundreds of quotes.
- **Random selection is not cryptographic.** `random` only picks a quote
  to show.
- **The output directory is not checked for symlinks.** If `./docs` is a
  symlink, pages are written where it points.
