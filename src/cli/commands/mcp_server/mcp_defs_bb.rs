use serde_json::Value;

use super::mcp_defs_shared::{
	ParamDef, ToolDef, tool_to_json,
};

/// Workspace parameter
const P_WORKSPACE: ParamDef = ParamDef {
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

/// Page number parameter
const P_PAGE: ParamDef = ParamDef {
	name: "page",
	param_type: "number",
	description: "Page number (starts at 1)",
};

/// Static tool definitions for workspace tools
const WORKSPACE_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_current_user",
		description:
			"Bitbucket: get authenticated user",
		params: &[],
		required: &[],
	},
	ToolDef {
		name: "bb_list_workspaces",
		description:
			"Bitbucket: list accessible workspaces",
		params: &[P_PAGE],
		required: &[],
	},
	ToolDef {
		name: "bb_list_workspace_members",
		description:
			"Bitbucket: list workspace members",
		params: &[P_WORKSPACE, P_PAGE],
		required: &["workspace"],
	},
	ToolDef {
		name: "bb_list_workspace_projects",
		description:
			"Bitbucket: list workspace projects",
		params: &[P_WORKSPACE, P_PAGE],
		required: &["workspace"],
	},
];

/// Static tool definitions for repo tools
const REPO_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_list_repositories",
		description:
			"Bitbucket: list repos in workspace",
		params: &[P_WORKSPACE, P_PAGE],
		required: &["workspace"],
	},
	ToolDef {
		name: "bb_get_repository",
		description:
			"Bitbucket: get repository details",
		params: &[P_WORKSPACE, P_REPO],
		required: &["workspace", "repo_slug"],
	},
	ToolDef {
		name: "bb_list_default_reviewers",
		description:
			"Bitbucket: list default reviewers",
		params: &[P_WORKSPACE, P_REPO],
		required: &["workspace", "repo_slug"],
	},
];

/// Static tool definitions for ref tools
const REF_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_list_branches",
		description: "Bitbucket: list branches",
		params: &[P_WORKSPACE, P_REPO, P_PAGE],
		required: &["workspace", "repo_slug"],
	},
	ToolDef {
		name: "bb_list_tags",
		description: "Bitbucket: list tags",
		params: &[P_WORKSPACE, P_REPO, P_PAGE],
		required: &["workspace", "repo_slug"],
	},
];

pub fn bb_core_tool_defs() -> Vec<Value> {
	let mut defs = Vec::with_capacity(9);
	for def in WORKSPACE_TOOLS {
		defs.push(tool_to_json(def));
	}
	for def in REPO_TOOLS {
		defs.push(tool_to_json(def));
	}
	for def in REF_TOOLS {
		defs.push(tool_to_json(def));
	}
	defs
}
