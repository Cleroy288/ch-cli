//! Register rustean MCP server with Claude Code.
//!
//! Checks `.mcp.json` for a valid `rustean-memory`
//! entry pointing to the current binary. Runs
//! `claude mcp add` if missing or stale.

use std::path::Path;
use std::process::Command;

/// MCP config file at project root
const MCP_CONFIG: &str = ".mcp.json";
/// Server name registered with Claude Code
const SERVER_NAME: &str = "rustean-memory";
/// Claude CLI binary name
const CLAUDE_BIN: &str = "claude";
/// Subcommand for MCP server mode
const MCP_ARG: &str = "mcp-server";

/// Ensure rustean MCP server is registered.
///
/// Reads `.mcp.json`, checks if `rustean-memory`
/// points to the current binary. If not, runs
/// `claude mcp add --scope project` to register.
pub fn ensure_mcp_registered() {
	let exe = match std::env::current_exe() {
		Ok(path) => path,
		Err(_) => return, // can't detect binary path
	};
	let exe_str = exe.to_string_lossy();

	if is_registered(&exe_str) {
		return;
	}

	register_mcp_server(&exe_str);
}

/// Check if `.mcp.json` contains our server entry
fn is_registered(exe_path: &str) -> bool {
	let config = Path::new(MCP_CONFIG);
	if !config.exists() {
		return false;
	}

	let content = match std::fs::read_to_string(config)
	{
		Ok(text) => text,
		Err(_) => return false,
	};

	has_valid_entry(&content, exe_path)
}

/// Parse JSON and check for matching server entry
pub fn has_valid_entry(
	content: &str,
	exe_path: &str,
) -> bool {
	let parsed: serde_json::Value =
		match serde_json::from_str(content) {
			Ok(val) => val,
			Err(_) => return false,
		};

	let servers = &parsed["mcpServers"][SERVER_NAME];
	servers["command"].as_str() == Some(exe_path)
}

/// Run `claude mcp add` to register the server
fn register_mcp_server(exe_path: &str) {
	let result = Command::new(CLAUDE_BIN)
		.args(["mcp", "add"])
		.args(["--transport", "stdio"])
		.args(["--scope", "project"])
		.arg(SERVER_NAME)
		.arg("--")
		.arg(exe_path)
		.arg(MCP_ARG)
		.output();

	if let Err(err) = result {
		eprintln!(
			"Warning: could not register MCP \
			 server: {err}"
		);
	}
}
