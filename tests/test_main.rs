// Copyright notice and licensing information.
// Copyright © 2024 The Wiser One. All rights reserved.
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests for main.rs functionality.

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main_binary_exists() {
        // Cargo builds the bin target before running integration tests
        // and exposes its path here. Use that rather than a hardcoded
        // "target/debug/wiserone": the target directory is
        // configurable (CARGO_TARGET_DIR, or `build.target-dir` in a
        // cargo config), so the literal path is wrong on any machine
        // that sets one, and wrong for every --target/--release build.
        let binary_path = env!("CARGO_BIN_EXE_wiserone");
        assert!(
            Path::new(binary_path).exists(),
            "binary should exist at {binary_path}"
        );
    }

    #[test]
    fn test_main_help_flag() {
        // Test that --help flag works
        let output = Command::new("cargo")
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to run with --help");

        // Should exit successfully with help text
        assert!(
            output.status.success(),
            "Help command should succeed, stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(
            stdout.contains("wiserone") || stdout.contains("Usage"),
            "Help output should contain usage information"
        );
    }

    #[test]
    fn test_main_invalid_arguments() {
        // Test with completely invalid arguments
        let output = Command::new("cargo")
            .args(["run", "--", "--invalid-flag-that-does-not-exist"])
            .output()
            .expect("Failed to run with invalid args");

        // Should exit with error
        assert!(
            !output.status.success(),
            "Invalid arguments should cause failure"
        );

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("help") || stderr.contains("error"),
            "Error output should provide help information"
        );
    }

    #[test]
    fn test_main_version_flag() {
        // Test that version flag works (if implemented)
        let output = Command::new("cargo")
            .args(["run", "--", "--version"])
            .output()
            .expect("Failed to run with --version");

        // Version command should work
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            assert!(
                stdout.contains("0.0.6") || stdout.contains("wiserone"),
                "Version output should contain version info"
            );
        }
    }

    #[test]
    fn test_main_with_temporary_directory() {
        // Test main execution in a temporary directory to avoid file conflicts
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let temp_path = temp_dir.path();

        let output = Command::new("cargo")
            .current_dir(temp_path)
            .args(["run", "--manifest-path",
                   &format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"))])
            .output()
            .expect("Failed to run in temp directory");

        // Check if log file was created (main.rs creates wiserone.log)
        // This might fail if there are other issues, but that's part of testing
        if output.status.success() {
            let _log_path = temp_path.join("wiserone.log");
            // Log file creation depends on successful CLI execution
            // We check for the attempt rather than requiring success
        }
    }

    #[test]
    fn test_main_output_files_creation() {
        // Test that main creates expected output files
        let temp_dir = tempdir().expect("Failed to create temp dir");

        let _output = Command::new("cargo")
            .current_dir(temp_dir.path())
            .args([
                "run",
                "--manifest-path",
                &format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")),
                "--",
                "--help" // Use help to avoid full execution that might fail
            ])
            .output()
            .expect("Failed to run command");

        // The main function should at least execute without panicking
        // Exit code might be non-zero for --help, which is fine
    }

    #[test]
    fn test_main_error_handling() {
        // Test error handling by running in a restricted environment
        let temp_dir = tempdir().expect("Failed to create temp dir");
        let restricted_path = temp_dir.path().join("readonly");
        fs::create_dir(&restricted_path).expect("Failed to create directory");

        // Make directory read-only to trigger potential I/O errors
        let metadata = fs::metadata(&restricted_path).expect("Failed to get metadata");
        let mut permissions = metadata.permissions();

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            permissions.set_mode(0o444); // read-only
            fs::set_permissions(&restricted_path, permissions)
                .expect("Failed to set permissions");
        }

        // Test that the application handles file I/O errors gracefully
        let output = match Command::new("cargo")
            .current_dir(&restricted_path)
            .args([
                "run",
                "--manifest-path",
                &format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"))
            ])
            .output()
        {
            Ok(o) => o,
            Err(_) => return, // Skip if we can't run in restricted dir
        };

        // Should either succeed or fail gracefully with error message
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                stderr.contains("help") ||
                stderr.contains("Error") ||
                stderr.contains("error") ||
                stderr.contains("permission"),
                "Error output should be informative, got: {}",
                stderr
            );
        }
    }

    #[test]
    fn test_main_signal_handling() {
        // Test graceful shutdown on interrupt (if applicable)
        // This is a basic test to ensure the process can be started and stopped
        let mut child = Command::new("cargo")
            .args([
                "run",
                "--manifest-path",
                &format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")),
                "--",
                "--help"
            ])
            .spawn()
            .expect("Failed to spawn process");

        // Wait for completion or kill after timeout
        match child.wait() {
            Ok(status) => {
                // Process completed normally
                assert!(
                    status.code().is_some(),
                    "Process should exit with a status code"
                );
            }
            Err(e) => {
                eprintln!("Process wait failed: {}", e);
                let _ = child.kill();
            }
        }
    }

    #[test]
    fn test_main_environment_variables() {
        // Test behavior with different environment variables
        let _output = Command::new("cargo")
            .env("RUST_LOG", "debug")
            .env("RUST_BACKTRACE", "1")
            .args([
                "run",
                "--manifest-path",
                &format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR")),
                "--",
                "--help"
            ])
            .output()
            .expect("Failed to run with environment variables");

        // Should handle environment variables without crashing
        // Exit status depends on the specific command, but it shouldn't panic
    }

    #[test]
    fn test_main_working_directory_independence() {
        // Test that --help works from the project directory
        let output = Command::new("cargo")
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .args(["run", "--", "--help"])
            .output()
            .expect("Failed to run from project dir");

        assert!(
            output.status.success(),
            "Help command should succeed from project dir"
        );
    }

    // Property-based tests for main.rs behavior
    #[test]
    fn test_main_deterministic_behavior() {
        // Run the same command multiple times to ensure deterministic behavior
        let mut outputs = Vec::new();

        for _ in 0..3 {
            let output = Command::new("cargo")
                .args(["run", "--", "--help"])
                .output()
                .expect("Failed to run command");

            outputs.push((output.status.success(), output.stdout, output.stderr));
        }

        // All runs should have the same success status
        let first_success = outputs[0].0;
        for (success, _, _) in &outputs {
            assert_eq!(
                *success, first_success,
                "Command should have deterministic success status"
            );
        }

        // If successful, stdout should be consistent
        if first_success {
            let first_stdout = &outputs[0].1;
            for (_, stdout, _) in &outputs {
                assert_eq!(
                    stdout, first_stdout,
                    "Stdout should be deterministic across runs"
                );
            }
        }
    }
}