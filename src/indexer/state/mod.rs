//! Index state management for incremental parsing and persistence.
//!
//! This module provides:
//! - FileState: Track individual file metadata (mtime, hash)
//! - IndexState: Manage the overall index state
//! - Persistence: Save/load index state to disk
//! - Change detection: Detect added, modified, deleted files

mod change_detection;
mod file_state;
mod paths;
mod persistence;
mod types;
mod types_default;

/// The directory name for storing index data
pub const INDEX_DIR_NAME: &str = ".ch-index";

/// Current index format version
pub const INDEX_VERSION: u32 = 1;

// Re-export all public types for backward compatibility
pub use types::ChangeSet;
pub use types::FileChange;
pub use types::FileState;
pub use types::IndexState;
