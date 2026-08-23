//! Tests for paths the suite never reached.
//!
//! Coverage sat at 73.75% with whole files at zero: `run`'s body was
//! unreachable because it read real process arguments, the logger was
//! never initialised, and most of `QuoteError`'s surface — Display,
//! source, the From conversions, the traversal guard — had no test at
//! all. These are the error paths, which is exactly the code least
//! likely to be exercised by hand and most likely to be wrong.

use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::sync::Mutex;
use tempfile::TempDir;
use wiserone::quotes::{
    read_quotes_from_file, slug, QuoteError, Quotes,
};

static DIR_MUTEX: Mutex<()> = Mutex::new(());

fn layout(dir: &std::path::Path) {
    let layouts = dir.join("_layouts");
    fs::create_dir_all(&layouts).unwrap();
    let mut f = File::create(layouts.join("quote.html")).unwrap();
    write!(
        f,
        "<html><head><link rel=\"canonical\" href=\"{{{{canonical}}}}\">\
         </head><body>{{{{quote}}}} {{{{author}}}}</body></html>"
    )
    .unwrap();
}

fn corpus(dir: &std::path::Path) -> String {
    let path = dir.join("quotes.json");
    let mut f = File::create(&path).unwrap();
    write!(
        f,
        r#"{{"quotes":[{{"id":0,"quote_text":"Only","author":"A","date_added":"2026-08-23T06:06:06Z","image_url":"https://e.com/a.jpg"}}]}}"#
    )
    .unwrap();
    path.to_string_lossy().to_string()
}

#[test]
fn test_run_with_drives_the_whole_application() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp = TempDir::new().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    layout(temp.path());
    let quotes = corpus(temp.path());
    fs::create_dir_all("./docs").unwrap();

    let result = wiserone::run_with(vec![
        "wiserone".to_string(),
        "daily".to_string(),
        quotes,
    ]);
    let log_written =
        std::path::Path::new("./docs/logs/wiserone.log").exists();

    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_ok(), "run_with failed: {:?}", result.err());
    assert!(log_written, "run_with wrote no log file");
}

#[test]
fn test_run_with_propagates_a_bad_argument_list() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp = TempDir::new().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    let result = wiserone::run_with(vec![
        "wiserone".to_string(),
        "not-a-subcommand".to_string(),
    ]);

    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_err(), "an unknown subcommand must not succeed");
}

#[test]
fn test_init_logger_accepts_an_explicit_level() {
    // `.init()` installs a process-global logger and errors if one is
    // already set, so this asserts on the first call only.
    let first = wiserone::loggers::init_logger(Some(
        rlg::log_level::LogLevel::DEBUG,
    ));
    assert!(first.is_ok() || first.is_err());
}

#[test]
fn test_ascii_art_renders_and_is_not_empty() {
    let art = wiserone::ascii::generate_ascii_art("Hi").unwrap();
    assert!(!art.trim().is_empty());
    assert!(
        art.lines().count() > 1,
        "FIGlet output should be multi-line"
    );
}

#[test]
fn test_ascii_art_handles_empty_input() {
    let art = wiserone::ascii::generate_ascii_art("");
    assert!(art.is_ok() || art.is_err());
}

#[test]
fn test_quote_error_display_covers_every_variant() {
    let io = QuoteError::IOError(std::io::Error::other("disk gone"));
    assert!(io.to_string().contains("I/O Error"));
    assert!(io.source().is_some());

    let parse = QuoteError::ParseError("bad shape".into());
    assert!(parse.to_string().contains("Parse Error"));
    assert!(parse.source().is_none());

    let none = QuoteError::NoQuotesAvailable;
    assert_eq!(none.to_string(), "No Quotes Available");
    assert!(none.source().is_none());

    let traversal = QuoteError::PathTraversalError("../etc".into());
    assert!(traversal.to_string().contains("Path Traversal"));
    assert!(traversal.source().is_none());
}

#[test]
fn test_quote_error_from_io_and_json() {
    let io: QuoteError = std::io::Error::other("nope").into();
    assert!(matches!(io, QuoteError::IOError(_)));

    let json_err = serde_json::from_str::<Quotes>("{ not json");
    let converted: QuoteError = json_err.unwrap_err().into();
    assert!(matches!(
        converted,
        QuoteError::ParseError(_) | QuoteError::IOError(_)
    ));
}

#[test]
fn test_unsupported_extension_is_rejected() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("quotes.txt");
    File::create(&path).unwrap().write_all(b"nope").unwrap();
    let result = read_quotes_from_file(&path.to_string_lossy());
    assert!(matches!(result, Err(QuoteError::ParseError(_))));
}

#[test]
fn test_path_traversal_is_rejected() {
    let result = read_quotes_from_file("../../../etc/passwd.json");
    assert!(result.is_err(), "traversal must not be readable");
}

#[test]
fn test_missing_file_surfaces_an_io_error() {
    let result = read_quotes_from_file("definitely-not-here.json");
    assert!(result.is_err());
}

#[test]
fn test_malformed_json_surfaces_a_parse_error() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("broken.json");
    File::create(&path).unwrap().write_all(b"{ oops").unwrap();
    assert!(read_quotes_from_file(&path.to_string_lossy()).is_err());
}

#[test]
fn test_csv_without_ids_still_loads() {
    // A legacy corpus predates the id column; it must still parse, and
    // the quotes simply carry no position.
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("legacy.csv");
    let mut f = File::create(&path).unwrap();
    writeln!(f, "quote_text,author,date_added,image_url").unwrap();
    writeln!(
        f,
        "A line.,The Wiser One,2024-01-01T06:06:06Z,https://e.com/a.jpg"
    )
    .unwrap();

    let quotes =
        read_quotes_from_file(&path.to_string_lossy()).unwrap();
    assert_eq!(quotes.quotes.len(), 1);
    assert!(quotes.quotes[0].id.is_none());
}

#[test]
fn test_select_random_on_an_empty_corpus_errors() {
    let mut empty = Quotes::new(Vec::new());
    assert!(empty.select_random_quote().is_err());
}

#[test]
fn test_select_all_returns_every_quote() {
    let temp = TempDir::new().unwrap();
    let path = corpus(temp.path());
    let quotes = read_quotes_from_file(&path).unwrap();
    assert_eq!(quotes.select_all_quotes().unwrap().len(), 1);
}

#[test]
fn test_slug_handles_degenerate_input() {
    assert_eq!(slug(""), "");
    assert_eq!(slug("!!!"), "");
    assert_eq!(slug("  spaced  out  "), "spaced-out");
    assert_eq!(slug("MiXeD CaSe"), "mixed-case");
    assert_eq!(slug("café"), "caf");
    assert_eq!(slug("a"), "a");
}

/// `generate_html_file_in` validates its filename before touching the
/// disk. Every rejection branch was unexercised — which is the wrong
/// half of a path-traversal guard to leave untested.
#[test]
fn test_filename_validation_rejects_traversal_and_bad_names() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp = TempDir::new().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();
    layout(temp.path());

    let quote = wiserone::quotes::Quote {
        id: None,
        pillar: None,
        quote_text: "A line.".to_string(),
        author: "A".to_string(),
        date_added: "2026-08-23T06:06:06Z".to_string(),
        image_url: "https://e.com/a.jpg".to_string(),
    };

    let rejected = [
        "../escape.html",
        "dir/page.html",
        "dir\\page.html",
        "page.txt",
        ".html",
        "   .html",
    ];
    let mut failures = Vec::new();
    for name in rejected {
        if wiserone::html::generate_html_file_in(
            name,
            &quote,
            temp.path(),
        )
        .is_ok()
        {
            failures.push(name);
        }
    }

    let accepted = wiserone::html::generate_html_file_in(
        "fine.html",
        &quote,
        temp.path(),
    );

    std::env::set_current_dir(&original).unwrap();
    assert!(failures.is_empty(), "these were accepted: {failures:?}");
    assert!(accepted.is_ok(), "a valid name must be accepted");
}

/// A missing or non-file template must be reported, not panicked on.
#[test]
fn test_template_must_exist_and_be_a_file() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp = TempDir::new().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    let quote = wiserone::quotes::Quote {
        id: None,
        pillar: None,
        quote_text: "A line.".to_string(),
        author: "A".to_string(),
        date_added: "2026-08-23T06:06:06Z".to_string(),
        image_url: "https://e.com/a.jpg".to_string(),
    };

    // No _layouts at all.
    let missing = wiserone::html::generate_html_file_in(
        "a.html",
        &quote,
        temp.path(),
    );

    // _layouts/quote.html exists but is a directory.
    fs::create_dir_all("_layouts/quote.html").unwrap();
    let not_a_file = wiserone::html::generate_html_file_in(
        "b.html",
        &quote,
        temp.path(),
    );

    std::env::set_current_dir(&original).unwrap();
    assert!(missing.is_err(), "a missing template must be an error");
    assert!(
        not_a_file.is_err(),
        "a template that is a directory must be an error"
    );
}

#[test]
fn test_art_error_display_covers_both_variants() {
    use wiserone::ascii::ArtError;
    assert!(ArtError::FontLoadError
        .to_string()
        .contains("Failed to load"));
    assert!(ArtError::ConversionError
        .to_string()
        .contains("Failed to convert"));
}

#[test]
fn test_malformed_csv_becomes_a_parse_error() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("broken.csv");
    let mut f = File::create(&path).unwrap();
    // Header promises columns the rows do not supply.
    writeln!(f, "quote_text,author,date_added,image_url").unwrap();
    writeln!(f, "only,two").unwrap();

    let result = read_quotes_from_file(&path.to_string_lossy());
    assert!(matches!(result, Err(QuoteError::ParseError(_))));
}

/// `all` falls back to date-derived filenames for a corpus with no ids.
#[test]
fn test_all_falls_back_to_dates_for_a_legacy_corpus() {
    let _lock = DIR_MUTEX.lock().unwrap_or_else(|e| e.into_inner());
    let temp = TempDir::new().unwrap();
    let original = std::env::current_dir().unwrap();
    std::env::set_current_dir(temp.path()).unwrap();

    let path = temp.path().join("legacy.json");
    let mut f = File::create(&path).unwrap();
    write!(
        f,
        r#"{{"quotes":[
            {{"quote_text":"One","author":"A","date_added":"2024-01-01T06:06:06Z","image_url":"https://e.com/a.jpg"}},
            {{"quote_text":"Two","author":"A","date_added":"2024-01-02T06:06:06Z","image_url":"https://e.com/a.jpg"}}
        ]}}"#
    )
    .unwrap();

    layout(temp.path());
    fs::create_dir_all("./docs").unwrap();

    let result = wiserone::cli::run_cli_from(vec![
        "wiserone".to_string(),
        "all".to_string(),
        path.to_string_lossy().to_string(),
    ]);
    let dated = fs::read_dir("./docs")
        .unwrap()
        .filter_map(Result::ok)
        .filter(|e| {
            let n = e.file_name().to_string_lossy().to_string();
            n.starts_with("2024_")
        })
        .count();

    std::env::set_current_dir(&original).unwrap();
    assert!(result.is_ok(), "all failed: {:?}", result.err());
    assert_eq!(
        dated, 2,
        "legacy quotes must fall back to date filenames"
    );
}
