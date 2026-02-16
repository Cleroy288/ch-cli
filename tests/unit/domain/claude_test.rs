//! Tests for Claude domain types — serde round-trips.

use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage,
};

/// Test ClaudeResponse serializes and deserializes
#[test]
fn serialize_response_roundtrip() {
	// Arrange
	let resp = ClaudeResponse {
		result: "hello world".to_string(),
		session_id: "sess-1".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: Some(0.005),
		duration_ms: 1200,
		usage: ClaudeUsage {
			input_tokens: 100,
			output_tokens: 50,
			cache_read_tokens: 10,
			cache_creation_tokens: 5,
		},
	};

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();
	let parsed: ClaudeResponse =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert_eq!(parsed.result, "hello world");
	assert_eq!(parsed.session_id, "sess-1");
	assert_eq!(parsed.usage.input_tokens, 100);
}

/// Test ClaudeResponse with no cost
#[test]
fn response_without_cost() {
	// Arrange
	let resp = ClaudeResponse {
		result: "test".to_string(),
		session_id: "s-2".to_string(),
		is_error: false,
		num_turns: 0,
		cost_usd: None,
		duration_ms: 0,
		usage: ClaudeUsage {
			input_tokens: 0,
			output_tokens: 0,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
	};

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();

	// Assert
	assert!(json.contains("\"cost_usd\":null"));
}

/// Test ClaudeResponse is_error flag serializes
#[test]
fn response_error_flag_roundtrip() {
	// Arrange
	let resp = ClaudeResponse {
		result: "error msg".to_string(),
		session_id: "s-3".to_string(),
		is_error: true,
		num_turns: 0,
		cost_usd: None,
		duration_ms: 500,
		usage: ClaudeUsage {
			input_tokens: 10,
			output_tokens: 5,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
	};

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();
	let parsed: ClaudeResponse =
		serde_json::from_str(&json).unwrap();

	// Assert
	assert!(parsed.is_error);
	assert_eq!(parsed.result, "error msg");
}
