//! Tests for MCP result formatting.

use rustean::cli::commands::mcp_server::mcp_format;
use rustean::domain::memory::{
	AiResponse, Interaction, UserInput,
};
use rustean::indexer::memory::types::MemoryStats;

/// Helper: build a test interaction
fn make_interaction(
	input_text: &str,
	response_text: &str,
) -> Interaction {
	Interaction {
		id: "test-id".into(),
		timestamp: 1000,
		session_id: "sess-1".into(),
		input: UserInput {
			text: input_text.into(),
			command: None,
			files: Vec::new(),
		},
		response: AiResponse::Answer {
			text: response_text.into(),
		},
	}
}

/// Empty list returns no-results message
#[test]
fn format_empty_returns_no_results() {
	// Arrange / Act
	let text = mcp_format::format_interactions(&[]);

	// Assert
	assert_eq!(text, "(no results)");
}

/// Non-empty list shows input and response text
#[test]
fn format_shows_input_and_response() {
	// Arrange
	let items =
		vec![make_interaction("hello", "world")];

	// Act
	let text =
		mcp_format::format_interactions(&items);

	// Assert
	assert!(text.contains("hello"));
	assert!(text.contains("world"));
	assert!(text.contains("sess-1"));
}

/// Stats format includes all fields
#[test]
fn format_stats_all_fields() {
	// Arrange
	let stats = MemoryStats {
		total: 42,
		sessions: 5,
		size_bytes: 1024,
		last_updated: 9999,
	};

	// Act
	let text = mcp_format::format_stats(&stats);

	// Assert
	assert!(text.contains("42"));
	assert!(text.contains("5"));
	assert!(text.contains("1024"));
	assert!(text.contains("9999"));
}
