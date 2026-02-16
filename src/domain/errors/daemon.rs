//! Error types for daemon lifecycle operations.

/// Errors during daemon start/stop/communication
#[derive(Debug, thiserror::Error)]
pub enum DaemonError {
	#[error("Daemon not running")]
	NotRunning,

	#[error("Daemon already running (pid: {0})")]
	AlreadyRunning(u32),

	#[error("Failed to start daemon: {0}")]
	StartFailed(String),

	#[error("Failed to stop daemon: {0}")]
	StopFailed(String),

	#[error("Communication error: {0}")]
	Communication(String),

	#[error("IO error: {0}")]
	IoError(#[from] std::io::Error),

	#[error("Model error: {0}")]
	Model(#[from] super::model::ModelError),
}
