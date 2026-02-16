//! JSON-RPC 2.0 types and response builders
//! for the MCP stdio protocol.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC protocol version
const JSONRPC_VERSION: &str = "2.0";

/// Incoming JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
	pub jsonrpc: String,    // must be "2.0"
	pub id: Option<Value>,  // None = notification
	pub method: String,     // e.g. "tools/list"
	pub params: Option<Value>, // method arguments
}

/// Outgoing JSON-RPC response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
	pub jsonrpc: String,     // always "2.0"
	pub id: Value,           // matches request id
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<Value>, // success payload
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<RpcError>, // error payload
}

/// JSON-RPC error object
#[derive(Debug, Serialize)]
pub struct RpcError {
	pub code: i64,           // error code
	pub message: String,     // human-readable msg
}

/// Build a success response as JSON string
pub fn success_response(
	id: Value,
	result: Value,
) -> String {
	let resp = JsonRpcResponse {
		jsonrpc: JSONRPC_VERSION.to_string(),
		id,
		result: Some(result),
		error: None,
	};
	serde_json::to_string(&resp)
		.unwrap_or_default()
}

/// Build an error response as JSON string
pub fn error_response(
	id: Value,
	code: i64,
	msg: &str,
) -> String {
	let resp = JsonRpcResponse {
		jsonrpc: JSONRPC_VERSION.to_string(),
		id,
		result: None,
		error: Some(RpcError {
			code,
			message: msg.to_string(),
		}),
	};
	serde_json::to_string(&resp)
		.unwrap_or_default()
}
