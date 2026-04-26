//! Tests for memory store helpers.

use std::path::PathBuf;

use rustean::indexer::memory::store_helpers;

use crate::helpers::factories::make_interaction;

/// parse_line deserializes valid JSON
#[test]
fn parse_line_valid_json() {
	let item = make_interaction("hello");
	let json =
		serde_json::to_string(&item).unwrap();

	let parsed = store_helpers::parse_line(&json);

	assert!(parsed.is_some());
	assert_eq!(parsed.unwrap().id, item.id);
}

/// parse_line returns None for empty/blank input
#[test]
fn parse_line_empty_returns_none() {
	assert!(store_helpers::parse_line("").is_none());
	assert!(
		store_helpers::parse_line("  ").is_none()
	);
}

/// parse_line returns None for malformed JSON
#[test]
fn parse_line_invalid_returns_none() {
	let result =
		store_helpers::parse_line("{broken");
	assert!(result.is_none());
}

/// read_lines returns empty vec for missing file
#[test]
fn read_lines_missing_returns_empty() {
	let path = PathBuf::from("/nonexistent.jsonl");
	let lines = store_helpers::read_lines(&path);
	assert!(lines.is_empty());
}

/// session_id_from_path extracts the file stem
#[test]
fn session_id_from_path_extracts_stem() {
	let path =
		PathBuf::from("/tmp/sessions/s-123.jsonl");
	let id =
		store_helpers::session_id_from_path(&path);
	assert_eq!(id, Some("s-123".to_string()));
}
