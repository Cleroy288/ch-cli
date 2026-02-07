//! Indexing functions for trigram index
//!
//! Functions to add and remove files from the trigram index

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::indexer::trigram::types::{Trigram, TrigramIndex};

impl TrigramIndex {
	/// Create a new empty trigram index
	pub fn new() -> Self {
		Self::default()
	}

	/// Index a file's content
	pub fn index_file(&mut self, path: &Path, content: &str) {
		// trigrams extracted from content
		let trigrams = Self::extract_trigrams(content);

		for trigram in trigrams {
			self.index
				.entry(trigram)
				.or_default()
				.insert(path.to_path_buf());
		}

		self.file_count += 1;
	}

	/// Index multiple files from disk
	pub fn index_files(&mut self, files: &[PathBuf]) -> io::Result<()> {
		for path in files {
			if let Ok(content) = fs::read_to_string(path) {
				self.index_file(path, &content);
			}
		}
		Ok(())
	}

	/// Remove a file from the index
	pub fn remove_file(&mut self, path: &Path) {
		for files in self.index.values_mut() {
			files.remove(path);
		}
		self.file_count = self.file_count.saturating_sub(1);
	}

	/// Extract trigrams from a string
	pub fn extract_trigrams(s: &str) -> Vec<Trigram> {
		let bytes = s.as_bytes();

		if bytes.len() < 3 {
			return Vec::new();
		}

		bytes.windows(3).map(|w| [w[0], w[1], w[2]]).collect()
	}
}
