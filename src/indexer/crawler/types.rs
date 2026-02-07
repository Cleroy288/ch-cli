//! Types and configuration for the file system crawler.
//!
//! This module contains data structures for crawl statistics,
//! file results, and crawler configuration.

use std::path::PathBuf;

use crate::indexer::parser::ParseError;
use crate::indexer::Symbol;

use super::language::Language;

/// Statistics about a crawl/index operation
#[derive(Debug, Clone, Default)]
pub struct CrawlStats {
	/// Total files discovered
	pub files_found: usize,
	/// Files successfully parsed
	pub files_parsed: usize,
	/// Files that failed to parse
	pub files_failed: usize,
	/// Total symbols extracted
	pub symbols_found: usize,
	/// Time taken in milliseconds
	pub duration_ms: u64,
}

/// Result of crawling a single file
#[derive(Debug)]
pub struct FileResult {
	/// Path to the file
	pub path: PathBuf,
	/// Symbols found (empty if parsing failed)
	pub symbols: Vec<Symbol>,
	/// Error if parsing failed
	pub error: Option<ParseError>,
}

/// Configuration for the crawler
#[derive(Debug, Clone)]
pub struct CrawlerConfig {
	/// Languages to index
	pub languages: Vec<Language>,
	/// Whether to respect .gitignore
	pub respect_gitignore: bool,
	/// Whether to follow symlinks
	pub follow_symlinks: bool,
	/// Maximum file size to parse (in bytes)
	pub max_file_size: Option<u64>,
	/// Additional patterns to ignore
	pub ignore_patterns: Vec<String>,
	/// Number of threads (0 = auto)
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
