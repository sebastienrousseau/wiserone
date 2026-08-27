// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! # `wiserone` 🦀

// Crate configuration
#![deny(dead_code)]
#![deny(missing_debug_implementations)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]
#![doc(
    html_favicon_url = "",
    html_logo_url = "",
    html_root_url = "https://docs.rs/wiserone"
)]

// Import necessary dependencies
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use dtt::datetime::DateTime;
use rlg::log::Log;
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;

use crate::loggers::init_logger;

/// The directory where output files (including logs) are stored.
const OUTPUT_DIR: &str = "./docs";

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

    // A clap error here is `--help`, `--version`, or a usage mistake.
    // `Error::exit` renders it the way a CLI should: help and version to
    // stdout with status 0, usage errors to stderr with status 2.
    // `run_with` returns the error instead of exiting, so that it stays
    // callable from a test without killing the test process — which is
    // the same split as `run_cli` and `run_cli_from`.
    match run_with(std::env::args_os()) {
        Err(e) => match e.downcast::<clap::Error>() {
            Ok(clap_error) => clap_error.exit(),
            Err(other) => Err(other),
        },
        ok => ok,
    }
}

/// Runs the application against an explicit argument list.
///
/// [`run`] delegates here after initialising the logger. Split out for
/// the same reason [`cli::run_cli_from`] exists: `run` reads
/// `std::env::args_os()`, which under a test harness holds the
/// harness's own arguments, so the whole body was unreachable from the
/// suite and sat at zero coverage.
///
/// # Errors
///
/// Returns an error if the log directory or file cannot be created, if
/// the arguments fail to parse, or if generating the quote fails.
pub fn run_with<I, T>(args: I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    // Define date and time
    let date = DateTime::new();
    let iso = date.format_rfc3339()?;

    // Ensure log directory exists and open log file
    let log_dir = Path::new(OUTPUT_DIR).join("logs");
    fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join("wiserone.log");
    let mut log_file = File::create(&log_path)?;

    // Call into the CLI with the supplied arguments
    cli::run_cli_from(args)?;

    // Generate a log entry
    let quote_log = Log::build(
        LogLevel::INFO,
        "Quote HTML file generated successfully.",
    )
    .time(&iso)
    .component("process")
    .format(LogFormat::CLF);

    // Write the log to both the console and the file
    writeln!(log_file, "{}", quote_log)?;

    Ok(())
}
