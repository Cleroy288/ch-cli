use serde_json::Value;

use super::mcp_defs_shared::{
	ParamDef, ToolDef, tool_to_json,
};

/// Project key parameter
const P_PROJECT: ParamDef = ParamDef {
	name: "project_key",
	param_type: "string",
	description: "Project key (e.g. \"PROJ\")",
};

/// Max results parameter
const P_MAX: ParamDef = ParamDef {
	name: "max_results",
	param_type: "number",
	description: "Max results to return",
};

/// Start-at parameter
const P_START: ParamDef = ParamDef {
	name: "start_at",
	param_type: "number",
	description: "Page start index (0-based)",
};

/// Project tools
const PROJECT_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "jira_list_projects",
		description: "Jira: list projects",
		params: &[P_MAX, P_START],
		required: &[],
	},
	ToolDef {
		name: "jira_get_project",
		description: "Jira: get project details",
		params: &[P_PROJECT],
		required: &["project_key"],
	},
	ToolDef {
		name: "jira_list_components",
		description:
			"Jira: list project components",
		params: &[P_PROJECT],
		required: &["project_key"],
	},
	ToolDef {
		name: "jira_list_versions",
		description:
			"Jira: list project versions",
		params: &[P_PROJECT],
		required: &["project_key"],
	},
	ToolDef {
		name: "jira_get_watchers",
		description: "Jira: get issue watchers",
		params: &[ParamDef {
			name: "issue_key",
			param_type: "string",
			description:
				"Issue key (e.g. \"PROJ-123\")",
		}],
		required: &["issue_key"],
	},
];

/// Metadata tools (priorities, statuses, etc.)
const META_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "jira_list_priorities",
		description: "Jira: list priorities",
		params: &[P_MAX, P_START],
		required: &[],
	},
	ToolDef {
		name: "jira_list_statuses",
		description: "Jira: list statuses",
		params: &[P_MAX, P_START],
		required: &[],
	},
	ToolDef {
		name: "jira_list_issue_types",
		description: "Jira: list issue types",
		params: &[P_MAX, P_START],
		required: &[],
	},
	ToolDef {
		name: "jira_list_labels",
		description: "Jira: list labels",
		params: &[P_MAX, P_START],
		required: &[],
	},
];

pub fn jira_meta_tool_defs() -> Vec<Value> {
	let mut defs = Vec::with_capacity(9);
	for def in PROJECT_TOOLS {
		defs.push(tool_to_json(def));
	}
	for def in META_TOOLS {
		defs.push(tool_to_json(def));
	}
	defs
}
