// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(test)]
mod tests {
    use wiserone::macro_log;
    use rlg::log_level::LogLevel;
    use rlg::log_format::LogFormat;

    #[test]
    fn test_logging() {
        let expected_description = "Log message";
        let log_entry = macro_log!(
            "session_id",
            "time",
            &LogLevel::INFO,
            "component",
            expected_description,
            &LogFormat::CLF
        );

        assert_eq!(log_entry.description, expected_description);
    }
}