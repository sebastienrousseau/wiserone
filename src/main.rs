// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! The main function of the program.
//!
//! Calls the `run()` function from the `ssg` module to run the static
//! site generator.
//!
//! If an error occurs while running the `run()` function, the function
//! prints an error message to standard error and exits the program with
//! a non-zero status code.
fn main() {
    if let Err(err) = wiserone::run() {
        eprintln!("{}", err);
        eprintln!(
            "\nPlease run `wiserone --help` for more information.\n"
        );
        std::process::exit(1);
    }
}
