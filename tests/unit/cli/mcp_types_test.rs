//! Tests for MCP JSON-RPC types and builders.

use rustean::cli::commands::mcp_server::mcp_types::{
	JsonRpcRequest, error_response,
	success_response,
};
use serde_json::{Value, json};

/// Success response produces valid JSON
#[test]
fn success_response_valid_json() {
	// Arrange
	let id = json!(1);
	let result = json!({"ok": true});

	// Act
	let raw = success_response(id, result);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	assert_eq!(parsed["jsonrpc"], "2.0");
	assert_eq!(parsed["id"], 1);
	assert_eq!(parsed["result"]["ok"], true);
	assert!(parsed.get("error").is_none());
}

/// Error response includes code and message
#[test]
fn error_response_valid_json() {
	// Arrange / Act
	let raw = error_response(
		json!(2), -32600, "Bad request",
	);
	let parsed: Value =
		serde_json::from_str(&raw).unwrap();

	// Assert
	assert_eq!(parsed["error"]["code"], -32600);
	assert_eq!(
		parsed["error"]["message"], "Bad request",
	);
}

/// Deserialization extracts all fields
#[test]
fn parse_request_extracts_fields() {
	// Arrange
	let raw = r#"{"jsonrpc":"2.0","id":5,"method":"tools/list","params":null}"#;

	// Act
	let req: JsonRpcRequest =
		serde_json::from_str(raw).unwrap();

	// Assert
	assert_eq!(req.method, "tools/list");
	assert_eq!(req.id, Some(json!(5)));
}

/// Notification has id = None
#[test]
fn parse_notification_has_no_id() {
	// Arrange
	let raw = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;

	// Act
	let req: JsonRpcRequest =
		serde_json::from_str(raw).unwrap();

	// Assert
	assert!(req.id.is_none());
}
