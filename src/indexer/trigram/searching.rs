use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::indexer::trigram::types::{
	Trigram, TrigramIndex, TrigramStats,
};

impl TrigramIndex {
	/// Find candidate files that might contain the identifier
	pub fn candidate_files(
		&self,
		identifier: &str,
	) -> HashSet<PathBuf> {
		let trigrams = Self::extract_trigrams(identifier);

		if trigrams.is_empty() {
			return self.all_files();
		}

		intersect_trigram_matches(&self.index, &trigrams)
	}

	pub fn all_files(&self) -> HashSet<PathBuf> {
		self.index.values().flatten().cloned().collect()
	}

	pub fn stats(&self) -> TrigramStats {
		let avg = if self.index.is_empty() {
			0.0
		} else {
			let total: usize =
				self.index.values().map(|files| files.len()).sum();
			total as f64 / self.index.len() as f64
		};

		TrigramStats {
			trigram_count: self.index.len(),
			file_count: self.file_count,
			avg_files_per_trigram: avg,
		}
	}
}

/// Intersect file sets for each trigram in the query
fn intersect_trigram_matches(
	index: &HashMap<Trigram, HashSet<PathBuf>>,
	trigrams: &[Trigram],
) -> HashSet<PathBuf> {
	let mut result: Option<HashSet<PathBuf>> = None;

	for trigram in trigrams {
		let Some(files) = index.get(trigram) else {
			return HashSet::new();
		};
		result = Some(intersect_or_init(result, files));
	}

	result.unwrap_or_default()
}

/// Intersect with accumulator, or initialize if first set
fn intersect_or_init(
	acc: Option<HashSet<PathBuf>>,
	files: &HashSet<PathBuf>,
) -> HashSet<PathBuf> {
	match acc {
		None => files.clone(),
		Some(prev) => {
			prev.intersection(files).cloned().collect()
		}
	}
}
