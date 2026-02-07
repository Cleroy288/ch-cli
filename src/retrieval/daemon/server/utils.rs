//! Utility functions for the daemon server
//!
//! Contains helper functions for cache management.

use super::types::MAX_CACHED_PROJECTS;
use super::ModelDaemon;

/// Evict oldest project if cache exceeds limit
pub fn evict_lru_if_needed(daemon: &mut ModelDaemon) {
	while daemon.project_cache.len() > MAX_CACHED_PROJECTS {
		// Find oldest project by last_indexed timestamp
		let oldest = daemon
			.project_cache
			.iter()
			.min_by_key(|(_, v)| v.last_indexed)
			.map(|(k, _)| k.clone());

		if let Some(path) = oldest {
			eprintln!("[daemon] Evicting LRU project: {:?}", path);
			daemon.project_cache.remove(&path);
		} else {
			break;
		}
	}
}

