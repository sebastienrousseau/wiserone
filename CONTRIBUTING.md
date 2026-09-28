# Contributing to `wiserone`

Welcome! We're thrilled that you're interested in contributing to the
`wiserone` library. Whether you're looking to evangelize, submit feedback,
or contribute code, we appreciate your involvement in making `wiserone` a
better tool for everyone. Here's how you can get started.

## Evangelize

One of the simplest ways to help us out is by spreading the word about
wiserone. We believe that a bigger, more involved community makes for a
better framework, and that better frameworks make the world a better
place. If you know people who might benefit from using wiserone, please
let them know!

## How to Contribute

If you're interested in making a more direct contribution, there are
several ways you can help us improve wiserone. Here are some guidelines
for submitting feedback, bug reports, and code contributions.

### Feedback

Your feedback is incredibly valuable to us, and we're always looking for
ways to make wiserone better. If you have ideas, suggestions, or questions
about wiserone, we'd love to hear them. Here's how you can provide
feedback:

- [Open a new issue][2] to submit feedback.
- Use a descriptive title that clearly summarizes your feedback.
- Provide a detailed description of the issue or suggestion.
- Be patient while we review and respond to your feedback.

### Bug Reports

If you encounter a bug while using wiserone, please let us know so we can
fix it. Here's how you can submit a bug report:

- [Open a new issue][2] to report the bug.
- Use a descriptive title that clearly summarizes the bug.
- Provide a detailed description of the issue, including steps to
  reproduce it.
- Be patient while we review and respond to your bug report.

### Code Contributions

If you're interested in contributing code to wiserone, we're excited to
have your help! Here's what you need to know:

#### Feature Requests

If you have an idea for a new feature or improvement, we'd love to hear
it. Here's how you can contribute code for a new feature to wiserone:

- Fork the repo.
- Clone the wiserone[1] repo by running:
  `git clone https://github.com/sebastienrousseau/wiserone`
- Edit files in the `src/` folder. The `src/` folder contains the source
  code for wiserone.
- Submit a pull request, and we'll review and merge your changes if they
  fit with our vision for wiserone.

#### Submitting Code

If you've identified a bug or have a specific code improvement in mind,
we welcome your pull requests. Here's how to submit your code changes:

- Fork the repo.
- Clone the wiserone repo by running:
  `git clone https://github.com/sebastienrousseau/wiserone`
- Edit files in the `src/` folder. The `src/` folder contains the source
  code for wiserone.
- Submit a pull request, and we'll review and merge your changes if they
  fit with our vision for wiserone.

### Requirements for Contributions

Every pull request must pass the CI gate before it can merge. The
standards it enforces, and how to run each one locally, are in
[DEVELOPMENT.md](DEVELOPMENT.md):

- **Formatting:** `cargo fmt --all -- --check` (configuration in
  `rustfmt.toml`).
- **Lints:** `cargo clippy --all-targets --all-features -- -D warnings`,
  with the stricter lint table in `Cargo.toml`. Warnings are errors.
- **No unsafe code:** the crate is `#![forbid(unsafe_code)]`.
- **Complexity:** every function stays within cyclomatic 10, cognitive
  15, Halstead difficulty 30 and 60 lines (`make complexity`).
- **Minimum Rust:** changes must build and pass on Rust 1.88.0.
- **Documentation:** public items are documented; Markdown passes
  markdownlint and codespell.
- **Commits:** [Conventional Commits](https://www.conventionalcommits.org/)
  (`feat:`, `fix:`, `docs:`, ...), one logical change per commit.

### Testing Policy

Tests are mandatory, not optional:

- **New functionality** must come with tests in the automated suite
  (`tests/` or unit tests beside the code) that exercise it.
- **Bug fixes** must add a regression test that fails without the fix.
- **Coverage** may not drop below the 92% floor enforced in CI, and the
  floor only moves up (see [docs/POLICIES.md](docs/POLICIES.md)).
- Tests must not write into the project's own `docs/` directory; use the
  `*_in` functions with a scratch directory
  ([docs/TESTING.md](docs/TESTING.md)).

A pull request that adds behaviour without a test is asked for one
before review continues.

### Code Review

Every change reaches `main` through a pull request. Reviewers check:

- **Correctness:** the change does what it claims, including error paths
  and edge cases (empty corpus, bad paths, unusual input).
- **Tests:** new behaviour and fixed bugs are covered, per the policy
  above.
- **Security:** input and output paths stay validated, no new `unsafe`,
  no new network or filesystem access without a reason.
- **Output stability:** changes to which quote is selected, page file
  names or canonical URLs are called out as breaking.
- **Scope:** one logical change, with a commit message explaining why.

All CI checks and every review conversation must be resolved before
merging; `main` is protected to enforce both.

We hope that this guide has been helpful in explaining how you can
contribute to wiserone. Thank you for your interest and involvement in our
project!

[1]: https://github.com/sebastienrousseau/wiserone
[2]: https://github.com/sebastienrousseau/wiserone/issues/new
