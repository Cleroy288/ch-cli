//! Tests for MCP tool execution.

use rustean::cli::commands::mcp_server::{
	mcp_tools, mcp_types::McpContext,
};
use serde_json::{Value, json};

/// Build a context with no Atlassian credentials
fn empty_ctx() -> McpContext {
	McpContext::from_credentials(None)
}

/// Search returns content array with text type
#[test]
fn search_returns_content_array() {
	// Arrange
	let ctx = empty_ctx();
	let params = json!({
		"name": "memory_search",
		"arguments": { "query": "test" },
	});

	// Act
	let raw = mcp_tools::execute_tool(
		json!(1), Some(&params), &ctx,
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	if let Some(result) = parsed.get("result") {
		let items = result["content"]
			.as_array().unwrap();
		assert_eq!(items[0]["type"], "text");
	}
}

/// Recent returns content array
#[test]
fn recent_returns_content_array() {
	// Arrange
	let ctx = empty_ctx();
	let params = json!({
		"name": "memory_recent",
		"arguments": { "limit": 5 },
	});

	// Act
	let raw = mcp_tools::execute_tool(
		json!(2), Some(&params), &ctx,
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	if let Some(result) = parsed.get("result") {
		let items = result["content"]
			.as_array().unwrap();
		assert_eq!(items[0]["type"], "text");
	}
}

/// Stats returns content array
#[test]
fn stats_returns_content_array() {
	// Arrange
	let ctx = empty_ctx();
	let params = json!({
		"name": "memory_stats",
		"arguments": {},
	});

	// Act
	let raw = mcp_tools::execute_tool(
		json!(3), Some(&params), &ctx,
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	if let Some(result) = parsed.get("result") {
		let items = result["content"]
			.as_array().unwrap();
		assert_eq!(items[0]["type"], "text");
	}
}

/// Unknown tool returns error
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
		json!(4), Some(&params), &ctx,
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	assert_eq!(parsed["error"]["code"], -32602);
}
