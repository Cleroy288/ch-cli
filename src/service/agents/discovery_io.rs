use std::io::{BufRead, Write};
use std::time::{Duration, Instant};

use crate::domain::errors::agent::{
	AgentError, AgentResult,
};

/// Timeout for entire discovery per server
const DISCOVERY_TIMEOUT: Duration =
	Duration::from_secs(5);

pub(super) fn send_initialize(
	stdin: &mut impl Write,
) -> AgentResult<()> {
	let msg = concat!(
		r#"{"jsonrpc":"2.0","id":1,"method":"#,
		r#""initialize","params":{"#,
		r#""protocolVersion":"2024-11-05","#,
		r#""capabilities":{},"#,
		r#""clientInfo":{"name":"rustean","#,
		r#""version":"1.0"}}}"#,
		"\n"
	);
	stdin.write_all(msg.as_bytes()).map_err(
		|err| AgentError::Io(err.to_string()),
	)
}

pub(super) fn send_initialized(
	stdin: &mut impl Write,
) -> AgentResult<()> {
	let msg = concat!(
		r#"{"jsonrpc":"2.0","method":"#,
		r#""notifications/initialized"}"#,
		"\n"
	);
	stdin.write_all(msg.as_bytes()).map_err(
		|err| AgentError::Io(err.to_string()),
	)
}

pub(super) fn send_tools_list(
	stdin: &mut impl Write,
) -> AgentResult<()> {
	let msg = concat!(
		r#"{"jsonrpc":"2.0","id":2,"#,
		r#""method":"tools/list"}"#,
		"\n"
	);
	stdin.write_all(msg.as_bytes()).map_err(
		|err| AgentError::Io(err.to_string()),
	)
}

pub(super) fn read_response(
	reader: &mut impl BufRead,
	start: Instant,
) -> AgentResult<String> {
	if start.elapsed() > DISCOVERY_TIMEOUT {
		return Err(AgentError::Timeout);
	}
	let mut line = String::new();
	let n = reader.read_line(&mut line).map_err(
		|err| AgentError::Io(err.to_string()),
	)?;
	if n == 0 {
		return Err(AgentError::Protocol(
			"server closed stdout".into(),
		));
	}
	Ok(line)
}
