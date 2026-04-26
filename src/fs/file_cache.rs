use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};

use super::entry::FsEntry;
use super::file_cache_scan::scan_root;

/// Cloning shares the same underlying data (Arc).
#[derive(Clone)]
pub struct FileCache {
	pub(crate) entries: Arc<Mutex<Vec<FsEntry>>>,
	pub(crate) dirty: Arc<AtomicBool>,
}

fn from_entries(v: Vec<FsEntry>) -> FileCache {
	FileCache {
		entries: Arc::new(Mutex::new(v)),
		dirty: Arc::new(AtomicBool::new(false)),
	}
}

impl FileCache {
	pub fn new_with_scan() -> Self {
		from_entries(scan_root())
	}

	pub fn empty() -> Self {
		from_entries(Vec::new())
	}

	pub fn with_entries(entries: Vec<FsEntry>) -> Self {
		from_entries(entries)
	}

	pub fn snapshot(&self) -> Vec<FsEntry> {
		self.entries
			.lock()
			.map(|lock| lock.clone())
			.unwrap_or_default()
	}
}
