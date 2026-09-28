<!-- SPDX-License-Identifier: Apache-2.0 OR MIT -->

# Roadmap

wiserone is a free, inspirational project and is in **maintenance mode**.
It powers [wiserone.com](https://wiserone.com) and does that job; there
is no plan to grow it into a general tool, and no commercial plan. The
aim is to keep it correct, secure and current with as little human time
as possible.

## Automated upkeep

- **Dependencies:** Dependabot opens updates; patch and minor updates
  merge automatically once the CI Gate passes. Major updates wait for a
  person.
- **Corpus:** a weekly workflow mirrors [wiserone.com's pool](https://wiserone.com/quotes.json)
  into `quotes/` and merges it automatically once the CI Gate passes.
  A pool that breaks an invariant (contiguous ids, no duplicates, unique
  slugs) fails the corpus tests and waits for a person.
- **Health:** nightly checks keep one rolling issue for pending updates;
  CodeQL, OpenSSF Scorecard and the external link check run on schedule.

## What still needs a person

- **Security reports and real bugs**, handled per
  [SECURITY.md](.github/SECURITY.md) and [CONTRIBUTING.md](CONTRIBUTING.md).
- **Releases**, cut from a tag signed with the maintainer's key, when code
  changes. Dependency-only updates need no release.
- **Major dependency updates**, reviewed before merging.

## Not planned

- New features, configurable sites, plugins or themes: wiserone renders
  one site.
- Calendar-based selection: quotes rotate through a pool by design
  ([ADR 0001](docs/adr/0001-quote-pool-and-rotation.md)).
- Work toward further certification levels, beyond keeping what is met.

## Done

- `wiserone all` scales linearly (0.0.10).
- HTML-escaped page output, security assurance case, and the OpenSSF Best
  Practices passing badge (0.0.9).

Releases advance by 0.0.1 ([docs/POLICIES.md](docs/POLICIES.md)).
