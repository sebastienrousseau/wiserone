// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Tests for the macros module.

use std::collections::HashMap;

// Import all macros from wiserone
use wiserone::{
    wiserone, wiserone_assert, wiserone_join, wiserone_map,
    wiserone_max, wiserone_min, wiserone_print, wiserone_print_vec,
    wiserone_split, wiserone_vec,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiserone_print() {
        // Test basic print functionality - this will print to stdout
        wiserone_print!("Hello");
        wiserone_print!(42);
    }

    #[test]
    fn test_wiserone_vec_empty() {
        let v: Vec<i32> = wiserone_vec![];
        assert_eq!(v, Vec::<i32>::new());
        assert!(v.is_empty());
    }

    #[test]
    fn test_wiserone_vec_single_element() {
        let v = wiserone_vec![42];
        assert_eq!(v, vec![42]);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], 42);
    }

    #[test]
    fn test_wiserone_vec_multiple_elements() {
        let v = wiserone_vec![1, 2, 3, 4, 5];
        assert_eq!(v, vec![1, 2, 3, 4, 5]);
        assert_eq!(v.len(), 5);
    }

    #[test]
    fn test_wiserone_vec_different_types() {
        let strings =
            wiserone_vec!["hello".to_string(), "world".to_string()];
        assert_eq!(
            strings,
            vec!["hello".to_string(), "world".to_string()]
        );

        let floats = wiserone_vec![1.1, 2.2, 3.3];
        assert_eq!(floats, vec![1.1, 2.2, 3.3]);
    }

    #[test]
    fn test_wiserone_map_empty() {
        let m: HashMap<i32, i32> = wiserone_map![];
        assert_eq!(m, HashMap::new());
        assert!(m.is_empty());
    }

    #[test]
    fn test_wiserone_map_single_pair() {
        let m = wiserone_map!["key" => "value"];
        let mut expected = HashMap::new();
        let _ = expected.insert("key", "value");
        assert_eq!(m, expected);
        assert_eq!(m.len(), 1);
        assert_eq!(m["key"], "value");
    }

    #[test]
    fn test_wiserone_map_multiple_pairs() {
        let m = wiserone_map![
            "name" => "John",
            "age" => "30",
            "city" => "New York"
        ];
        assert_eq!(m.len(), 3);
        assert_eq!(m["name"], "John");
        assert_eq!(m["age"], "30");
        assert_eq!(m["city"], "New York");
    }

    #[test]
    fn test_wiserone_map_different_types() {
        let m = wiserone_map![
            1 => "one",
            2 => "two",
            3 => "three"
        ];
        assert_eq!(m.len(), 3);
        assert_eq!(m[&1], "one");
        assert_eq!(m[&2], "two");
        assert_eq!(m[&3], "three");
    }

    #[test]
    fn test_wiserone_assert_simple_true() {
        wiserone_assert!(true);
    }

    #[test]
    #[should_panic(expected = "Assertion failed")]
    fn test_wiserone_assert_simple_false() {
        wiserone_assert!(false);
    }

    #[test]
    fn test_wiserone_assert_complex_expressions() {
        // Test complex expressions now that the macro is fixed
        wiserone_assert!(1 + 1 == 2);
        wiserone_assert!(vec![1, 2, 3].len() == 3);
        wiserone_assert!("hello".starts_with("he"));
        let x = 10;
        wiserone_assert!(x > 5 && x < 20);
    }

    #[test]
    fn test_wiserone_min_single_value() {
        let result = wiserone_min!(42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_wiserone_max_single_value() {
        let result = wiserone_max!(42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_wiserone_min_multiple_values() {
        // Now works with multiple values
        assert_eq!(wiserone_min!(5, 3, 8), 3);
        assert_eq!(wiserone_min!(1, 2, 3, 4, 5), 1);
        assert_eq!(wiserone_min!(10, 5), 5);
        assert_eq!(wiserone_min!(3, 3, 3), 3);
        assert_eq!(wiserone_min!(-5, 0, 5), -5);
    }

    #[test]
    fn test_wiserone_max_multiple_values() {
        // Now works with multiple values
        assert_eq!(wiserone_max!(5, 3, 8), 8);
        assert_eq!(wiserone_max!(1, 2, 3, 4, 5), 5);
        assert_eq!(wiserone_max!(10, 5), 10);
        assert_eq!(wiserone_max!(3, 3, 3), 3);
        assert_eq!(wiserone_max!(-5, 0, 5), 5);
    }

    #[test]
    fn test_wiserone_split_empty_string() {
        let result = wiserone_split!("");
        assert_eq!(result, Vec::<String>::new());
        assert!(result.is_empty());
    }

    #[test]
    fn test_wiserone_split_single_word() {
        let result = wiserone_split!("hello");
        assert_eq!(result, vec!["hello".to_string()]);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_wiserone_split_multiple_words() {
        let result = wiserone_split!("hello world rust");
        assert_eq!(
            result,
            vec![
                "hello".to_string(),
                "world".to_string(),
                "rust".to_string()
            ]
        );
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_wiserone_split_extra_whitespace() {
        let result = wiserone_split!("  hello   world  ");
        assert_eq!(
            result,
            vec!["hello".to_string(), "world".to_string()]
        );
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_wiserone_split_tabs_and_newlines() {
        let result = wiserone_split!("hello\tworld\nrust");
        assert_eq!(
            result,
            vec![
                "hello".to_string(),
                "world".to_string(),
                "rust".to_string()
            ]
        );
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_wiserone_split_unicode() {
        let result = wiserone_split!("hello 世界 مرحبا");
        assert_eq!(
            result,
            vec![
                "hello".to_string(),
                "世界".to_string(),
                "مرحبا".to_string()
            ]
        );
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_wiserone_join_empty() {
        let result = wiserone_join!();
        assert_eq!(result, String::new());
        assert!(result.is_empty());
    }

    #[test]
    fn test_wiserone_join_single_string() {
        let result = wiserone_join!("hello");
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_wiserone_join_multiple_strings() {
        let result = wiserone_join!("hello", " ", "world", "!");
        assert_eq!(result, "hello world!");
    }

    #[test]
    fn test_wiserone_join_mixed_types() {
        let number = 42;
        let result =
            wiserone_join!("The answer is ", number.to_string());
        assert_eq!(result, "The answer is 42");
    }

    #[test]
    fn test_wiserone_join_unicode() {
        let result = wiserone_join!("Hello ", "世界", " مرحبا");
        assert_eq!(result, "Hello 世界 مرحبا");
    }

    #[test]
    fn test_wiserone_print_vec_single_element() {
        let v = vec!["test"];
        // This will print to stdout - we can't easily capture it in a test
        // but we can ensure it doesn't panic
        wiserone_print_vec![&v];
    }

    #[test]
    fn test_wiserone_print_vec_multiple_elements() {
        let v1 = vec![1, 2, 3];
        // Test vector printing
        wiserone_print_vec![&v1];
    }

    #[test]
    fn test_wiserone_print_vec_empty() {
        let v: Vec<i32> = vec![];
        wiserone_print_vec![&v];
    }

    // Property-based testing patterns
    #[test]
    fn test_wiserone_vec_preserves_order() {
        let input = [1, 3, 2, 5, 4];
        let result = wiserone_vec![1, 3, 2, 5, 4];
        assert_eq!(result, input.to_vec());
    }

    #[test]
    fn test_wiserone_min_max_single_value_consistency() {
        let val = 42;
        let min_val = wiserone_min!(val);
        let max_val = wiserone_max!(val);

        assert_eq!(min_val, max_val);
        assert_eq!(min_val, 42);
        assert_eq!(max_val, 42);
    }

    #[test]
    fn test_wiserone_min_max_multiple_value_consistency() {
        let values = [5, 3, 8, 1, 9];
        let min_val = wiserone_min!(5, 3, 8, 1, 9);
        let max_val = wiserone_max!(5, 3, 8, 1, 9);

        assert_eq!(min_val, *values.iter().min().unwrap());
        assert_eq!(max_val, *values.iter().max().unwrap());
    }

    #[test]
    fn test_wiserone_quote_macro() {
        let quote = wiserone! {
            quote_text: "The only way to do great work is to love what you do.",
            author: "Steve Jobs",
            date_added: "2024-01-01T00:00:00Z",
            image_url: "https://example.com/image.jpg"
        };

        assert_eq!(
            quote.quote_text,
            "The only way to do great work is to love what you do."
        );
        assert_eq!(quote.author, "Steve Jobs");
        assert_eq!(quote.date_added, "2024-01-01T00:00:00Z");
        assert_eq!(quote.image_url, "https://example.com/image.jpg");
    }

    #[test]
    fn test_wiserone_split_join_roundtrip() {
        let original = "hello world test";
        let split_result = wiserone_split!(original);

        // Manual join to test roundtrip (since wiserone_join doesn't add spaces)
        let rejoined = split_result.join(" ");
        assert_eq!(rejoined, original);
    }

    #[test]
    fn test_wiserone_map_key_access() {
        let m = wiserone_map!["test" => "value"];
        assert!(m.contains_key("test"));
        assert!(!m.contains_key("nonexistent"));
    }

    // Edge case testing
    #[test]
    fn test_macro_edge_cases_boundary_values() {
        // Test with boundary values
        let min_val = wiserone_min!(i32::MIN);
        assert_eq!(min_val, i32::MIN);

        let max_val = wiserone_max!(i32::MAX);
        assert_eq!(max_val, i32::MAX);

        // Test with multiple boundary values
        assert_eq!(wiserone_min!(i32::MIN, i32::MAX), i32::MIN);
        assert_eq!(wiserone_max!(i32::MIN, i32::MAX), i32::MAX);
        assert_eq!(wiserone_min!(0, i32::MIN, i32::MAX), i32::MIN);
        assert_eq!(wiserone_max!(0, i32::MIN, i32::MAX), i32::MAX);
    }

    #[test]
    fn test_macro_edge_cases_zero_values() {
        let zero_min = wiserone_min!(0);
        assert_eq!(zero_min, 0);

        let zero_max = wiserone_max!(0);
        assert_eq!(zero_max, 0);
    }

    #[test]
    fn test_macro_edge_cases_empty_string_operations() {
        let empty_split = wiserone_split!("");
        assert!(empty_split.is_empty());

        let empty_join = wiserone_join!("");
        assert_eq!(empty_join, "");
    }
}

// Concurrent access testing
#[cfg(test)]
mod concurrent_tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_macro_thread_safety() {
        let handles: Vec<_> = (0..10)
            .map(|i| {
                thread::spawn(move || {
                    let v = wiserone_vec![i, i + 1, i + 2];
                    // Test with multiple values now that macros are fixed
                    let min_val = wiserone_min!(i, i + 1, i + 2);
                    let max_val = wiserone_max!(i, i + 1, i + 2);

                    assert_eq!(v, vec![i, i + 1, i + 2]);
                    assert_eq!(min_val, i);
                    assert_eq!(max_val, i + 2);
                })
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
