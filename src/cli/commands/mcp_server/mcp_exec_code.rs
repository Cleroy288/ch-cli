use std::path::Path;

use serde_json::Value;

use crate::service::{
	DefaultSearchService, SearchService,
};

use super::mcp_format_code;
use super::mcp_helpers::{
	extract_bool, extract_string,
	wrap_text_content,
};
use super::mcp_types::error_response;

pub fn exec_code_goto(
	id: Value,
	args: &Value,
) -> String {
	let symbol = extract_string(args, "symbol")
		.unwrap_or_default();
	let svc = DefaultSearchService::new();
	match svc.find_definition(
		&symbol, Path::new("."),
	) {
		Ok(hits) => {
			let text = mcp_format_code
				::format_definitions(&hits);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

pub fn exec_code_refs(
	id: Value,
	args: &Value,
) -> String {
	let symbol = extract_string(args, "symbol")
		.unwrap_or_default();
	let include_def =
		extract_bool(args, "include_definition");
	let svc = DefaultSearchService::new();
	match svc.find_references(
		&symbol, Path::new("."), include_def,
	) {
		Ok(result) => {
			let text = super::mcp_format_nav
				::format_references(&result);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

pub fn exec_code_callers(
	id: Value,
	args: &Value,
) -> String {
	let symbol = extract_string(args, "symbol")
		.unwrap_or_default();
	let svc = DefaultSearchService::new();
	match svc.search_callers(
		&symbol, Path::new("."),
	) {
		Ok(callers) => {
			let text = mcp_format_code
				::format_callers(&callers);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}
