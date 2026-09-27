// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Fuzzes corpus parsing, daily selection and slug generation.
//!
//! Any bytes that parse as a corpus must select a quote for every day
//! number, including negative ones and the extremes of `i64`, and every
//! slug must keep the site's URL rule: at most 64 characters, only
//! `[a-z0-9-]`, no leading, trailing or doubled hyphen.

#![no_main]

use libfuzzer_sys::fuzz_target;
use wiserone::quotes::{slug, Quotes};

const DAYS: [i64; 7] = [i64::MIN, -739_000, -1, 0, 1, 739_000, i64::MAX];

fn check_slug(text: &str) {
    let s = slug(text);
    assert!(s.len() <= 64, "slug longer than 64: {s:?}");
    assert!(
        s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-'),
        "slug has a character outside [a-z0-9-]: {s:?}"
    );
    assert!(!s.starts_with('-') && !s.ends_with('-') && !s.contains("--"), "{s:?}");
}

fuzz_target!(|data: &[u8]| {
    if let Ok(text) = std::str::from_utf8(data) {
        check_slug(text);
    }
    let Ok(quotes) = serde_json::from_slice::<Quotes>(data) else {
        return;
    };
    let non_empty = !quotes.quotes.is_empty();
    for day in DAYS {
        assert_eq!(quotes.select_daily_quote(day).is_ok(), non_empty);
    }
    for quote in quotes.select_all_quotes().unwrap_or_default() {
        check_slug(&quote.quote_text);
    }
});
