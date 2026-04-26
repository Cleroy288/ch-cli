use serde_json::{Value, json};

use super::mcp_tool_defs;
use super::mcp_tools;
use super::mcp_types::{
	JsonRpcRequest, McpContext, error_response,
	success_response,
};

const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "rustean";
const SERVER_VERSION: &str =
	env!("CARGO_PKG_VERSION");

/// None for notifications (no id).
pub fn handle_message(
	line: &str,
	ctx: &McpContext,
) -> Option<String> {
	let req: JsonRpcRequest =
		match serde_json::from_str(line) {
			Ok(req) => req,
			Err(_) => {
				return Some(error_response(
					Value::Null,
					-32700,
					"Parse error",
				));
			}
		};
	route_method(&req, ctx)
}

fn route_method(
	req: &JsonRpcRequest,
	ctx: &McpContext,
) -> Option<String> {
	let Some(id) = &req.id else {
		return None;
	};
	let id = id.clone();
	let resp = match req.method.as_str() {
		"initialize" => handle_initialize(id),
		"tools/list" => {
			handle_tools_list(id, ctx)
		}
		"tools/call" => handle_tools_call(
			id,
			req.params.as_ref(),
			ctx,
		),
		_ => error_response(
			id, -32601, "Method not found",
		),
	};
	Some(resp)
}

fn handle_initialize(id: Value) -> String {
	let result = json!({
		"protocolVersion": PROTOCOL_VERSION,
		"capabilities": { "tools": {} },
		"serverInfo": {
			"name": SERVER_NAME,
			"version": SERVER_VERSION,
		},
	});
	success_response(id, result)
}

fn handle_tools_list(
	id: Value,
	ctx: &McpContext,
) -> String {
	let tools =
		mcp_tool_defs::tool_definitions(ctx);
	let result = json!({ "tools": tools });
	success_response(id, result)
}

/// Handle tools/call request
fn handle_tools_call(
	id: Value,
	params: Option<&Value>,
	ctx: &McpContext,
) -> String {
	mcp_tools::execute_tool(id, params, ctx)
}
