use serde_json::Value;

use crate::service::atlassian::client_bb::BbClient;

use super::mcp_exec_bb::build_bb_path;
use super::mcp_helpers::{
	extract_string, wrap_text_content,
};
use super::mcp_helpers_bb::{
	BbRoute, exec_bb_route, query_sep,
};
use super::mcp_types::error_response;

/// Commit, source, pipeline, infra routes
const ROUTES: &[BbRoute] = &[
	BbRoute {
		name: "bb_list_commits",
		path: "/repositories/{workspace}\
			/{repo_slug}/commits",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_get_commit",
		path: "/repositories/{workspace}\
			/{repo_slug}/commit/{commit_hash}",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_list_pipelines",
		path: "/repositories/{workspace}\
			/{repo_slug}/pipelines",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_get_pipeline",
		path: "/repositories/{workspace}\
			/{repo_slug}/pipelines\
			/{pipeline_uuid}",
		is_text: false,
		is_list: false,
	},
	BbRoute {
		name: "bb_list_pipeline_steps",
		path: "/repositories/{workspace}\
			/{repo_slug}/pipelines\
			/{pipeline_uuid}/steps",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_environments",
		path: "/repositories/{workspace}\
			/{repo_slug}/environments",
		is_text: false,
		is_list: true,
	},
	BbRoute {
		name: "bb_list_downloads",
		path: "/repositories/{workspace}\
			/{repo_slug}/downloads",
		is_text: false,
		is_list: true,
	},
];

pub fn exec_bb_pipe_tool(
	id: Value,
	name: &str,
	args: &Value,
	client: &BbClient,
) -> String {
	for route in ROUTES {
		if route.name == name {
			let mut path =
				build_bb_path(route.path, args);
			add_branch_param(&mut path, args);
			return exec_bb_route(
				id, args, route, client, &path,
			);
		}
	}
	// Special: bb_get_source has custom path
	if name == "bb_get_source" {
		return exec_source(id, args, client);
	}
	error_response(
		id, -32602, "Unknown Bitbucket tool",
	)
}

fn exec_source(
	id: Value,
	args: &Value,
	client: &BbClient,
) -> String {
	let workspace =
		extract_string(args, "workspace")
			.unwrap_or_default();
	let repo =
		extract_string(args, "repo_slug")
			.unwrap_or_default();
	let rev =
		extract_string(args, "revision")
			.unwrap_or_default();
	let file =
		extract_string(args, "path")
			.unwrap_or_default();
	let path = format!(
		"/repositories/{workspace}/{repo}\
		/src/{rev}/{file}",
	);
	match client.get(&path) {
		Ok(body) => wrap_text_content(id, &body),
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

/// Append branch param for bb_list_commits
fn add_branch_param(
	path: &mut String,
	args: &Value,
) {
	if let Some(branch) =
		extract_string(args, "branch")
	{
		path.push(query_sep(path));
		path.push_str(
			&format!("branch={branch}"),
		);
	}
}
