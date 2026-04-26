use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::domain::mcp_config::McpServerConfig;

use super::config_parse::parse_flat_config;
use super::config_plugins_path::{
	plugin_config_path, short_plugin_name,
};

pub use super::config_plugins_path::{
	latest_version_dir,
};

pub fn plugin_servers() -> Vec<McpServerConfig> {
	let Some(home) = claude_home() else {
		return Vec::new();
	};
	let names = enabled_plugin_names(&home);
	names
		.iter()
		.flat_map(|name| {
			load_plugin_servers(&home, name)
		})
		.collect()
}

pub fn enabled_plugin_names(
	claude_home: &Path,
) -> Vec<String> {
	let path = claude_home.join("settings.json");
	let Ok(data) = std::fs::read_to_string(path)
	else {
		return Vec::new();
	};
	let Ok(root) =
		serde_json::from_str::<Value>(&data)
	else {
		return Vec::new();
	};
	extract_enabled_names(&root)
}

/// Extract names where value is `true`
fn extract_enabled_names(
	root: &Value,
) -> Vec<String> {
	let Some(map) = root
		.get("enabledPlugins")
		.and_then(|val| val.as_object())
	else {
		return Vec::new();
	};
	map.iter()
		.filter(|(_, val)| {
			val.as_bool() == Some(true)
		})
		.map(|(key, _)| key.clone())
		.collect()
}

fn load_plugin_servers(
	claude_home: &Path,
	plugin_name: &str,
) -> Vec<McpServerConfig> {
	let Some(config_path) = plugin_config_path(
		claude_home, plugin_name,
	) else {
		return Vec::new();
	};
	let Ok(json) = std::fs::read_to_string(
		&config_path,
	) else {
		return Vec::new();
	};
	let source = short_plugin_name(plugin_name);
	parse_flat_config(&json, &source)
}

/// `~/.claude` path if HOME is set
fn claude_home() -> Option<PathBuf> {
	let home = std::env::var("HOME").ok()?;
	Some(PathBuf::from(home).join(".claude"))
}
