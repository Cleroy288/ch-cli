use std::path::PathBuf;

use crate::indexer::parser::ParseError;
use crate::indexer::Symbol;

use super::language::Language;

#[derive(Debug, Clone, Default)]
pub struct CrawlStats {
	pub files_found: usize,
	pub files_parsed: usize,
	pub files_failed: usize,
	pub symbols_found: usize,
	/// Wall-clock ms
	pub duration_ms: u64,
}

#[derive(Debug)]
pub struct FileResult {
	pub path: PathBuf,
	pub symbols: Vec<Symbol>,
	pub error: Option<ParseError>,
}

#[derive(Debug, Clone)]
pub struct CrawlerConfig {
	pub languages: Vec<Language>,
	pub respect_gitignore: bool,
	pub follow_symlinks: bool,
	/// In bytes
	pub max_file_size: Option<u64>,
	pub ignore_patterns: Vec<String>,
	/// 0 = auto-detect
	pub num_threads: usize,
}

impl Default for CrawlerConfig {
	fn default() -> Self {
		Self {
			languages: vec![Language::Rust, Language::Markdown],
			respect_gitignore: true,
			follow_symlinks: false,
			max_file_size: Some(1024 * 1024), // 1MB default
			ignore_patterns: vec![
				"target".to_string(),
				"node_modules".to_string(),
				".git".to_string(),
			],
			num_threads: 0, // Auto-detect
		}
	}
}
