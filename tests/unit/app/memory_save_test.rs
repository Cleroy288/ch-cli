//! Tests for TUI memory auto-save helpers.

use rustean::app::memory_save;
use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage,
};
use rustean::message::segment::MessageSegment;

/// Helper: build a test ClaudeResponse
fn make_response(
	text: &str,
	is_error: bool,
) -> ClaudeResponse {
	ClaudeResponse {
		result: text.to_string(),
		session_id: "test-sess".to_string(),
		subtype: "success".to_string(),
		is_error,
		num_turns: 1,
		cost_usd: Some(0.001),
		duration_ms: 500,
		duration_api_ms: 300,
		usage: ClaudeUsage {
			input_tokens: 10,
			output_tokens: 5,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
		intent: Default::default(),
	}
}

/// Plain text input extracts text field
#[test]
fn extract_user_input_text_only() {
	// Arrange
	let segments = vec![
		MessageSegment::Text("hello world".into()),
	];

	// Act
	let input = memory_save::extract_user_input(
		"hello world", &segments,
	);

	// Assert
	assert_eq!(input.text, "hello world");
	assert!(input.files.is_empty());
	assert!(input.command.is_none());
}

/// FileRef segments produce file paths
#[test]
fn extract_user_input_with_file_refs() {
	// Arrange
	let segments = vec![
		MessageSegment::FileReference {
			full_path: "/src/main.rs".into(),
			display_name: "main.rs".into(),
		},
	];

	// Act
	let input = memory_save::extract_user_input(
		"check main.rs", &segments,
	);

	// Assert
	assert_eq!(input.files.len(), 1);
	assert_eq!(input.files[0], "/src/main.rs");
}

/// Mixed segments collect all path-bearing refs
#[test]
fn extract_user_input_mixed_segments() {
	// Arrange
	let segments = vec![
		MessageSegment::Text("look at".into()),
		MessageSegment::FileReference {
			full_path: "/a.rs".into(),
			display_name: "a.rs".into(),
		},
		MessageSegment::FolderReference {
			full_path: "/src".into(),
			display_name: "src".into(),
		},
	];

	// Act
	let input = memory_save::extract_user_input(
		"look at a.rs src", &segments,
	);

	// Assert
	assert_eq!(input.files.len(), 2);
	assert_eq!(input.files[0], "/a.rs");
	assert_eq!(input.files[1], "/src");
}

/// Non-error response becomes AiResponse::Answer
#[test]
fn build_ai_response_success() {
	// Arrange
	let resp = make_response("done", false);

	// Act
	let ai = memory_save::build_ai_response(&resp);

	// Assert
	match ai {
		rustean::domain::memory::AiResponse::Answer {
			text,
		} => assert_eq!(text, "done"),
		_ => panic!("expected Answer variant"),
	}
}

/// Error response becomes AiResponse::Question
#[test]
fn build_ai_response_error() {
	// Arrange
	let resp = make_response("oops", true);

	// Act
	let ai = memory_save::build_ai_response(&resp);

	// Assert
	match ai {
		rustean::domain::memory::AiResponse::Question {
			text,
		} => assert_eq!(text, "oops"),
		_ => panic!("expected Question variant"),
	}
}
