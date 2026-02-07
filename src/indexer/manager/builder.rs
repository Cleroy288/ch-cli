//! IndexManager builder methods.

use std::path::Path;

use crate::indexer::crawler::CrawlerConfig;

use super::types::ProgressCallback;

/// The main index manager for semantic code indexing
pub struct IndexManager {
	/// Crawler configuration
	#[doc(hidden)]
	pub crawler_config: CrawlerConfig,
	/// Optional progress callback
	#[doc(hidden)]
	pub progress_callback: Option<ProgressCallback>,
	/// Whether to build a semantic graph for name resolution
	#[doc(hidden)]
	pub enable_semantic_analysis: bool,
	/// Whether to extract references from AST
	#[doc(hidden)]
	pub enable_reference_extraction: bool,
	/// Whether to persist the index to disk (enables incremental indexing)
	#[doc(hidden)]
	pub enable_persistence: bool,
}

impl IndexManager {
	/// Create a new IndexManager with default configuration
	pub fn new() -> Self {
		Self {
			crawler_config: CrawlerConfig::default(),
			progress_callback: None,
			enable_semantic_analysis: false,
			enable_reference_extraction: false,
			enable_persistence: false,
		}
	}

	/// Create an IndexManager with custom crawler configuration
	pub fn with_config(config: CrawlerConfig) -> Self {
		Self {
			crawler_config: config,
			progress_callback: None,
			enable_semantic_analysis: false,
			enable_reference_extraction: false,
			enable_persistence: false,
		}
	}

	/// Set a progress callback
	pub fn on_progress<F>(mut self, callback: F) -> Self
	where
		F: Fn(usize, usize, &Path) + Send + Sync + 'static,
	{
		self.progress_callback = Some(Box::new(callback));
		self
	}
}

impl Default for IndexManager {
	fn default() -> Self {
		Self::new()
	}
}
