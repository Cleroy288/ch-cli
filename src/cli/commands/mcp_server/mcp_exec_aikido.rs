use serde_json::Value;

use crate::service::aikido::AikidoClient;

use super::mcp_exec_aikido_filter::{
	exec_get_issues, exec_issue_counts,
};
use super::mcp_format_aikido;
use super::mcp_helpers::wrap_text_content;
use super::mcp_types::error_response;

/// Dispatch aikido_* tool calls
pub fn exec_aikido_tool(
	id: Value,
	name: &str,
	args: &Value,
	client: &AikidoClient,
) -> String {
	match name {
		"aikido_get_issues" => {
			exec_get_issues(id, args, client)
		}
		"aikido_list_repos" => {
			exec_list_repos(id, client)
		}
		"aikido_list_containers" => {
			exec_list_containers(id, client)
		}
		"aikido_issue_counts" => {
			exec_issue_counts(id, args, client)
		}
		"aikido_get_issue" => {
			exec_get_issue(id, args, client)
		}
		_ => error_response(
			id,
			-32602,
			"Unknown Aikido tool",
		),
	}
}

fn exec_list_repos(
	id: Value,
	client: &AikidoClient,
) -> String {
	match client.fetch_repos() {
		Ok(repos) => {
			let text =
				mcp_format_aikido::format_repos(
					&repos,
				);
			wrap_text_content(id, &text)
		}
		Err(e) => error_response(
			id, -32000, &e.to_string(),
		),
	}
}

fn exec_list_containers(
	id: Value,
	client: &AikidoClient,
) -> String {
	match client.fetch_containers() {
		Ok(items) => {
			let text = mcp_format_aikido
				::format_containers(&items);
			wrap_text_content(id, &text)
		}
		Err(e) => error_response(
			id, -32000, &e.to_string(),
		),
	}
}

fn exec_get_issue(
	id: Value,
	args: &Value,
	client: &AikidoClient,
) -> String {
	let issue_id = args
		.get("issue_id")
		.and_then(Value::as_u64)
		.unwrap_or(0);
	match client.fetch_detail(issue_id) {
		Ok(detail) => {
			let text = mcp_format_aikido
				::format_detail(&detail);
			wrap_text_content(id, &text)
		}
		Err(e) => error_response(
			id, -32000, &e.to_string(),
		),
	}
}
