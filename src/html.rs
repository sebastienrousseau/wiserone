// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use crate::quotes::{slug, Quote};
use dtt::datetime::DateTime;
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;
use rlg::macro_log;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};
use uuid::Uuid;

/// The directory where HTML files are generated.
const OUTPUT_DIR: &str = "./docs";

/// The path to the HTML template file.
const TEMPLATE_PATH: &str = "_layouts/quote.html";

/// Validates that a filename is safe for use in file operations.
///
/// # Arguments
///
/// * `filename` - The filename to validate.
///
/// # Returns
///
/// Returns `Ok(())` if the filename is safe, or an error if unsafe.
fn validate_filename(filename: &str) -> Result<(), Box<dyn Error>> {
    // Check for directory traversal sequences
    if filename.contains("..")
        || filename.contains('/')
        || filename.contains('\\')
    {
        return Err(
            "Invalid filename: contains directory traversal characters"
                .into(),
        );
    }

    // Check for valid HTML extension
    if !filename.ends_with(".html") {
        return Err("Invalid filename: must end with .html".into());
    }

    // Check for empty or whitespace-only names
    let name_without_ext = filename.trim_end_matches(".html");
    if name_without_ext.is_empty()
        || name_without_ext.chars().all(|c| c.is_whitespace())
    {
        return Err("Invalid filename: name cannot be empty".into());
    }

    Ok(())
}

/// Validates that the template file exists and is readable.
///
/// # Returns
///
/// Returns `Ok(())` if the template is valid, or an error otherwise.
fn validate_template() -> Result<(), Box<dyn Error>> {
    let template_path = Path::new(TEMPLATE_PATH);

    if !template_path.exists() {
        return Err(format!(
            "Template file not found: {}",
            TEMPLATE_PATH
        )
        .into());
    }

    if !template_path.is_file() {
        return Err(format!(
            "Template path is not a file: {}",
            TEMPLATE_PATH
        )
        .into());
    }

    Ok(())
}

/// Creates an HTML file based on the provided quote.
///
/// # Arguments
///
/// * `filename` - The name of the file to be created (must be a simple filename, not a path).
/// * `quote` - A reference to the quote to be used.
///
/// # Returns
///
/// Returns `Ok(())` if the file is successfully created, or an error
/// otherwise.
///
/// # Security
///
/// This function validates the filename to prevent directory traversal attacks.
/// Files are always created in the designated output directory (./docs).
pub fn generate_html_file(
    filename: &str,
    quote: &Quote,
) -> Result<(), Box<dyn Error>> {
    generate_html_file_in(filename, quote, Path::new(OUTPUT_DIR))
}

/// Generates an HTML file for `quote` inside `output_dir`.
///
/// Behaves exactly like [`generate_html_file`], but writes into the
/// directory given rather than the default `./docs`. Prefer this in
/// tests so a run never touches the project's own output tree.
///
/// # Arguments
///
/// * `filename` - name of the file to create inside `output_dir`.
/// * `quote` - the quote rendered into the template.
/// * `output_dir` - directory written into; created if absent.
///
/// # Errors
///
/// Returns an error if the filename fails validation, the template is
/// missing, or the directory or file cannot be written.
///
/// # Security
///
/// The filename is validated to prevent directory traversal; files are
/// always created inside `output_dir`.
pub fn generate_html_file_in(
    filename: &str,
    quote: &Quote,
    output_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    // Validate filename to prevent path traversal
    validate_filename(filename)?;

    // Validate template exists before reading
    validate_template()?;

    let mut layout = fs::read_to_string(TEMPLATE_PATH)?;

    // Define date and time
    let dt = DateTime::new();
    let iso = dt.format_rfc3339()?;
    let year = dt.year();
    let month = &iso[5..7];
    let day = dt.day();

    // The canonical is the quote's own page on wiserone.com.
    //
    // This used to be `if is_today { index.html } else { <date>.html }`,
    // which was wrong twice over. The condition compared `dt` against
    // itself — `year == dt.year() && ...` — so it was always true and the
    // else branch was unreachable. And the URL it built,
    // `wiserone.com/YYYY_MM_DD.html`, has never been a page the site
    // serves; dated URLs use hyphens and, since the corpus became a
    // pool, they all canonicalise to `/q/<slug>/` anyway.
    let prefix =
        format!("https://wiserone.com/q/{}/", slug(&quote.quote_text));

    println!("Prefix: {}", prefix);

    // Replace the placeholders with values from the quote
    layout = layout.replace("{{apple_touch_icon_sizes}}", "192x192");
    layout = layout.replace("{{author}}", &quote.author);
    layout = layout.replace("{{banner}}", &quote.image_url);
    layout = layout.replace("{{cdn}}", "https://cloudcdn.pro");
    layout = layout.replace("{{charset}}", "utf-8");
    layout = layout.replace("{{description}}", "Daily nuggets of wisdom in a clean, minimalist design, inspiring deeper thought and personal growth with every visit.");
    layout = layout.replace("{{hreflang}}", "en");
    layout = layout.replace("{{item_pub_date}}", &quote.date_added);
    layout = layout.replace(
        "{{date}}",
        quote.date_added.split('T').next().unwrap_or(""),
    );
    layout = layout.replace(
        "{{logo}}",
        "https://cloudcdn.pro/clients/wiserone/v1/logos/wiserone.svg",
    );
    layout = layout.replace("{{measurementID}}", "G-4HKZ6N3QSC");
    layout = layout.replace("{{name}}", "wiserone");
    layout = layout.replace("{{title}}", &quote.quote_text);
    layout = layout.replace("{{url}}", "https://wiserone.com");
    layout = layout.replace("{{canonical}}", &prefix);

    fs::create_dir_all(output_dir)?;
    let path = output_dir.join(filename);
    let mut file = File::create(&path)?;
    file.write_all(layout.as_bytes())?;

    // Ensure log directory exists and open log file
    let log_dir = output_dir.join("logs");
    fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join("wiserone.log");
    let mut log_file = File::create(&log_path)?;

    // Collect filenames into a vector, exclude .DS_Store, and sort them alphabetically
    let mut filenames: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|entry| {
            entry.ok().map(|e| {
                let path = e.path();
                let path_str = path.to_string_lossy().into_owned();
                path_str
            })
        })
        .filter(|filename| !filename.ends_with(".DS_Store"))
        .collect();

    filenames.sort(); // Sort filenames alphabetically

    // Iterate over sorted filenames and log each one
    for filename in &filenames {
        let uuid = Uuid::new_v4();

        // Write the log to both the console and the file
        let file_log = macro_log!(
            &uuid.to_string(),
            &iso,
            &LogLevel::INFO,
            "process",
            &format!("The HTML File is created at `{}`.", filename),
            &LogFormat::CLF
        );
        writeln!(log_file, "{}", file_log)?;

        // Assuming year, month, and day are already defined correctly
        let today_formatted = format!(
            "{year}_{month:02}_{day:02}",
            year = year,
            month = month,
            day = day
        );

        // Create the file path for the current day's file if it doesn't already exist
        let today_file_path =
            output_dir.join(format!("{}.html", today_formatted));

        if today_file_path.exists() {
            let content = fs::read_to_string(&today_file_path)?;
            let index_path = output_dir.join("index.html");
            fs::write(index_path, content.as_bytes())?;

            // Write the log to both the console and the file
            let file_log = macro_log!(
                &Uuid::new_v4().to_string(),
                &iso,
                &LogLevel::INFO,
                "process",
                &format!(
                    "index.html updated with content from {}",
                    today_file_path.display()
                ),
                &LogFormat::CLF
            );
            writeln!(log_file, "{}", file_log)?;
        } else {
            // Write the log to both the console and the file
            let file_log = macro_log!(
                &Uuid::new_v4().to_string(),
                &iso,
                &LogLevel::INFO,
                "process",
                &format!(
                    "No file found at {}",
                    today_file_path.display()
                ),
                &LogFormat::CLF
            );
            writeln!(log_file, "{}", file_log)?;
        }
    }
    println!("- info:wiserone: add file at `{}`", path.display());
    Ok(())
}
