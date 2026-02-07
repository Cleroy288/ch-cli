//! Core LLM Response Parsing
//!
//! Parses LLM JSON responses into SearchSpec.

use crate::retrieval::daemon::protocol::{
	FileFilter, SearchSpec,
};

use super::super::parsing::{
	extract_identifiers, filter_stop_words,
};
use super::json::{
	extract_json_filters, extract_json_hints,
	extract_json_symbols,
};
use super::fallback::detect_fallback_intent;

/// Default prompt template for query expansion
pub const QUERY_EXPANSION_PROMPT: &str = r#"You are a code \
search assistant. Given a natural language query about code, \
extract:
1. Symbol names (functions, classes, structs, etc.) to search for
2. The user's intent (find_definition, find_usages, understand, \
modify, debug, search)
3. File patterns to filter (e.g., "*.rs", "src/auth/**")

Query: {query}

Respond in this exact JSON format:
{
  "symbols": ["symbol1", "symbol2"],
  "intent": "search",
  "file_patterns": ["*.rs"],
  "hints": ["any additional context"]
}

JSON response:"#;

/// Parse LLM response JSON into SearchSpec
pub fn parse_llm_response(
	query: &str,
	response: &str,
) -> SearchSpec {
	if let Ok(json) =
		serde_json::from_str::<serde_json::Value>(response)
	{
		parse_json_spec(query, &json)
	} else {
		fallback_parse(query)
	}
}

/// Parse a valid JSON value into SearchSpec
fn parse_json_spec(
	query: &str,
	json: &serde_json::Value,
) -> SearchSpec {
	let raw_symbols = extract_json_symbols(json);
	let mut symbols = filter_stop_words(raw_symbols);

	if symbols.is_empty() {
		symbols = extract_identifiers(query);
	}

	let intent = json
		.get("intent")
		.and_then(|v| v.as_str())
		.map(super::super::parsing::parse_intent)
		.unwrap_or(
			crate::retrieval::daemon::protocol::QueryIntent::Search,
		);

	let file_filters = extract_json_filters(json);
	let hints = extract_json_hints(json);

	SearchSpec {
		original_query: query.to_string(),
		symbol_names: symbols,
		intent,
		file_filters,
		context_hints: hints,
	}
}

/// Fallback parser when LLM response is not valid JSON
pub fn fallback_parse(query: &str) -> SearchSpec {
	let symbols = extract_identifiers(query);
	let intent = detect_fallback_intent(query);

	SearchSpec {
		original_query: query.to_string(),
		symbol_names: symbols,
		intent,
		file_filters: Vec::new(),
		context_hints: Vec::new(),
	}
}
