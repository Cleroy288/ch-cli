use serde_json::{Value, json};

/// Static tool definition
pub struct ToolDef {
	/// Tool name (e.g. "bb_list_workspaces")
	pub name: &'static str,
	/// Human-readable description
	pub description: &'static str,
	/// Parameter definitions
	pub params: &'static [ParamDef],
	/// Required parameter names
	pub required: &'static [&'static str],
}

/// Single parameter definition
pub struct ParamDef {
	/// Parameter name
	pub name: &'static str,
	/// JSON schema type ("string", "number", etc.)
	pub param_type: &'static str,
	/// Human-readable description
	pub description: &'static str,
}

pub fn tool_to_json(def: &ToolDef) -> Value {
	let mut props = serde_json::Map::new();
	for param in def.params {
		props.insert(
			param.name.to_string(),
			json!({
				"type": param.param_type,
				"description": param.description,
			}),
		);
	}
	json!({
		"name": def.name,
		"description": def.description,
		"inputSchema": {
			"type": "object",
			"properties": props,
			"required": def.required,
		},
	})
}
