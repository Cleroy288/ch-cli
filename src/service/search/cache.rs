//! Index cache for the search service.
//!
//! Avoids rebuilding IndexManager on every call
//! by caching the IndexResult per project root.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::domain::errors::search::SearchError;
use crate::indexer::IndexResult;

use super::cache_helpers::{
	build_index, canonicalize, io_err,
	read_disk_timestamp,
};

/// Cache holding IndexResult per project root
#[derive(Default)]
pub struct IndexCache {
	/// cached entries keyed by canonical root
	entries: Mutex<HashMap<PathBuf, CachedEntry>>,
}

/// A single cached index entry
struct CachedEntry {
	/// the cached index result
	result: IndexResult,
	/// timestamp when cached (secs since epoch)
	cached_at: u64,
}

impl IndexCache {
	/// Create an empty cache
	pub fn new() -> Self {
		Self {
			entries: Mutex::new(HashMap::new()),
		}
	}

	/// Get or build index, calling `f` with result.
	/// Rebuilds when persisted state is newer.
	pub fn with_index<F, R>(
		&self,
		path: &Path,
		semantic: bool,
		func: F,
	) -> Result<R, SearchError>
	where
		F: FnOnce(
			&IndexResult,
		) -> Result<R, SearchError>,
	{
		let root = canonicalize(path);
		let disk_ts = read_disk_timestamp(&root);
		self.ensure_fresh(&root, disk_ts, semantic)?;
		let guard = self
			.entries
			.lock()
			.map_err(|_| io_err("lock poisoned"))?;
		let entry = guard
			.get(&root)
			.ok_or_else(|| io_err("no entry"))?;
		func(&entry.result)
	}

	/// Ensure the cache has a fresh entry
	fn ensure_fresh(
		&self,
		root: &PathBuf,
		disk_ts: u64,
		semantic: bool,
	) -> Result<(), SearchError> {
		let stale = self.is_stale(root, disk_ts)?;
		if !stale {
			return Ok(());
		}
		let result = build_index(root, semantic)?;
		let stamp = read_disk_timestamp(root);
		let entry = CachedEntry {
			result,
			cached_at: stamp,
		};
		let mut guard = self
			.entries
			.lock()
			.map_err(|_| io_err("lock poisoned"))?;
		guard.insert(root.clone(), entry);
		Ok(())
	}

	/// Check if cache entry is stale or missing
	fn is_stale(
		&self,
		root: &PathBuf,
		disk_ts: u64,
	) -> Result<bool, SearchError> {
		let guard = self
			.entries
			.lock()
			.map_err(|_| io_err("lock poisoned"))?;
		Ok(match guard.get(root) {
			None => true,
			Some(ent) => ent.cached_at < disk_ts,
		})
	}
}
