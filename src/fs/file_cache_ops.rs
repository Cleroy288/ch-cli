use std::sync::atomic::Ordering;

use super::file_cache::FileCache;
use super::file_cache_scan::scan_root;

impl FileCache {
	/// Rescan the project root and update entries.
	///
	/// Called by the watcher thread after reindex.
	pub fn refresh(&self) {
		let fresh = scan_root();
		if let Ok(mut lock) = self.entries.lock() {
			*lock = fresh;
		}
		self.dirty.store(true, Ordering::Release);
	}

	pub fn is_dirty(&self) -> bool {
		self.dirty.load(Ordering::Acquire)
	}

	pub fn clear_dirty(&self) {
		self.dirty.store(false, Ordering::Release);
	}
}
