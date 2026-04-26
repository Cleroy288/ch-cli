use super::search::SearchError;
use super::watcher::WatcherError;

#[derive(Debug, thiserror::Error)]
pub enum IndexError {
	#[error("{0}")]
	Io(#[from] std::io::Error),

	#[error("{0}")]
	Search(#[from] SearchError),

	#[error("{0}")]
	Watcher(#[from] WatcherError),

	#[error("parse failed: {0}")]
	Parser(String),
}

pub type IndexManagerResult<T> =
	Result<T, IndexError>;
