//! Shared file system cache for the picker.
//!
//! Holds a pre-scanned directory tree in memory,
//! shared between the watcher thread and the TUI.
//! The watcher refreshes the cache after file changes;
//! the picker reads from a local copy.

use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::entry::FsEntry;

/// Thread-safe file system cache.
///
/// Cloning shares the same underlying data (Arc).
#[derive(Clone)]
pub struct FileCache {
	/// shared entries from the last scan
	entries: Arc<Mutex<Vec<FsEntry>>>,
	/// flag set when entries were updated
	dirty: Arc<AtomicBool>,
}

impl FileCache {
	/// Create a new cache and scan the current dir
	pub fn new_with_scan() -> Self {
		let entries = scan_root();
		Self {
			entries: Arc::new(Mutex::new(entries)),
			dirty: Arc::new(AtomicBool::new(false)),
		}
	}

	/// Create an empty cache (no scan). For tests.
	pub fn empty() -> Self {
		Self {
			entries: Arc::new(Mutex::new(Vec::new())),
			dirty: Arc::new(AtomicBool::new(false)),
		}
	}

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

	/// Check if cache was updated since last read
	pub fn is_dirty(&self) -> bool {
		self.dirty.load(Ordering::Acquire)
	}

	/// Clear the dirty flag after syncing
	pub fn clear_dirty(&self) {
		self.dirty.store(false, Ordering::Release);
	}

	/// Clone all entries from the cache
	pub fn snapshot(&self) -> Vec<FsEntry> {
		self.entries
			.lock()
			.map(|lock| lock.clone())
			.unwrap_or_default()
	}
}

/// Dirs to skip in picker scan (internal/build)
const SKIP_DIRS: &[&str] = &[
	".git", ".hg", ".svn", ".rustean-index",
	"target", "node_modules", ".cache",
];

/// Max recursion depth for picker scan
const MAX_SCAN_DEPTH: usize = 10;

/// Scan project root, skipping internal dirs
fn scan_root() -> Vec<FsEntry> {
	let mut entries = Vec::new();
	scan_filtered(Path::new("."), &mut entries, 0);
	entries.sort_by(sort_dirs_first);
	entries
}

/// Recursive scan skipping SKIP_DIRS
fn scan_filtered(
	dir: &Path,
	entries: &mut Vec<FsEntry>,
	depth: usize,
) {
	if depth > MAX_SCAN_DEPTH {
		return;
	}
	let Ok(read_dir) = std::fs::read_dir(dir) else {
		return;
	};
	for item in read_dir.flatten() {
		process_item(&item, entries, depth);
	}
}

/// Process one dir entry, skip internal dirs
fn process_item(
	item: &std::fs::DirEntry,
	entries: &mut Vec<FsEntry>,
	depth: usize,
) {
	let path = item.path();
	let Ok(meta) = item.metadata() else {
		return;
	};
	if meta.is_dir() {
		let name = file_name_str(&path);
		entries.push(FsEntry::new(path.clone(), true));
		if !SKIP_DIRS.contains(&name) {
			scan_filtered(&path, entries, depth + 1);
		}
	} else if meta.is_file() {
		entries.push(FsEntry::new(path, false));
	}
}

/// Extract filename as &str for skip check
fn file_name_str(path: &Path) -> &str {
	path.file_name()
		.and_then(|n| n.to_str())
		.unwrap_or("")
}

/// Sort: directories first, then alphabetical
fn sort_dirs_first(
	lhs: &FsEntry,
	rhs: &FsEntry,
) -> std::cmp::Ordering {
	match (lhs.is_dir, rhs.is_dir) {
		(true, false) => std::cmp::Ordering::Less,
		(false, true) => std::cmp::Ordering::Greater,
		_ => lhs.name.to_lowercase().cmp(
			&rhs.name.to_lowercase(),
		),
	}
}
