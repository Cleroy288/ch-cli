//! Tests for Claude response line rendering.

use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage,
};
use rustean::ui::components::debug_render::{
	build_claude_loading_lines,
	build_claude_response_lines,
};

/// Helper: build a test ClaudeResponse
fn make_response() -> ClaudeResponse {
	ClaudeResponse {
		result: "Hello from Claude".to_string(),
		session_id: "s-1".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: Some(0.005),
		duration_ms: 2000,
		usage: ClaudeUsage {
			input_tokens: 100,
			output_tokens: 50,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
	}
}

/// Collect all span text from lines into one string
fn lines_to_text(
	lines: &[ratatui::text::Line<'static>],
) -> String {
	lines
		.iter()
		.map(|line| {
			line.spans
				.iter()
				.map(|span| span.content.to_string())
				.collect::<String>()
		})
		.collect::<Vec<_>>()
		.join("\n")
}

/// Response lines contain the result text
#[test]
fn response_lines_contain_result_text() {
	// Arrange
	let resp = make_response();

	// Act
	let lines =
		build_claude_response_lines(Some(&resp));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("Hello from Claude"));
}

/// Response lines include usage footer
#[test]
fn response_lines_include_usage_footer() {
	// Arrange
	let resp = make_response();

	// Act
	let lines =
		build_claude_response_lines(Some(&resp));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("tokens:"));
	assert!(text.contains("turns:"));
	assert!(text.contains("time:"));
}

/// None response returns empty vec
#[test]
fn none_response_returns_empty() {
	// Act
	let lines = build_claude_response_lines(None);

	// Assert
	assert!(lines.is_empty());
}

/// Loading lines contain thinking indicator
#[test]
fn loading_lines_contain_thinking() {
	// Act
	let lines = build_claude_loading_lines();
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("Thinking"));
}
