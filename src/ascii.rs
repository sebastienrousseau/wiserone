// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use figlet_rs::FIGlet;
use std::error::Error;
use std::fmt;

/// Error type for ASCII art generation failures.
#[derive(Debug)]
pub enum ArtError {
    /// Represents a failure to load the FIGlet.
    FontLoadError,
    /// Represents a failure to convert text to ASCII art.
    ConversionError,
}

impl fmt::Display for ArtError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::FontLoadError => {
                write!(f, "Failed to load FIGlet")
            }
            Self::ConversionError => {
                write!(f, "Failed to convert text to ASCII art")
            }
        }
    }
}

impl Error for ArtError {}

/// Generates ASCII art from the given text using the standard `FIGlet`.
///
/// # Arguments
///
/// * `text` - The text to convert to ASCII art.
///
/// # Errors
///
/// This function returns an `Err` in the following situations:
///
/// - If the standard `FIGlet` fails to load (`FontLoadError`).
/// - If the text cannot be converted to ASCII art (`ConversionError`).
///
pub fn generate_ascii_art(text: &str) -> Result<String, ArtError> {
    let standard_font =
        FIGlet::standard().map_err(|_| ArtError::FontLoadError)?;
    let figure =
        standard_font.convert(text).ok_or(ArtError::ConversionError)?;
    Ok(figure.to_string())
}
