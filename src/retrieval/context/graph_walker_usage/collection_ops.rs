//! UsageCollection Operations
//!
//! Provides sorting, truncation, and conversion operations
//! for UsageCollection.

use super::collection_core::{UsageCollection, UsageInfo};

impl UsageCollection {
	/// Sort usages by file path, then by line number
	pub fn sort_by_file(&mut self) {
		self.usages.sort_by(|lhs, rhs| {
			lhs.file
				.cmp(&rhs.file)
				.then(lhs.line.cmp(&rhs.line))
		});
	}

	/// Truncate to a maximum number of usages
	pub fn truncate(&mut self, limit: usize) {
		self.usages.truncate(limit);
	}

	/// Convert to a vector of UsageInfo
	pub fn into_vec(self) -> Vec<UsageInfo> {
		self.usages
	}

	/// Get the number of usages
	pub fn len(&self) -> usize {
		self.usages.len()
	}

	/// Check if the collection is empty
	pub fn is_empty(&self) -> bool {
		self.usages.is_empty()
	}
}
