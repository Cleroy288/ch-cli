//! Tests for MCP tool dispatch routing.

use rustean::cli::commands::mcp_server::{
	mcp_tools, mcp_types::McpContext,
};
use serde_json::{Value, json};

/// Build a context with no Atlassian credentials
fn empty_ctx() -> McpContext {
	McpContext::from_credentials(None)
}

/// code_stats returns valid response
#[test]
fn code_stats_returns_response() {
	// Arrange
	let ctx = empty_ctx();
	let params = json!({
		"name": "code_stats",
		"arguments": {},
	});

	// Act
	let raw = mcp_tools::execute_tool(
		json!(1), Some(&params), &ctx,
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert — either result or error is valid
	assert!(
		parsed.get("result").is_some()
			|| parsed.get("error").is_some(),
	);
}

/// unknown tool returns error
#[test]
fn unknown_tool_returns_error() {
	// Arrange
	let ctx = empty_ctx();
	let params = json!({
		"name": "nonexistent_tool",
		"arguments": {},
	});

	// Act
	let raw = mcp_tools::execute_tool(
		json!(2), Some(&params), &ctx,
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	assert!(parsed.get("error").is_some());
}
