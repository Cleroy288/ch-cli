//! IndexManager builder methods.

use std::path::Path;

use crate::indexer::crawler::CrawlerConfig;

use super::types::ProgressCallback;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Default)]
pub struct IndexManagerFlags {
	pub semantic_analysis: bool,
	pub reference_extraction: bool,
	pub persistence: bool,
}

/// The main index manager for semantic code indexing
pub struct IndexManager {
	pub(crate) crawler_config: CrawlerConfig,
	pub(crate) progress_callback: Option<ProgressCallback>,
	pub(crate) flags: IndexManagerFlags,
}

impl IndexManager {
	pub fn new() -> Self {
		Self::with_config(CrawlerConfig::default())
	}

	pub fn with_config(
		config: CrawlerConfig,
	) -> Self {
		Self {
			crawler_config: config,
			progress_callback: None,
			flags: IndexManagerFlags::default(),
		}
	}

	pub fn flags(&self) -> &IndexManagerFlags {
		&self.flags
	}

	pub fn has_progress_callback(&self) -> bool {
		self.progress_callback.is_some()
	}
}

impl IndexManager {
	/// Set a progress callback
	pub fn on_progress<F>(
		mut self,
		callback: F,
	) -> Self
	where
		F: Fn(usize, usize, &Path)
			+ Send
			+ Sync
			+ 'static,
	{
		self.progress_callback =
			Some(Box::new(callback));
		self
	}
}

impl Default for IndexManager {
	fn default() -> Self {
		Self::new()
	}
}
