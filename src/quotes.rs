use std::error::Error;
use std::fs;

use serde::{Deserialize, Serialize};
use vrd::Random;

/// Struct representing a collection of quotes.
#[derive(Deserialize, Debug)]
pub struct Quotes {
    /// Vector containing multiple `Quote` structs.
    pub quotes: Vec<Quote>,
    /// Vector to track which quotes have been used.
    pub used_indices: Vec<bool>,
}

/// Struct representing a single quote.
#[derive(
    Clone,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct Quote {
    /// The text of the quote.
    pub quote_text: String,
    /// The author of the quote.
    pub author: String,
    /// The date the quote was added.
    pub date_added: String,
    /// URL of an image associated with the quote.
    pub image_url: String,
}

impl Quotes {
    /// Initializes a new `Quotes` struct.
    ///
    /// # Arguments
    ///
    /// * `quotes` - Vector of `Quote` structs.
    ///
    /// # Returns
    ///
    /// Returns a `Quotes` instance with `used_indices` initialized to
    /// track usage of quotes.
    pub fn new(quotes: Vec<Quote>) -> Self {
        let used_indices = vec![false; quotes.len()];
        Quotes { quotes, used_indices }
    }

    /// Selects a random quote that has not been used yet.
    ///
    /// # Returns
    ///
    /// Returns a reference to a randomly selected `Quote` or an error
    /// if no available quotes.
    pub fn select_random_quote(
        &mut self,
    ) -> Result<&Quote, Box<dyn Error>> {
        if self.quotes.is_empty()
            || self.used_indices.iter().all(|&used| used)
        {
            return Err("No available quotes".into());
        }

        let mut rng = Random::new();
        let mut rand_index =
            rng.int(0, self.quotes.len() as i32 - 1) as usize;

        while self.used_indices[rand_index] {
            rand_index =
                rng.int(0, self.quotes.len() as i32 - 1) as usize;
        }

        self.used_indices[rand_index] = true;
        Ok(&self.quotes[rand_index])
    }
    /// Selects all quotes.
    ///
    /// # Returns
    ///
    /// Returns all the quotes or an error if no available quotes.
    pub fn select_all_quotes(&self) -> Result<Vec<&Quote>, Box<dyn Error>> {
        if self.quotes.is_empty() {
            return Err("No available quotes".into());
        }

        let mut sorted_quotes = self.quotes.iter().collect::<Vec<&Quote>>();
        sorted_quotes.sort_by_key(|quote| &quote.date_added);

        Ok(sorted_quotes)
    }
}
/// Reads and parses quotes from a JSON file.
///
/// # Arguments
///
/// * `file_path` - Path to the JSON file containing quotes.
///
/// # Returns
///
/// Returns a `Quotes` struct if successful, or an error if the file
/// cannot be read or parsed.
pub fn read_and_parse_quotes(
    file_path: &str,
) -> Result<Quotes, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;

    // Define a temporary struct to match the JSON structure
    #[derive(Deserialize)]
    struct TempQuotes {
        quotes: Vec<Quote>,
    }

    let temp_quotes: TempQuotes = serde_json::from_str(&file_content)?;

    // Now use the `quotes` field from `TempQuotes` to initialize
    // `Quotes`
    Ok(Quotes::new(temp_quotes.quotes))
}
/// Reads and parses all the quotes from a JSON file.
///
/// # Arguments
///
/// * `file_path` - Path to the JSON file containing quotes.
///
/// # Returns
///
/// Returns all the `Quotes` struct if successful, or an error if the file
/// cannot be read or parsed.
pub fn read_and_parse_all_quotes(
    file_path: &str,
) -> Result<Quotes, Box<dyn Error>> {
    let file_content = fs::read_to_string(file_path)?;

    // Define a temporary struct to match the JSON structure
    #[derive(Deserialize)]
    struct TempQuotes {
        quotes: Vec<Quote>,
    }

    let temp_quotes: TempQuotes = serde_json::from_str(&file_content)?;

    // Now use the `quotes` field from `TempQuotes` to initialize
    // `Quotes`
    Ok(Quotes::new(temp_quotes.quotes))
}