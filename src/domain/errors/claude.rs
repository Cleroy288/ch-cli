//! Error types for Claude CLI interactions.

/// Errors from Claude Code CLI operations
#[derive(Debug, thiserror::Error)]
pub enum ClaudeError {
	#[error("Claude CLI not installed on PATH")]
	NotInstalled,

	#[error("Claude process failed: {0}")]
	ProcessFailed(String),

	#[error("Invalid JSON response: {0}")]
	InvalidJson(String),

	#[error("Claude API error: {0}")]
	ApiError(String),
}

/// Convenience alias for Claude results
pub type ClaudeResult<T> = Result<T, ClaudeError>;
