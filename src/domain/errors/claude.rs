#[derive(Debug, thiserror::Error)]
pub enum ClaudeError {
	#[error("claude CLI not found on PATH")]
	NotInstalled,

	#[error("claude process: {0}")]
	Process(String),

	#[error("invalid JSON response: {0}")]
	Json(String),

	#[error("claude API: {0}")]
	Api(String),
}

pub type ClaudeResult<T> = Result<T, ClaudeError>;
