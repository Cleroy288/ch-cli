//! Default ModelCache Implementation
//!
//! Provides the Default trait implementation for ModelCache
//! using platform-specific cache directory detection.

use std::collections::HashMap;
use std::path::PathBuf;

use super::cache::ModelCache;

impl Default for ModelCache {
	fn default() -> Self {
		let cache_dir = detect_cache_directory();

		Self::new(cache_dir).unwrap_or_else(|_| Self {
			cache_dir: PathBuf::from(".ch-cli/models"),
			models: HashMap::new(),
		})
	}
}

/// Detect platform-specific cache directory
fn detect_cache_directory() -> PathBuf {
	directories::ProjectDirs::from("com", "ch-cli", "ch-cli")
		.map(|d| d.cache_dir().join("models"))
		.unwrap_or_else(|| PathBuf::from(".ch-cli/models"))
}
