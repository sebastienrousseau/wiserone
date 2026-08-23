// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use csv;
use serde::{Deserialize, Serialize};
use serde_json;
use std::{
    error::Error,
    fmt, fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};
use vrd::Random;

/// Builds the URL slug wiserone.com publishes a quote under.
///
/// The site's canonical page for every quote is `/q/<slug>/`, and each
/// dated URL points its `rel=canonical` there, so this is the only
/// truthful canonical for a generated page.
///
/// Lowercases, strips anything outside `[a-z0-9]` to single hyphens,
/// and truncates to 64 characters on a hyphen boundary — the same rule
/// the site's generator applies. Kept in step by
/// `test_slug_matches_published_urls`.
#[must_use]
pub fn slug(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut pending_hyphen = false;
    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            if pending_hyphen && !out.is_empty() {
                out.push('-');
            }
            pending_hyphen = false;
            out.push(ch.to_ascii_lowercase());
        } else {
            pending_hyphen = true;
        }
    }

    if out.len() > 64 {
        out.truncate(64);
        if let Some(cut) = out.rfind('-') {
            out.truncate(cut);
        }
    }
    out
}

/// Days elapsed since 0001-01-01 in the proleptic Gregorian calendar.
///
/// This is the ordinal Python's `date.toordinal()` returns, which is
/// what wiserone.com rotates on; 1970-01-01 is 719163. Pair it with
/// [`Quotes::select_daily_quote`] to show the same quote the site does.
///
/// Uses UTC, so the quote changes at midnight UTC everywhere rather
/// than drifting with the machine's timezone.
#[must_use]
pub fn current_day_number() -> i64 {
    const UNIX_EPOCH_ORDINAL: i64 = 719_163;
    const SECONDS_PER_DAY: i64 = 86_400;

    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64);
    UNIX_EPOCH_ORDINAL + seconds.div_euclid(SECONDS_PER_DAY)
}

/// Struct representing a single quote.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Quote {
    /// Position in the pool.
    ///
    /// wiserone.com selects the quote of the day by this index, so it is
    /// what orders the corpus. Optional so that a legacy file without it
    /// still loads.
    #[serde(default)]
    pub id: Option<usize>,
    /// Thematic block this quote belongs to, e.g. `elimination`.
    #[serde(default)]
    pub pillar: Option<String>,
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

    /// Selects the quote for a given day, the way the website does.
    ///
    /// `day_number` is a proleptic Gregorian ordinal — the value
    /// Python's `date.toordinal()` returns, where 1970-01-01 is 719163 —
    /// and the quote is `pool[day_number % len]`. Given the same corpus
    /// in the same order, this and wiserone.com show the same quote on
    /// the same day.
    ///
    /// The pool is ordered by [`Quote::id`]; entries without one keep
    /// their position in the file, which is why a legacy corpus still
    /// works but will not agree with the site.
    ///
    /// # Errors
    ///
    /// Returns an error if there are no quotes available.
    pub fn select_daily_quote(
        &self,
        day_number: i64,
    ) -> Result<&Quote, Box<dyn Error>> {
        if self.quotes.is_empty() {
            return Err("No available quotes".into());
        }

        let mut ordered: Vec<&Quote> = self.quotes.iter().collect();
        ordered.sort_by_key(|q| q.id.unwrap_or(usize::MAX));

        let len = ordered.len() as i64;
        // Floored modulo: a negative ordinal would otherwise index
        // backwards and panic.
        let index = (((day_number % len) + len) % len) as usize;
        Ok(ordered[index])
    }

    /// Selects a random quote.
    ///
    /// # Returns
    ///
    /// Returns a reference to a randomly selected `Quote` or an error
    /// if there are no quotes available.
    pub fn select_random_quote(
        &mut self,
    ) -> Result<&Quote, Box<dyn Error>> {
        if self.quotes.is_empty() {
            return Err("No available quotes".into());
        }

        // Random number generator.
        let mut rng = Random::new();
        // Random index selection.
        let rand_index =
            rng.int(0, self.quotes.len() as i32 - 1) as usize;

        Ok(&self.quotes[rand_index])
    }

    /// Selects all quotes, sorted by the date added.
    ///
    /// # Returns
    ///
    /// Returns all quotes or an error if no quotes are available.
    pub fn select_all_quotes(
        &self,
    ) -> Result<Vec<&Quote>, Box<dyn Error>> {
        if self.quotes.is_empty() {
            return Err("No available quotes".into());
        }

        let mut sorted_quotes =
            self.quotes.iter().collect::<Vec<&Quote>>();
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

    /// Error variant for path traversal attempts.
    PathTraversalError(String),
}

impl fmt::Display for QuoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuoteError::IOError(err) => write!(f, "I/O Error: {}", err),
            QuoteError::ParseError(msg) => {
                write!(f, "Parse Error: {}", msg)
            }
            QuoteError::NoQuotesAvailable => {
                write!(f, "No Quotes Available")
            }
            QuoteError::PathTraversalError(msg) => {
                write!(f, "Path Traversal Error: {}", msg)
            }
        }
    }
}

impl Error for QuoteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            QuoteError::IOError(err) => Some(err),
            QuoteError::ParseError(_) => None,
            QuoteError::NoQuotesAvailable => None,
            QuoteError::PathTraversalError(_) => None,
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

/// Validates that a file path is safe and doesn't contain path traversal sequences.
///
/// # Arguments
///
/// * `file_path` - The file path to validate.
///
/// # Returns
///
/// Returns `Ok(())` if the path is safe, or a `PathTraversalError` if unsafe.
fn validate_file_path(file_path: &str) -> Result<(), QuoteError> {
    // Check for directory traversal sequences
    if file_path.contains("..") {
        return Err(QuoteError::PathTraversalError(
            "Path contains directory traversal sequence".into(),
        ));
    }

    // Validate file extension
    let path = Path::new(file_path);
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") | Some("csv") => Ok(()),
        _ => Err(QuoteError::ParseError(
            "Only .json and .csv files are supported".into(),
        )),
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
///
/// # Security
///
/// This function validates the file path to prevent directory traversal
/// attacks. Only relative paths with .json or .csv extensions are
/// allowed.
pub fn read_quotes_from_file(
    file_path: &str,
) -> Result<Quotes, QuoteError> {
    // Validate the file path for security
    validate_file_path(file_path)?;

    let path = Path::new(file_path);
    match path.extension().and_then(|s| s.to_str()) {
        Some("json") => read_quotes_from_json(file_path),
        Some("csv") => read_quotes_from_csv(file_path),
        _ => Err(QuoteError::ParseError(
            "Unsupported file format".into(),
        )),
    }
}

/// Reads and parses quotes from a JSON file.
fn read_quotes_from_json(
    file_path: &str,
) -> Result<Quotes, QuoteError> {
    let file_content = fs::read_to_string(file_path)?;
    let quotes: Quotes = serde_json::from_str(&file_content)?;
    Ok(quotes)
}

/// Reads and parses quotes from a CSV file.
fn read_quotes_from_csv(file_path: &str) -> Result<Quotes, QuoteError> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let quotes = rdr
        .deserialize()
        .collect::<Result<Vec<Quote>, csv::Error>>()?;
    Ok(Quotes::new(quotes))
}
