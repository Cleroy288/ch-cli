//! Tests for Claude CLI JSON parsing.

use rustean::service::claude::parse_claude_response;

/// Parse a valid full JSON response
#[test]
fn parse_valid_json_extracts_all_fields() {
	// Arrange
	let json = r#"{
		"result": "Hello from Claude",
		"session_id": "abc-123",
		"is_error": false,
		"num_turns": 2,
		"total_cost_usd": 0.012,
		"duration_ms": 3500,
		"usage": {
			"input_tokens": 200,
			"output_tokens": 100,
			"cache_read_input_tokens": 50,
			"cache_creation_input_tokens": 10
		}
	}"#;

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.result, "Hello from Claude");
	assert_eq!(resp.session_id, "abc-123");
	assert_eq!(resp.num_turns, 2);
	assert_eq!(resp.usage.input_tokens, 200);
}

/// Parse minimal JSON with missing optional fields
#[test]
fn parse_minimal_json_uses_defaults() {
	// Arrange
	let json = r#"{
		"result": "ok",
		"session_id": "s-1"
	}"#;

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.result, "ok");
	assert!(!resp.is_error);
	assert_eq!(resp.num_turns, 0);
	assert!(resp.cost_usd.is_none());
	assert_eq!(resp.usage.input_tokens, 0);
}

/// Invalid JSON returns ClaudeError::InvalidJson
#[test]
fn parse_invalid_json_returns_error() {
	// Arrange
	let json = "not json at all";

	// Act
	let result = parse_claude_response(json);

	// Assert
	assert!(result.is_err());
	let err = result.unwrap_err();
	assert!(
		format!("{}", err).contains("Invalid JSON")
	);
}

/// Error response returns ClaudeError::ApiError
#[test]
fn parse_error_response_returns_api_error() {
	// Arrange
	let json = r#"{
		"result": "Rate limited",
		"session_id": "s-2",
		"is_error": true
	}"#;

	// Act
	let result = parse_claude_response(json);

	// Assert
	assert!(result.is_err());
	let err = result.unwrap_err();
	assert!(
		format!("{}", err).contains("API error")
	);
}

/// Empty JSON object uses all defaults
#[test]
fn parse_empty_object_uses_all_defaults() {
	// Arrange
	let json = "{}";

	// Act
	let resp = parse_claude_response(json).unwrap();

	// Assert
	assert_eq!(resp.result, "");
	assert_eq!(resp.session_id, "");
	assert!(!resp.is_error);
}
