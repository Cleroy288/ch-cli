//! Change detection for incremental indexing.
//!
//! Detects added, modified, deleted, and unchanged files.

use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use super::types::{ChangeSet, FileState, IndexState};

impl IndexState {
	/// Update file state after indexing
	pub fn update_file(
		&mut self,
		path: &Path,
		symbol_count: usize,
	) -> std::io::Result<()> {
		// canonicalized path to match the root
		let canonical_path = path
			.canonicalize()
			.unwrap_or_else(|_| path.to_path_buf());

		// file_state: new FileState created from the canonical path
		let mut file_state = FileState::from_path(&canonical_path, &self.root)?;
		file_state.symbol_count = symbol_count;

		self.files.insert(file_state.path.clone(), file_state);
		Ok(())
	}

	/// Remove a file from the index
	pub fn remove_file(&mut self, path: &Path) {
		// canonicalized path to match stored paths
		let canonical_path = path
			.canonicalize()
			.unwrap_or_else(|_| path.to_path_buf());

		// path relative to the root
		let relative = canonical_path
			.strip_prefix(&self.root)
			.unwrap_or(&canonical_path);

		self.files.remove(relative);
	}

	/// Detect changes between current disk state and indexed state
	pub fn detect_changes(&self, current_files: &[PathBuf]) -> ChangeSet {
		// changes: accumulator for detected file changes
		let mut changes = ChangeSet::default();

		// Check for added and modified files
		for file_path in current_files {
			self.classify_file(
				file_path, &mut changes,
			);
		}

		// current_set: set of current file paths for lookup
		let current_set: HashSet<_> = current_files
			.iter()
			.filter_map(|path| {
				let canonical = path.canonicalize().ok()?;
				let relative = canonical
					.strip_prefix(&self.root)
					.ok()?
					.to_path_buf();
				Some(relative)
			})
			.collect();

		// Check for deleted files
		for indexed_path in self.files.keys() {
			if !current_set.contains(indexed_path) {
				changes.deleted.push(self.root.join(indexed_path));
			}
		}

		changes
	}

	/// Classify a single file as added, modified, or unchanged
	fn classify_file(
		&self,
		file_path: &Path,
		changes: &mut ChangeSet,
	) {
		let canonical = file_path
			.canonicalize()
			.unwrap_or_else(|_| file_path.to_path_buf());
		let relative = canonical
			.strip_prefix(&self.root)
			.unwrap_or(&canonical);

		match self.files.get(relative) {
			Some(state) if state.has_changed(&self.root) => {
				changes.modified.push(file_path.to_path_buf());
			}
			Some(_) => {
				changes.unchanged.push(file_path.to_path_buf());
			}
			None => {
				changes.added.push(file_path.to_path_buf());
			}
		}
	}

	/// Update the last_updated timestamp
	pub fn touch(&mut self) {
		self.last_updated = SystemTime::now()
			.duration_since(SystemTime::UNIX_EPOCH)
			.map(|dur| dur.as_secs())
			.unwrap_or(0);
	}
}
