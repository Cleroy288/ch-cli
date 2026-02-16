//! MCP tool execution dispatch.
//!
//! Routes tools/call requests to the memory
//! service and wraps results as MCP content.

use std::path::Path;

use serde_json::{Value, json};

use crate::service::{
	DefaultMemoryService, MemoryService,
};

use super::mcp_format;
use super::mcp_types::{
	error_response, success_response,
};

/// Default search/recent limit
const DEFAULT_LIMIT: usize = 10;

/// Dispatch a tools/call request by tool name
pub fn execute_tool(
	id: Value,
	params: Option<&Value>,
) -> String {
	let name = params
		.and_then(|val| val.get("name"))
		.and_then(Value::as_str)
		.unwrap_or("");
	let args = params
		.and_then(|val| val.get("arguments"))
		.unwrap_or(&Value::Null);
	match name {
		"memory_search" => exec_search(id, args),
		"memory_recent" => exec_recent(id, args),
		"memory_stats" => exec_stats(id),
		_ => error_response(
			id, -32602, "Unknown tool",
		),
	}
}

/// Execute memory_search tool
fn exec_search(id: Value, args: &Value) -> String {
	let query = args.get("query")
		.and_then(Value::as_str)
		.unwrap_or("");
	let limit = extract_limit(args);
	let svc = DefaultMemoryService::new();
	let root = Path::new(".");
	match svc.search(root, query, limit) {
		Ok(hits) => {
			let text =
				mcp_format::format_search_hits(&hits);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

/// Execute memory_recent tool
fn exec_recent(id: Value, args: &Value) -> String {
	let limit = extract_limit(args);
	let svc = DefaultMemoryService::new();
	let root = Path::new(".");
	match svc.show_recent(root, limit) {
		Ok(items) => {
			let text =
				mcp_format::format_interactions(&items);
			wrap_text_content(id, &text)
		}
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

/// Execute memory_stats tool
fn exec_stats(id: Value) -> String {
	let svc = DefaultMemoryService::new();
	let root = Path::new(".");
	match svc.stats(root) {
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

/// Extract limit param with default fallback
fn extract_limit(args: &Value) -> usize {
	args.get("limit")
		.and_then(Value::as_u64)
		.map(|num| num as usize)
		.unwrap_or(DEFAULT_LIMIT)
}

/// Wrap text in MCP content array response
fn wrap_text_content(
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
