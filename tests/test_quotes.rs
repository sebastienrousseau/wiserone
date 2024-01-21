// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::collections::HashSet;
use wiserone::quotes::{Quotes, Quote};

/// Test the creation and field access of the Quote struct.
#[test]
fn test_quote_struct() {
    let quote = Quote {
        quote_text: "Test quote".to_string(),
        author: "Test author".to_string(),
        date_added: "2024-01-21".to_string(),
        image_url: "http://example.com/image.jpg".to_string(),
    };

    assert_eq!(quote.quote_text, "Test quote");
    assert_eq!(quote.author, "Test author");
    assert_eq!(quote.date_added, "2024-01-21");
    assert_eq!(quote.image_url, "http://example.com/image.jpg");
}

/// Test the initialization of the Quotes struct.
#[test]
fn test_quotes_struct_initialization() {
    let quotes_vec = vec![
        Quote {
            quote_text: "Quote 1".to_string(),
            author: "Author 1".to_string(),
            date_added: "2024-01-21".to_string(),
            image_url: "http://example.com/image1.jpg".to_string(),
        },
        Quote {
            quote_text: "Quote 2".to_string(),
            author: "Author 2".to_string(),
            date_added: "2024-01-22".to_string(),
            image_url: "http://example.com/image2.jpg".to_string(),
        },
    ];

    let quotes = Quotes::new(quotes_vec.clone());
    assert_eq!(quotes.quotes, quotes_vec);
}

/// Test the selection of a random quote from the Quotes struct.
#[test]
fn test_random_quote_selection() {
    let quotes_vec = vec![
        Quote {
            quote_text: "Test quote 1".to_string(),
            author: "Author 1".to_string(),
            date_added: "2024-01-01".to_string(),
            image_url: "http://example.com/image1.jpg".to_string(),
        },
        // ... more quotes ...
    ];

    let mut quotes = Quotes::new(quotes_vec);

    let selected_quote_text = match quotes.select_random_quote() {
        Ok(quote) => quote.quote_text.clone(),
        Err(e) => panic!("Failed to select a random quote: {}", e),
    };

    let quotes_texts: HashSet<_> = quotes.quotes.iter().map(|q| &q.quote_text).collect();

    assert!(quotes_texts.contains(&selected_quote_text));
}

/// Test the selection of all quotes from the Quotes struct.
#[test]
fn test_all_quotes_selection() {
    let quotes_vec = vec![
        Quote {
            quote_text: "Test quote 1".to_string(),
            author: "Author 1".to_string(),
            date_added: "2024-01-01".to_string(),
            image_url: "http://example.com/image1.jpg".to_string(),
        },
        //... more quotes...
        Quote {
            quote_text: "Test quote 2".to_string(),
            author: "Author 2".to_string(),
            date_added: "2024-01-02".to_string(),
            image_url: "http://example.com/image2.jpg".to_string(),
        },
    ];
    let quotes = Quotes::new(quotes_vec.clone());
    let all_quotes = match quotes.select_all_quotes() {
        Ok(quotes) => quotes,
        Err(e) => panic!("Failed to select all quotes: {}", e),
    };
    assert_eq!(all_quotes.len(), quotes_vec.len());
    assert_eq!(all_quotes[0].quote_text, quotes_vec[0].quote_text);
    assert_eq!(all_quotes[1].quote_text, quotes_vec[1].quote_text);
    assert_eq!(all_quotes[0].author, quotes_vec[0].author);
    assert_eq!(all_quotes[1].author, quotes_vec[1].author);
    assert_eq!(all_quotes[0].date_added, quotes_vec[0].date_added);
    assert_eq!(all_quotes[1].date_added, quotes_vec[1].date_added);
    assert_eq!(all_quotes[0].image_url, quotes_vec[0].image_url);
    assert_eq!(all_quotes[1].image_url, quotes_vec[1].image_url);
}

/// Test the equality and inequality of Quote structs.
#[test]
fn test_quote_equality() {
    let quote1 = Quote {
        quote_text: "Same quote".to_string(),
        author: "Same author".to_string(),
        date_added: "2024-01-21".to_string(),
        image_url: "http://example.com/image.jpg".to_string(),
    };

    let quote2 = quote1.clone();

    assert_eq!(quote1, quote2);

    let quote3 = Quote {
        quote_text: "Different quote".to_string(),
        // other fields same as quote1
        author: "Same author".to_string(),
        date_added: "2024-01-21".to_string(),
        image_url: "http://example.com/image.jpg".to_string(),
    };

    assert_ne!(quote1, quote3);
}

/// Test selecting a random quote when the vector is empty.
#[test]
fn test_select_random_quote_empty_vector() {
    let mut quotes = Quotes::new(Vec::new());
    assert!(matches!(quotes.select_random_quote(), Err(_)));
}

/// Test selecting all quotes when the vector is empty.
#[test]
fn test_select_all_quotes_empty_vector() {
    let quotes = Quotes::new(Vec::new()); // empty vector

    match quotes.select_all_quotes() {
        Ok(_) => panic!("Expected an error for empty quotes vector, but got Ok"),
        Err(e) => assert_eq!(e.to_string(), "No available quotes"),
    }
}

/// Test the serialization and deserialization of Quote structs.
#[test]
fn test_quote_serialization_deserialization() {
    let quote = Quote {
        quote_text: "Test quote".to_string(),
        author: "Test author".to_string(),
        date_added: "2024-01-21".to_string(),
        image_url: "http://example.com/image.jpg".to_string(),
    };

    let serialized = serde_json::to_string(&quote).unwrap();
    let deserialized: Quote = serde_json::from_str(&serialized).unwrap();

    assert_eq!(quote, deserialized);
}

/// Test the creation of an invalid quote with empty fields.
#[test]
fn test_invalid_quote_creation() {
    let quote = Quote {
        quote_text: "".to_string(),
        author: "".to_string(),
        date_added: "2024-01-21".to_string(),
        image_url: "http://example.com/image.jpg".to_string(),
    };

    assert_eq!(quote.quote_text, "");
    assert_eq!(quote.author, "");
}
