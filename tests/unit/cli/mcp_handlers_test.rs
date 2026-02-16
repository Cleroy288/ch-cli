//! Tests for MCP message routing and handlers.

use rustean::cli::commands::mcp_server::mcp_handlers;
use serde_json::Value;

/// Initialize returns protocol + serverInfo
#[test]
fn initialize_returns_capabilities() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;

	// Act
	let resp = mcp_handlers::handle_message(msg)
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
		"rustean-memory",
	);
}

/// tools/list returns all three tool names
#[test]
fn tools_list_returns_three_tools() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;

	// Act
	let resp = mcp_handlers::handle_message(msg)
		.expect("should return response");
	let parsed: Value =
		serde_json::from_str(&resp).unwrap();

	// Assert
	let tools = parsed["result"]["tools"]
		.as_array().unwrap();
	assert_eq!(tools.len(), 3);
	let names: Vec<&str> = tools.iter()
		.map(|t| t["name"].as_str().unwrap())
		.collect();
	assert!(names.contains(&"memory_search"));
	assert!(names.contains(&"memory_recent"));
	assert!(names.contains(&"memory_stats"));
}

/// Notification returns None (no response)
#[test]
fn notification_returns_none() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;

	// Act
	let resp = mcp_handlers::handle_message(msg);

	// Assert
	assert!(resp.is_none());
}

/// Unknown method returns -32601
#[test]
fn unknown_method_returns_error() {
	// Arrange
	let msg = r#"{"jsonrpc":"2.0","id":3,"method":"foo/bar"}"#;

	// Act
	let resp = mcp_handlers::handle_message(msg)
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

	// Act
	let resp = mcp_handlers::handle_message(msg)
		.expect("should return response");
	let parsed: Value =
		serde_json::from_str(&resp).unwrap();

	// Assert
	assert_eq!(parsed["error"]["code"], -32700);
}
