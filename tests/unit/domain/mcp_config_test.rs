//! Tests for domain MCP config types.

use rustean::domain::mcp_config::{
	McpServerConfig, McpServerKind,
};

/// McpServerKind::Stdio serializes correctly
#[test]
fn stdio_kind_serializes() {
	let json = serde_json::to_string(
		&McpServerKind::Stdio,
	)
	.unwrap();
	assert_eq!(json, "\"stdio\"");
}

/// McpServerKind::Http serializes correctly
#[test]
fn http_kind_serializes() {
	let json = serde_json::to_string(
		&McpServerKind::Http,
	)
	.unwrap();
	assert_eq!(json, "\"http\"");
}

/// is_stdio returns true for Stdio kind
#[test]
fn is_stdio_returns_true_for_stdio() {
	let config = McpServerConfig {
		name: "test".to_string(),
		kind: McpServerKind::Stdio,
		command: Some("node".to_string()),
		args: vec![],
		url: None,
		source: "test".to_string(),
	};
	assert!(config.is_stdio());
}

/// is_stdio returns false for Http kind
#[test]
fn is_stdio_returns_false_for_http() {
	let config = McpServerConfig {
		name: "test".to_string(),
		kind: McpServerKind::Http,
		command: None,
		args: vec![],
		url: Some("https://x.com".to_string()),
		source: "test".to_string(),
	};
	assert!(!config.is_stdio());
}

/// McpServerKind implements Copy and Eq
#[test]
fn server_kind_is_copy_and_eq() {
	let kind = McpServerKind::Stdio;
	let copy = kind;
	assert_eq!(kind, copy);
}
