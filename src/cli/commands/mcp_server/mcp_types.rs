use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::domain::credentials
	::AikidoCredentials;
use crate::service::aikido::AikidoClient;
use crate::service::atlassian
	::client_bb::BbClient;
use crate::service::atlassian
	::client_jira::JiraClient;
use crate::service::atlassian
	::types::AtlassianCredentials;

/// JSON-RPC protocol version
const JSONRPC_VERSION: &str = "2.0";

/// Incoming JSON-RPC request
#[derive(Debug, Deserialize)]
pub struct JsonRpcRequest {
	pub jsonrpc: String,
	pub id: Option<Value>,
	pub method: String,
	pub params: Option<Value>,
}

/// Outgoing JSON-RPC response
#[derive(Debug, Serialize)]
pub struct JsonRpcResponse {
	pub jsonrpc: String,
	pub id: Value,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result: Option<Value>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<RpcError>,
}

/// JSON-RPC error object
#[derive(Debug, Serialize)]
pub struct RpcError {
	pub code: i64,
	pub message: String,
}

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

/// MCP server runtime context
pub struct McpContext {
	/// Bitbucket client (if configured)
	pub bb_client: Option<BbClient>,
	/// Jira client (if configured)
	pub jira_client: Option<JiraClient>,
	/// Aikido Security client (if configured)
	pub aikido_client: Option<AikidoClient>,
}

impl McpContext {
	pub fn build(
		atl: Option<AtlassianCredentials>,
		aikido: Option<AikidoCredentials>,
	) -> Self {
		let (bb_client, jira_client) = match atl {
			Some(creds) => (
				creds.bitbucket
					.map(BbClient::new),
				creds.jira
					.map(JiraClient::new),
			),
			None => (None, None),
		};
		let aikido_client =
			aikido.map(AikidoClient::new);
		Self {
			bb_client,
			jira_client,
			aikido_client,
		}
	}

	/// Legacy constructor (Atlassian only)
	pub fn from_credentials(
		creds: Option<AtlassianCredentials>,
	) -> Self {
		Self::build(creds, None)
	}
}

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
