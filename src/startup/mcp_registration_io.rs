use std::io::Write;
use std::path::Path;
use std::process::Command;

use super::mcp_registration::{
	CLAUDE_BIN, MCP_ARG, MCP_CONFIG,
	OLD_SERVER_NAME, SERVER_NAME,
};

pub(crate) fn read_mcp_config() -> String {
	let config = Path::new(MCP_CONFIG);
	if !config.exists() {
		return String::new();
	}
	std::fs::read_to_string(config)
		.unwrap_or_default()
}

pub(crate) fn remove_old_entry() {
	let _ = Command::new(CLAUDE_BIN)
		.args(["mcp", "remove"])
		.arg(OLD_SERVER_NAME)
		.args(["--scope", "project"])
		.output();
}

pub(crate) fn register_mcp_server(
	exe_path: &str,
) {
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
		let _ = writeln!(
			std::io::stderr(),
			"Warning: could not register MCP \
			 server: {err}"
		);
	}
}
