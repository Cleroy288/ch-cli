use std::process::Child;

use crate::domain::backend::BackendResponse;
use crate::domain::backend_kind::BackendKind;
use crate::domain::claude::StreamChunk;
use crate::domain::errors::BackendResult;

/// Abstraction over CLI-based AI backends.
///
/// Each provider (Claude, Gemini, ...) implements
/// this trait. Consumers call spawn/execute/parse
/// without knowing which binary is behind it.
pub trait CliBackend: Send + Sync {
	fn name(&self) -> &str;
	fn kind(&self) -> BackendKind;
	fn valid_models(&self) -> &[&str];
	fn default_model(&self) -> &str;
	fn supports_sessions(&self) -> bool;
	fn supports_effort(&self) -> bool;

	/// Valid effort levels for a given model.
	/// Empty slice = effort not supported.
	fn valid_efforts(
		&self,
		model: &str,
	) -> &[&str];

	/// Default effort level when switching to model.
	fn default_effort_for_model(
		&self,
		model: &str,
	) -> &str;

	/// Context window size in tokens for a model.
	/// Returns 0 if unknown.
	fn context_window(
		&self,
		model: &str,
	) -> u64;

	/// Spawn streaming child process.
	fn spawn_streaming(
		&self,
		prompt: &str,
		session_id: Option<&str>,
		model: &str,
		effort: &str,
		system_prompt: Option<&str>,
	) -> BackendResult<Child>;

	/// Blocking execution, returns raw JSON text.
	fn execute(
		&self,
		prompt: &str,
		session_id: Option<&str>,
		model: &str,
		effort: &str,
		system_prompt: Option<&str>,
	) -> BackendResult<String>;

	/// Parse one line from stream-json output.
	fn parse_stream_line(
		&self,
		line: &str,
	) -> Vec<StreamChunk>;

	/// Parse a full JSON response.
	fn parse_response(
		&self,
		json: &str,
	) -> BackendResult<BackendResponse>;
}
