//! Tests for MCP config parsing.

use rustean::domain::mcp_config::McpServerKind;
use rustean::service::agents::config_parse::{
	parse_flat_config, parse_wrapped_config,
};

/// Wrapped config parses stdio server
#[test]
fn wrapped_parses_stdio_server() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"ctx7": {
				"type": "stdio",
				"command": "npx",
				"args": ["-y", "@ctx7/mcp"]
			}
		}
	}"#;

	// Act
	let servers =
		parse_wrapped_config(json, "project");

	// Assert
	assert_eq!(servers.len(), 1);
	assert_eq!(servers[0].name, "ctx7");
	assert_eq!(servers[0].kind, McpServerKind::Stdio);
	assert_eq!(
		servers[0].command.as_deref(),
		Some("npx"),
	);
	assert_eq!(
		servers[0].args,
		vec!["-y", "@ctx7/mcp"],
	);
	assert_eq!(servers[0].source, "project");
}

/// Wrapped config parses HTTP server
#[test]
fn wrapped_parses_http_server() {
	// Arrange
	let json = r#"{
		"mcpServers": {
			"remote": {
				"type": "http",
				"url": "https://api.example.com/mcp"
			}
		}
	}"#;

	// Act
	let servers =
		parse_wrapped_config(json, "project");

	// Assert
	assert_eq!(servers.len(), 1);
	assert_eq!(servers[0].kind, McpServerKind::Http);
	assert_eq!(
		servers[0].url.as_deref(),
		Some("https://api.example.com/mcp"),
	);
}

/// Wrapped config with missing mcpServers key
#[test]
fn wrapped_returns_empty_without_key() {
	let json = r#"{"other": {}}"#;
	let servers =
		parse_wrapped_config(json, "project");
	assert!(servers.is_empty());
}

/// Flat config parses multiple servers
#[test]
fn flat_parses_multiple_servers() {
	// Arrange
	let json = r#"{
		"tool-a": {
			"command": "node",
			"args": ["server.js"]
		},
		"tool-b": {
			"command": "python",
			"args": ["-m", "server"]
		}
	}"#;

	// Act
	let servers =
		parse_flat_config(json, "plugin");

	// Assert
	assert_eq!(servers.len(), 2);
}

/// Invalid JSON returns empty vec
#[test]
fn invalid_json_returns_empty() {
	let servers =
		parse_wrapped_config("not json", "x");
	assert!(servers.is_empty());
}

/// Entry without command is skipped
#[test]
fn stdio_without_command_is_skipped() {
	let json = r#"{
		"mcpServers": {
			"broken": {
				"type": "stdio",
				"args": ["a"]
			}
		}
	}"#;
	let servers =
		parse_wrapped_config(json, "x");
	assert!(servers.is_empty());
}

/// Default type is stdio when not specified
#[test]
fn default_type_is_stdio() {
	let json = r#"{
		"tool": {
			"command": "npx",
			"args": ["-y", "@pkg/mcp"]
		}
	}"#;
	let servers = parse_flat_config(json, "p");
	assert_eq!(servers.len(), 1);
	assert_eq!(servers[0].kind, McpServerKind::Stdio);
}
