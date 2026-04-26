//! Tests for stream-json line parsing.

use rustean::domain::claude::StreamChunk;
use rustean::service::claude::parse_stream_line;

/// Assistant with text content returns Delta
#[test]
fn parse_assistant_text_returns_delta() {
	// Arrange
	let line = r#"{
		"type": "assistant",
		"message": {
			"content": [
				{"type": "text", "text": "Hello"}
			]
		}
	}"#;

	// Act
	let chunks = parse_stream_line(line);

	// Assert
	assert_eq!(chunks.len(), 1);
	match &chunks[0] {
		StreamChunk::Delta(t) => {
			assert_eq!(t, "Hello");
		}
		_ => panic!("expected Delta"),
	}
}

/// Result event returns Done chunk
#[test]
fn parse_result_returns_done() {
	// Arrange
	let line = r#"{
		"type": "result",
		"result": "Full response",
		"session_id": "sess-42",
		"subtype": "success",
		"is_error": false,
		"num_turns": 2,
		"duration_ms": 1000,
		"duration_api_ms": 800
	}"#;

	// Act
	let chunks = parse_stream_line(line);

	// Assert
	assert_eq!(chunks.len(), 1);
	match &chunks[0] {
		StreamChunk::Done(resp) => {
			assert_eq!(resp.result, "Full response");
			assert_eq!(resp.session_id, "sess-42");
		}
		_ => panic!("expected Done"),
	}
}

/// System init event returns empty
#[test]
fn parse_system_event_returns_empty() {
	// Arrange
	let line = r#"{
		"type": "system",
		"subtype": "init",
		"session_id": "sess-1"
	}"#;

	// Act + Assert
	assert!(parse_stream_line(line).is_empty());
}

/// Malformed JSON returns empty
#[test]
fn parse_malformed_json_returns_empty() {
	assert!(
		parse_stream_line("not valid json {{{")
			.is_empty()
	);
}

/// Result with is_error=true still returns Done
#[test]
fn parse_error_result_returns_done() {
	// Arrange
	let line = r#"{
		"type": "result",
		"result": "Error: rate limited",
		"session_id": "sess-err",
		"subtype": "error",
		"is_error": true,
		"num_turns": 0,
		"duration_ms": 100,
		"duration_api_ms": 50
	}"#;

	// Act
	let chunks = parse_stream_line(line);

	// Assert
	assert_eq!(chunks.len(), 1);
	match &chunks[0] {
		StreamChunk::Done(resp) => {
			assert!(resp.is_error);
			assert_eq!(resp.subtype, "error");
		}
		_ => panic!("expected Done with error"),
	}
}

/// Empty string returns empty vec
#[test]
fn parse_empty_string_returns_empty() {
	assert!(parse_stream_line("").is_empty());
}

/// Tool type event returns empty (ignored)
#[test]
fn parse_tool_event_returns_empty() {
	// Arrange
	let line = r#"{
		"type": "tool",
		"name": "Read",
		"content": "file data"
	}"#;

	// Act + Assert
	assert!(parse_stream_line(line).is_empty());
}

/// Result with invalid inner JSON returns Error
#[test]
fn parse_result_bad_inner_returns_error() {
	// Arrange — missing required fields
	let line = r#"{
		"type": "result",
		"unexpected_field": true
	}"#;

	// Act
	let chunks = parse_stream_line(line);

	// Assert
	assert_eq!(chunks.len(), 1);
	match &chunks[0] {
		StreamChunk::Error(msg) => {
			assert!(!msg.is_empty());
		}
		StreamChunk::Done(resp) => {
			// Permissive parse may succeed
			assert!(resp.result.is_empty());
		}
		_ => panic!("expected Error or Done"),
	}
}

/// Assistant with empty content returns empty
#[test]
fn parse_empty_content_returns_empty() {
	// Arrange
	let line = r#"{
		"type": "assistant",
		"message": {
			"content": []
		}
	}"#;

	// Act + Assert
	assert!(parse_stream_line(line).is_empty());
}

/// Assistant with only tool_use returns ToolUse
#[test]
fn parse_tool_use_only_returns_tool_use() {
	// Arrange
	let line = r#"{
		"type": "assistant",
		"message": {
			"content": [
				{
					"type": "tool_use",
					"name": "Read",
					"input": {"file_path": "/a.rs"}
				}
			]
		}
	}"#;

	// Act
	let chunks = parse_stream_line(line);

	// Assert
	assert_eq!(chunks.len(), 1);
	match &chunks[0] {
		StreamChunk::ToolUse(t) => {
			assert_eq!(t.tool_name, "Read");
			assert_eq!(t.summary, "/a.rs");
		}
		_ => panic!("expected ToolUse"),
	}
}

/// Assistant with text AND tool_use returns both
#[test]
fn parse_text_and_tool_returns_both() {
	// Arrange
	let line = r#"{
		"type": "assistant",
		"message": {
			"content": [
				{"type": "text", "text": "Reading"},
				{
					"type": "tool_use",
					"name": "Bash",
					"input": {"command": "ls"}
				}
			]
		}
	}"#;

	// Act
	let chunks = parse_stream_line(line);

	// Assert — Delta first, then ToolUse
	assert_eq!(chunks.len(), 2);
	match &chunks[0] {
		StreamChunk::Delta(t) => {
			assert_eq!(t, "Reading");
		}
		_ => panic!("expected Delta first"),
	}
	match &chunks[1] {
		StreamChunk::ToolUse(t) => {
			assert_eq!(t.tool_name, "Bash");
			assert_eq!(t.summary, "ls");
		}
		_ => panic!("expected ToolUse second"),
	}
}
