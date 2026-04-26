use std::process::{Command, Output};

use crate::domain::errors::ClaudeError;

use super::cli_args::{
	self, CLAUDE_BIN, FLAG_EFFORT, FLAG_MODEL,
	FLAG_OUTPUT_FMT, FLAG_PRINT, FLAG_RESUME,
	FLAG_SKIP_PERMS, FMT_JSON,
};

/// Spawns `claude -p "prompt" --output-format json`
/// with `--resume <id>` if a session exists,
/// and `--model <alias>`.
pub fn execute_claude_cli(
	prompt: &str,
	session_id: Option<&str>,
	model: &str,
	effort: &str,
) -> Result<String, ClaudeError> {
	let out = spawn_process(
		prompt, session_id, model, effort,
	)?;
	check_exit_status(&out)?;
	into_stdout(out)
}

fn spawn_process(
	prompt: &str,
	session_id: Option<&str>,
	model: &str,
	effort: &str,
) -> Result<Output, ClaudeError> {
	let mut cmd = Command::new(CLAUDE_BIN);
	cmd.arg(FLAG_PRINT)
		.arg(prompt)
		.arg(FLAG_OUTPUT_FMT)
		.arg(FMT_JSON)
		.arg(FLAG_SKIP_PERMS)
		.arg(FLAG_MODEL)
		.arg(model)
		.arg(FLAG_EFFORT)
		.arg(effort);

	if let Some(id) = session_id {
		cmd.arg(FLAG_RESUME).arg(id);
	}

	cmd.output().map_err(cli_args::map_io_error)
}

fn into_stdout(
	output: Output,
) -> Result<String, ClaudeError> {
	String::from_utf8(output.stdout).map_err(|e| {
		ClaudeError::Process(e.to_string())
	})
}

fn check_exit_status(
	output: &Output,
) -> Result<(), ClaudeError> {
	if !output.status.success() {
		let stderr =
			String::from_utf8_lossy(&output.stderr);
		return Err(ClaudeError::Process(
			stderr.to_string(),
		));
	}
	Ok(())
}
