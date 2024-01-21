// Copyright notice and licensing information.
// Copyright © 2024 WiserOne. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate wiserone;

// Importing necessary modules and traits from the standard library and `wiserone` crate.
use std::error::Error;
use wiserone::html::generate_html_file;
use wiserone::quotes::read_and_parse_quotes;
use wiserone::run;

// The `main` function is the entry point of the program.
// It returns a Result type, indicating that it might return an error.
fn main() -> Result<(), Box<dyn Error>> {
    // Calling the `run` function from the `wiserone` crate.
    // This function encapsulates the main application logic.
    println!("Running the main application logic:");
    run()?;

    // Reading and parsing quotes from a JSON file.
    // The function `read_and_parse_quotes` is expected to read a JSON file,
    // parse it, and return a collection of quotes.
    println!("Reading and parsing quotes from a JSON file:");
    let mut quotes = read_and_parse_quotes("./quotes/01-quotes.json")?;
    println!("Quotes: {:?}", quotes);

    // Selecting a random quote from the collection.
    // This part assumes that the collection of quotes has a method
    // `select_random_quote` which randomly selects and returns a quote.
    println!("Selecting a random quote:");
    let random_quote = quotes.select_random_quote()?;
    println!("Random Quote: {:?}", random_quote);

    // Generating an HTML file that displays the random quote.
    // The function `generate_html_file` takes a filename and a quote,
    // and creates an HTML file with the quote.
    println!("Generating an HTML file for the random quote:");
    let filename = "example_quote.html";
    generate_html_file(filename, random_quote)?;
    println!("Generated HTML file: {}", filename);

    // If everything executes successfully, return Ok.
    Ok(())
}

