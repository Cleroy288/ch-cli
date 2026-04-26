//! Tests for subtype and duration_api_ms parsing.

use rustean::service::claude::parse_claude_response;

/// Subtype is extracted when present
#[test]
fn parse_subtype_extracts_value() {
	// Arrange
	let json = r#"{
		"result": "ok",
		"session_id": "s-1",
		"subtype": "error_max_turns"
	}"#;

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.subtype, "error_max_turns");
}

/// Missing subtype defaults to "success"
#[test]
fn parse_missing_subtype_defaults_success() {
	// Arrange
	let json = r#"{
		"result": "ok",
		"session_id": "s-1"
	}"#;

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.subtype, "success");
}

/// duration_api_ms is extracted when present
#[test]
fn parse_duration_api_ms_extracts_value() {
	// Arrange
	let json = r#"{
		"result": "ok",
		"session_id": "s-1",
		"duration_ms": 5000,
		"duration_api_ms": 3200
	}"#;

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.duration_api_ms, 3200);
}

/// Missing duration_api_ms defaults to 0
#[test]
fn parse_missing_duration_api_ms_defaults_zero() {
	// Arrange
	let json = r#"{
		"result": "ok",
		"session_id": "s-1"
	}"#;

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.duration_api_ms, 0);
}
