// Copyright notice and licensing information.
// Copyright © 2024 WiserOne. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # `wiserone` 🦀

// Crate configuration
#![cfg_attr(feature = "bench", feature(test))]
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![warn(unreachable_pub)]
#![doc(
    html_favicon_url = "",
    html_logo_url = "",
    html_root_url = "https://docs.rs/wiserone"
)]

// Import necessary dependencies
use std::error::Error;
use std::fs::File;
use std::io::Write;

use dtt::DateTime;
use rlg::{macro_log, LogFormat, LogLevel};

use crate::loggers::init_logger;

/// The `ascii` module contains functions for generating ASCII art.
pub mod ascii;

/// The `cli` module contains functions for processing command-line
/// input.
pub mod cli;

/// The `html` module contains functions for generating HTML files.
pub mod html;

/// The `quotes` module contains functions for reading and parsing
/// quotes.
pub mod quotes;

/// The `sitemap` module contains functions for generating a sitemap.xml
pub mod sitemap;

/// The `loggers` module contains the loggers for the library.
pub mod loggers;

/// The `macros` module contains functions for generating macros.
pub mod macros;

/// Entry point of the application.
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if an operation fails.
pub fn run() -> Result<(), Box<dyn Error>> {
    // Initialize the logger using the `env_logger` crate
    init_logger(None)?;

    // Define date and time
    let date = DateTime::new();
    let iso = date.iso_8601;

    // Open the log file for appending
    let mut log_file = File::create("./wiserone.log")?;

    // Call the `run_cli()` function from the `cli` module
    cli::run_cli()?;

    // Generate a log entry
    let quote_log =
        macro_log!(
            "id",
            &iso,
            &LogLevel::INFO,
            "process",
            "Quote HTML file generated successfully.",
            &LogFormat::CLF
        );

    // Write the log to both the console and the file
    writeln!(log_file, "{}", quote_log)?;

    Ok(())
}
