//! Tests for MCP message routing and handlers.

use rustean::cli::commands::mcp_server::{
	mcp_handlers, mcp_types::McpContext,
};
use serde_json::Value;

/// Build a context with no Atlassian credentials
fn empty_ctx() -> McpContext {
	McpContext::from_credentials(None)
}

/// Initialize returns protocol + serverInfo
#[test]
fn initialize_returns_capabilities() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
	let ctx = empty_ctx();

	// Act
	let resp =
		mcp_handlers::handle_message(msg, &ctx)
			.expect("should return response");
	let parsed: Value =
		serde_json::from_str(&resp).unwrap();

	// Assert
	let result = &parsed["result"];
	assert_eq!(
		result["protocolVersion"], "2024-11-05",
	);
	assert_eq!(
		result["serverInfo"]["name"],
		"rustean",
	);
}

/// tools/list returns 10 tools without creds
#[test]
fn tools_list_returns_base_tools() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
	let ctx = empty_ctx();

	// Act
	let resp =
		mcp_handlers::handle_message(msg, &ctx)
			.expect("should return response");
	let parsed: Value =
		serde_json::from_str(&resp).unwrap();

	// Assert — 10 base tools (no Atlassian)
	let tools = parsed["result"]["tools"]
		.as_array().unwrap();
	assert_eq!(tools.len(), 10);
}

/// Notification returns None (no response)
#[test]
fn notification_returns_none() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
	let ctx = empty_ctx();

	// Act
	let resp =
		mcp_handlers::handle_message(msg, &ctx);

	// Assert
	assert!(resp.is_none());
}

/// Unknown method returns -32601
#[test]
fn unknown_method_returns_error() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","id":3,"method":"foo/bar"}"#;
	let ctx = empty_ctx();

	// Act
	let resp =
		mcp_handlers::handle_message(msg, &ctx)
			.expect("should return response");
	let parsed: Value =
		serde_json::from_str(&resp).unwrap();

	// Assert
	assert_eq!(parsed["error"]["code"], -32601);
}

/// Invalid JSON returns parse error -32700
#[test]
fn invalid_json_returns_parse_error() {
	// Arrange
	let msg = "not valid json{{{";
	let ctx = empty_ctx();

	// Act
	let resp =
		mcp_handlers::handle_message(msg, &ctx)
			.expect("should return response");
	let parsed: Value =
		serde_json::from_str(&resp).unwrap();

	// Assert
	assert_eq!(parsed["error"]["code"], -32700);
}
