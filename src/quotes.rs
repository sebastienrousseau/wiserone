// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use csv;
use serde_json;
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt, fs, path::Path};
use vrd::Random;

/// Struct representing a single quote.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Quote {
    /// The text of the quote.
    pub quote_text: String,
    /// The author of the quote.
    pub author: String,
    /// The date when the quote was added to the JSON file.
    pub date_added: String,
    /// The URL of the image associated with the quote.
    pub image_url: String,
}

/// Struct representing a collection of quotes.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Quotes {
    /// Vector of `Quote` structs.
    pub quotes: Vec<Quote>,
}

impl Quotes {
    /// Initializes a new `Quotes` struct with the provided quotes.
    ///
    /// # Arguments
    ///
    /// * `quotes` - A vector of `Quote` structs.
    pub fn new(quotes: Vec<Quote>) -> Self {
        Quotes { quotes }
    }

    /// Selects a random quote.
    ///
    /// # Returns
    ///
    /// Returns a reference to a randomly selected `Quote` or an error
    /// if there are no quotes available.
    pub fn select_random_quote(&mut self) -> Result<&Quote, Box<dyn Error>> {
        if self.quotes.is_empty() {
            return Err("No available quotes".into());
        }

        // Random number generator.
        let mut rng = Random::new();
        // Random index selection.
        let rand_index = rng.int(0, self.quotes.len() as i32 - 1) as usize;

        Ok(&self.quotes[rand_index])
    }

    /// Selects all quotes, sorted by the date added.
    ///
    /// # Returns
    ///
    /// Returns all quotes or an error if no quotes are available.
    pub fn select_all_quotes(&self) -> Result<Vec<&Quote>, Box<dyn Error>> {
        if self.quotes.is_empty() {
            return Err("No available quotes".into());
        }

        let mut sorted_quotes = self.quotes.iter().collect::<Vec<&Quote>>();
        sorted_quotes.sort_by_key(|quote| &quote.date_added);

        Ok(sorted_quotes)
    }
}

/// Custom error type for quote handling.
#[derive(Debug)]
pub enum QuoteError {
    /// Error variant for I/O-related errors.
    IOError(std::io::Error),

    /// General error variant for parsing errors.
    ParseError(String),

    /// Error variant for when no quotes are available.
    NoQuotesAvailable,
}

impl fmt::Display for QuoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuoteError::IOError(err) => write!(f, "I/O Error: {}", err),
            QuoteError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            QuoteError::NoQuotesAvailable => write!(f, "No Quotes Available"),
        }
    }
}

impl Error for QuoteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QuoteError::IOError(err) => Some(err),
            QuoteError::ParseError(_) => None,
            QuoteError::NoQuotesAvailable => None,
        }
    }
}

impl From<std::io::Error> for QuoteError {
    fn from(error: std::io::Error) -> Self {
        QuoteError::IOError(error)
    }
}

impl From<serde_json::Error> for QuoteError {
    fn from(error: serde_json::Error) -> Self {
        QuoteError::ParseError(error.to_string())
    }
}

impl From<csv::Error> for QuoteError {
    fn from(error: csv::Error) -> Self {
        QuoteError::ParseError(error.to_string())
    }
}

/// Reads and parses quotes from a file (either JSON or CSV).
///
/// # Arguments
///
/// * `file_path` - Path to the file (JSON or CSV) containing quotes.
///
/// # Returns
///
/// Returns a `Quotes` struct if successful, or an error if the file
/// cannot be read or parsed.
pub fn read_quotes_from_file(file_path: &str) -> Result<Quotes, QuoteError> {
    let path = Path::new(file_path);
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => read_quotes_from_json(file_path),
        Some("csv") => read_quotes_from_csv(file_path),
        _ => Err(QuoteError::ParseError("Unsupported file format".into())),
    }
}

/// Reads and parses quotes from a JSON file.
fn read_quotes_from_json(file_path: &str) -> Result<Quotes, QuoteError> {
    let file_content = fs::read_to_string(file_path)?;
    let quotes: Quotes = serde_json::from_str(&file_content)?;
    Ok(quotes)
}

/// Reads and parses quotes from a CSV file.
fn read_quotes_from_csv(file_path: &str) -> Result<Quotes, QuoteError> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let quotes = rdr.deserialize().collect::<Result<Vec<Quote>, csv::Error>>()?;
    Ok(Quotes::new(quotes))
}
