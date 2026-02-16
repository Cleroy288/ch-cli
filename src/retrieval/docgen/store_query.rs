//! DocStore query operations.
//!
//! Provides filtering and search methods for
//! documentation entries by status, file, and name.

use std::path::Path;

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::entry_types::DocStatus;
use crate::retrieval::docgen::store_core::DocStore;

/// Query methods for filtering entries.
impl DocStore {
	/// Get all pending entries.
	pub fn get_pending(&self) -> Vec<&DocEntry> {
		self.entries
			.values()
			.filter(|entry| {
				entry.status == DocStatus::Pending
			})
			.collect()
	}

	/// Get all pending entry IDs.
	pub fn get_pending_ids(&self) -> Vec<String> {
		self.entries
			.values()
			.filter(|entry| {
				entry.status == DocStatus::Pending
			})
			.map(|entry| entry.id.clone())
			.collect()
	}

	/// Check if all docs are ready.
	pub fn is_ready(&self) -> bool {
		self.generation_complete
	}

	/// Get entries for a specific file.
	pub fn get_by_file(
		&self,
		file_path: &Path,
	) -> Vec<&DocEntry> {
		self.entries
			.values()
			.filter(|entry| {
				entry.file_path == file_path
			})
			.collect()
	}

	/// Get entry by file path and symbol name.
	///
	/// More precise than get_by_name when multiple
	/// symbols share the same name across files.
	pub fn get_by_file_and_name(
		&self,
		file_path: &Path,
		name: &str,
	) -> Option<&DocEntry> {
		self.entries.values().find(|entry| {
			entry.file_path == file_path
				&& entry.name == name
		})
	}

	/// Search entries by name pattern.
	pub fn search_by_name(
		&self,
		pattern: &str,
	) -> Vec<&DocEntry> {
		let pattern_lower = pattern.to_lowercase();
		self.entries
			.values()
			.filter(|entry| {
				entry
					.name
					.to_lowercase()
					.contains(&pattern_lower)
			})
			.collect()
	}
}
