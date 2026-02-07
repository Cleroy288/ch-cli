//! File watcher for automatic re-indexing on file changes.
//!
//! This module provides file system watching capabilities using
//! the `notify` crate, with debouncing to avoid excessive
//! re-indexing on rapid file changes.

mod error;
mod event;
mod file_watcher;
mod file_watcher_ops;
mod filter;

// Re-export all public types for backward compatibility
pub use error::{WatcherError, WatcherResult};
pub use event::{ChangeKind, FileChangeEvent};
pub use file_watcher::FileWatcher;
