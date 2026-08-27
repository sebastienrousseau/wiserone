// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use clap::Parser;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use dtt::datetime::DateTime;
use rlg::log::Log;
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;

use crate::ascii::generate_ascii_art;
use crate::html::generate_html_file;
use crate::quotes::{current_day_number, read_quotes_from_file};
use crate::sitemap::generate_sitemap_file;

/// The directory where output files (including logs) are stored.
const OUTPUT_DIR: &str = "./docs";

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
    /// Selects the quote of the day — the same one wiserone.com shows —
    /// and creates an HTML file based on it.
    Daily {
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
    match run_cli_from(std::env::args_os()) {
        // A clap error here is `--help`, `--version`, or a usage
        // mistake. `Error::exit` renders it the way a CLI should:
        // help and version to stdout with status 0, usage errors to
        // stderr with status 2. Returning it instead would make
        // `wiserone --help` exit non-zero.
        Err(e) => match e.downcast::<clap::Error>() {
            Ok(clap_error) => clap_error.exit(),
            Err(other) => Err(other),
        },
        ok => ok,
    }
}

/// Runs the CLI against an explicit argument list.
///
/// [`run_cli`] delegates here with [`std::env::args_os`]. Prefer this
/// wherever the arguments should not depend on how the process was
/// started: under `cargo bench` the process arguments include
/// `--bench`, and under `cargo test` they belong to the test harness,
/// so parsing them would fail for reasons unrelated to this CLI.
///
/// # Errors
///
/// Returns an error if `args` fail to parse, or if generating the
/// output files fails.
pub fn run_cli_from<I, T>(args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    // Ensure log directory exists and open log file
    let log_dir = Path::new(OUTPUT_DIR).join("logs");
    fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join("wiserone.log");
    let mut log_file = File::create(&log_path)?;

    // Define date and time
    let dt = DateTime::new();
    let iso = dt.format_rfc3339()?;
    let year = dt.year();
    let month = &iso[5..7];
    let day = dt.day();
    let date = format!("{}_{}_{}", year, month, day);

    // Generate a log entry
    let ascii_art_log = Log::build(
        LogLevel::INFO,
        "ASCII art generation event started.",
    )
    .time(&iso)
    .component("process")
    .format(LogFormat::CLF);

    match generate_ascii_art("The Wiser One") {
        Ok(ascii_art) => println!("{}", ascii_art),
        Err(e) => eprintln!("Error generating ASCII art: {:?}", e),
    }

    // Write the log to both the console and the file
    writeln!(log_file, "{}", ascii_art_log)?;

    // Parse the command line arguments using the `clap` crate.
    let command = Command::try_parse_from(args)?;

    match command {
        Command::Random { filename } => {
            println!(
                "- info:wiserone: begin generating a random quote"
            );
            // Construct the HTML filename using `iso`
            let html_filename = format!("{}.html", date);

            // Read and parse quotes, then select a random quote
            let mut quotes = read_quotes_from_file(&filename)?;
            let quote = quotes.select_random_quote()?;
            generate_html_file(&html_filename, quote)?;
            generate_sitemap_file("https://wiserone.com/")?;
        }
        Command::Daily { filename } => {
            println!(
                "- info:wiserone: begin generating the quote of the day"
            );
            let html_filename = format!("{}.html", date);
            let quotes = read_quotes_from_file(&filename)?;
            let quote =
                quotes.select_daily_quote(current_day_number())?;
            generate_html_file(&html_filename, quote)?;
            generate_sitemap_file("https://wiserone.com/")?;
        }
        Command::All { filename } => {
            println!("- info:wiserone: begin generating all quotes");
            // Read and parse all quotes
            let quotes = read_quotes_from_file(&filename)?;

            // Generate an HTML file for each quote
            for quote in quotes.select_all_quotes()? {
                // Name by pool position, not by date. `date_added` used
                // to be one-per-day and unique; in a pool it records the
                // day a line was written, and six of the current 136
                // share a date. Naming by it silently overwrote files.
                let html_filename = match quote.id {
                    Some(id) => format!("quote-{:04}.html", id),
                    None => {
                        let date_part = quote
                            .date_added
                            .split('T')
                            .next()
                            .unwrap_or("");
                        format!("{}.html", date_part.replace('-', "_"))
                    }
                };
                generate_html_file(&html_filename, quote)?;
                generate_sitemap_file("https://wiserone.com/")?;
            }
            println!("- info:wiserone: end generating all quotes\n\n");
        }
    }

    Ok(())
}
