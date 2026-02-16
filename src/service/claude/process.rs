//! Execute the Claude Code CLI as a child process.

use std::process::{Command, Output};

use crate::domain::errors::ClaudeError;

/// CLI binary name on PATH
const CLAUDE_BIN: &str = "claude";
/// Flag for non-interactive prompt mode
const FLAG_PRINT: &str = "-p";
/// Flag to request JSON output
const FLAG_OUTPUT_FMT: &str = "--output-format";
/// JSON output format value
const FMT_JSON: &str = "json";
/// Flag to continue the most recent session
const FLAG_CONTINUE: &str = "--continue";
/// Flag to bypass all permission checks
const FLAG_SKIP_PERMS: &str =
	"--dangerously-skip-permissions";

/// Run Claude CLI and return raw stdout.
///
/// Spawns `claude -p "prompt" --output-format json`
/// with pre-approved tools and optional `--continue`.
pub fn execute_claude_cli(
	prompt: &str,
	continue_session: bool,
) -> Result<String, ClaudeError> {
	let output =
		spawn_process(prompt, continue_session)?;
	check_exit_status(&output)?;
	parse_stdout(&output)
}

/// Build and run the claude command
fn spawn_process(
	prompt: &str,
	continue_session: bool,
) -> Result<Output, ClaudeError> {
	let mut cmd = Command::new(CLAUDE_BIN);
	cmd.arg(FLAG_PRINT)
		.arg(prompt)
		.arg(FLAG_OUTPUT_FMT)
		.arg(FMT_JSON)
		.arg(FLAG_SKIP_PERMS);

	if continue_session {
		cmd.arg(FLAG_CONTINUE);
	}

	cmd.output().map_err(map_io_error)
}

/// Extract stdout as a UTF-8 string
fn parse_stdout(
	output: &Output,
) -> Result<String, ClaudeError> {
	String::from_utf8(output.stdout.clone())
		.map_err(|err| {
			ClaudeError::ProcessFailed(
				err.to_string(),
			)
		})
}

/// Map IO error to ClaudeError
fn map_io_error(err: std::io::Error) -> ClaudeError {
	if err.kind() == std::io::ErrorKind::NotFound {
		ClaudeError::NotInstalled
	} else {
		ClaudeError::ProcessFailed(err.to_string())
	}
}

/// Check process exit status
fn check_exit_status(
	output: &Output,
) -> Result<(), ClaudeError> {
	if !output.status.success() {
		let stderr =
			String::from_utf8_lossy(&output.stderr);
		return Err(ClaudeError::ProcessFailed(
			stderr.to_string(),
		));
	}
	Ok(())
}
