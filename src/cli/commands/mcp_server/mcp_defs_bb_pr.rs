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

/// PR tools — list, get, diff, diffstat, etc.
const PR_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_list_pull_requests",
		description: "Bitbucket: list pull requests",
		params: &[
			P_WS,
			P_REPO,
			P_PAGE,
			ParamDef {
				name: "state",
				param_type: "string",
				description:
					"Filter: OPEN|MERGED|DECLINED",
			},
		],
		required: &["workspace", "repo_slug"],
	},
	ToolDef {
		name: "bb_get_pull_request",
		description: "Bitbucket: get a pull request",
		params: &[P_WS, P_REPO, P_PR],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
	ToolDef {
		name: "bb_get_pr_diff",
		description: "Bitbucket: get PR diff",
		params: &[P_WS, P_REPO, P_PR],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
	ToolDef {
		name: "bb_get_pr_diffstat",
		description:
			"Bitbucket: get PR diffstat summary",
		params: &[P_WS, P_REPO, P_PR],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
	ToolDef {
		name: "bb_list_pr_comments",
		description: "Bitbucket: list PR comments",
		params: &[P_WS, P_REPO, P_PR, P_PAGE],
		required: &[
			"workspace",
			"repo_slug",
			"pull_request_id",
		],
	},
];

pub fn bb_pr_tool_defs() -> Vec<Value> {
	PR_TOOLS.iter().map(tool_to_json).collect()
}
