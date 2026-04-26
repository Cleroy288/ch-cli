use std::path::Path;

use serde_json::Value;

use crate::service::search::types::{
	SearchFlags, SearchOptions,
};
use crate::service::{
	DefaultSearchService, SearchService,
};

use super::mcp_format_code;
use super::mcp_helpers::{
	extract_bool, extract_limit, extract_string,
	parse_symbol_kind, wrap_text_content,
};
use super::mcp_types::error_response;

pub fn exec_code_search(
	id: Value,
	args: &Value,
) -> String {
	let fuzzy = extract_bool(args, "fuzzy");
	let flags = SearchFlags {
		fuzzy, ..Default::default()
	};
	let query = extract_string(args, "query")
		.unwrap_or_default();
	let limit = extract_limit(args);
	let kind = extract_string(args, "kind")
		.and_then(|val| parse_symbol_kind(&val));
	let opts = SearchOptions {
		limit, kind, flags,
	};
	let svc = DefaultSearchService::new();
	match svc.search(&query, Path::new("."), &opts) {
		Ok(result) => {
			let text = mcp_format_code
				::format_search_hits(&result.hits);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}
