// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use rlg::log_level::LogLevel;

/// Initializes the logging system.
///
/// This function is provided for backwards compatibility and returns `Ok(())`.
pub fn init_logger(
    _default_log_level: Option<LogLevel>,
) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
