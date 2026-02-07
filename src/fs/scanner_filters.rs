//! FileScanner filter and search operations.
//!
//! Provides filtering by type (files/directories)
//! and case-insensitive name search.

use super::scanner::FileScanner;
use crate::fs::FsEntry;

/// Filter and search methods for FileScanner.
impl FileScanner {
	/// Get only directories
	pub fn directories(&self) -> Vec<&FsEntry> {
		self.entries()
			.iter()
			.filter(|e| e.is_dir)
			.collect()
	}

	/// Get only files
	pub fn files(&self) -> Vec<&FsEntry> {
		self.entries()
			.iter()
			.filter(|e| !e.is_dir)
			.collect()
	}

	/// Search entries by name (case-insensitive).
	///
	/// Can filter by files only or directories only.
	/// Empty query returns all matching entries.
	pub fn search(
		&self,
		query: &str,
		files_only: bool,
		dirs_only: bool,
	) -> Vec<&FsEntry> {
		let query_lower = query.to_lowercase();

		self.entries()
			.iter()
			.filter(|e| {
				if files_only && e.is_dir {
					return false;
				}
				if dirs_only && !e.is_dir {
					return false;
				}
				if query_lower.is_empty() {
					return true;
				}
				e.name
					.to_lowercase()
					.contains(&query_lower)
			})
			.collect()
	}
}

