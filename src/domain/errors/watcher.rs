#[derive(Debug, thiserror::Error)]
pub enum WatcherError {
	#[error("{0}")]
	Notify(String),

	#[error("channel receive: {0}")]
	Receive(
		#[from] std::sync::mpsc::RecvError,
	),

	#[error("channel receive timeout")]
	Timeout,
}

pub type WatcherResult<T> =
	Result<T, WatcherError>;
