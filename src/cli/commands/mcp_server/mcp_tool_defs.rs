use serde_json::Value;

use super::mcp_defs_aikido;
use super::mcp_tool_defs_extra::{
	bb_defs, jira_defs, nav_defs,
};
use super::mcp_types::McpContext;
use super::{
	mcp_defs_code, mcp_defs_memory,
};

pub fn tool_definitions(
	ctx: &McpContext,
) -> Vec<Value> {
	let mut defs = Vec::with_capacity(54);
	defs.extend(memory_defs());
	defs.extend(code_defs());
	defs.extend(nav_defs());
	if ctx.bb_client.is_some() {
		defs.extend(bb_defs());
	}
	if ctx.jira_client.is_some() {
		defs.extend(jira_defs());
	}
	if ctx.aikido_client.is_some() {
		defs.extend(aikido_defs());
	}
	defs
}

/// Memory tool schemas (3)
fn memory_defs() -> Vec<Value> {
	vec![
		mcp_defs_memory::memory_search_def(),
		mcp_defs_memory::memory_recent_def(),
		mcp_defs_memory::memory_stats_def(),
	]
}

/// Code search tool schemas (4)
fn code_defs() -> Vec<Value> {
	vec![
		mcp_defs_code::code_search_def(),
		mcp_defs_code::code_goto_def(),
		mcp_defs_code::code_refs_def(),
		mcp_defs_code::code_callers_def(),
	]
}

/// Aikido Security tool schemas (5)
fn aikido_defs() -> Vec<Value> {
	vec![
		mcp_defs_aikido::aikido_get_issues_def(),
		mcp_defs_aikido::aikido_list_repos_def(),
		mcp_defs_aikido
			::aikido_list_containers_def(),
		mcp_defs_aikido
			::aikido_issue_counts_def(),
		mcp_defs_aikido::aikido_get_issue_def(),
	]
}
