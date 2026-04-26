use std::path::Path;

use serde_json::Value;

use crate::service::search::types_navigation::{
	SymbolListOptions,
};
use crate::service::{
	DefaultIndexService, DefaultSearchService,
	IndexService, SearchService,
};

use super::mcp_helpers::{
	extract_string, parse_symbol_kind,
	wrap_text_content,
};
use super::mcp_types::error_response;

pub fn exec_code_symbols(
	id: Value,
	args: &Value,
) -> String {
	let file = extract_string(args, "file");
	let kind = extract_string(args, "kind")
		.and_then(|val| parse_symbol_kind(&val));
	let opts = SymbolListOptions { file, kind };
	let svc = DefaultSearchService::new();
	match svc.list_symbols(Path::new("."), &opts) {
		Ok(entries) => {
			let text = super::mcp_format_code
				::format_symbol_list(&entries);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

pub fn exec_code_info(
	id: Value,
	args: &Value,
) -> String {
	let symbol = extract_string(args, "symbol")
		.unwrap_or_default();
	let svc = DefaultSearchService::new();
	match svc.get_symbol_info(
		&symbol, Path::new("."),
	) {
		Ok(Some(details)) => {
			let text = super::mcp_format_info
				::format_symbol_details(&details);
			wrap_text_content(id, &text)
		}
		Ok(None) => wrap_text_content(
			id, "(symbol not found)",
		),
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

pub fn exec_code_stats(id: Value) -> String {
	let svc = DefaultIndexService::default();
	match svc.get_stats(Path::new(".")) {
		Ok(Some(stats)) => {
			let text = super::mcp_format_nav
				::format_index_stats(&stats);
			wrap_text_content(id, &text)
		}
		Ok(None) => wrap_text_content(
			id, "(no index found)",
		),
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}
