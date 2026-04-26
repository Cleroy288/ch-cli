//! Tests for MCP shared helpers.

use rustean::cli::commands::mcp_server
	::mcp_helpers;
use serde_json::json;

/// extract_limit returns value when present
#[test]
fn extract_limit_returns_value() {
	// Arrange
	let args = json!({"limit": 5});

	// Act
	let limit = mcp_helpers::extract_limit(&args);

	// Assert
	assert_eq!(limit, 5);
}

/// extract_limit returns default when absent
#[test]
fn extract_limit_returns_default() {
	// Arrange
	let args = json!({});

	// Act
	let limit = mcp_helpers::extract_limit(&args);

	// Assert
	assert_eq!(limit, 10);
}

/// extract_string returns Some for present key
#[test]
fn extract_string_present() {
	// Arrange
	let args = json!({"query": "hello"});

	// Act
	let val =
		mcp_helpers::extract_string(&args, "query");

	// Assert
	assert_eq!(val, Some("hello".to_string()));
}

/// extract_string returns None for missing key
#[test]
fn extract_string_missing() {
	// Arrange
	let args = json!({});

	// Act
	let val =
		mcp_helpers::extract_string(&args, "query");

	// Assert
	assert_eq!(val, None);
}

/// extract_bool returns true when set
#[test]
fn extract_bool_true() {
	// Arrange
	let args = json!({"fuzzy": true});

	// Act / Assert
	assert!(mcp_helpers::extract_bool(
		&args, "fuzzy",
	));
}
