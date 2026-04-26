use serde_json::Value;

use crate::service::atlassian::client_bb::BbClient;

use super::mcp_helpers::extract_string;
use super::mcp_helpers_bb::{
	BbRoute, exec_bb_route, query_sep,
};

/// All Bitbucket API routes
const ROUTES: &[BbRoute] = &[
	BbRoute {
		name: "bb_current_user",
		path: "/user",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_list_workspaces",
		path: "/workspaces",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_workspace_members",
		path: "/workspaces/{workspace}/members",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_workspace_projects",
		path: "/workspaces/{workspace}/projects",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_repositories",
		path: "/repositories/{workspace}",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_get_repository",
		path: "/repositories/{workspace}\
			/{repo_slug}",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_list_default_reviewers",
		path: "/repositories/{workspace}\
			/{repo_slug}/default-reviewers",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_branches",
		path: "/repositories/{workspace}\
			/{repo_slug}/refs/branches",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_tags",
		path: "/repositories/{workspace}\
			/{repo_slug}/refs/tags",
		is_text: false,
		is_list: true,
	},
];

pub fn exec_bb_tool(
	id: Value,
	name: &str,
	args: &Value,
	client: &BbClient,
) -> String {
	for route in ROUTES {
		if route.name == name {
			let path =
				build_bb_path(route.path, args);
			return exec_bb_route(
				id, args, route, client, &path,
			);
		}
	}
	// Delegate to PR/extra routes
	super::mcp_exec_bb_pr::exec_bb_pr_tool(
		id, name, args, client,
	)
}

/// Substitute placeholders in path template
pub fn build_bb_path(
	template: &str,
	args: &Value,
) -> String {
	let mut path = template.to_string();
	let params = [
		"workspace",
		"repo_slug",
		"pull_request_id",
		"commit_hash",
		"pipeline_uuid",
		"revision",
	];
	for key in &params {
		let placeholder =
			format!("{{{key}}}");
		if let Some(val) =
			extract_string(args, key)
		{
			path = path.replace(
				&placeholder, &val,
			);
		}
	}
	append_page_param(&mut path, args);
	path
}

/// Append pagination params (?pagelen=100&page=N)
fn append_page_param(
	path: &mut String,
	args: &Value,
) {
	path.push(query_sep(path));
	path.push_str("pagelen=100");
	if let Some(page) =
		args.get("page").and_then(Value::as_u64)
	{
		path.push_str(&format!("&page={page}"));
	}
}
