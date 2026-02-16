use std::path::Path;

use crate::fs::{FileCache, FsEntry};
use crate::picker::PickerMode;

/// Filesystem scanner wrapper for the Picker.
///
/// Holds a local copy of the shared file cache.
/// Syncs from cache when the watcher flags it dirty.
pub struct PickerScanner {
	/// shared file cache (watcher updates this)
	cache: FileCache,
	/// local copy of entries for borrowing
	local_entries: Vec<FsEntry>,
}

impl PickerScanner {
	/// Create a new PickerScanner from a shared cache
	pub fn new(cache: FileCache) -> Self {
		let local_entries = cache.snapshot();
		cache.clear_dirty();
		Self { cache, local_entries }
	}

	/// Sync local entries from cache if dirty.
	///
	/// Called from the event loop before rendering.
	pub fn sync_if_dirty(&mut self) {
		if !self.cache.is_dirty() {
			return;
		}
		self.local_entries = self.cache.snapshot();
		self.cache.clear_dirty();
	}

	/// Get filtered results for current mode.
	///
	/// Filters by browse directory and query string.
	pub fn get_results(
		&self,
		mode: &PickerMode,
		query: &str,
	) -> Vec<&FsEntry> {
		match mode {
			PickerMode::Browse { dir } => {
				self.search_in_dir(query, dir)
			}
			_ => Vec::new(),
		}
	}

	/// Search entries within a directory by name.
	///
	/// Only returns direct children of `dir`.
	fn search_in_dir(
		&self,
		query: &str,
		dir: &Path,
	) -> Vec<&FsEntry> {
		let query_lower = query.to_lowercase();

		self.local_entries
			.iter()
			.filter(|entry| {
				is_direct_child(entry, dir)
					&& matches_query(
						entry, &query_lower,
					)
			})
			.collect()
	}
}

/// Check if entry is a direct child of the dir
fn is_direct_child(
	entry: &FsEntry,
	dir: &Path,
) -> bool {
	entry.path.parent() == Some(dir)
}

/// Check if entry name matches the search query
fn matches_query(
	entry: &FsEntry,
	query_lower: &str,
) -> bool {
	if query_lower.is_empty() {
		return true;
	}
	entry.name.to_lowercase().contains(query_lower)
}

impl Default for PickerScanner {
	fn default() -> Self {
		let cache = FileCache::empty();
		Self::new(cache)
	}
}
