// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Integration tests for the command-line interface (`wiserone::cli`).

use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::Mutex;
use tempfile::TempDir;
use wiserone::cli::Command;

/// Mutex to serialize tests that change the process-wide current directory.
static DIR_MUTEX: Mutex<()> = Mutex::new(());

/// Test data for quote testing
fn create_test_quote_json(temp_dir: &Path) -> String {
    let test_quotes = r#"{
        "quotes": [
            {
                "quote_text": "Test quote 1",
                "author": "Test Author 1",
                "date_added": "2024-01-01T10:00:00Z",
                "image_url": "https://example.com/image1.jpg"
            },
            {
                "quote_text": "Test quote 2",
                "author": "Test Author 2",
                "date_added": "2024-01-02T10:00:00Z",
                "image_url": "https://example.com/image2.jpg"
            }
        ]
    }"#;

    let quotes_file = temp_dir.join("test_quotes.json");
    let mut file = File::create(&quotes_file).unwrap();
    file.write_all(test_quotes.as_bytes()).unwrap();

    quotes_file.to_string_lossy().to_string()
}

fn create_test_quote_csv(temp_dir: &Path) -> String {
    let test_csv = "quote_text,author,date_added,image_url\n\
                    \"CSV Quote 1\",\"CSV Author 1\",\"2024-01-01T10:00:00Z\",\"https://example.com/csv1.jpg\"\n\
                    \"CSV Quote 2\",\"CSV Author 2\",\"2024-01-02T10:00:00Z\",\"https://example.com/csv2.jpg\"";

    let csv_file = temp_dir.join("test_quotes.csv");
    let mut file = File::create(&csv_file).unwrap();
    file.write_all(test_csv.as_bytes()).unwrap();

    csv_file.to_string_lossy().to_string()
}

fn create_layout_template(temp_dir: &Path) {
    let layout_dir = temp_dir.join("_layouts");
    fs::create_dir_all(&layout_dir).unwrap();

    let template = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{title}}</title>
    <meta charset="{{charset}}">
    <meta name="description" content="{{description}}">
    <meta name="author" content="{{author}}">
    <link rel="canonical" href="{{canonical}}">
</head>
<body>
    <h1>{{title}}</h1>
    <p>By: {{author}}</p>
    <p>Date: {{date}}</p>
    <img src="{{banner}}" alt="Quote banner">
</body>
</html>"#;

    let template_file = layout_dir.join("quote.html");
    let mut file = File::create(&template_file).unwrap();
    file.write_all(template.as_bytes()).unwrap();
}

/// Test Command enum parsing for Random variant
#[test]
fn test_command_random_variant() {
    let cmd = Command::Random { filename: "test.json".to_string() };

    match cmd {
        Command::Random { filename } => {
            assert_eq!(filename, "test.json");
        }
        _ => panic!("Expected Random variant"),
    }
}

/// Test Command enum parsing for All variant
#[test]
fn test_command_all_variant() {
    let cmd = Command::All { filename: "test.json".to_string() };

    match cmd {
        Command::All { filename } => {
            assert_eq!(filename, "test.json");
        }
        _ => panic!("Expected All variant"),
    }
}

/// Test run_cli with valid JSON file - Random command simulation
#[test]
fn test_run_cli_with_json_file() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();

    // Change to temp directory for isolated testing
    std::env::set_current_dir(temp_dir.path()).unwrap();

    // Setup test environment
    let quotes_file = create_test_quote_json(temp_dir.path());
    create_layout_template(temp_dir.path());

    // Create docs directory
    fs::create_dir_all("./docs").unwrap();

    // Test requires actual CLI parsing which is complex to mock
    // This tests the core file operations in isolation
    let result = std::panic::catch_unwind(|| {
        // We can't easily test run_cli() directly due to clap parsing
        // So we test the underlying components it uses
        let mut quotes =
            wiserone::quotes::read_quotes_from_file(&quotes_file)
                .unwrap();
        let quote = quotes.select_random_quote().unwrap();
        assert!(!quote.quote_text.is_empty());
        assert!(!quote.author.is_empty());
    });

    // Restore original directory
    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok());
}

/// Test file I/O error handling in CLI operations
#[test]
fn test_cli_file_io_error_handling() {
    let temp_dir = TempDir::new().unwrap();

    // Test with non-existent file (absolute path)
    let non_existent = temp_dir.path().join("non_existent.json");
    let result = wiserone::quotes::read_quotes_from_file(
        &non_existent.to_string_lossy(),
    );
    assert!(result.is_err());

    // Test with invalid JSON
    let invalid_json_file = temp_dir.path().join("invalid.json");
    let mut file = File::create(&invalid_json_file).unwrap();
    file.write_all(b"invalid json content").unwrap();

    let result = wiserone::quotes::read_quotes_from_file(
        &invalid_json_file.to_string_lossy(),
    );
    assert!(result.is_err());
}

/// Test CSV file processing path
#[test]
fn test_cli_csv_file_processing() {
    let temp_dir = TempDir::new().unwrap();

    let csv_file = create_test_quote_csv(temp_dir.path());
    let result = wiserone::quotes::read_quotes_from_file(&csv_file);

    assert!(result.is_ok());
    let quotes = result.unwrap();
    assert_eq!(quotes.quotes.len(), 2);
    assert_eq!(quotes.quotes[0].quote_text, "CSV Quote 1");
}

/// Test date formatting logic
#[test]
fn test_date_formatting() {
    use dtt::datetime::DateTime;

    let dt = DateTime::new();
    let iso = dt.format_rfc3339().expect("rfc3339");
    let year = dt.year();
    let month = &iso[5..7];
    let day = dt.day();
    let formatted_date = format!("{}_{}_{}", year, month, day);

    // Verify format matches YYYY_MM_DD pattern
    let parts: Vec<&str> = formatted_date.split('_').collect();
    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0].len(), 4); // YYYY
    assert_eq!(parts[1].len(), 2); // MM
    assert_eq!(parts[2], day.to_string()); // DD
}

/// Test quote date parsing and filename generation
#[test]
fn test_quote_date_parsing() {
    let date_added = "2024-01-15T10:30:00Z";
    let date_part = date_added.split('T').next().unwrap_or("");
    let formatted_date = date_part.replace('-', "_");

    assert_eq!(date_part, "2024-01-15");
    assert_eq!(formatted_date, "2024_01_15");

    let html_filename = format!("{}.html", formatted_date);
    assert_eq!(html_filename, "2024_01_15.html");
}

/// Test empty date handling
#[test]
fn test_empty_date_handling() {
    let empty_date = "";
    let date_part = empty_date.split('T').next().unwrap_or("");
    assert_eq!(date_part, "");

    let formatted_date = date_part.replace('-', "_");
    assert_eq!(formatted_date, "");
}

/// Test malformed date handling
#[test]
fn test_malformed_date_handling() {
    let malformed_date = "not-a-date";
    let date_part = malformed_date.split('T').next().unwrap_or("");
    assert_eq!(date_part, "not-a-date");

    let formatted_date = date_part.replace('-', "_");
    assert_eq!(formatted_date, "not_a_date");
}

/// Property-based test for date formatting consistency
#[test]
fn test_date_formatting_properties() {
    use dtt::datetime::DateTime;

    // Test that date formatting is deterministic
    let dt1 = DateTime::new();
    let dt2 = DateTime::new();

    let iso1 = dt1.format_rfc3339().expect("rfc3339");
    let iso2 = dt2.format_rfc3339().expect("rfc3339");

    let month1 = &iso1[5..7];
    let month2 = &iso2[5..7];

    // Months should be valid (01-12)
    let month_num1: u8 = month1.parse().unwrap();
    let month_num2: u8 = month2.parse().unwrap();

    assert!((1..=12).contains(&month_num1));
    assert!((1..=12).contains(&month_num2));
}

/// Drives `run_cli_from` end to end for the `daily` subcommand.
///
/// The other CLI tests stop short of `run_cli_from`, so command
/// dispatch, HTML generation and the sitemap write were never executed
/// by the suite — `daily` shipped with its whole path uncovered.
#[test]
fn test_run_cli_daily_generates_a_page() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let quotes_file = create_test_quote_json(temp_dir.path());
    create_layout_template(temp_dir.path());
    fs::create_dir_all("./docs").unwrap();

    let result = wiserone::cli::run_cli_from(vec![
        "wiserone".to_string(),
        "daily".to_string(),
        quotes_file,
    ]);

    // The generator writes the dated page and mirrors it to index.html.
    let dated = fs::read_dir("./docs")
        .unwrap()
        .filter_map(Result::ok)
        .any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".html") && name != "index.html"
        });
    let index_exists = Path::new("./docs/index.html").exists();

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "daily failed: {:?}", result.err());
    assert!(dated, "daily wrote no dated page");
    assert!(
        index_exists,
        "daily did not mirror the page to index.html"
    );
}

/// `random` must also complete through the real dispatch path.
#[test]
fn test_run_cli_random_generates_a_page() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let quotes_file = create_test_quote_json(temp_dir.path());
    create_layout_template(temp_dir.path());
    fs::create_dir_all("./docs").unwrap();

    let result = wiserone::cli::run_cli_from(vec![
        "wiserone".to_string(),
        "random".to_string(),
        quotes_file,
    ]);
    let wrote_something = fs::read_dir("./docs")
        .unwrap()
        .filter_map(Result::ok)
        .any(|e| {
            e.path().extension().and_then(|s| s.to_str())
                == Some("html")
        });

    std::env::set_current_dir(&original_dir).unwrap();
    assert!(result.is_ok(), "random failed: {:?}", result.err());
    assert!(wrote_something, "random wrote no page");
}

/// `all` must give every quote its own file.
///
/// It used to name pages from `date_added`, which is no longer unique:
/// quotes written on the same day silently overwrote each other. Three
/// same-day quotes produced one file.
#[test]
fn test_run_cli_all_does_not_overwrite_same_day_quotes() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp_dir = TempDir::new().unwrap();
    let original_dir = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp_dir.path()).unwrap();

    let path = temp_dir.path().join("same-day.json");
    let mut file = File::create(&path).unwrap();
    write!(
        file,
        r#"{{"quotes":[
            {{"id":0,"quote_text":"First","author":"A","date_added":"2026-08-23T06:06:06Z","image_url":"https://e.com/a.jpg"}},
            {{"id":1,"quote_text":"Second","author":"A","date_added":"2026-08-23T06:06:06Z","image_url":"https://e.com/a.jpg"}},
            {{"id":2,"quote_text":"Third","author":"A","date_added":"2026-08-23T06:06:06Z","image_url":"https://e.com/a.jpg"}}
        ]}}"#
    )
    .unwrap();

    create_layout_template(temp_dir.path());
    fs::create_dir_all("./docs").unwrap();

    let result = wiserone::cli::run_cli_from(vec![
        "wiserone".to_string(),
        "all".to_string(),
        path.to_string_lossy().to_string(),
    ]);

    let pages = fs::read_dir("./docs")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            e.file_name().to_string_lossy().starts_with("quote-")
        })
        .count();

    std::env::set_current_dir(&original_dir).unwrap();

    assert!(result.is_ok(), "all failed: {:?}", result.err());
    assert_eq!(
        pages, 3,
        "three quotes written on one day must produce three pages"
    );
}
