use std::collections::HashSet;
use std::path::PathBuf;

use crate::domain::mcp_config::McpServerConfig;

use super::config_parse::parse_wrapped_config;
use super::config_plugins::plugin_servers;

/// Self-entry name to skip
const SELF_NAME: &str = "rustean";

/// Discover all Claude Code MCP servers
/// Sources in priority order: managed > project >
/// user > plugins. Deduplicates by server name --
/// first occurrence wins.
pub fn discover_claude_servers(
) -> Vec<McpServerConfig> {
	let mut servers = managed_servers();
	servers.extend(project_servers());
	servers.extend(user_servers());
	servers.extend(plugin_servers());
	let mut seen = HashSet::new();
	servers.retain(|s| seen.insert(s.name.clone()));
	servers
}

pub fn project_servers() -> Vec<McpServerConfig> {
	let path = std::path::Path::new(".mcp.json");
	let Ok(json) = std::fs::read_to_string(path)
	else {
		return Vec::new();
	};
	parse_wrapped_config(&json, "project")
		.into_iter()
		.filter(|srv| srv.name != SELF_NAME)
		.collect()
}

pub fn user_servers() -> Vec<McpServerConfig> {
	let Some(home) = std::env::var("HOME").ok()
	else {
		return Vec::new();
	};
	let path =
		PathBuf::from(home).join(".claude.json");
	let Ok(json) = std::fs::read_to_string(&path)
	else {
		return Vec::new();
	};
	parse_wrapped_config(&json, "user")
		.into_iter()
		.filter(|srv| srv.name != SELF_NAME)
		.collect()
}

pub fn managed_servers() -> Vec<McpServerConfig> {
	let path = managed_mcp_path();
	let Ok(json) = std::fs::read_to_string(&path)
	else {
		return Vec::new();
	};
	parse_wrapped_config(&json, "managed")
}

/// Platform-specific managed-mcp.json path
fn managed_mcp_path() -> PathBuf {
	#[cfg(target_os = "macos")]
	{
		PathBuf::from(
			"/Library/Application Support",
		)
		.join("ClaudeCode")
		.join("managed-mcp.json")
	}
	#[cfg(target_os = "linux")]
	{
		PathBuf::from("/etc/claude-code")
			.join("managed-mcp.json")
	}
	#[cfg(not(any(
		target_os = "macos",
		target_os = "linux"
	)))]
	{
		PathBuf::from("managed-mcp.json")
	}
}
