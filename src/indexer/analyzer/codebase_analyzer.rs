use std::collections::HashMap;
use std::path::Path;

use ignore::WalkBuilder;

use crate::indexer::crawler::DetectedLanguage;
use super::detection::{detect_project_type, should_skip_path};
use super::CodebaseAnalysis;

type LangCounts = (HashMap<DetectedLanguage, usize>, usize);

pub struct CodebaseAnalyzer {
	max_files: usize,
}

impl CodebaseAnalyzer {
	pub fn new() -> Self {
		Self { max_files: 10_000 }
	}

	pub fn with_max_files(max_files: usize) -> Self {
		Self { max_files }
	}

	pub fn analyze<P: AsRef<Path>>(&self, root: P) -> CodebaseAnalysis {
		let root = root.as_ref();
		let project_type = detect_project_type(root);

		let (language_counts, total_source_files) =
			self.count_languages(root);

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

	fn count_languages(
		&self,
		root: &Path,
	) -> LangCounts {
		let mut counts = HashMap::new();
		let mut total = 0;

		let walker = WalkBuilder::new(root)
			.hidden(true)
			.git_ignore(true)
			.git_global(true)
			.git_exclude(true)
			.build();

		for entry in walker.flatten().take(self.max_files) {
			let path = entry.path();
			if path.is_dir() || should_skip_path(path) {
				continue;
			}
			let lang = path
				.extension()
				.and_then(|ext| ext.to_str())
				.and_then(DetectedLanguage::from_extension);
			if let Some(lang) = lang {
				*counts.entry(lang).or_insert(0) += 1;
				total += 1;
			}
		}

		(counts, total)
	}
}

impl Default for CodebaseAnalyzer {
	fn default() -> Self {
		Self::new()
	}
}
