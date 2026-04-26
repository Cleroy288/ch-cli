use serde::Deserialize;

/// Discovered MCP tool from an external server
#[derive(Debug, Clone)]
pub struct McpToolInfo {
	pub name: String,
	pub description: String,
}

/// JSON-RPC response for tools/list
#[derive(Debug, Deserialize)]
pub struct ToolsListResponse {
	pub result: Option<ToolsListResult>,
}

/// Result payload of tools/list
#[derive(Debug, Deserialize)]
pub struct ToolsListResult {
	pub tools: Vec<ToolDef>,
}

/// Tool definition in tools/list result
#[derive(Debug, Deserialize)]
pub struct ToolDef {
	pub name: String,
	#[serde(default)]
	pub description: Option<String>,
}

pub fn to_tool_infos(
	defs: Vec<ToolDef>,
) -> Vec<McpToolInfo> {
	defs.into_iter()
		.map(|def| McpToolInfo {
			name: def.name,
			description: def
				.description
				.unwrap_or_default(),
		})
		.collect()
}
