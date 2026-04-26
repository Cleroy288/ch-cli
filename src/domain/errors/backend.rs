#[derive(Debug, thiserror::Error)]
pub enum BackendError {
	#[error("CLI binary not found on PATH")]
	NotInstalled,

	#[error("process error: {0}")]
	Process(String),

	#[error("invalid JSON response: {0}")]
	Json(String),

	#[error("API error: {0}")]
	Api(String),
}

pub type BackendResult<T> = Result<T, BackendError>;
