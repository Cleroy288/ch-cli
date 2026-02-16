//! IndexManager builder methods.

use std::path::Path;

use crate::indexer::crawler::CrawlerConfig;

use super::types::ProgressCallback;

#[allow(clippy::struct_excessive_bools)]
/// Boolean feature toggles for IndexManager
#[derive(Debug, Clone, Default)]
pub struct IndexManagerFlags {
	/// Build a semantic graph for name resolution
	pub semantic_analysis: bool,
	/// Extract references from AST
	pub reference_extraction: bool,
	/// Persist the index to disk (incremental)
	pub persistence: bool,
}

/// The main index manager for semantic code indexing
pub struct IndexManager {
	/// Crawler configuration
	#[doc(hidden)]
	pub crawler_config: CrawlerConfig,
	/// Optional progress callback
	#[doc(hidden)]
	pub progress_callback: Option<ProgressCallback>,
	/// Feature toggles
	#[doc(hidden)]
	pub flags: IndexManagerFlags,
}

impl IndexManager {
	/// Create a new IndexManager with default config
	pub fn new() -> Self {
		Self {
			crawler_config: CrawlerConfig::default(),
			progress_callback: None,
			flags: IndexManagerFlags::default(),
		}
	}

	/// Create IndexManager with custom crawler config
	pub fn with_config(
		config: CrawlerConfig,
	) -> Self {
		Self {
			crawler_config: config,
			progress_callback: None,
			flags: IndexManagerFlags::default(),
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
