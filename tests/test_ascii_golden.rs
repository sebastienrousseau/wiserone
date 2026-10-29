// Copyright notice and licensing information.
// Copyright © 2024-2026 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Golden tests for `generate_ascii_art`. Each file under `tests/golden/ascii`
//! is the output of figlet-rs 1.0.0's standard font for the same text,
//! captured before the in-crate renderer replaced that dependency.

use wiserone::ascii::{generate_ascii_art, ArtError};

#[test]
fn renders_like_figlet_rs() {
    let cases = [
        (
            "The Wiser One",
            include_str!("golden/ascii/the_wiser_one.txt"),
        ),
        ("Hi", include_str!("golden/ascii/hi.txt")),
        // Not strings wiserone renders, but they exercise the smushing rules.
        ("Hello, world!", include_str!("golden/ascii/hello_world.txt")),
        ("!@#$%^&*()_+", include_str!("golden/ascii/punctuation.txt")),
    ];
    for (text, expected) in cases {
        assert_eq!(
            generate_ascii_art(text).unwrap(),
            expected,
            "{text:?}"
        );
    }
}

#[test]
fn rejects_text_the_font_cannot_render() {
    assert_eq!(generate_ascii_art(""), Err(ArtError::ConversionError));
    assert_eq!(
        generate_ascii_art("\u{1F600}"),
        Err(ArtError::ConversionError)
    );
}
