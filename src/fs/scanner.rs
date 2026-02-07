use std::fs;
use std::path::Path;

use crate::domain::MAX_RECURSION_DEPTH;
use crate::fs::FsEntry;

/// File system scanner for indexing files and dirs.
///
/// Recursively scans directories up to
/// MAX_RECURSION_DEPTH and provides methods for
/// filtering and searching entries.
pub struct FileScanner {
	entries: Vec<FsEntry>,
}

impl FileScanner {
	/// Create a new FileScanner
	pub fn new() -> Self {
		Self {
			entries: Vec::new(),
		}
	}

	/// Scan a directory and index all entries.
	///
	/// Entries are sorted: directories first, then
	/// files, alphabetically within each group.
	pub fn scan_directory<P: AsRef<Path>>(
		&mut self,
		path: P,
	) -> std::io::Result<()> {
		self.entries.clear();
		self.scan_recursive(path.as_ref(), 0)?;

		self.entries.sort_by(|a, b| {
			match (a.is_dir, b.is_dir) {
				(true, false) => {
					std::cmp::Ordering::Less
				}
				(false, true) => {
					std::cmp::Ordering::Greater
				}
				_ => a
					.name
					.to_lowercase()
					.cmp(&b.name.to_lowercase()),
			}
		});

		Ok(())
	}

	/// Get all entries
	pub fn entries(&self) -> &[FsEntry] {
		&self.entries
	}
}

/// Recursive scanning implementation.
impl FileScanner {
	/// Recursively scan a directory.
	///
	/// Uses let-else pattern for clean error
	/// handling. Unreadable entries are skipped.
	fn scan_recursive(
		&mut self,
		path: &Path,
		depth: usize,
	) -> std::io::Result<()> {
		if depth > MAX_RECURSION_DEPTH {
			return Ok(());
		}

		let entries = fs::read_dir(path)?;

		for entry in entries {
			let Ok(entry) = entry else {
				continue;
			};

			let entry_path = entry.path();

			let Ok(metadata) = entry.metadata()
			else {
				continue;
			};

			if metadata.is_dir() {
				self.entries.push(FsEntry::new(
					entry_path.clone(),
					true,
				));
				let _ = self.scan_recursive(
					&entry_path,
					depth + 1,
				);
			} else if metadata.is_file() {
				self.entries
					.push(FsEntry::new(entry_path, false));
			}
		}

		Ok(())
	}
}

impl Default for FileScanner {
	fn default() -> Self {
		Self::new()
	}
}

