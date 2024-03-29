// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

#[cfg(test)]
mod tests {

    use rlg::macro_log;
    use rlg::{LogFormat, LogLevel};

    #[test]
    fn test_logging() {
        // Create a log entry
        let log_entry =
            macro_log!(
                "session_id",
                "time",
                &LogLevel::INFO,
                "component",
                "Log message",
                &LogFormat::CLF
            );

        // Define expected values
        let expected_session_id = "session_id";
        let expected_time = "time";
        let expected_level = LogLevel::INFO;
        let expected_component = "component";
        let expected_description = "Log message";
        let expected_format = LogFormat::CLF;

        // Assert that individual fields match the expected values
        assert_eq!(log_entry.session_id, expected_session_id);
        assert_eq!(log_entry.time, expected_time);
        assert_eq!(log_entry.level, expected_level);
        assert_eq!(log_entry.component, expected_component);
        assert_eq!(log_entry.description, expected_description);
        assert_eq!(log_entry.format, expected_format);
    }
}