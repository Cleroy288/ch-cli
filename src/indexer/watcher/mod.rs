mod error;
mod event;
mod file_watcher;
mod file_watcher_ops;
mod filter;

// Re-export all public types for backward compatibility
pub use error::{WatcherError, WatcherResult};
pub use event::{ChangeKind, FileChangeEvent};
pub use file_watcher::FileWatcher;
