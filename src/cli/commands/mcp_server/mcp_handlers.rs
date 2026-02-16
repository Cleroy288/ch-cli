//! MCP message routing and protocol handlers.
//!
//! Parses JSON-RPC messages and routes to the
//! appropriate handler based on method name.

use serde_json::{Value, json};

use super::mcp_tool_defs;
use super::mcp_tools;
use super::mcp_types::{
	JsonRpcRequest, error_response,
	success_response,
};

/// MCP protocol version
const PROTOCOL_VERSION: &str = "2024-11-05";
/// Server name
const SERVER_NAME: &str = "rustean-memory";
/// Server version
const SERVER_VERSION: &str = "0.1.0";

/// Handle a single line from stdin.
/// Returns Some(response) for requests,
/// None for notifications.
pub fn handle_message(
	line: &str,
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
	route_method(&req)
}

/// Route request to the right handler
fn route_method(
	req: &JsonRpcRequest,
) -> Option<String> {
	// Notifications have no id -> no response
	let Some(id) = &req.id else {
		return None;
	};
	let id = id.clone();
	let resp = match req.method.as_str() {
		"initialize" => handle_initialize(id),
		"tools/list" => handle_tools_list(id),
		"tools/call" => {
			handle_tools_call(id, req.params.as_ref())
		}
		_ => error_response(
			id, -32601, "Method not found",
		),
	};
	Some(resp)
}

/// Handle initialize request
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

/// Handle tools/list request
fn handle_tools_list(id: Value) -> String {
	let tools = mcp_tool_defs::tool_definitions();
	let result = json!({ "tools": tools });
	success_response(id, result)
}

/// Handle tools/call request
fn handle_tools_call(
	id: Value,
	params: Option<&Value>,
) -> String {
	mcp_tools::execute_tool(id, params)
}
