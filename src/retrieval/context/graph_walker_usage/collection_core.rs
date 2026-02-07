//! Core UsageCollection Implementation
//!
//! Provides the basic usage collection structure
//! with deduplication.

use std::collections::HashSet;
use std::path::PathBuf;

use crate::indexer::ReferenceContext;

/// Information about a single usage of a symbol
#[derive(Debug, Clone)]
pub struct UsageInfo {
	/// File where the usage occurs
	pub file: PathBuf,
	/// Line number of the usage
	pub line: usize,
	/// Context of the usage (Call, Type, etc.)
	pub context: ReferenceContext,
	/// Code snippet with context lines
	pub snippet: Option<String>,
	/// Name of the symbol containing this usage
	pub containing_symbol: Option<String>,
}

/// Collection of usages with deduplication
/// Ensures no duplicate (file, line) pairs are added
pub struct UsageCollection {
	/// List of unique usages
	pub(super) usages: Vec<UsageInfo>,
	/// Set of (file, line) pairs for deduplication
	pub(super) seen: HashSet<(PathBuf, usize)>,
}

impl UsageCollection {
	/// Create a new empty usage collection
	pub fn new() -> Self {
		Self {
			usages: Vec::new(),
			seen: HashSet::new(),
		}
	}

	/// Add a usage if not already present
	pub fn add(&mut self, usage: UsageInfo) {
		let key = (usage.file.clone(), usage.line);
		if !self.seen.contains(&key) {
			self.seen.insert(key);
			self.usages.push(usage);
		}
	}

	/// Check if a file+line pair is already present
	pub fn contains(
		&self,
		file: &std::path::Path,
		line: usize,
	) -> bool {
		self.seen.contains(&(file.to_path_buf(), line))
	}
}

impl Default for UsageCollection {
	fn default() -> Self {
		Self::new()
	}
}
