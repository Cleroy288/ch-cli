//! Tests for MCP discovery type parsing.

use rustean::service::agents::discovery_types::{
	ToolDef, ToolsListResponse, to_tool_infos,
};

/// to_tool_infos converts defs to infos
#[test]
fn converts_defs_to_infos() {
	// Arrange
	let defs = vec![
		ToolDef {
			name: "search".to_string(),
			description: Some("Find code".into()),
		},
		ToolDef {
			name: "nav".to_string(),
			description: None,
		},
	];

	// Act
	let infos = to_tool_infos(defs);

	// Assert
	assert_eq!(infos.len(), 2);
	assert_eq!(infos[0].name, "search");
	assert_eq!(infos[0].description, "Find code");
	assert_eq!(infos[1].name, "nav");
	assert_eq!(infos[1].description, "");
}

/// ToolsListResponse deserializes from JSON
#[test]
fn tools_list_response_deserializes() {
	// Arrange
	let json = r#"{
		"jsonrpc": "2.0",
		"id": 2,
		"result": {
			"tools": [
				{
					"name": "resolve-library-id",
					"description": "Resolve library"
				}
			]
		}
	}"#;

	// Act
	let resp: ToolsListResponse =
		serde_json::from_str(json).unwrap();

	// Assert
	let result = resp.result.unwrap();
	assert_eq!(result.tools.len(), 1);
	assert_eq!(
		result.tools[0].name,
		"resolve-library-id",
	);
}

/// Empty tools array parses correctly
#[test]
fn empty_tools_array_parses() {
	let json = r#"{
		"result": {"tools": []}
	}"#;
	let resp: ToolsListResponse =
		serde_json::from_str(json).unwrap();
	let result = resp.result.unwrap();
	assert!(result.tools.is_empty());
}

/// Missing result field gives None
#[test]
fn missing_result_gives_none() {
	let json = r#"{"jsonrpc": "2.0", "id": 2}"#;
	let resp: ToolsListResponse =
		serde_json::from_str(json).unwrap();
	assert!(resp.result.is_none());
}

/// to_tool_infos with empty vec returns empty
#[test]
fn empty_defs_returns_empty_infos() {
	let infos = to_tool_infos(vec![]);
	assert!(infos.is_empty());
}
