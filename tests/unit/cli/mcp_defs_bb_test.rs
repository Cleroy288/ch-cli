//! Tests for Bitbucket tool definitions.

use rustean::cli::commands::mcp_server::{
	mcp_defs_bb, mcp_defs_bb_extra,
	mcp_defs_bb_pipe, mcp_defs_bb_pr,
};

/// Core BB defs produce 9 tools
#[test]
fn bb_core_defs_count() {
	// Act
	let defs = mcp_defs_bb::bb_core_tool_defs();

	// Assert
	assert_eq!(defs.len(), 9);
}

/// PR defs produce 5 tools
#[test]
fn bb_pr_defs_count() {
	// Act
	let defs = mcp_defs_bb_pr::bb_pr_tool_defs();

	// Assert
	assert_eq!(defs.len(), 5);
}

/// Extra defs produce 7 tools
#[test]
fn bb_extra_defs_count() {
	// Act
	let defs =
		mcp_defs_bb_extra::bb_extra_tool_defs();

	// Assert
	assert_eq!(defs.len(), 7);
}

/// Pipe defs produce 5 tools
#[test]
fn bb_pipe_defs_count() {
	// Act
	let defs =
		mcp_defs_bb_pipe::bb_pipe_tool_defs();

	// Assert
	assert_eq!(defs.len(), 5);
}

/// All BB defs total 26 (9+5+7+5)
#[test]
fn bb_total_defs_count() {
	// Act
	let total =
		mcp_defs_bb::bb_core_tool_defs().len()
		+ mcp_defs_bb_pr::bb_pr_tool_defs().len()
		+ mcp_defs_bb_extra::bb_extra_tool_defs()
			.len()
		+ mcp_defs_bb_pipe::bb_pipe_tool_defs()
			.len();

	// Assert
	assert_eq!(total, 26);
}
