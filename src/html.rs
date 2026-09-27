// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use crate::quotes::{slug, Quote};
use dtt::datetime::DateTime;
use rlg::log::Log;
use rlg::log_format::LogFormat;
use rlg::log_level::LogLevel;
use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
};

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

    let layout =
        render_page(quote, &fs::read_to_string(TEMPLATE_PATH)?);

    fs::create_dir_all(output_dir)?;
    let path = output_dir.join(filename);
    let mut file = File::create(&path)?;
    file.write_all(layout.as_bytes())?;

    log_and_refresh_index(output_dir)?;
    println!("- info:wiserone: add file at `{}`", path.display());
    Ok(())
}

/// Fills the page template's placeholders from `quote`.
fn render_page(quote: &Quote, template: &str) -> String {
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

    // Replace the placeholders with values from the quote. The order is
    // the one the replacements have always run in.
    let date = quote.date_added.split('T').next().unwrap_or("");
    let replacements: [(&str, &str); 15] = [
        ("{{apple_touch_icon_sizes}}", "192x192"),
        ("{{author}}", &quote.author),
        ("{{banner}}", &quote.image_url),
        ("{{cdn}}", "https://cloudcdn.pro"),
        ("{{charset}}", "utf-8"),
        ("{{description}}", "Daily nuggets of wisdom in a clean, minimalist design, inspiring deeper thought and personal growth with every visit."),
        ("{{hreflang}}", "en"),
        ("{{item_pub_date}}", &quote.date_added),
        ("{{date}}", date),
        ("{{logo}}", "https://cloudcdn.pro/clients/wiserone/v1/logos/wiserone.svg"),
        ("{{measurementID}}", "G-4HKZ6N3QSC"),
        ("{{name}}", "wiserone"),
        ("{{title}}", &quote.quote_text),
        ("{{url}}", "https://wiserone.com"),
        ("{{canonical}}", &prefix),
    ];
    replacements
        .iter()
        .fold(template.to_owned(), |page, (key, value)| {
            page.replace(key, value)
        })
}

/// Every entry in `output_dir` except `.DS_Store`, sorted alphabetically.
fn sorted_entries(
    output_dir: &Path,
) -> Result<Vec<String>, Box<dyn Error>> {
    let mut filenames: Vec<_> = fs::read_dir(output_dir)?
        .filter_map(|entry| {
            entry.ok().map(|e| e.path().to_string_lossy().into_owned())
        })
        .filter(|filename| !filename.ends_with(".DS_Store"))
        .collect();
    filenames.sort();
    Ok(filenames)
}

/// Logs every file in `output_dir`, refreshing `index.html` from
/// today's page after each one, as generation always has.
fn log_and_refresh_index(
    output_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    // Define date and time
    let dt = DateTime::new();
    let iso = dt.format_rfc3339()?;
    let today_formatted = format!(
        "{year}_{month:02}_{day:02}",
        year = dt.year(),
        month = &iso[5..7],
        day = dt.day()
    );

    // Ensure log directory exists and open log file
    let log_dir = output_dir.join("logs");
    fs::create_dir_all(&log_dir)?;
    let log_path = log_dir.join("wiserone.log");
    let mut log_file = File::create(&log_path)?;
    let filenames = sorted_entries(output_dir)?;

    // Create the file path for the current day's file
    let today_file_path =
        output_dir.join(format!("{}.html", today_formatted));

    // Iterate over sorted filenames and log each one
    for filename in &filenames {
        let msg =
            format!("The HTML File is created at `{}`.", filename);
        write_log(&mut log_file, &iso, &msg)?;
        refresh_index(
            output_dir,
            &today_file_path,
            &mut log_file,
            &iso,
        )?;
    }
    Ok(())
}

/// Copies today's page to `index.html` when it exists, logging either way.
fn refresh_index(
    output_dir: &Path,
    today_file_path: &Path,
    log_file: &mut File,
    iso: &str,
) -> Result<(), Box<dyn Error>> {
    let msg = if today_file_path.exists() {
        let content = fs::read_to_string(today_file_path)?;
        fs::write(output_dir.join("index.html"), content.as_bytes())?;
        format!(
            "index.html updated with content from {}",
            today_file_path.display()
        )
    } else {
        format!("No file found at {}", today_file_path.display())
    };
    write_log(log_file, iso, &msg)
}

/// Writes one CLF-formatted INFO entry to `log_file`.
fn write_log(
    log_file: &mut File,
    iso: &str,
    msg: &str,
) -> Result<(), Box<dyn Error>> {
    let entry = Log::build(LogLevel::INFO, msg)
        .time(iso)
        .component("process")
        .format(LogFormat::CLF);
    writeln!(log_file, "{}", entry)?;
    Ok(())
}
