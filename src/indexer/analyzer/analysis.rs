use std::collections::HashMap;

use crate::indexer::crawler::{DetectedLanguage, Language};
use super::ProjectType;

#[derive(Debug, Clone)]
pub struct CodebaseAnalysis {
	pub primary_language: Option<DetectedLanguage>,
	pub language_counts: HashMap<DetectedLanguage, usize>,
	pub total_source_files: usize,
	pub is_primary_supported: bool,
	pub project_type: Option<ProjectType>,
}

impl CodebaseAnalysis {
	pub fn supported_languages(&self) -> Vec<(Language, usize)> {
		let mut supported: Vec<(Language, usize)> = self
			.language_counts
			.iter()
			.filter_map(|(lang, count)| {
				lang.as_supported().map(|sup| (sup, *count))
			})
			.collect();
		supported.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
		supported
	}

	pub fn unsupported_languages(
		&self,
	) -> Vec<(DetectedLanguage, usize)> {
		let mut unsupported: Vec<(DetectedLanguage, usize)> = self
			.language_counts
			.iter()
			.filter(|(lang, _)| !lang.is_supported())
			.map(|(lang, count)| (*lang, *count))
			.collect();
		unsupported.sort_by(|lhs, rhs| rhs.1.cmp(&lhs.1));
		unsupported
	}

	pub fn has_supported_files(&self) -> bool {
		self.language_counts
			.iter()
			.any(|(lang, count)| lang.is_supported() && *count > 0)
	}

	pub fn supported_file_count(&self) -> usize {
		self.language_counts
			.iter()
			.filter(|(lang, _)| lang.is_supported())
			.map(|(_, count)| count)
			.sum()
	}
}
