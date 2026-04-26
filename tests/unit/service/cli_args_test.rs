//! Tests for service::claude::cli_args shared module

use rustean::service::claude::cli_args;

/// NotFound IO error maps to NotInstalled
#[test]
fn map_io_error_not_found_returns_not_installed() {
	// Arrange
	let err = std::io::Error::new(
		std::io::ErrorKind::NotFound,
		"not found",
	);

	// Act
	let result = cli_args::map_io_error(err);

	// Assert
	assert_eq!(
		format!("{result}"),
		"claude CLI not found on PATH"
	);
}

/// Other IO error maps to Process variant
#[test]
fn map_io_error_other_returns_process_failed() {
	// Arrange
	let err = std::io::Error::new(
		std::io::ErrorKind::PermissionDenied,
		"denied",
	);

	// Act
	let result = cli_args::map_io_error(err);

	// Assert
	let msg = format!("{result}");
	assert!(msg.contains("denied"));
}

/// Constants have expected values
#[test]
fn constants_have_expected_values() {
	// Assert
	assert_eq!(cli_args::CLAUDE_BIN, "claude");
	assert_eq!(cli_args::FLAG_PRINT, "-p");
	assert_eq!(cli_args::FMT_JSON, "json");
	assert_eq!(cli_args::FMT_STREAM, "stream-json");
	assert_eq!(cli_args::FLAG_RESUME, "--resume");
	assert_eq!(cli_args::FLAG_VERBOSE, "--verbose");
}
