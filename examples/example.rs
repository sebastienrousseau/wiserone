// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

extern crate wiserone;

// Importing necessary modules and traits from the standard library and `wiserone` crate.
use serde_json::to_string_pretty;
use std::error::Error;
use wiserone::html::generate_html_file;
use wiserone::quotes::read_quotes_from_file;

// The `main` function is the entry point of the program.
// It returns a Result type, indicating that it might return an error.
fn main() -> Result<(), Box<dyn Error>> {
    // Reading and parsing quotes from a JSON file.
    // The function `read_and_parse_quotes` is expected to read a JSON file,
    // parse it, and return a collection of quotes.
    println!("Reading and parsing quotes from a JSON file:");
    let mut quotes = read_quotes_from_file("./quotes/01-quotes.json")?;
    let json_string = to_string_pretty(&quotes).unwrap();
    println!("Quotes:\n{}\n", json_string);

    // Selecting a random quote from the collection.
    println!("Selecting a random quote:");
    let random_quote = quotes.select_random_quote()?;
    let json_string = to_string_pretty(&random_quote).unwrap();
    println!("Random Quote: {}\n", json_string);

    // Generating an HTML file that displays the random quote.
    // The function `generate_html_file` takes a filename and a quote,
    // and creates an HTML file with the quote.
    println!("Generating an HTML file for the random quote:");
    let filename = "../examples/example_quote.html";
    generate_html_file(filename, random_quote)?;
    println!("Generated HTML file: {}\n", filename);

    // If everything executes successfully, return Ok.
    Ok(())
}

