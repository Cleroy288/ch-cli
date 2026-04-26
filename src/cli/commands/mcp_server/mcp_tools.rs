use serde_json::Value;

use super::mcp_dispatch;
use super::mcp_exec_aikido;
use super::mcp_tools_exec::{
	exec_recent, exec_search, exec_stats,
};
use super::mcp_types::{
	McpContext, error_response,
};
use super::{mcp_exec_bb, mcp_exec_jira};

pub fn execute_tool(
	id: Value,
	params: Option<&Value>,
	ctx: &McpContext,
) -> String {
	let name = params
		.and_then(|val| val.get("name"))
		.and_then(Value::as_str)
		.unwrap_or("");
	let args = params
		.and_then(|val| val.get("arguments"))
		.unwrap_or(&Value::Null);
	dispatch_tool(id, name, args, ctx)
}

/// Route by tool name prefix
fn dispatch_tool(
	id: Value,
	name: &str,
	args: &Value,
	ctx: &McpContext,
) -> String {
	match name {
		"memory_search" => exec_search(id, args),
		"memory_recent" => exec_recent(id, args),
		"memory_stats" => exec_stats(id),
		_ if name.starts_with("bb_") => {
			dispatch_bb(id, name, args, ctx)
		}
		_ if name.starts_with("jira_") => {
			dispatch_jira(id, name, args, ctx)
		}
		_ if name.starts_with("aikido_") => {
			dispatch_aikido(id, name, args, ctx)
		}
		_ => mcp_dispatch::dispatch_code(
			id, name, args,
		),
	}
}

fn dispatch_bb(
	id: Value,
	name: &str,
	args: &Value,
	ctx: &McpContext,
) -> String {
	match &ctx.bb_client {
		Some(client) => mcp_exec_bb::exec_bb_tool(
			id, name, args, client,
		),
		None => error_response(
			id,
			-32000,
			"Bitbucket not configured",
		),
	}
}

fn dispatch_jira(
	id: Value,
	name: &str,
	args: &Value,
	ctx: &McpContext,
) -> String {
	match &ctx.jira_client {
		Some(client) => {
			mcp_exec_jira::exec_jira_tool(
				id, name, args, client,
			)
		}
		None => error_response(
			id,
			-32000,
			"Jira not configured",
		),
	}
}

fn dispatch_aikido(
	id: Value,
	name: &str,
	args: &Value,
	ctx: &McpContext,
) -> String {
	match &ctx.aikido_client {
		Some(client) => {
			mcp_exec_aikido::exec_aikido_tool(
				id, name, args, client,
			)
		}
		None => error_response(
			id,
			-32000,
			"Aikido not configured",
		),
	}
}
