// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Regression guard: no test may clear the project's own `docs/` tree.
//!
//! `docs/` is the generator's output directory at the repository root.
//! `tests/test_sitemap.rs` once called `fs::remove_dir_all("./docs")` to
//! get a clean slate, which deleted every tracked file in it — ~30,000
//! lines of deletions left staged in the working tree, ready for an
//! unsuspecting `git add -A`.
//!
//! Two legitimate ways for a test to touch `docs/` exist, and this guard
//! allows both:
//!
//! * take an explicit directory — `generate_sitemap_file_in` and
//!   `generate_html_file_in` accept one, so a scratch dir works; or
//! * change the process working directory into a temp dir first, in
//!   which case `"./docs"` resolves inside that temp dir. Because
//!   `set_current_dir` is process-wide, such tests MUST serialize on a
//!   mutex — `tests/test_html.rs` does this with `DIR_MUTEX`.
//!
//! So a bare `"./docs"` is only a problem in a file that never changes
//! directory, and a `remove_dir_all` of it is never acceptable.
//!
//! Source-level rather than filesystem-level on purpose: integration
//! tests are separate binaries running in parallel, so asserting
//! "docs/ still exists" would race whichever test removed it.

use std::fs;
use std::path::Path;

#[test]
fn no_test_clears_the_projects_docs_dir() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let this_file = "test_docs_not_touched.rs";
    let mut offenders = Vec::new();

    for entry in fs::read_dir(&tests_dir).expect("read tests/") {
        let path = entry.expect("dir entry").path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(n) => n.to_owned(),
            None => continue,
        };
        if name == this_file
            || path.extension().and_then(|s| s.to_str()) != Some("rs")
        {
            continue;
        }

        let source =
            fs::read_to_string(&path).expect("read test source");
        // A file that relocates the process cwd resolves "./docs"
        // inside its own temp dir, so only destructive calls matter.
        let relocates_cwd = source.contains("set_current_dir");

        for (lineno, line) in source.lines().enumerate() {
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            let mentions_docs = code.contains("\"./docs\"")
                || code.contains("\"./docs/");
            if !mentions_docs {
                continue;
            }
            let destructive = code.contains("remove_dir_all")
                || code.contains("remove_dir")
                || code.contains("remove_file");
            if destructive || !relocates_cwd {
                offenders.push(format!(
                    "{}:{}: {}",
                    name,
                    lineno + 1,
                    line.trim()
                ));
            }
        }
    }

    assert!(
        offenders.is_empty(),
        "tests must not clear or write the project's ./docs tree.\n\
         Use `generate_sitemap_file_in` / `generate_html_file_in` with a \
         scratch directory, or change into a temp dir first (serialized \
         on a mutex, as tests/test_html.rs does).\n  {}",
        offenders.join("\n  ")
    );
}
