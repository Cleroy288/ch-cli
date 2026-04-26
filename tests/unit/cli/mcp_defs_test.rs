//! Tests for MCP tool definitions.
//!
//! Validates each definition has a name and
//! inputSchema with correct structure.

use rustean::cli::commands::mcp_server::{
	mcp_defs_code, mcp_tool_defs,
	mcp_types::McpContext,
};

/// Build a context with no Atlassian credentials
fn empty_ctx() -> McpContext {
	McpContext::from_credentials(None)
}

/// Base tool count is 10 without creds
#[test]
fn base_tool_count_is_ten() {
	// Arrange
	let ctx = empty_ctx();

	// Act
	let defs =
		mcp_tool_defs::tool_definitions(&ctx);

	// Assert
	assert_eq!(defs.len(), 10);
}

/// Every def has name and inputSchema
#[test]
fn all_defs_have_name_and_schema() {
	// Arrange
	let ctx = empty_ctx();
	let defs =
		mcp_tool_defs::tool_definitions(&ctx);

	// Act / Assert
	for def in &defs {
		assert!(
			def.get("name").is_some(),
			"missing name in {def}",
		);
		assert!(
			def.get("inputSchema").is_some(),
			"missing schema in {def}",
		);
	}
}

/// code_search has required query field
#[test]
fn code_search_requires_query() {
	// Arrange / Act
	let def = mcp_defs_code::code_search_def();

	// Assert
	let required = def["inputSchema"]["required"]
		.as_array().unwrap();
	assert!(required.contains(
		&serde_json::json!("query"),
	));
}

/// code_goto has required symbol field
#[test]
fn code_goto_requires_symbol() {
	// Arrange / Act
	let def = mcp_defs_code::code_goto_def();

	// Assert
	let required = def["inputSchema"]["required"]
		.as_array().unwrap();
	assert!(required.contains(
		&serde_json::json!("symbol"),
	));
}
