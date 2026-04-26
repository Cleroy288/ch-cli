use serde_json::Value;

use super::{
	mcp_defs_bb, mcp_defs_bb_extra,
	mcp_defs_bb_pipe, mcp_defs_bb_pr,
	mcp_defs_jira, mcp_defs_jira_meta,
	mcp_defs_nav,
};

/// Navigation + stats tool schemas (3)
pub(crate) fn nav_defs() -> Vec<Value> {
	vec![
		mcp_defs_nav::code_symbols_def(),
		mcp_defs_nav::code_info_def(),
		mcp_defs_nav::code_stats_def(),
	]
}

/// Bitbucket tool schemas (25)
pub(crate) fn bb_defs() -> Vec<Value> {
	let mut defs = Vec::with_capacity(25);
	defs.extend(
		mcp_defs_bb::bb_core_tool_defs(),
	);
	defs.extend(
		mcp_defs_bb_pr::bb_pr_tool_defs(),
	);
	defs.extend(
		mcp_defs_bb_extra::bb_extra_tool_defs(),
	);
	defs.extend(
		mcp_defs_bb_pipe::bb_pipe_tool_defs(),
	);
	defs
}

/// Jira tool schemas (14)
pub(crate) fn jira_defs() -> Vec<Value> {
	let mut defs = Vec::with_capacity(14);
	defs.extend(
		mcp_defs_jira::jira_issue_tool_defs(),
	);
	defs.extend(
		mcp_defs_jira_meta::jira_meta_tool_defs(),
	);
	defs
}
