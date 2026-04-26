use serde_json::{Value, json};

/// Tool definition for code_symbols
pub fn code_symbols_def() -> Value {
	json!({
		"name": "code_symbols",
		"description":
			"List symbols in the project",
		"inputSchema": {
			"type": "object",
			"properties": {
				"file": {
					"type": "string",
					"description":
						"Filter by file path",
				},
				"kind": {
					"type": "string",
					"description":
						"Filter by symbol kind",
				},
			},
		},
	})
}

/// Tool definition for code_info
pub fn code_info_def() -> Value {
	json!({
		"name": "code_info",
		"description":
			"Get detailed info about a symbol",
		"inputSchema": {
			"type": "object",
			"properties": {
				"symbol": {
					"type": "string",
					"description":
						"Symbol name to inspect",
				},
			},
			"required": ["symbol"],
		},
	})
}

/// Tool definition for code_stats
pub fn code_stats_def() -> Value {
	json!({
		"name": "code_stats",
		"description":
			"Show index statistics",
		"inputSchema": {
			"type": "object",
			"properties": {},
		},
	})
}
