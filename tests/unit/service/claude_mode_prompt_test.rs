//! Tests for query mode detection and prompt injection.

use rustean::domain::claude::QueryMode;
use rustean::service::claude::{
	detect_query_mode, mode_instruction,
};

/// /Q prefix detects Question mode
#[test]
fn detect_question_mode() {
	// Arrange + Act
	let (mode, text) =
		detect_query_mode("/Q how does it work?");

	// Assert
	assert_eq!(mode, Some(QueryMode::Question));
	assert_eq!(text, "how does it work?");
}

/// /A prefix detects Action mode
#[test]
fn detect_action_mode() {
	// Arrange + Act
	let (mode, text) =
		detect_query_mode("/A fix the parser");

	// Assert
	assert_eq!(mode, Some(QueryMode::Action));
	assert_eq!(text, "fix the parser");
}

/// /P prefix detects Plan mode
#[test]
fn detect_plan_mode() {
	// Arrange + Act
	let (mode, text) =
		detect_query_mode("/P add auth system");

	// Assert
	assert_eq!(mode, Some(QueryMode::Plan));
	assert_eq!(text, "add auth system");
}

/// No prefix returns None and original text
#[test]
fn detect_no_prefix_returns_none() {
	// Arrange + Act
	let (mode, text) =
		detect_query_mode("just a question");

	// Assert
	assert_eq!(mode, None);
	assert_eq!(text, "just a question");
}

/// /Q without trailing space is not a mode prefix
#[test]
fn detect_q_no_space_returns_none() {
	// Arrange + Act
	let (mode, text) =
		detect_query_mode("/Qhow does it work?");

	// Assert
	assert_eq!(mode, None);
	assert_eq!(text, "/Qhow does it work?");
}

/// Lowercase /q is not a valid prefix
#[test]
fn detect_lowercase_q_returns_none() {
	// Arrange + Act
	let (mode, text) =
		detect_query_mode("/q how does it work?");

	// Assert
	assert_eq!(mode, None);
	assert_eq!(text, "/q how does it work?");
}

/// Empty string returns None
#[test]
fn detect_empty_returns_none() {
	// Arrange + Act
	let (mode, text) = detect_query_mode("");

	// Assert
	assert_eq!(mode, None);
	assert_eq!(text, "");
}

/// mode_instruction returns non-empty for each mode
#[test]
fn instructions_are_non_empty() {
	// Assert
	assert!(!mode_instruction(QueryMode::Question)
		.is_empty());
	assert!(!mode_instruction(QueryMode::Action)
		.is_empty());
	assert!(!mode_instruction(QueryMode::Plan)
		.is_empty());
}

/// All mode instructions include MCP codebase access
#[test]
fn instructions_include_mcp_access() {
	// Arrange
	let needle = "codebase-retrieval";

	// Assert
	assert!(mode_instruction(QueryMode::Question)
		.contains(needle));
	assert!(mode_instruction(QueryMode::Action)
		.contains(needle));
	assert!(mode_instruction(QueryMode::Plan)
		.contains(needle));
}
