//! Tests for Claude Code MCP source discovery.

use rustean::domain::mcp_config::McpServerKind;
use rustean::service::agents::config_claude::{
	discover_claude_servers, managed_servers,
	user_servers,
};

/// User servers returns empty or valid entries
#[test]
fn user_servers_empty_without_home() {
	let servers = user_servers();
	for srv in &servers {
		assert_eq!(srv.source, "user");
	}
}

/// Managed servers returns empty when no file
#[test]
fn managed_servers_empty_no_file() {
	// Arrange — managed-mcp.json absent on dev

	// Act
	let servers = managed_servers();

	// Assert
	assert!(servers.is_empty());
}

/// User servers filters out self entry
#[test]
fn user_servers_filters_self() {
	let servers = user_servers();
	let found =
		servers.iter().any(|s| s.name == "rustean");
	assert!(!found, "self-entry must be filtered");
}

/// User servers parses stdio type correctly
#[test]
fn user_servers_parses_stdio_kind() {
	let servers = user_servers();
	for srv in &servers {
		if srv.command.is_some() {
			assert_eq!(srv.kind, McpServerKind::Stdio);
		}
	}
}

/// Discover deduplicates by server name
#[test]
fn discover_has_no_duplicate_names() {
	// Act
	let servers = discover_claude_servers();

	// Assert — no two servers share a name
	let mut names: Vec<&str> =
		servers.iter().map(|s| s.name.as_str()).collect();
	let total = names.len();
	names.sort();
	names.dedup();
	assert_eq!(
		names.len(),
		total,
		"duplicate server names found",
	);
}
