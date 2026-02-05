//! File system module
//!
//! Provides functionality for scanning directories and representing filesystem entries.
//!
//! # Modules
//! - `entry`: FsEntry struct representing a file or directory
//! - `scanner`: FileScanner for recursive directory scanning

pub mod entry;
pub mod scanner;

// Re-export commonly used types
pub use entry::FsEntry;
pub use scanner::FileScanner;
