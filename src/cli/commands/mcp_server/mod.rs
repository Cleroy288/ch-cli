//! MCP stdio server for Claude Code integration.
//!
//! Reads newline-delimited JSON-RPC from stdin,
//! routes to handlers, writes responses to stdout.

use std::io::{self, BufRead, Write};

use crate::domain::errors::command::CommandResult;

pub mod mcp_format;
pub mod mcp_handlers;
pub mod mcp_tool_defs;
pub mod mcp_tools;
pub mod mcp_types;

/// Run the MCP stdio server loop
pub fn mcp_server_command() -> CommandResult {
	let stdin = io::stdin().lock();
	let mut stdout = io::stdout().lock();

	for line in stdin.lines() {
		let line = line?;
		if line.trim().is_empty() {
			continue;
		}
		if let Some(resp) =
			mcp_handlers::handle_message(&line)
		{
			writeln!(stdout, "{resp}")?;
			stdout.flush()?;
		}
	}
	Ok(())
}
