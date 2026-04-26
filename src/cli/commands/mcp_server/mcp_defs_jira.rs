use serde_json::Value;

use super::mcp_defs_shared::{
	ParamDef, ToolDef, tool_to_json,
};

/// Issue key parameter
const P_ISSUE: ParamDef = ParamDef {
	name: "issue_key",
	param_type: "string",
	description: "Issue key (e.g. \"PROJ-123\")",
};

/// Fields filter parameter
const P_FIELDS: ParamDef = ParamDef {
	name: "fields",
	param_type: "string",
	description:
		"Comma-separated fields to return",
};

/// Max results parameter
const P_MAX: ParamDef = ParamDef {
	name: "max_results",
	param_type: "number",
	description: "Max results to return",
};

/// Start-at parameter for pagination
const P_START: ParamDef = ParamDef {
	name: "start_at",
	param_type: "number",
	description: "Page start index (0-based)",
};

/// Issue-centric tools
const ISSUE_TOOLS: &[ToolDef] = &[
	ToolDef {
		name: "jira_search",
		description:
			"Jira: search issues with JQL",
		params: &[
			ParamDef {
				name: "jql",
				param_type: "string",
				description: "JQL query string",
			},
			P_FIELDS,
			P_MAX,
			P_START,
		],
		required: &["jql"],
	},
	ToolDef {
		name: "jira_get_issue",
		description: "Jira: get issue details",
		params: &[P_ISSUE, P_FIELDS],
		required: &["issue_key"],
	},
	ToolDef {
		name: "jira_list_comments",
		description: "Jira: list issue comments",
		params: &[P_ISSUE, P_MAX, P_START],
		required: &["issue_key"],
	},
	ToolDef {
		name: "jira_get_transitions",
		description: "Jira: get issue transitions",
		params: &[P_ISSUE],
		required: &["issue_key"],
	},
	ToolDef {
		name: "jira_get_changelog",
		description: "Jira: get issue changelog",
		params: &[P_ISSUE, P_MAX, P_START],
		required: &["issue_key"],
	},
];

pub fn jira_issue_tool_defs() -> Vec<Value> {
	ISSUE_TOOLS.iter().map(tool_to_json).collect()
}
