use std::process::{Child, Command, Stdio};

use crate::domain::errors::ClaudeError;

use super::cli_args::{
	self, CLAUDE_BIN, FLAG_APPEND_SYS, FLAG_EFFORT,
	FLAG_MODEL, FLAG_OUTPUT_FMT, FLAG_PARTIAL,
	FLAG_PRINT, FLAG_RESUME, FLAG_SKIP_PERMS,
	FLAG_VERBOSE, FMT_STREAM,
};

/// Spawn Claude CLI with stream-json output.
/// Accepts an optional system prompt appended to
/// the default system prompt via
/// `--append-system-prompt`.
pub fn spawn_streaming(
	prompt: &str,
	session_id: Option<&str>,
	model: &str,
	effort: &str,
	system_prompt: Option<&str>,
) -> Result<Child, ClaudeError> {
	let mut cmd =
		build_base_cmd(prompt, model, effort);
	append_system(&mut cmd, system_prompt);
	append_resume(&mut cmd, session_id);
	cmd.stdout(Stdio::piped())
		.stderr(Stdio::piped())
		.spawn()
		.map_err(cli_args::map_io_error)
}

fn build_base_cmd(
	prompt: &str,
	model: &str,
	effort: &str,
) -> Command {
	let mut cmd = Command::new(CLAUDE_BIN);
	cmd.arg(FLAG_PRINT)
		.arg(prompt)
		.arg(FLAG_OUTPUT_FMT)
		.arg(FMT_STREAM)
		.arg(FLAG_VERBOSE)
		.arg(FLAG_PARTIAL)
		.arg(FLAG_SKIP_PERMS)
		.arg(FLAG_MODEL)
		.arg(model)
		.arg(FLAG_EFFORT)
		.arg(effort);
	cmd
}

/// Append system prompt if provided
fn append_system(
	cmd: &mut Command,
	system_prompt: Option<&str>,
) {
	if let Some(prompt) = system_prompt {
		cmd.arg(FLAG_APPEND_SYS).arg(prompt);
	}
}

/// Append --resume flag if session_id is present
fn append_resume(
	cmd: &mut Command,
	session_id: Option<&str>,
) {
	if let Some(id) = session_id {
		cmd.arg(FLAG_RESUME).arg(id);
	}
}
