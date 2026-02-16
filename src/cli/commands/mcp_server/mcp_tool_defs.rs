//! MCP tool definition schemas.
//!
//! Returns JSON schema for each tool exposed
//! to Claude Code via the MCP protocol.

use serde_json::{Value, json};

/// Build the list of available tool definitions
pub fn tool_definitions() -> Vec<Value> {
	vec![search_def(), recent_def(), stats_def()]
}

/// Tool definition for memory_search
fn search_def() -> Value {
	json!({
		"name": "memory_search",
		"description":
			"Search past interactions by keyword",
		"inputSchema": {
			"type": "object",
			"properties": {
				"query": {
					"type": "string",
					"description": "Search query",
				},
				"limit": {
					"type": "number",
					"description":
						"Max results (default 10)",
				},
			},
			"required": ["query"],
		},
	})
}

/// Tool definition for memory_recent
fn recent_def() -> Value {
	json!({
		"name": "memory_recent",
		"description":
			"Show recent interactions",
		"inputSchema": {
			"type": "object",
			"properties": {
				"limit": {
					"type": "number",
					"description":
						"Max results (default 10)",
				},
			},
		},
	})
}

/// Tool definition for memory_stats
fn stats_def() -> Value {
	json!({
		"name": "memory_stats",
		"description":
			"Show memory statistics",
		"inputSchema": {
			"type": "object",
			"properties": {},
		},
	})
}
