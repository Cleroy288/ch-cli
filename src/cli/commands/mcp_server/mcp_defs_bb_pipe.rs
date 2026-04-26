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

/// Page parameter
const P_PAGE: ParamDef = ParamDef {
	name: "page",
	param_type: "number",
	description: "Page number (starts at 1)",
};

/// Pipeline UUID parameter
const P_PIPE_UUID: ParamDef = ParamDef {
	name: "pipeline_uuid",
	param_type: "string",
	description: "Pipeline UUID (with braces)",
};

/// Pipeline and infra tools
const PIPELINE_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "bb_list_pipelines",
		description: "Bitbucket: list pipelines",
		params: &[P_WS, P_REPO, P_PAGE],
		required: &["workspace", "repo_slug"],
	},
	ToolDef {
		name: "bb_get_pipeline",
		description:
			"Bitbucket: get pipeline details",
		params: &[P_WS, P_REPO, P_PIPE_UUID],
		required: &[
			"workspace",
			"repo_slug",
			"pipeline_uuid",
		],
	},
	ToolDef {
		name: "bb_list_pipeline_steps",
		description:
			"Bitbucket: list pipeline steps",
		params: &[P_WS, P_REPO, P_PIPE_UUID],
		required: &[
			"workspace",
			"repo_slug",
			"pipeline_uuid",
		],
	},
	ToolDef {
		name: "bb_list_environments",
		description:
			"Bitbucket: list deploy environments",
		params: &[P_WS, P_REPO],
		required: &["workspace", "repo_slug"],
	},
	ToolDef {
		name: "bb_list_downloads",
		description:
			"Bitbucket: list repo downloads",
		params: &[P_WS, P_REPO, P_PAGE],
		required: &["workspace", "repo_slug"],
	},
];

pub fn bb_pipe_tool_defs() -> Vec<Value> {
	PIPELINE_TOOLS
		.iter()
		.map(tool_to_json)
		.collect()
}
