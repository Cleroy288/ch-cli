use serde_json::Value;

use crate::service::atlassian
	::client_jira::JiraClient;

use super::mcp_exec_jira_path::{
	append_jira_params, build_jira_path,
	urlencoded,
};
use super::mcp_helpers::{
	extract_string, wrap_text_content,
};
use super::mcp_types::error_response;

/// Route entry for a Jira API endpoint
struct JiraRoute {
	/// Tool name
	name: &'static str,
	/// URL path template with placeholders
	path: &'static str,
}

/// All Jira API routes
const ROUTES: &[JiraRoute] = &[
	JiraRoute {
		name: "jira_get_issue",
		path: "/issue/{issue_key}",
	},
	JiraRoute {
		name: "jira_list_comments",
		path: "/issue/{issue_key}/comment",
	},
	JiraRoute {
		name: "jira_get_transitions",
		path: "/issue/{issue_key}/transitions",
	},
	JiraRoute {
		name: "jira_get_changelog",
		path: "/issue/{issue_key}/changelog",
	},
	JiraRoute {
		name: "jira_get_watchers",
		path: "/issue/{issue_key}/watchers",
	},
	JiraRoute {
		name: "jira_list_projects",
		path: "/project",
	},
	JiraRoute {
		name: "jira_get_project",
		path: "/project/{project_key}",
	},
	JiraRoute {
		name: "jira_list_components",
		path: "/project/{project_key}/components",
	},
	JiraRoute {
		name: "jira_list_versions",
		path: "/project/{project_key}/versions",
	},
	JiraRoute {
		name: "jira_list_priorities",
		path: "/priority",
	},
	JiraRoute {
		name: "jira_list_statuses",
		path: "/status",
	},
	JiraRoute {
		name: "jira_list_issue_types",
		path: "/issuetype",
	},
	JiraRoute {
		name: "jira_list_labels",
		path: "/label",
	},
];

pub fn exec_jira_tool(
	id: Value,
	name: &str,
	args: &Value,
	client: &JiraClient,
) -> String {
	// Special case: jira_search uses JQL
	if name == "jira_search" {
		return exec_search(id, args, client);
	}
	for route in ROUTES {
		if route.name == name {
			return exec_route(
				id, args, route, client,
			);
		}
	}
	error_response(
		id, -32602, "Unknown Jira tool",
	)
}

fn exec_route(
	id: Value,
	args: &Value,
	route: &JiraRoute,
	client: &JiraClient,
) -> String {
	let path = build_jira_path(route.path, args);
	match client.get(&path) {
		Ok(body) => wrap_text_content(id, &body),
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}

fn exec_search(
	id: Value,
	args: &Value,
	client: &JiraClient,
) -> String {
	let jql = extract_string(args, "jql")
		.unwrap_or_default();
	let encoded = urlencoded(&jql);
	let mut path =
		format!("/search/jql?jql={encoded}");
	append_jira_params(&mut path, args);
	match client.get(&path) {
		Ok(body) => wrap_text_content(id, &body),
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}
