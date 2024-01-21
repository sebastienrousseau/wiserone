// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use clap::Parser;
use std::error::Error;
use std::fs::File;
use std::io::Write;

use dtt::DateTime;
use rlg::{macro_log, LogFormat, LogLevel};

use crate::html::generate_html_file;
use crate::sitemap::generate_sitemap_file;
use crate::quotes::read_quotes_from_file;
use crate::ascii::generate_ascii_art;

#[derive(Parser)]
#[clap(author, version, about)]
/// A command line program that generates an HTML file containing a
/// quote from the JSON file. The program can generate a random quote
/// or all quotes from the JSON file.

#[derive(Debug)]
pub enum Command {
    /// Selects a random quote from the JSON or CSV file and creates an HTML
    /// file based on the quote.
    Random {
        /// The name of the JSON or CSV file containing quotes.
        filename: String,
    },
    /// Selects all quotes from the JSON or CSV file and creates an HTML file
    /// for each quote.
    All {
        /// The name of the JSON file containing quotes.
        filename: String,
    },
}

/// The entry point of the program.
///
/// # Arguments
///
/// * `args`: The command line arguments passed to the program.
///
/// # Returns
///
/// * `i32`: An exit code indicating the success or failure of the
///   program.
pub fn run_cli() -> Result<(), Box<dyn Error>> {
    // Open the log file for appending
    let mut log_file = File::create("./wiserone.log")?;

    // Define date and time
    let dt = DateTime::new();
    let iso = dt.iso_8601;
    let year = dt.year;
    let month = &iso[5..7];
    let day = dt.day;
    let date = format!("{}_{}_{}", year, month, day);

    // Generate a log entry
    let ascii_art_log =
        macro_log!(
            "id",
            &iso,
            &LogLevel::INFO,
            "process",
            "ASCII art generation event started.",
            &LogFormat::CLF
        );

    match generate_ascii_art("The Wiser One") {
        Ok(ascii_art) => println!("{}", ascii_art),
        Err(e) => eprintln!("Error generating ASCII art: {:?}", e),
    }

    // Write the log to both the console and the file
    writeln!(log_file, "{}", ascii_art_log)?;

    // Parse the command line arguments using the `clap` crate.
    let command = Command::parse();

    match command {
        Command::Random { filename } => {
            println!("- info:wiserone: begin generating a random quote");
            // Construct the HTML filename using `iso`
            let html_filename = format!("{}.html", date);

            // Read and parse quotes, then select a random quote
            let mut quotes = read_quotes_from_file(&filename)?;
            let quote = quotes.select_random_quote()?;
            generate_html_file(&html_filename, quote)?;
            generate_sitemap_file("https://wiserone.com/")?;
        },
        Command::All { filename } => {
            println!("- info:wiserone: begin generating all quotes");
            // Read and parse all quotes
            let quotes = read_quotes_from_file(&filename)?;

            // Generate an HTML file for each quote
            for quote in quotes.select_all_quotes()? {
                // Split `date_added` at "T" and take the date part
                let date_part = quote.date_added.split('T').next().unwrap_or("");

                // Replace "-" with "_" to format as "YYYY_MM_DD"
                let formatted_date = date_part.replace('-', "_");

                let html_filename = format!("{}.html", formatted_date);
                generate_html_file(&html_filename, quote)?;
                generate_sitemap_file("https://wiserone.com/")?;
            }
            println!("- info:wiserone: end generating all quotes\n\n");
        },
    }

    Ok(())
}
