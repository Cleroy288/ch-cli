//! Type definitions for index state management.
//!
//! This module provides:
//! - FileState: Track individual file metadata (mtime, hash)
//! - FileChange: Enum for file change detection results
//! - IndexState: Manage the overall index state
//! - ChangeSet: Set of detected file changes

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::indexer::semantic::SymbolReference;
use crate::indexer::symbols::Symbol;

use super::INDEX_VERSION;

/// State of a single indexed file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileState {
	/// Path to the file (relative to project root)
	pub path: PathBuf,
	/// Last modification time (as seconds since UNIX epoch)
	pub mtime: u64,
	/// File size in bytes
	pub size: u64,
	/// Number of symbols extracted from this file
	pub symbol_count: usize,
}

/// Change detection result for a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChange {
	/// File is new (not in previous index)
	Added,
	/// File has been modified
	Modified,
	/// File has been deleted
	Deleted,
	/// File is unchanged
	Unchanged,
}

/// The complete index state for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexState {
	/// Version of the index format
	pub version: u32,
	/// Root directory of the indexed project
	pub root: PathBuf,
	/// When the index was last updated (seconds since UNIX epoch)
	pub last_updated: u64,
	/// State of each indexed file
	pub files: HashMap<PathBuf, FileState>,
	/// All symbols from all files
	#[serde(skip)]
	pub symbols: Vec<Symbol>,
	/// All references from all files (for cross-file usages)
	#[serde(skip)]
	pub references: Vec<SymbolReference>,
}

impl IndexState {
	/// Create a new empty index state
	pub fn new(root: PathBuf) -> Self {
		// canonical_root: the canonicalized root path
		let canonical_root = root.canonicalize().unwrap_or(root);

		Self {
			version: INDEX_VERSION,
			root: canonical_root,
			last_updated: SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.map(|d| d.as_secs())
				.unwrap_or(0),
			files: HashMap::new(),
			symbols: Vec::new(),
			references: Vec::new(),
		}
	}
}

/// Set of file changes detected
#[derive(Debug, Default, Clone)]
pub struct ChangeSet {
	/// Files that are new
	pub added: Vec<PathBuf>,
	/// Files that have been modified
	pub modified: Vec<PathBuf>,
	/// Files that have been deleted
	pub deleted: Vec<PathBuf>,
	/// Files that are unchanged
	pub unchanged: Vec<PathBuf>,
}

impl ChangeSet {
	/// Check if there are any changes
	pub fn has_changes(&self) -> bool {
		!self.added.is_empty() || !self.modified.is_empty() || !self.deleted.is_empty()
	}

	/// Get all files that need to be re-indexed
	pub fn files_to_index(&self) -> Vec<&PathBuf> {
		self.added.iter().chain(self.modified.iter()).collect()
	}

	/// Total number of changes
	pub fn total_changes(&self) -> usize {
		self.added.len() + self.modified.len() + self.deleted.len()
	}
}
