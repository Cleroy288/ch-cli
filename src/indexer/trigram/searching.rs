//! Search functions for trigram index
//!
//! Functions to find candidate files and retrieve statistics

use std::collections::HashSet;
use std::path::PathBuf;

use crate::indexer::trigram::types::{TrigramIndex, TrigramStats};

impl TrigramIndex {
	/// Find candidate files that might contain the identifier
	pub fn candidate_files(
		&self,
		identifier: &str
	) -> HashSet<PathBuf> {
		// trigrams from search term
		let trigrams = Self::extract_trigrams(identifier);

		if trigrams.is_empty() {
			// identifier too short, return all files
			return self.all_files();
		}

		let mut result: Option<HashSet<PathBuf>> = None; // intersection result

		// intersect all trigram matches
		for trigram in trigrams {
			if let Some(files) = self.index.get(&trigram) {
				match &mut result {
					None => result = Some(files.clone()),
					Some(r) => *r = r.intersection(files).cloned().collect(),
				}
			} else {
				// trigram not found, no matches
				return HashSet::new();
			}
		}

		result.unwrap_or_default()
	}

	/// Get all indexed files
	pub fn all_files(&self) -> HashSet<PathBuf> {
		self.index.values().flatten().cloned().collect()
	}

	/// Get statistics about the index
	pub fn stats(&self) -> TrigramStats {
		let avg = if self.index.is_empty() {
			0.0
		} else {
			let total: usize = self.index.values().map(|v| v.len()).sum();
			total as f64 / self.index.len() as f64
		};

		TrigramStats {
			trigram_count: self.index.len(),
			file_count: self.file_count,
			avg_files_per_trigram: avg,
		}
	}
}
