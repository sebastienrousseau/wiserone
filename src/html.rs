// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use dtt::DateTime;
use rlg::{macro_log, LogFormat, LogLevel};
use std::{error::Error, fs::{self, File}, io::Write, path::Path};
use uuid::Uuid;
use crate::quotes::Quote;

/// Creates an HTML file based on the provided quote.
///
/// # Arguments
///
/// * `filename` - The name of the file to be created.
/// * `quote` - A reference to the quote to be used.
///
/// # Returns
///
/// Returns `Ok(())` if the file is successfully created, or an error
/// otherwise.
pub fn generate_html_file(
    filename: &str,
    quote: &Quote,
) -> Result<(), Box<dyn Error>> {
    let mut layout = fs::read_to_string("_layouts/quote.html")?;

    // Define date and time
    let dt = DateTime::new();
    let iso = dt.iso_8601;
    let year = dt.year;
    let month = &iso[5..7];
    let day = dt.day;
    
    // Determine if the date matches today
    let is_today = year == dt.year && month == &format!("{:02}", dt.month) && day == dt.day;

    let date = format!("{}_{}_{}", year, month, day);
    let prefix = if is_today {
        format!("https://wiserone.com/index.html") // If the date is today
    } else {
        format!("https://wiserone.com/{}.html", date) // For any other date
    };

    println!("Prefix: {}", prefix);

    // Replace the placeholders with values from the quote
    layout = layout.replace("{{apple_touch_icon_sizes}}", "192x192");
    layout = layout.replace("{{author}}", &quote.author);
    layout = layout.replace("{{banner}}", &quote.image_url);
    layout = layout.replace("{{cdn}}", "https://kura.pro");
    layout = layout.replace("{{charset}}", "utf-8");
    layout = layout.replace("{{description}}", "Daily nuggets of wisdom in a clean, minimalist design, inspiring deeper thought and personal growth with every visit.");
    layout = layout.replace("{{hreflang}}", "en");
    layout = layout.replace("{{item_pub_date}}", &quote.date_added);
    layout = layout.replace("{{date}}", quote.date_added.split('T').next().unwrap_or(""));
    layout = layout.replace("{{logo}}", "https://kura.pro/wiserone/images/logos/wiserone.webp");
    layout = layout.replace("{{measurementID}}", "G-4HKZ6N3QSC");
    layout = layout.replace("{{name}}", "wiserone");
    layout = layout.replace("{{title}}", &quote.quote_text);
    layout = layout.replace("{{url}}", "https://wiserone.com");
    layout = layout.replace("{{canonical}}", &prefix);

    fs::create_dir_all("./docs")?;
    let path = Path::new("./docs").join(filename);
    let mut file = fs::File::create(&path)?;
    file.write_all(layout.as_bytes())?;

    // Open the log file for appending
    let mut log_file = File::create("./wiserone.log")?;

    // Collect filenames into a vector, exclude .DS_Store, and sort them alphabetically
    let mut filenames: Vec<_> = fs::read_dir("./docs")?
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
    for filename in filenames {
        let uuid = Uuid::new_v4();
        let file_log = macro_log!(
            &uuid.to_string(),
            &iso,
            &LogLevel::INFO,
            "process",
            &format!("The HTML File is created at `{}`.", filename),
            &LogFormat::CLF
        );

        // Write the log to both the console and the file
        writeln!(log_file, "{}", file_log)?;

        // Define a function to check if the date is today
        let is_today = date.clone();

        // Remove "./docs/" from the beginning of the filename
        let relative_filename = filename.trim_start_matches("./docs/");

        // Remove the ".html" extension from the relative filename
        let relative_filename_without_extension = relative_filename.trim_end_matches(".html");

        if is_today == relative_filename_without_extension {
            let new_filename = "index.html".to_string();
            let new_full_path = Path::new("./docs").join(new_filename);
            let mut new_file = File::create(&new_full_path)?;
            let content = fs::read_to_string(&filename)?;
            new_file.write_all(content.as_bytes())?;
        }
    }
    println!("- info:wiserone: add file at `{}`", path.display());
    Ok(())
}