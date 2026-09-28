// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! xtaskops entrypointFunction: `main` - Main function for xtaskops
fn main() -> Result<(), anyhow::Error> {
    // Run the xtaskops main function
    xtaskops::tasks::main()
}
