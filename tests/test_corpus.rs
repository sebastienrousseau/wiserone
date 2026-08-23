//! Guards the corpus this crate ships.
//!
//! Every other test in this suite builds its own fixture, so nothing
//! read `quotes/quotes.json` at all: a truncated file, a shuffled pool
//! or a duplicated line would have shipped green. These assertions are
//! the crate-side equivalent of the gates wiserone.com runs over the
//! same data.

use std::collections::HashSet;
use wiserone::quotes::{read_quotes_from_file, slug, Quote, Quotes};

fn corpus() -> Quotes {
    read_quotes_from_file("./quotes/quotes.json")
        .expect("quotes/quotes.json must parse")
}

fn csv_corpus() -> Quotes {
    read_quotes_from_file("./quotes/quotes.csv")
        .expect("quotes/quotes.csv must parse")
}

#[test]
fn test_corpus_is_not_empty_and_is_calibrated_size() {
    let quotes = corpus();
    assert!(
        quotes.quotes.len() >= 100,
        "corpus shrank to {} quotes; the rotation becomes visible to a \
         returning reader below ~100",
        quotes.quotes.len()
    );
}

#[test]
fn test_every_quote_has_the_required_fields() {
    for (position, quote) in corpus().quotes.iter().enumerate() {
        assert!(
            !quote.quote_text.trim().is_empty(),
            "quote at position {position} has empty text"
        );
        assert!(
            !quote.author.trim().is_empty(),
            "quote at position {position} has no author"
        );
        assert!(
            quote.image_url.starts_with("https://"),
            "quote at position {position} has a non-HTTPS image URL"
        );
    }
}

#[test]
fn test_ids_are_present_and_contiguous_from_zero() {
    let quotes = corpus();
    let ids: Vec<usize> = quotes
        .quotes
        .iter()
        .map(|q| q.id.expect("every shipped quote must carry an id"))
        .collect();

    let mut sorted = ids.clone();
    sorted.sort_unstable();
    let expected: Vec<usize> = (0..quotes.quotes.len()).collect();
    assert_eq!(
        sorted, expected,
        "ids must be contiguous from 0 — the daily rotation indexes by \
         them, and a gap silently shifts every day's quote"
    );
}

#[test]
fn test_no_duplicate_quotes() {
    let quotes = corpus();
    let mut seen = HashSet::new();
    for quote in &quotes.quotes {
        assert!(
            seen.insert(quote.quote_text.clone()),
            "duplicate quote: {}",
            quote.quote_text
        );
    }
}

#[test]
fn test_no_two_quotes_share_a_slug() {
    // Slugs are the site's canonical URLs; a collision would point two
    // quotes at one page.
    let quotes = corpus();
    let mut seen = HashSet::new();
    for quote in &quotes.quotes {
        let s = slug(&quote.quote_text);
        assert!(!s.is_empty(), "empty slug for: {}", quote.quote_text);
        assert!(seen.insert(s.clone()), "slug collision on {s}");
    }
}

#[test]
fn test_json_and_csv_carry_the_same_corpus() {
    // The two formats are advertised as interchangeable in the README.
    let json: Vec<String> =
        corpus().quotes.iter().map(|q| q.quote_text.clone()).collect();
    let csv: Vec<String> = csv_corpus()
        .quotes
        .iter()
        .map(|q| q.quote_text.clone())
        .collect();
    assert_eq!(json, csv, "quotes.json and quotes.csv have diverged");
}

#[test]
fn test_csv_preserves_ids_so_daily_selection_agrees() {
    let json_ids: Vec<Option<usize>> =
        corpus().quotes.iter().map(|q| q.id).collect();
    let csv_ids: Vec<Option<usize>> =
        csv_corpus().quotes.iter().map(|q| q.id).collect();
    assert_eq!(
        json_ids, csv_ids,
        "the CSV must carry ids, or `daily` picks a different quote \
         from it than from the JSON"
    );
}

#[test]
fn test_daily_selection_is_deterministic_and_wraps() {
    let quotes = corpus();
    let len = quotes.quotes.len() as i64;

    let first = quotes.select_daily_quote(739_851).unwrap();
    let again = quotes.select_daily_quote(739_851).unwrap();
    assert_eq!(first.quote_text, again.quote_text);

    let after_a_pass = quotes.select_daily_quote(739_851 + len).unwrap();
    assert_eq!(first.quote_text, after_a_pass.quote_text);

    let next_day = quotes.select_daily_quote(739_852).unwrap();
    assert_ne!(first.quote_text, next_day.quote_text);
}

#[test]
fn test_daily_selection_matches_the_website() {
    // 2026-08-23 as date.toordinal(); the site served this line that day.
    let quotes = corpus();
    let quote = quotes.select_daily_quote(739_851).unwrap();
    assert_eq!(
        quote.quote_text,
        "If nobody's upset about what you cut, you didn't cut enough."
    );
}

#[test]
fn test_daily_selection_handles_a_negative_ordinal() {
    let quotes = corpus();
    assert!(quotes.select_daily_quote(-1).is_ok());
    assert!(quotes.select_daily_quote(i64::MIN + 1).is_ok());
}

#[test]
fn test_daily_selection_on_an_empty_corpus_errors() {
    let quotes = Quotes::new(Vec::<Quote>::new());
    assert!(quotes.select_daily_quote(739_851).is_err());
}

#[test]
fn test_slug_matches_published_urls() {
    // Taken from live wiserone.com/q/ pages. If the site's slug rule
    // changes, this fails rather than the canonical quietly 404ing.
    let cases = [
        (
            "If everyone likes it immediately, you built what they already had.",
            "if-everyone-likes-it-immediately-you-built-what-they-already-had",
        ),
        (
            "Say no to a hundred good things. That's the only way the great thing gets your whole attention.",
            "say-no-to-a-hundred-good-things-that-s-the-only-way-the-great",
        ),
        (
            "If nobody's upset about what you cut, you didn't cut enough.",
            "if-nobody-s-upset-about-what-you-cut-you-didn-t-cut-enough",
        ),
        (
            "Taste is knowing which good idea to throw away.",
            "taste-is-knowing-which-good-idea-to-throw-away",
        ),
        (
            "What you've already spent isn't a reason. It's just why it hurts.",
            "what-you-ve-already-spent-isn-t-a-reason-it-s-just-why-it-hurts",
        ),
    ];
    for (text, expected) in cases {
        assert_eq!(slug(text), expected, "slug drifted for: {text}");
    }
}

#[test]
fn test_slug_never_exceeds_the_length_budget() {
    for quote in &corpus().quotes {
        assert!(
            slug(&quote.quote_text).len() <= 64,
            "slug too long for: {}",
            quote.quote_text
        );
    }
}

#[test]
fn test_slug_has_no_leading_or_trailing_hyphen() {
    for quote in &corpus().quotes {
        let s = slug(&quote.quote_text);
        assert!(!s.starts_with('-') && !s.ends_with('-'), "bad slug: {s}");
    }
}
