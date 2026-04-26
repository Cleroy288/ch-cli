use serde_json::{Value, json};

/// Tool definition for memory_search
pub fn memory_search_def() -> Value {
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
pub fn memory_recent_def() -> Value {
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
pub fn memory_stats_def() -> Value {
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
