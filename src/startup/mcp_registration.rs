use super::mcp_registration_io::{
	read_mcp_config, register_mcp_server,
	remove_old_entry,
};

pub(crate) const MCP_CONFIG: &str = ".mcp.json";
pub(crate) const SERVER_NAME: &str = "rustean";
pub(crate) const OLD_SERVER_NAME: &str =
	"rustean-memory";
pub(crate) const CLAUDE_BIN: &str = "claude";
pub(crate) const MCP_ARG: &str = "mcp-server";

pub fn ensure_mcp_registered() {
	let Ok(exe) = std::env::current_exe() else {
		return;
	};
	let exe_str = exe.to_string_lossy();
	let content = read_mcp_config();
	if has_old_entry(&content) {
		remove_old_entry();
	}
	// Re-read: remove_old_entry modifies the file
	let content = read_mcp_config();
	if !has_valid_entry(&content, &exe_str) {
		register_mcp_server(&exe_str);
	}
}

pub fn has_valid_entry(
	content: &str,
	exe_path: &str,
) -> bool {
	let parsed = parse_mcp_json(content);
	let server =
		&parsed["mcpServers"][SERVER_NAME];
	server["command"].as_str() == Some(exe_path)
}

pub fn has_old_entry(content: &str) -> bool {
	let parsed = parse_mcp_json(content);
	parsed["mcpServers"][OLD_SERVER_NAME]
		.is_object()
}

/// Parse MCP config JSON, returning null on error
fn parse_mcp_json(
	content: &str,
) -> serde_json::Value {
	serde_json::from_str(content)
		.unwrap_or(serde_json::Value::Null)
}
