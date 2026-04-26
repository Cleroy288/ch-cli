use std::path::Path;

use crate::fs::{FileCache, FsEntry};
use crate::picker::PickerMode;

pub struct PickerScanner {
	cache: FileCache,
	local_entries: Vec<FsEntry>,
}

impl PickerScanner {
	pub fn new(cache: FileCache) -> Self {
		let local_entries = cache.snapshot();
		cache.clear_dirty();
		Self { cache, local_entries }
	}

	pub fn sync_if_dirty(&mut self) {
		if !self.cache.is_dirty() {
			return;
		}
		self.local_entries = self.cache.snapshot();
		self.cache.clear_dirty();
	}

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

	fn search_in_dir(
		&self,
		query: &str,
		dir: &Path,
	) -> Vec<&FsEntry> {
		let lower = query.to_lowercase();
		self.local_entries
			.iter()
			.filter(|e| {
				is_direct_child(e, dir)
					&& matches_name(e, &lower)
			})
			.collect()
	}
}

fn is_direct_child(
	entry: &FsEntry,
	dir: &Path,
) -> bool {
	entry.path.parent() == Some(dir)
}

fn matches_name(
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
		Self::new(FileCache::empty())
	}
}
