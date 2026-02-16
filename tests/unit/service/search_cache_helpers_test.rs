//! Tests for search cache helper functions.

use std::path::PathBuf;

use rustean::service::search::cache_helpers::{
	canonicalize, io_err, read_disk_timestamp,
};

/// canonicalize on nonexistent path returns original
#[test]
fn canonicalize_nonexistent_returns_original() {
	// Arrange
	let fake = PathBuf::from("/nonexistent/path");

	// Act
	let result = canonicalize(&fake);

	// Assert
	assert_eq!(result, fake);
}

/// canonicalize on existing path returns canonical
#[test]
fn canonicalize_existing_returns_canonical() {
	// Arrange
	let tmp = std::env::temp_dir();

	// Act
	let result = canonicalize(&tmp);

	// Assert — should be a valid absolute path
	assert!(result.is_absolute());
}

/// io_err creates a SearchError with the message
#[test]
fn io_err_creates_search_error() {
	// Arrange
	let msg = "test failure reason";

	// Act
	let err = io_err(msg);

	// Assert
	let text = format!("{err}");
	assert!(
		text.contains("test failure reason"),
		"error should contain message: {text}",
	);
}

/// read_disk_timestamp with no state returns zero
#[test]
fn read_disk_timestamp_no_state_returns_zero() {
	// Arrange — empty temp dir, no index state
	let dir = std::env::temp_dir()
		.join("rustean-test-no-state");
	std::fs::create_dir_all(&dir).ok();

	// Act
	let ts = read_disk_timestamp(&dir);

	// Assert
	assert_eq!(ts, 0);

	// Cleanup
	std::fs::remove_dir_all(&dir).ok();
}
