use serde_json::Value;

use super::mcp_defs_shared::{
	ParamDef, ToolDef, tool_to_json,
};

/// Workspace parameter
const P_WS: ParamDef = ParamDef {
	name: "workspace",
	param_type: "string",
	description: "Workspace slug or UUID",
};

/// Repo slug parameter
const P_REPO: ParamDef = ParamDef {
	name: "repo_slug",
	param_type: "string",
	description: "Repository slug",
};

/// PR ID parameter
const P_PR: ParamDef = ParamDef {
	name: "pull_request_id",
	param_type: "number",
	description: "Pull request ID",
};

/// Page parameter
const P_PAGE: ParamDef = ParamDef {
	name: "page",
	param_type: "number",
	description: "Page number (starts at 1)",
};

/// PR activity, commits, statuses, tasks
const PR_EXTRA_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_get_pr_activity",
		description: "Bitbucket: get PR activity log",
		params: &[P_WS, P_REPO, P_PR, P_PAGE],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
	ToolDef {
		name: "bb_list_pr_commits",
		description: "Bitbucket: list PR commits",
		params: &[P_WS, P_REPO, P_PR, P_PAGE],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
	ToolDef {
		name: "bb_list_pr_statuses",
		description:
			"Bitbucket: list statuses on a PR",
		params: &[P_WS, P_REPO, P_PR],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
	ToolDef {
		name: "bb_list_pr_tasks",
		description: "Bitbucket: list PR tasks",
		params: &[P_WS, P_REPO, P_PR, P_PAGE],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
];

/// Commit, source, pipeline, environment tools
const INFRA_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_list_commits",
		description: "Bitbucket: list repo commits",
		params: &[
			P_WS,
			P_REPO,
			P_PAGE,
			ParamDef {
				name: "branch",
				param_type: "string",
				description: "Branch to filter",
			},
		],
		required: &["workspace", "repo_slug"],
	},
	ToolDef {
		name: "bb_get_commit",
		description: "Bitbucket: get commit details",
		params: &[
			P_WS,
			P_REPO,
			ParamDef {
				name: "commit_hash",
				param_type: "string",
				description: "Commit hash",
			},
		],
		required: &[
			"workspace",
			"repo_slug",
			"commit_hash",
		],
	},
	ToolDef {
		name: "bb_get_source",
		description:
			"Bitbucket: browse source at revision",
		params: &[
			P_WS,
			P_REPO,
			ParamDef {
				name: "revision",
				param_type: "string",
				description:
					"Commit hash, branch, or tag",
			},
			ParamDef {
				name: "path",
				param_type: "string",
				description: "File path",
			},
		],
		required: &[
			"workspace",
			"repo_slug",
			"revision",
			"path",
		],
	},
];

pub fn bb_extra_tool_defs() -> Vec<Value> {
	let mut defs = Vec::with_capacity(7);
	for def in PR_EXTRA_TOOLS {
		defs.push(tool_to_json(def));
	}
	for def in INFRA_TOOLS {
		defs.push(tool_to_json(def));
	}
	defs
}
