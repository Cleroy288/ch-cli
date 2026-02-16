//! File watcher operations

use std::time::Duration;

use crate::indexer::watcher::error::{WatcherError, WatcherResult};
use crate::indexer::watcher::event::FileChangeEvent;
use crate::indexer::watcher::filter::filter_rust_paths;

use super::file_watcher::FileWatcher;

impl FileWatcher {
    /// Check for file changes (non-blocking)
    pub fn poll(&self) -> Option<FileChangeEvent> {
        match self.receiver.try_recv() {
            Ok(Ok(event)) => filter_rust_paths(event),
            _ => None,
        }
    }

    /// Wait for the next file change event with timeout
    pub fn wait(
        &self,
        timeout: Duration,
    ) -> WatcherResult<Option<FileChangeEvent>> {
        match self.receiver.recv_timeout(timeout) {
            Ok(Ok(event)) => Ok(filter_rust_paths(event)),
            Ok(Err(err)) => {
                Err(WatcherError::NotifyError(err))
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                Ok(None)
            }
            Err(
                std::sync::mpsc::RecvTimeoutError::Disconnected,
            ) => {
                let err = std::sync::mpsc::RecvError;
                Err(WatcherError::ReceiveError(err))
            }
        }
    }
}
