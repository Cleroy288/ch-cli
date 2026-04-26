use std::path::{Path, PathBuf};

use crate::indexer::trigram::types::{Trigram, TrigramIndex};

impl TrigramIndex {
	pub fn new() -> Self {
		Self::default()
	}

	/// Index a file's content
	pub fn index_file(
		&mut self,
		path: &Path,
		content: &str,
	) {
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
	pub fn index_files(
		&mut self,
		files: &[PathBuf],
	) -> std::io::Result<()> {
		for path in files {
			if let Ok(content) = std::fs::read_to_string(path) {
				self.index_file(path, &content);
			}
		}
		Ok(())
	}

	pub fn remove_file(&mut self, path: &Path) {
		self.index.retain(|_, files| {
			files.remove(path);
			!files.is_empty()
		});
		self.file_count =
			self.file_count.saturating_sub(1);
	}

	pub fn extract_trigrams(text: &str) -> Vec<Trigram> {
		let bytes = text.as_bytes();
		if bytes.len() < 3 {
			return Vec::new();
		}
		bytes
			.windows(3)
			.map(|win| [win[0], win[1], win[2]])
			.collect()
	}
}
