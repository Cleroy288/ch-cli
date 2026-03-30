//! Tests for Claude response line rendering.

use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage, ToolActivity,
};
use rustean::ui::components::debug_render::{
	build_claude_loading_lines,
	build_claude_response_lines,
	build_claude_streaming_lines,
};

/// Helper: build a test ClaudeResponse
fn make_response() -> ClaudeResponse {
	ClaudeResponse {
		result: "Hello from Claude".to_string(),
		session_id: "s-1".to_string(),
		subtype: "success".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: Some(0.005),
		duration_ms: 2000,
		duration_api_ms: 1500,
		usage: ClaudeUsage {
			input_tokens: 100,
			output_tokens: 50,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
		intent: Default::default(),
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

/// Response lines show token counts
#[test]
fn response_lines_show_token_counts() {
	// Arrange
	let resp = make_response();

	// Act
	let lines =
		build_claude_response_lines(Some(&resp));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("\u{2191}100"));
	assert!(text.contains("\u{2193}50"));
}

/// Response lines show duration
#[test]
fn response_lines_show_duration() {
	// Arrange
	let resp = make_response();

	// Act
	let lines =
		build_claude_response_lines(Some(&resp));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("2.0s"));
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
	let lines = build_claude_loading_lines(None);
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("Thinking"));
}

/// Loading lines show tool name when active
#[test]
fn loading_lines_show_tool_status() {
	// Arrange
	let tool = ToolActivity {
		tool_name: "Read".into(),
		summary: "src/main.rs".into(),
	};

	// Act
	let lines =
		build_claude_loading_lines(Some(&tool));
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("Read"));
	assert!(text.contains("src/main.rs"));
}

/// Streaming lines show tool status above text
#[test]
fn streaming_lines_show_tool_status() {
	// Arrange
	let tool = ToolActivity {
		tool_name: "Grep".into(),
		summary: "auth".into(),
	};

	// Act
	let lines = build_claude_streaming_lines(
		"results here",
		Some(&tool),
	);
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("Grep"));
	assert!(text.contains("auth"));
	assert!(text.contains("results here"));
}

/// Streaming lines without tool show just text
#[test]
fn streaming_lines_no_tool_show_text_only() {
	// Act
	let lines = build_claude_streaming_lines(
		"hello world",
		None,
	);
	let text = lines_to_text(&lines);

	// Assert
	assert!(text.contains("hello world"));
	assert!(!text.contains("Read"));
}
