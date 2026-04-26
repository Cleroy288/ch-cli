//! Index Manager - High-level orchestration for semantic indexing.
//!
//! This module provides the main entry point for indexing entire projects,
//! coordinating the crawler, parser, search index, and semantic analysis.
//!
//! Supports both full and incremental indexing with persistent storage.

mod builder;
mod config;
mod error;
mod helpers;
mod incremental;
mod indexing;
mod indexing_fns;
mod indexing_helpers;
mod parsing;
mod parsing_rust;
mod query;
mod types;
mod watching;

pub use builder::{IndexManager, IndexManagerFlags};
pub use error::{IndexError, IndexManagerResult};
pub use types::{IndexResult, IndexStats, ProgressCallback, WatchCallback};
