use std::process::Child;

use crate::domain::backend::BackendResponse;
use crate::domain::backend_kind::BackendKind;
use crate::domain::claude::StreamChunk;
use crate::domain::errors::{
	BackendError, BackendResult,
};

use super::traits::CliBackend;

const NOT_YET: &str =
	"Gemini CLI backend not yet implemented";

/// Gemini CLI backend (stub).
///
/// Infrastructure is in place — detection, config
/// persistence, `/backend gemini` command — but the
/// actual CLI interaction (flags, parsing) is not
/// implemented yet. Every method returns an error.
pub struct GeminiBackend;

impl CliBackend for GeminiBackend {
	fn name(&self) -> &str {
		"gemini"
	}

	fn kind(&self) -> BackendKind {
		BackendKind::GeminiCli
	}

	fn valid_models(&self) -> &[&str] {
		&[]
	}

	fn default_model(&self) -> &str {
		""
	}

	fn supports_sessions(&self) -> bool {
		false
	}

	fn supports_effort(&self) -> bool {
		false
	}

	fn valid_efforts(
		&self,
		_model: &str,
	) -> &[&str] {
		&[]
	}

	fn default_effort_for_model(
		&self,
		_model: &str,
	) -> &str {
		""
	}

	fn context_window(
		&self,
		_model: &str,
	) -> u64 {
		0
	}

	fn spawn_streaming(
		&self,
		_prompt: &str,
		_session_id: Option<&str>,
		_model: &str,
		_effort: &str,
		_system_prompt: Option<&str>,
	) -> BackendResult<Child> {
		Err(BackendError::Process(
			NOT_YET.to_string(),
		))
	}

	fn execute(
		&self,
		_prompt: &str,
		_session_id: Option<&str>,
		_model: &str,
		_effort: &str,
		_system_prompt: Option<&str>,
	) -> BackendResult<String> {
		Err(BackendError::Process(
			NOT_YET.to_string(),
		))
	}

	fn parse_stream_line(
		&self,
		_line: &str,
	) -> Vec<StreamChunk> {
		Vec::new()
	}

	fn parse_response(
		&self,
		_json: &str,
	) -> BackendResult<BackendResponse> {
		Err(BackendError::Process(
			NOT_YET.to_string(),
		))
	}
}
