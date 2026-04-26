use serde_json::Value;

use crate::domain::mcp_config::McpServerConfig;

pub use super::config_parse_entry::{
	parse_server_entry,
};

/// Format: `{"mcpServers": {"name": {server}}}`
pub fn parse_wrapped_config(
	json: &str,
	source: &str,
) -> Vec<McpServerConfig> {
	let Ok(root) =
		serde_json::from_str::<Value>(json)
	else {
		return Vec::new();
	};
	let Some(servers) = root.get("mcpServers")
	else {
		return Vec::new();
	};
	parse_server_map(servers, source)
}

/// Format: `{"name": {"command": ..., "args": [...]}}`
pub fn parse_flat_config(
	json: &str,
	source: &str,
) -> Vec<McpServerConfig> {
	let Ok(root) =
		serde_json::from_str::<Value>(json)
	else {
		return Vec::new();
	};
	parse_server_map(&root, source)
}

fn parse_server_map(
	obj: &Value,
	source: &str,
) -> Vec<McpServerConfig> {
	let Some(map) = obj.as_object() else {
		return Vec::new();
	};
	map.iter()
		.filter_map(|(name, val)| {
			parse_server_entry(name, val, source)
		})
		.collect()
}
