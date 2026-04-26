pub use crate::domain::errors::watcher::{
	WatcherError, WatcherResult,
};

impl From<notify::Error> for WatcherError {
	fn from(err: notify::Error) -> Self {
		Self::Notify(err.to_string())
	}
}
