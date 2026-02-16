//! Tests for memory store helpers.

use std::path::PathBuf;

use rustean::indexer::memory::store_helpers;

use crate::helpers::factories::make_interaction;

/// Test parse_line with valid JSON
#[test]
fn parse_line_valid_json() {
	// Arrange
	let item = make_interaction("hello");
	let json =
		serde_json::to_string(&item).unwrap();

	// Act
	let parsed = store_helpers::parse_line(&json);

	// Assert
	assert!(parsed.is_some());
	assert_eq!(parsed.unwrap().id, item.id);
}

/// Test parse_line with empty string
#[test]
fn parse_line_empty_returns_none() {
	assert!(store_helpers::parse_line("").is_none());
	assert!(
		store_helpers::parse_line("  ").is_none()
	);
}

/// Test parse_line with invalid JSON
#[test]
fn parse_line_invalid_returns_none() {
	let result =
		store_helpers::parse_line("{broken");
	assert!(result.is_none());
}

/// Test read_lines on missing file
#[test]
fn read_lines_missing_returns_empty() {
	let path = PathBuf::from("/nonexistent.jsonl");
	let lines = store_helpers::read_lines(&path);
	assert!(lines.is_empty());
}

/// Test session_id_from_path extracts stem
#[test]
fn session_id_from_path_extracts_stem() {
	let path =
		PathBuf::from("/tmp/sessions/s-123.jsonl");
	let id =
		store_helpers::session_id_from_path(&path);
	assert_eq!(id, Some("s-123".to_string()));
}
