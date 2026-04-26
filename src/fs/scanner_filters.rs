use super::scanner::FileScanner;
use crate::fs::FsEntry;

impl FileScanner {
	pub fn directories(&self) -> Vec<&FsEntry> {
		self.entries()
			.iter()
			.filter(|entry| entry.is_dir)
			.collect()
	}

	pub fn files(&self) -> Vec<&FsEntry> {
		self.entries()
			.iter()
			.filter(|entry| !entry.is_dir)
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
			.filter(|entry| {
				if files_only && entry.is_dir {
					return false;
				}
				if dirs_only && !entry.is_dir {
					return false;
				}
				if query_lower.is_empty() {
					return true;
				}
				entry.name
					.to_lowercase()
					.contains(&query_lower)
			})
			.collect()
	}
}

