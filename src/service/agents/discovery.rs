use std::io::BufReader;
use std::process::{Command, Stdio};
use std::time::Instant;

use crate::domain::errors::agent::{
	AgentError, AgentResult,
};
use crate::domain::mcp_config::McpServerConfig;

use super::discovery_io::{
	read_response, send_initialize,
	send_initialized, send_tools_list,
};
use super::discovery_types::{
	McpToolInfo, ToolsListResponse, to_tool_infos,
};

/// Discover tools from a stdio MCP server
pub fn discover_tools_stdio(
	config: &McpServerConfig,
) -> AgentResult<Vec<McpToolInfo>> {
	let cmd = config.command.as_deref()
		.ok_or(AgentError::Protocol(
			"no command".into(),
		))?;
	let start = Instant::now();
	let mut child =
		spawn_child(cmd, &config.args)?;
	let result =
		run_handshake(&mut child, start);
	let _ = child.kill();
	result
}

/// Spawn the child process with piped I/O
fn spawn_child(
	cmd: &str,
	args: &[String],
) -> AgentResult<std::process::Child> {
	Command::new(cmd)
		.args(args)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.stderr(Stdio::null())
		.spawn()
		.map_err(|err| {
			AgentError::Spawn(err.to_string())
		})
}

fn run_handshake(
	child: &mut std::process::Child,
	start: Instant,
) -> AgentResult<Vec<McpToolInfo>> {
	let stdin = child.stdin.as_mut()
		.ok_or(AgentError::Protocol(
			"no stdin".into(),
		))?;
	let stdout = child.stdout.take()
		.ok_or(AgentError::Protocol(
			"no stdout".into(),
		))?;
	let mut reader = BufReader::new(stdout);

	send_initialize(stdin)?;
	read_response(&mut reader, start)?;
	send_initialized(stdin)?;
	send_tools_list(stdin)?;
	parse_tools_response(&mut reader, start)
}

fn parse_tools_response(
	reader: &mut impl std::io::BufRead,
	start: Instant,
) -> AgentResult<Vec<McpToolInfo>> {
	let line = read_response(reader, start)?;
	let resp: ToolsListResponse =
		serde_json::from_str(&line)
			.map_err(|err| {
				AgentError::Json(err.to_string())
			})?;
	let result = resp.result.ok_or(
		AgentError::Protocol(
			"no result in response".into(),
		),
	)?;
	Ok(to_tool_infos(result.tools))
}
