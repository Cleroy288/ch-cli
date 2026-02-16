//! Core CodebaseAnalysis structure definition.

use std::collections::HashMap;

use crate::indexer::crawler::{DetectedLanguage, Language};
use super::ProjectType;

/// Result of analyzing a codebase's language composition
#[derive(Debug, Clone)]
pub struct CodebaseAnalysis {
	/// Primary detected language (most files)
	pub primary_language: Option<DetectedLanguage>,
	/// All languages detected with file counts
	pub language_counts: HashMap<DetectedLanguage, usize>,
	/// Total source files analyzed
	pub total_source_files: usize,
	/// Whether the primary language is supported for indexing
	pub is_primary_supported: bool,
	/// Detected project type (if identifiable)
	pub project_type: Option<ProjectType>,
}

impl CodebaseAnalysis {
	/// Get supported languages (sorted by file count, descending)
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

	/// Get unsupported languages (sorted by file count, descending)
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

	/// Check if the codebase has any supported files for indexing
	pub fn has_supported_files(&self) -> bool {
		self.language_counts
			.iter()
			.any(|(lang, count)| lang.is_supported() && *count > 0)
	}

	/// Get the total count of supported files
	pub fn supported_file_count(&self) -> usize {
		self.language_counts
			.iter()
			.filter(|(lang, _)| lang.is_supported())
			.map(|(_, count)| count)
			.sum()
	}
}
