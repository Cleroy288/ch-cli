//! Error types for watcher operations.

/// Error type for watcher operations
#[derive(Debug, thiserror::Error)]
pub enum WatcherError {
	#[error("Failed to create watcher: {0}")]
	NotifyError(#[from] notify::Error),

	#[error("Channel receive error: {0}")]
	ReceiveError(#[from] std::sync::mpsc::RecvError),

	#[error("Channel receive timeout")]
	Timeout,
}

/// Result type alias for watcher operations
pub type WatcherResult<T> =
	std::result::Result<T, WatcherError>;
