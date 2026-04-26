use std::process::{Child, Command, Stdio};

use crate::domain::backend::BackendResponse;
use crate::domain::backend_kind::BackendKind;
use crate::domain::claude::StreamChunk;
use crate::domain::errors::{
	BackendError, BackendResult,
};

use super::claude_args::{
	self, CLAUDE_BIN, FLAG_APPEND_SYS,
	FLAG_EFFORT, FLAG_MODEL, FLAG_OUTPUT_FMT,
	FLAG_PARTIAL, FLAG_PRINT, FLAG_RESUME,
	FLAG_SKIP_PERMS, FLAG_VERBOSE, FMT_JSON,
	FMT_STREAM,
};
use super::claude_parse;
use super::traits::CliBackend;

const VALID_MODELS: &[&str] =
	&["haiku", "sonnet", "opus"];
const DEFAULT_MODEL: &str = "sonnet";

/// Effort levels supported by opus
const EFFORTS_OPUS: &[&str] =
	&["low", "medium", "high", "max"];
/// Effort levels supported by sonnet
const EFFORTS_SONNET: &[&str] =
	&["low", "medium", "high"];
/// Haiku does not support effort
const EFFORTS_NONE: &[&str] = &[];

/// Claude Code CLI backend.
pub struct ClaudeBackend;

impl CliBackend for ClaudeBackend {
	fn name(&self) -> &str {
		"claude"
	}

	fn kind(&self) -> BackendKind {
		BackendKind::ClaudeCode
	}

	fn valid_models(&self) -> &[&str] {
		VALID_MODELS
	}

	fn default_model(&self) -> &str {
		DEFAULT_MODEL
	}

	fn supports_sessions(&self) -> bool {
		true
	}

	fn supports_effort(&self) -> bool {
		true
	}

	fn valid_efforts(
		&self,
		model: &str,
	) -> &[&str] {
		match model {
			"opus" => EFFORTS_OPUS,
			"sonnet" => EFFORTS_SONNET,
			_ => EFFORTS_NONE,
		}
	}

	fn default_effort_for_model(
		&self,
		model: &str,
	) -> &str {
		match model {
			"opus" => "medium",
			"sonnet" => "high",
			_ => "high",
		}
	}

	fn context_window(
		&self,
		model: &str,
	) -> u64 {
		match model {
			"haiku" => 200_000,
			"sonnet" => 200_000,
			"opus" => 200_000,
			_ => 200_000,
		}
	}

	fn spawn_streaming(
		&self,
		prompt: &str,
		session_id: Option<&str>,
		model: &str,
		effort: &str,
		system_prompt: Option<&str>,
	) -> BackendResult<Child> {
		let mut cmd = build_stream_cmd(
			prompt, model, effort,
		);
		append_system(&mut cmd, system_prompt);
		append_resume(&mut cmd, session_id);
		cmd.stdout(Stdio::piped())
			.stderr(Stdio::piped())
			.spawn()
			.map_err(claude_args::map_io_error)
	}

	fn execute(
		&self,
		prompt: &str,
		session_id: Option<&str>,
		model: &str,
		effort: &str,
		system_prompt: Option<&str>,
	) -> BackendResult<String> {
		let out = spawn_json(
			prompt, session_id, model, effort,
			system_prompt,
		)?;
		check_exit(&out)?;
		into_stdout(out)
	}

	fn parse_stream_line(
		&self,
		line: &str,
	) -> Vec<StreamChunk> {
		claude_parse::parse_stream_line(line)
	}

	fn parse_response(
		&self,
		json: &str,
	) -> BackendResult<BackendResponse> {
		claude_parse::parse_response(json)
	}
}

// -- command builders --

fn build_stream_cmd(
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

fn spawn_json(
	prompt: &str,
	session_id: Option<&str>,
	model: &str,
	effort: &str,
	system_prompt: Option<&str>,
) -> BackendResult<std::process::Output> {
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
	append_system(&mut cmd, system_prompt);
	append_resume(&mut cmd, session_id);
	cmd.output().map_err(claude_args::map_io_error)
}

fn append_system(
	cmd: &mut Command,
	system_prompt: Option<&str>,
) {
	if let Some(prompt) = system_prompt {
		cmd.arg(FLAG_APPEND_SYS).arg(prompt);
	}
}

fn append_resume(
	cmd: &mut Command,
	session_id: Option<&str>,
) {
	if let Some(id) = session_id {
		cmd.arg(FLAG_RESUME).arg(id);
	}
}

fn check_exit(
	output: &std::process::Output,
) -> BackendResult<()> {
	if !output.status.success() {
		let stderr =
			String::from_utf8_lossy(&output.stderr);
		return Err(BackendError::Process(
			stderr.to_string(),
		));
	}
	Ok(())
}

fn into_stdout(
	output: std::process::Output,
) -> BackendResult<String> {
	String::from_utf8(output.stdout).map_err(|e| {
		BackendError::Process(e.to_string())
	})
}
