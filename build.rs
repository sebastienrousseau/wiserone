// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing
// terms. Copyright © 2024 WiserOne. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! This is the main function for the build script.
//!
//! Currently, it only instructs Cargo to re-run this build script if
//! `build.rs` is changed.
fn main() {
    // Avoid unnecessary re-building.
    println!("cargo:rerun-if-changed=build.rs");
}
