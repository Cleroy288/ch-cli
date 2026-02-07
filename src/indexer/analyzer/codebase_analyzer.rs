//! Codebase language analyzer.
//!
//! Detects the primary programming language(s) used in a codebase
//! by analyzing file extensions and project configuration files.

use std::collections::HashMap;
use std::path::Path;

use ignore::WalkBuilder;

use crate::indexer::crawler::DetectedLanguage;
use super::detection::{detect_project_type, should_skip_path};
use super::CodebaseAnalysis;

/// Analyzes a codebase to determine its primary programming language
pub struct CodebaseAnalyzer {
	/// Maximum files to scan (for performance on large repos)
	max_files: usize,
}

impl CodebaseAnalyzer {
	/// Create a new analyzer with default settings
	pub fn new() -> Self {
		Self {
			max_files: 10_000, // Reasonable limit for quick analysis
		}
	}

	/// Create an analyzer with custom max file limit
	pub fn with_max_files(max_files: usize) -> Self {
		Self { max_files }
	}

	/// Analyze a codebase and return language composition
	pub fn analyze<P: AsRef<Path>>(&self, root: P) -> CodebaseAnalysis {
		let root = root.as_ref();
		let mut language_counts = HashMap::new();
		let mut total_source_files = 0;

		// First, detect project type from config files
		let project_type = detect_project_type(root);

		// Walk the directory tree
		let walker = WalkBuilder::new(root)
			.hidden(true)
			.git_ignore(true)
			.git_global(true)
			.git_exclude(true)
			.build();

		for entry in walker.flatten().take(self.max_files) {
			let path = entry.path();

			// Skip directories
			if path.is_dir() {
				continue;
			}

			// Skip common non-source dirs even if not in .gitignore
			if should_skip_path(path) {
				continue;
			}

			// Detect language from extension
			if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
				if let Some(lang) = DetectedLanguage::from_extension(ext) {
					*language_counts.entry(lang).or_insert(0) += 1;
					total_source_files += 1;
				}
			}
		}

		// Determine primary language (most files)
		let primary_language = language_counts
			.iter()
			.max_by_key(|(_, count)| *count)
			.map(|(lang, _)| *lang);

		let is_primary_supported = primary_language
			.map(|lang| lang.is_supported())
			.unwrap_or(false);

		CodebaseAnalysis {
			primary_language,
			language_counts,
			total_source_files,
			is_primary_supported,
			project_type,
		}
	}
}

impl Default for CodebaseAnalyzer {
	fn default() -> Self {
		Self::new()
	}
}
