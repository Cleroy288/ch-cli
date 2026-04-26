use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use crate::domain::errors::search::SearchError;
use crate::indexer::IndexResult;

use super::cache_helpers::{
	build_index, canonicalize, io_err,
	read_disk_timestamp,
};

/// Caches IndexResult per canonical project root
#[derive(Default)]
pub struct IndexCache {
	entries: Mutex<HashMap<PathBuf, CachedEntry>>,
}

/// Cached index with staleness tracking
struct CachedEntry {
	result: IndexResult,
	/// seconds since epoch when cached
	cached_at: u64,
}

impl IndexCache {
	/// Create an empty cache
	pub fn new() -> Self {
		Self {
			entries: Mutex::new(HashMap::new()),
		}
	}

	/// Rebuilds when persisted state is newer.
	pub fn with_index<F, R>(
		&self,
		path: &Path,
		func: F,
	) -> Result<R, SearchError>
	where
		F: FnOnce(
			&IndexResult,
		) -> Result<R, SearchError>,
	{
		let root = canonicalize(path);
		self.ensure_fresh(&root)?;
		let guard = self
			.entries
			.lock()
			.map_err(|_| io_err("lock poisoned"))?;
		let entry = guard
			.get(&root)
			.ok_or_else(|| io_err("no entry"))?;
		func(&entry.result)
	}

	/// Check + rebuild under a single lock to
	/// prevent duplicate rebuilds (TOCTOU).
	fn ensure_fresh(
		&self,
		root: &Path,
	) -> Result<(), SearchError> {
		let disk_ts = read_disk_timestamp(root);
		let guard = self
			.entries
			.lock()
			.map_err(|_| io_err("lock poisoned"))?;
		let stale = match guard.get(root) {
			None => true,
			Some(ent) => ent.cached_at < disk_ts,
		};
		if !stale {
			return Ok(());
		}
		// Drop lock before expensive I/O, then
		// re-check after rebuild.
		drop(guard);
		let result = build_index(root)?;
		let stamp = read_disk_timestamp(root);
		let mut guard = self
			.entries
			.lock()
			.map_err(|_| io_err("lock poisoned"))?;
		// Only store if still stale (another thread
		// may have rebuilt while we were working).
		let still_stale = match guard.get(root) {
			None => true,
			Some(ent) => ent.cached_at < stamp,
		};
		if still_stale {
			let entry = CachedEntry {
				result,
				cached_at: stamp,
			};
			guard.insert(root.to_path_buf(), entry);
		}
		Ok(())
	}
}
