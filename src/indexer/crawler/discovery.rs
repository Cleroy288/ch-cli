//! File system discovery for the semantic indexer.
//!
//! This module provides efficient directory traversal with:
//! - `.gitignore` support via the `ignore` crate
//! - Language-based file filtering

use std::path::{Path, PathBuf};

use ignore::WalkBuilder;

use super::language::Language;
use super::types::CrawlerConfig;

/// File system crawler for discovering and indexing source files
pub struct Crawler {
	config: CrawlerConfig, // configuration for the crawler
}

impl Crawler {
	/// Create a new crawler with default configuration
	pub fn new() -> Self {
		Self {
			config: CrawlerConfig::default(),
		}
	}

	/// Create a new crawler with custom configuration
	pub fn with_config(config: CrawlerConfig) -> Self {
		Self { config }
	}

	/// Get the configuration
	pub fn config(&self) -> &CrawlerConfig {
		&self.config
	}

	/// Discover all indexable files in a directory
	///
	/// Walks the directory tree respecting gitignore rules and filters
	/// files by supported languages and size limits.
	pub fn discover_files<P: AsRef<Path>>(&self, root: P) -> Vec<PathBuf> {
		let root = root.as_ref(); // root directory to start crawling from
		let mut files = Vec::new(); // collected file paths

		let mut builder = WalkBuilder::new(root);
		builder
			.hidden(true) // Skip hidden files by default
			.git_ignore(self.config.respect_gitignore)
			.git_global(self.config.respect_gitignore)
			.git_exclude(self.config.respect_gitignore)
			.follow_links(self.config.follow_symlinks);

		// Add custom ignore patterns
		for pattern in &self.config.ignore_patterns {
			let _ = builder.add_ignore(Path::new(pattern));
		}

		for entry in builder.build().flatten() {
			let path = entry.path();

			// Skip directories
			if path.is_dir() {
				continue;
			}

			// Check if it's a supported language
			if Language::from_path(path).is_none() {
				continue;
			}

			// Check file size if configured
			if let Some(max_size) = self.config.max_file_size {
				if let Ok(metadata) = path.metadata() {
					if metadata.len() > max_size {
						continue;
					}
				}
			}

			files.push(path.to_path_buf());
		}

		files
	}
}

impl Default for Crawler {
	fn default() -> Self {
		Self::new()
	}
}
