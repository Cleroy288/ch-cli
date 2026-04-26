use super::cache::IndexCache;

/// Search service backed by SearchIndex + cache
pub struct DefaultSearchService {
	cache: IndexCache,
}

impl Default for DefaultSearchService {
	fn default() -> Self {
		Self { cache: IndexCache::new() }
	}
}

impl DefaultSearchService {
	pub fn new() -> Self {
		Self::default()
	}

	/// Access the index cache
	pub(super) fn cache(&self) -> &IndexCache {
		&self.cache
	}
}
