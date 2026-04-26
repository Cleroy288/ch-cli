use std::path::Path;

use serde_json::Value;

use crate::service::{
	DefaultMemoryService, MemoryService,
};

use super::mcp_format;
use super::mcp_helpers::{
	extract_limit, wrap_text_content,
};
use super::mcp_types::error_response;

pub(crate) fn exec_search(
	id: Value,
	args: &Value,
) -> String {
	let query = args.get("query")
		.and_then(Value::as_str)
		.unwrap_or("");
	let limit = extract_limit(args);
	let svc = DefaultMemoryService::default();
	match svc.search(Path::new("."), query, limit) {
		Ok(hits) => {
			let text =
				mcp_format::format_search_hits(
					&hits,
				);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

pub(crate) fn exec_recent(
	id: Value,
	args: &Value,
) -> String {
	let limit = extract_limit(args);
	let svc = DefaultMemoryService::default();
	match svc.show_recent(
		Path::new("."), limit,
	) {
		Ok(items) => {
			let text =
				mcp_format::format_interactions(
					&items,
				);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

pub(crate) fn exec_stats(id: Value) -> String {
	let svc = DefaultMemoryService::default();
	match svc.stats(Path::new(".")) {
		Ok(stats) => {
			let text =
				mcp_format::format_stats(&stats);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}
