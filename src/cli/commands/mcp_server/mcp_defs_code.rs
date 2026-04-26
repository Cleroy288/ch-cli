use serde_json::{Value, json};

/// Tool definition for code_search
#[allow(clippy::too_many_lines)]
pub fn code_search_def() -> Value {
	json!({
		"name": "code_search",
		"description":
			"Search code symbols by keyword",
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
				"kind": {
					"type": "string",
					"description":
						"Filter: function|struct|\
						 enum|trait|method|const",
				},
				"fuzzy": {
					"type": "boolean",
					"description":
						"Enable fuzzy matching",
				},
			},
			"required": ["query"],
		},
	})
}

/// Tool definition for code_goto
pub fn code_goto_def() -> Value {
	json!({
		"name": "code_goto",
		"description":
			"Find symbol definition location",
		"inputSchema": {
			"type": "object",
			"properties": {
				"symbol": {
					"type": "string",
					"description":
						"Symbol name to find",
				},
			},
			"required": ["symbol"],
		},
	})
}

/// Tool definition for code_refs
pub fn code_refs_def() -> Value {
	json!({
		"name": "code_refs",
		"description":
			"Find all references to a symbol",
		"inputSchema": {
			"type": "object",
			"properties": {
				"symbol": {
					"type": "string",
					"description":
						"Symbol name to search",
				},
				"include_definition": {
					"type": "boolean",
					"description":
						"Include definition site",
				},
			},
			"required": ["symbol"],
		},
	})
}

/// Tool definition for code_callers
pub fn code_callers_def() -> Value {
	json!({
		"name": "code_callers",
		"description":
			"Find callers of a function/method",
		"inputSchema": {
			"type": "object",
			"properties": {
				"symbol": {
					"type": "string",
					"description":
						"Function name to search",
				},
			},
			"required": ["symbol"],
		},
	})
}
