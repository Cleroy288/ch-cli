/// Errors from MCP agent discovery
#[derive(Debug, thiserror::Error)]
pub enum AgentError {
	#[error("spawn: {0}")]
	Spawn(String),

	#[error("I/O: {0}")]
	Io(String),

	#[error("discovery timeout")]
	Timeout,

	#[error("protocol: {0}")]
	Protocol(String),

	#[error("JSON: {0}")]
	Json(String),
}

pub type AgentResult<T> =
	Result<T, AgentError>;
