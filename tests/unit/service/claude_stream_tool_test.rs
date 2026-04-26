//! Tests for tool_use extraction from stream events.

use rustean::domain::claude::StreamChunk;
use rustean::service::claude::stream_tool_parse;
use serde_json::json;

/// Read tool extracts file_path as summary
#[test]
fn extract_read_tool_returns_file_path() {
	// Arrange
	let content = vec![json!({
		"type": "tool_use",
		"name": "Read",
		"input": {"file_path": "/src/main.rs"}
	})];

	// Act
	let chunk =
		stream_tool_parse::extract_tool_use(&content);

	// Assert
	match chunk {
		Some(StreamChunk::ToolUse(t)) => {
			assert_eq!(t.tool_name, "Read");
			assert_eq!(t.summary, "/src/main.rs");
		}
		_ => panic!("expected ToolUse"),
	}
}

/// Bash tool extracts command as summary
#[test]
fn extract_bash_tool_returns_command() {
	// Arrange
	let content = vec![json!({
		"type": "tool_use",
		"name": "Bash",
		"input": {"command": "cargo test"}
	})];

	// Act
	let chunk =
		stream_tool_parse::extract_tool_use(&content);

	// Assert
	match chunk {
		Some(StreamChunk::ToolUse(t)) => {
			assert_eq!(t.tool_name, "Bash");
			assert_eq!(t.summary, "cargo test");
		}
		_ => panic!("expected ToolUse"),
	}
}

/// Grep tool extracts pattern as summary
#[test]
fn extract_grep_tool_returns_pattern() {
	// Arrange
	let content = vec![json!({
		"type": "tool_use",
		"name": "Grep",
		"input": {"pattern": "fn main"}
	})];

	// Act
	let chunk =
		stream_tool_parse::extract_tool_use(&content);

	// Assert
	match chunk {
		Some(StreamChunk::ToolUse(t)) => {
			assert_eq!(t.tool_name, "Grep");
			assert_eq!(t.summary, "fn main");
		}
		_ => panic!("expected ToolUse"),
	}
}

/// Unknown tool returns empty summary
#[test]
fn extract_unknown_tool_returns_empty_summary() {
	// Arrange
	let content = vec![json!({
		"type": "tool_use",
		"name": "CustomTool",
		"input": {"data": "value"}
	})];

	// Act
	let chunk =
		stream_tool_parse::extract_tool_use(&content);

	// Assert
	match chunk {
		Some(StreamChunk::ToolUse(t)) => {
			assert_eq!(t.tool_name, "CustomTool");
			assert!(t.summary.is_empty());
		}
		_ => panic!("expected ToolUse"),
	}
}

/// No tool_use block returns None
#[test]
fn extract_no_tool_use_returns_none() {
	// Arrange
	let content = vec![json!({
		"type": "text",
		"text": "Hello"
	})];

	// Act + Assert
	assert!(
		stream_tool_parse::extract_tool_use(&content)
			.is_none()
	);
}

/// Empty content array returns None
#[test]
fn extract_empty_content_returns_none() {
	// Act + Assert
	let content: Vec<serde_json::Value> = vec![];
	assert!(
		stream_tool_parse::extract_tool_use(&content)
			.is_none()
	);
}

/// Long command summary gets truncated
#[test]
fn extract_long_summary_truncated() {
	// Arrange
	let long_cmd = "a".repeat(80);
	let content = vec![json!({
		"type": "tool_use",
		"name": "Bash",
		"input": {"command": long_cmd}
	})];

	// Act
	let chunk =
		stream_tool_parse::extract_tool_use(&content);

	// Assert
	match chunk {
		Some(StreamChunk::ToolUse(t)) => {
			assert!(t.summary.len() <= 63);
			assert!(t.summary.ends_with("..."));
		}
		_ => panic!("expected ToolUse"),
	}
}
