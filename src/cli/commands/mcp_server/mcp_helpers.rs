use serde_json::{Value, json};

use crate::indexer::SymbolKind;

use super::mcp_types::success_response;

/// Default search/recent limit
const DEFAULT_LIMIT: usize = 10;

/// Extract limit param with default fallback
pub fn extract_limit(args: &Value) -> usize {
	args.get("limit")
		.and_then(Value::as_u64)
		.map(|num| num as usize)
		.unwrap_or(DEFAULT_LIMIT)
}

/// Extract optional string param
pub fn extract_string(
	args: &Value,
	key: &str,
) -> Option<String> {
	args.get(key)
		.and_then(Value::as_str)
		.map(String::from)
}

/// Extract optional bool param (default false)
pub fn extract_bool(
	args: &Value,
	key: &str,
) -> bool {
	args.get(key)
		.and_then(Value::as_bool)
		.unwrap_or(false)
}

/// Wrap text in MCP content array response
pub fn wrap_text_content(
	id: Value,
	text: &str,
) -> String {
	let result = json!({
		"content": [{
			"type": "text",
			"text": text,
		}]
	});
	success_response(id, result)
}

/// Delegates to the canonical CLI parser.
pub fn parse_symbol_kind(
	kind_str: &str,
) -> Option<SymbolKind> {
	crate::cli::commands::search
		::parse_symbol_kind(kind_str)
		.ok()
}
