use serde_json::Value;

use super::mcp_types::error_response;
use super::{
	mcp_exec_code, mcp_exec_nav, mcp_exec_search,
};

/// Route a code_* tool to its executor
pub fn dispatch_code(
	id: Value,
	name: &str,
	args: &Value,
) -> String {
	match name {
		"code_search" =>
			mcp_exec_search::exec_code_search(
				id, args,
			),
		"code_goto" =>
			mcp_exec_code::exec_code_goto(
				id, args,
			),
		"code_refs" =>
			mcp_exec_code::exec_code_refs(
				id, args,
			),
		"code_callers" =>
			mcp_exec_code::exec_code_callers(
				id, args,
			),
		_ => dispatch_nav(id, name, args),
	}
}

/// Route navigation tools
fn dispatch_nav(
	id: Value,
	name: &str,
	args: &Value,
) -> String {
	match name {
		"code_symbols" =>
			mcp_exec_nav::exec_code_symbols(
				id, args,
			),
		"code_info" =>
			mcp_exec_nav::exec_code_info(
				id, args,
			),
		"code_stats" =>
			mcp_exec_nav::exec_code_stats(id),
		_ => error_response(
			id, -32602, "Unknown tool",
		),
	}
}
