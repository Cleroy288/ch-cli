//! Tests for Jira tool definitions.

use rustean::cli::commands::mcp_server::{
	mcp_defs_jira, mcp_defs_jira_meta,
};

/// Issue defs produce 5 tools
#[test]
fn jira_issue_defs_count() {
	// Act
	let defs =
		mcp_defs_jira::jira_issue_tool_defs();

	// Assert
	assert_eq!(defs.len(), 5);
}

/// Meta defs produce 9 tools
#[test]
fn jira_meta_defs_count() {
	// Act
	let defs =
		mcp_defs_jira_meta::jira_meta_tool_defs();

	// Assert
	assert_eq!(defs.len(), 9);
}

/// All Jira defs total 14
#[test]
fn jira_total_defs_count() {
	// Act
	let total =
		mcp_defs_jira::jira_issue_tool_defs().len()
		+ mcp_defs_jira_meta
			::jira_meta_tool_defs().len();

	// Assert
	assert_eq!(total, 14);
}

/// Each def has name and inputSchema
#[test]
fn jira_defs_have_required_fields() {
	// Act
	let defs =
		mcp_defs_jira::jira_issue_tool_defs();

	// Assert
	for def in &defs {
		assert!(def.get("name").is_some());
		assert!(
			def.get("inputSchema").is_some(),
		);
	}
}
