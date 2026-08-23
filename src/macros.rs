// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! # Macros for the `wiserone` crate.

/// This macro creates a Quote struct from the provided fields.
///
/// # Example
///
/// ```
/// use wiserone::wiserone;
///
/// let quote = wiserone! {
///     quote_text: "The only way to do great work is to love what you do.",
///     author: "Steve Jobs",
///     date_added: "2024-01-01T00:00:00Z",
///     image_url: "https://example.com/image.jpg"
/// };
/// ```
#[macro_export]
macro_rules! wiserone {
    (
        quote_text: $quote_text:expr,
        author: $author:expr,
        date_added: $date_added:expr,
        image_url: $image_url:expr $(,)?
    ) => {
        $crate::quotes::Quote {
            // Pool metadata is not part of the macro's surface: callers
            // construct one-off quotes, not corpus entries. Defaulted so
            // existing `wiserone! { ... }` sites keep compiling.
            id: None,
            pillar: None,
            quote_text: $quote_text.to_string(),
            author: $author.to_string(),
            date_added: $date_added.to_string(),
            image_url: $image_url.to_string(),
        }
    };
}

/// This macro prints the arguments to the console.
#[macro_export]
macro_rules! wiserone_print {
    ($($arg:tt)*) => {
        println!("{}", format_args!("{}", $($arg)*));
    };
}

/// This macro creates a new vector of the given elements.
#[macro_export]
macro_rules! wiserone_vec {
    ($($elem:expr),*) => {{
        let mut v = Vec::new();
        $(v.push($elem);)*
        v
    }};
}

/// This macro creates a new map of the given key-value pairs.
#[macro_export]
macro_rules! wiserone_map {
    ($($key:expr => $value:expr),*) => {{
        use std::collections::HashMap;
        let mut m = HashMap::new();
        $(m.insert($key, $value);)*
        m
    }};
}

/// This macro checks if the given expression is true.
///
/// # Example
///
/// ```
/// use wiserone::wiserone_assert;
///
/// wiserone_assert!(true);
/// wiserone_assert!(1 + 1 == 2);
/// wiserone_assert!(vec![1, 2, 3].len() == 3);
/// ```
#[macro_export]
macro_rules! wiserone_assert {
    ($cond:expr) => {
        if !$cond {
            panic!("Assertion failed: {}", stringify!($cond));
        }
    };
    ($cond:expr, $($msg:tt)+) => {
        if !$cond {
            panic!("Assertion failed: {}: {}", stringify!($cond), format!($($msg)+));
        }
    };
}

/// This macro returns the minimum of the given values.
///
/// # Example
///
/// ```
/// use wiserone::wiserone_min;
///
/// assert_eq!(wiserone_min!(5), 5);
/// assert_eq!(wiserone_min!(5, 3, 8), 3);
/// assert_eq!(wiserone_min!(1, 2, 3, 4, 5), 1);
/// ```
#[macro_export]
macro_rules! wiserone_min {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {{
        let first = $x;
        let rest = wiserone_min!($($rest),+);
        if first < rest { first } else { rest }
    }};
}

/// This macro returns the maximum of the given values.
///
/// # Example
///
/// ```
/// use wiserone::wiserone_max;
///
/// assert_eq!(wiserone_max!(5), 5);
/// assert_eq!(wiserone_max!(5, 3, 8), 8);
/// assert_eq!(wiserone_max!(1, 2, 3, 4, 5), 5);
/// ```
#[macro_export]
macro_rules! wiserone_max {
    ($x:expr) => {
        $x
    };
    ($x:expr, $($rest:expr),+) => {{
        let first = $x;
        let rest = wiserone_max!($($rest),+);
        if first > rest { first } else { rest }
    }};
}

/// This macro takes a string and splits it into a vector of words.
#[macro_export]
macro_rules! wiserone_split {
    ($s:expr) => {{
        let mut v = Vec::new();
        for w in $s.split_whitespace() {
            v.push(w.to_string());
        }
        v
    }};
}

/// This macro takes a vector of strings and joins them together into a
/// single string.
#[macro_export]
macro_rules! wiserone_join {
    ($($s:expr),*) => {{
        let mut s = String::new();
        $(
            s += &$s;
        )*
        s
    }};
}

/// This macro takes a vector of elements and prints them to the
/// console.
#[macro_export]
macro_rules! wiserone_print_vec {
    ($($v:expr),*) => {{
        for v in $($v),* {
            println!("{}", v);
        }
    }};
}
