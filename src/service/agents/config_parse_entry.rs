use serde_json::Value;

use crate::domain::mcp_config::{
	McpServerConfig, McpServerKind,
};

pub fn parse_server_entry(
	name: &str,
	value: &Value,
	source: &str,
) -> Option<McpServerConfig> {
	let kind = detect_kind(value);
	match kind {
		McpServerKind::Stdio => {
			parse_stdio_entry(name, value, source)
		}
		McpServerKind::Http => {
			parse_http_entry(name, value, source)
		}
	}
}

fn detect_kind(value: &Value) -> McpServerKind {
	let type_str = value
		.get("type")
		.and_then(|val| val.as_str())
		.unwrap_or("stdio");
	match type_str {
		"http" | "sse" => McpServerKind::Http,
		_ => McpServerKind::Stdio,
	}
}

fn parse_stdio_entry(
	name: &str,
	value: &Value,
	source: &str,
) -> Option<McpServerConfig> {
	let command = value
		.get("command")
		.and_then(|val| val.as_str())?;
	let args = extract_string_array(value, "args");
	Some(McpServerConfig {
		name: name.to_string(),
		kind: McpServerKind::Stdio,
		command: Some(command.to_string()),
		args,
		url: None,
		source: source.to_string(),
	})
}

fn parse_http_entry(
	name: &str,
	value: &Value,
	source: &str,
) -> Option<McpServerConfig> {
	let url = value
		.get("url")
		.and_then(|val| val.as_str())?;
	Some(McpServerConfig {
		name: name.to_string(),
		kind: McpServerKind::Http,
		command: None,
		args: Vec::new(),
		url: Some(url.to_string()),
		source: source.to_string(),
	})
}

/// Extract string array from a JSON field
pub(super) fn extract_string_array(
	value: &Value,
	field: &str,
) -> Vec<String> {
	value
		.get(field)
		.and_then(|val| val.as_array())
		.map(|arr| {
			arr.iter()
				.filter_map(|elem| elem.as_str())
				.map(String::from)
				.collect()
		})
		.unwrap_or_default()
}
