//! File system module
//!
//! Provides functionality for scanning directories and
//! representing filesystem entries.
//!
//! # Modules
//! - `entry`: FsEntry struct for file or directory
//! - `scanner`: FileScanner for recursive scanning

pub mod entry;
pub mod file_cache;
pub mod scanner;
#[doc(hidden)]
pub mod scanner_filters;

// Re-export commonly used types
pub use entry::FsEntry;
pub use file_cache::FileCache;
pub use scanner::FileScanner;
