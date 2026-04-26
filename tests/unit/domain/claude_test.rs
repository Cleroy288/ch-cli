//! Tests for Claude domain types — serde and intent.

use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage, ResponseIntent,
	detect_intent,
};
use rustean::domain::review::ReviewBlock;

/// ClaudeResponse serde roundtrip preserves fields
#[test]
fn serialize_response_roundtrip() {
	// Arrange
	let resp = ClaudeResponse {
		result: "hello world".to_string(),
		session_id: "sess-1".to_string(),
		subtype: "success".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: Some(0.005),
		duration_ms: 1200,
		duration_api_ms: 800,
		usage: ClaudeUsage {
			input_tokens: 100,
			output_tokens: 50,
			cache_read_tokens: 10,
			cache_creation_tokens: 5,
		},
		intent: Default::default(),
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

/// ClaudeResponse serializes None cost as null
#[test]
fn response_without_cost() {
	// Arrange
	let resp = ClaudeResponse {
		result: "test".to_string(),
		session_id: "s-2".to_string(),
		subtype: "success".to_string(),
		is_error: false,
		num_turns: 0,
		cost_usd: None,
		duration_ms: 0,
		duration_api_ms: 0,
		usage: ClaudeUsage {
			input_tokens: 0,
			output_tokens: 0,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
		intent: Default::default(),
	};

	// Act
	let json =
		serde_json::to_string(&resp).unwrap();

	// Assert
	assert!(json.contains("\"cost_usd\":null"));
}

/// ClaudeResponse is_error flag survives roundtrip
#[test]
fn response_error_flag_roundtrip() {
	// Arrange
	let resp = ClaudeResponse {
		result: "error msg".to_string(),
		session_id: "s-3".to_string(),
		subtype: "success".to_string(),
		is_error: true,
		num_turns: 0,
		cost_usd: None,
		duration_ms: 500,
		duration_api_ms: 300,
		usage: ClaudeUsage {
			input_tokens: 10,
			output_tokens: 5,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
		intent: Default::default(),
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

/// detect_intent returns Implementation when
/// blocks have file paths.
#[test]
fn detect_intent_with_file_paths() {
	// Arrange
	let blocks = vec![ReviewBlock::new(
		0,
		"rust".into(),
		"fn main() {}".into(),
		Some("src/main.rs".into()),
	)];

	// Act
	let intent = detect_intent(&blocks);

	// Assert
	assert_eq!(intent, ResponseIntent::Implementation);
}

/// detect_intent returns Question when no block
/// has a file path.
#[test]
fn detect_intent_without_file_paths() {
	// Arrange
	let blocks = vec![ReviewBlock::new(
		0,
		"rust".into(),
		"let x = 42;".into(),
		None,
	)];

	// Act
	let intent = detect_intent(&blocks);

	// Assert
	assert_eq!(intent, ResponseIntent::Question);
}

/// detect_intent returns Question for empty blocks.
#[test]
fn detect_intent_empty_blocks() {
	// Arrange + Act
	let intent = detect_intent(&[]);

	// Assert
	assert_eq!(intent, ResponseIntent::Question);
}
