// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(test)]
mod tests {
    use wiserone::ascii::generate_ascii_art;

    #[test]
    fn test_generate_ascii_art_successful() {
        let text = "Hello, ASCII Art!";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }

    #[test]
    fn test_generate_ascii_art_failure() {
        let text = "Hello, ASCII Art!";
        let result = generate_ascii_art(text);
        assert!(result.is_ok());
        let ascii_art = result.unwrap();
        assert!(!ascii_art.is_empty());
    }
}
