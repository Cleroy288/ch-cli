//! File watcher for automatic re-indexing on file changes.
//!
//! This module provides file system watching capabilities using the `notify` crate,
//! with debouncing to avoid excessive re-indexing on rapid file changes.

use std::path::{Path, PathBuf};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

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

pub type WatcherResult<T> = std::result::Result<T, WatcherError>;

/// A file change event
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    /// The paths that changed
    pub paths: Vec<PathBuf>,
    /// The kind of change
    pub kind: ChangeKind,
}

/// The kind of file change
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChangeKind {
    /// File was created
    Created,
    /// File was modified
    Modified,
    /// File was deleted
    Deleted,
    /// File was renamed (from/to)
    Renamed,
    /// Unknown change
    Other,
}

impl From<&EventKind> for ChangeKind {
    fn from(kind: &EventKind) -> Self {
        match kind {
            EventKind::Create(_) => ChangeKind::Created,
            EventKind::Modify(_) => ChangeKind::Modified,
            EventKind::Remove(_) => ChangeKind::Deleted,
            _ => ChangeKind::Other,
        }
    }
}

/// File system watcher for code changes
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    receiver: Receiver<Result<Event, notify::Error>>,
    root: PathBuf,
}

impl FileWatcher {
    /// Create a new file watcher for the given root directory
    pub fn new<P: AsRef<Path>>(root: P) -> WatcherResult<Self> {
        let (tx, rx): (Sender<Result<Event, notify::Error>>, Receiver<_>) = channel();

        let config = Config::default()
            .with_poll_interval(Duration::from_secs(2));

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            config,
        )?;

        Ok(Self {
            watcher,
            receiver: rx,
            root: root.as_ref().to_path_buf(),
        })
    }

    /// Start watching the root directory
    pub fn start(&mut self) -> WatcherResult<()> {
        self.watcher
            .watch(&self.root, RecursiveMode::Recursive)?;
        Ok(())
    }

    /// Stop watching
    pub fn stop(&mut self) -> WatcherResult<()> {
        self.watcher.unwatch(&self.root)?;
        Ok(())
    }

    /// Check for file changes (non-blocking)
    pub fn poll(&self) -> Option<FileChangeEvent> {
        match self.receiver.try_recv() {
            Ok(Ok(event)) => {
                // Filter out non-Rust files
                let rust_paths: Vec<_> = event
                    .paths
                    .into_iter()
                    .filter(|p| p.extension().map_or(false, |e| e == "rs"))
                    .collect();

                if rust_paths.is_empty() {
                    None
                } else {
                    Some(FileChangeEvent {
                        paths: rust_paths,
                        kind: ChangeKind::from(&event.kind),
                    })
                }
            }
            _ => None,
        }
    }

    /// Wait for the next file change event with timeout
    pub fn wait(&self, timeout: Duration) -> WatcherResult<Option<FileChangeEvent>> {
        match self.receiver.recv_timeout(timeout) {
            Ok(Ok(event)) => {
                let rust_paths: Vec<_> = event
                    .paths
                    .into_iter()
                    .filter(|p| p.extension().map_or(false, |e| e == "rs"))
                    .collect();

                if rust_paths.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(FileChangeEvent {
                        paths: rust_paths,
                        kind: ChangeKind::from(&event.kind),
                    }))
                }
            }
            Ok(Err(e)) => Err(WatcherError::NotifyError(e)),
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                Err(WatcherError::ReceiveError(std::sync::mpsc::RecvError))
            }
        }
    }

    /// Get the root directory being watched
    pub fn root(&self) -> &Path {
        &self.root
    }
}

