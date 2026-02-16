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

		self.entries.sort_by(|lhs, rhs| {
			match (lhs.is_dir, rhs.is_dir) {
				(true, false) => {
					std::cmp::Ordering::Less
				}
				(false, true) => {
					std::cmp::Ordering::Greater
				}
				_ => lhs
					.name
					.to_lowercase()
					.cmp(&rhs.name.to_lowercase()),
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

		for dir_entry in fs::read_dir(path)? {
			let Ok(dir_entry) = dir_entry else {
				continue;
			};
			self.process_entry(&dir_entry, depth);
		}

		Ok(())
	}

	/// Process a single directory entry,
	/// adding it and recursing into subdirs.
	fn process_entry(
		&mut self,
		dir_entry: &fs::DirEntry,
		depth: usize,
	) {
		let entry_path = dir_entry.path();
		let Ok(metadata) = dir_entry.metadata()
		else {
			return;
		};

		if metadata.is_dir() {
			self.entries.push(
				FsEntry::new(entry_path.clone(), true),
			);
			let _ = self
				.scan_recursive(&entry_path, depth + 1);
		} else if metadata.is_file() {
			self.entries
				.push(FsEntry::new(entry_path, false));
		}
	}
}

impl Default for FileScanner {
	fn default() -> Self {
		Self::new()
	}
}

