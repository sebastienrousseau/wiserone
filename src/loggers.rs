// Copyright notice and licensing information.
// These lines indicate the copyright of the software and its licensing
// terms. Copyright © 2024 WiserOne. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Application logging functionality
use std::io::Write;

use env_logger::Env;
use rlg::LogLevel;

/// Initializes the logging system.
///
/// This function sets up the logging system using the `env_logger`
/// crate. It takes a `default_log_level` parameter, which determines
/// the minimum log level to be displayed. The function returns a
/// `Result` type, which will be `Ok` if the logging system is
/// initialized successfully, or an error if there was a problem.
///
/// # Examples
///
/// ```
/// use rlg::LogLevel;
/// use wiserone::loggers::init_logger;
///
/// // Initialize the logging system with a default log level of `info`
/// init_logger(Some(LogLevel::INFO)).unwrap();
/// ```
pub fn init_logger(
    default_log_level: Option<LogLevel>,
) -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::default().default_filter_or(
        default_log_level.unwrap_or(LogLevel::INFO).to_string(),
    );

    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(buf, "[{}] - {}", record.level(), record.args())
        })
        .init();

    Ok(())
}
