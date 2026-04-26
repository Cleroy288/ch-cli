use serde_json::Value;

use crate::service::atlassian::client_bb::BbClient;

use super::mcp_exec_bb::build_bb_path;
use super::mcp_helpers::extract_string;
use super::mcp_helpers_bb::{
	BbRoute, exec_bb_route, query_sep,
};

/// PR and extra routes
const ROUTES: &[BbRoute] = &[
	BbRoute {
		name: "bb_list_pull_requests",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_get_pull_request",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_get_pr_diff",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/diff",
		is_text: true,
		is_list: false,
	},
	BbRoute {
		name: "bb_get_pr_diffstat",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/diffstat",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_list_pr_comments",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/comments",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_get_pr_activity",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/activity",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_pr_commits",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/commits",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_pr_statuses",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/statuses",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_list_pr_tasks",
		path: "/repositories/{workspace}\
			/{repo_slug}/pullrequests\
			/{pull_request_id}/tasks",
		is_text: false,
		is_list: true,
	},
];

pub fn exec_bb_pr_tool(
	id: Value,
	name: &str,
	args: &Value,
	client: &BbClient,
) -> String {
	for route in ROUTES {
		if route.name == name {
			let mut path =
				build_bb_path(route.path, args);
			add_state_param(&mut path, args);
			return exec_bb_route(
				id, args, route, client, &path,
			);
		}
	}
	// Delegate to pipeline/infra routes
	super::mcp_exec_bb_pipe::exec_bb_pipe_tool(
		id, name, args, client,
	)
}

/// Append state query param for list PRs
fn add_state_param(
	path: &mut String,
	args: &Value,
) {
	if let Some(state) =
		extract_string(args, "state")
	{
		path.push(query_sep(path));
		path.push_str(&format!("state={state}"));
	}
}
