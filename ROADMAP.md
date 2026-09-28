<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Roadmap

What wiserone intends to do over the next twelve months (to September
2027). It records direction, not promises: priorities can change, and
each item lands through a normal pull request and release. Items come from
the documented limitations in the [README](README.md#when-not-to-use-wiserone),
the [security assurance case](docs/ASSURANCE.md#known-limitations) and the
project's [OpenSSF Best Practices](https://www.bestpractices.dev/projects/14988)
self-assessment.

## Correctness and performance

- **Make `all` linear.** Today every page re-lists, re-logs and re-copies
  everything already written, so the run grows with the square of the
  corpus. Refresh `index.html`, the log and `sitemap.xml` once per run.
  This changes the log output, so it will be called out as breaking.
- **Bound input size.** Refuse corpora above a documented size instead of
  reading any file fully into memory.
- **Refuse a symlinked output directory,** so pages cannot be written
  outside `./docs`.

## Usability

- **Configurable paths.** Flags for the output directory, template and site
  URL, with today's values as defaults, so the tool is usable beyond
  wiserone.com without changing existing behaviour.

## Quality and assurance

- **Branch coverage.** Measure it, publish the number, and gate it in CI
  alongside the 92% line-coverage floor.
- **Reproducible builds in CI.** Build twice and compare, so the
  bit-for-bit claim is checked on every release, not once by hand.
- **API stability.** Settle the public API (`Quote`, `Quotes` and the
  selection functions), so `cargo-semver-checks` can gate releases once the
  version scheme reaches 0.1.0.

## Project health

- **A second maintainer.** Bring in a second person with release and
  security-response access, removing the single point of failure recorded
  in [GOVERNANCE.md](GOVERNANCE.md).
- **Two-person review** of changes once there is a second maintainer.
- **Newcomer tasks.** Keep a few small issues labelled `good first issue`.

## Not planned

- A general static-site generator, plugins or themes.
- Calendar-based selection: quotes rotate through a pool by design
  ([ADR 0001](docs/adr/0001-quote-pool-and-rotation.md)).

## Versioning

Releases advance by 0.0.1 and reach 0.1.0 only after 0.0.999
([docs/POLICIES.md](docs/POLICIES.md)), so every release in this period
is a 0.0.x release.
