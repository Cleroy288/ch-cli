//! Tests for streaming lines and api duration footer.

use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage,
};
use rustean::ui::components::debug_render::{
	build_claude_response_lines,
	build_claude_streaming_lines,
};

/// Helper: collect all span text into one string
fn lines_to_text(
	lines: &[ratatui::text::Line<'static>],
) -> String {
	lines
		.iter()
		.map(|line| {
			line.spans
				.iter()
				.map(|s| s.content.to_string())
				.collect::<String>()
		})
		.collect::<Vec<_>>()
		.join("\n")
}

/// Streaming lines render partial text
#[test]
fn streaming_lines_contain_text() {
	// Arrange + Act
	let lines =
		build_claude_streaming_lines("Hello world", None);
	let text = lines_to_text(&lines);

	// Assert — text present, spinner at end
	assert!(text.contains("Hello world"));
	assert!(lines.len() >= 3);
}

/// Meta lines show wall duration
#[test]
fn meta_lines_show_wall_duration() {
	// Arrange
	let resp = ClaudeResponse {
		result: "ok".to_string(),
		session_id: "s-1".to_string(),
		subtype: "success".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: None,
		duration_ms: 3000,
		duration_api_ms: 1900,
		usage: ClaudeUsage {
			input_tokens: 10,
			output_tokens: 5,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
		intent: Default::default(),
	};

	// Act
	let lines =
		build_claude_response_lines(Some(&resp));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("3.0s"));
	assert!(text.contains("\u{2191}10"));
	assert!(text.contains("\u{2193}5"));
}

/// Empty streaming text still produces lines
#[test]
fn streaming_lines_empty_text() {
	// Arrange + Act
	let lines = build_claude_streaming_lines("", None);

	// Assert — spinner line always present
	assert!(lines.len() >= 2);
}

/// Zero duration renders 0.0s in footer
#[test]
fn footer_zero_duration() {
	// Arrange
	let resp = ClaudeResponse {
		result: "ok".to_string(),
		session_id: "s-z".to_string(),
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
	let lines =
		build_claude_response_lines(Some(&resp));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("0.0s"));
	assert!(text.contains("\u{2191}0"));
	assert!(text.contains("\u{2193}0"));
}

/// Multiline streaming text renders all lines
#[test]
fn streaming_multiline_renders_all() {
	// Arrange
	let input = "line one\nline two\nline three";

	// Act
	let lines =
		build_claude_streaming_lines(input, None);
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("line one"));
	assert!(text.contains("line two"));
	assert!(text.contains("line three"));
}
