use std::sync::mpsc::{RecvError, RecvTimeoutError};
use std::time::Duration;

use crate::indexer::watcher::error::{
	WatcherError, WatcherResult,
};
use crate::indexer::watcher::event::FileChangeEvent;
use crate::indexer::watcher::filter::filter_supported_paths;

use super::file_watcher::FileWatcher;

impl FileWatcher {
	pub fn poll(&self) -> Option<FileChangeEvent> {
		match self.receiver.try_recv() {
			Ok(Ok(event)) => filter_supported_paths(event),
			_ => None,
		}
	}

	/// Wait for the next file change event with timeout
	pub fn wait(
		&self,
		timeout: Duration,
	) -> WatcherResult<Option<FileChangeEvent>> {
		match self.receiver.recv_timeout(timeout) {
			Ok(Ok(event)) => {
				Ok(filter_supported_paths(event))
			}
			Ok(Err(err)) => {
				Err(WatcherError::Notify(
				err.to_string(),
			))
			}
			Err(RecvTimeoutError::Timeout) => Ok(None),
			Err(RecvTimeoutError::Disconnected) => {
				Err(WatcherError::Receive(RecvError))
			}
		}
	}
}
