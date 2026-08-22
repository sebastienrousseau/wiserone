// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Regression guard: no test may write to or clear the committed `docs/`
//! tree.
//!
//! `docs/` holds 66 tracked files and is the directory GitHub Pages
//! serves for wiserone.com. A test that writes into it — or, as
//! `test_sitemap.rs` previously did, calls
//! `fs::remove_dir_all("./docs")` to get a clean slate — leaves ~30,000
//! tracked deletions in the working tree. A subsequent `git add -A`
//! commits them silently.
//!
//! This is a source-level check rather than a filesystem check on
//! purpose: integration tests are separate binaries that run in
//! parallel, so asserting "docs/ still exists" would race against
//! whichever test did the damage. Scanning the sources is
//! deterministic.

use std::fs;
use std::path::Path;

#[test]
fn no_test_references_the_committed_docs_dir() {
    let tests_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests");
    let this_file = "test_docs_not_touched.rs";

    let mut offenders = Vec::new();

    for entry in fs::read_dir(&tests_dir).expect("read tests/") {
        let path = entry.expect("dir entry").path();
        let name = match path.file_name().and_then(|s| s.to_str()) {
            Some(n) => n.to_owned(),
            None => continue,
        };
        if name == this_file || path.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }

        let source = fs::read_to_string(&path).expect("read test source");
        for (lineno, line) in source.lines().enumerate() {
            // Ignore comments, so documentation of this rule is allowed.
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            if code.contains("\"./docs\"") || code.contains("\"./docs/") {
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
        "integration tests must not target the committed ./docs tree \
         (it is 66 tracked files served by GitHub Pages). Use a scratch \
         directory and `generate_sitemap_file_in` instead.\n  {}",
        offenders.join("\n  ")
    );
}
