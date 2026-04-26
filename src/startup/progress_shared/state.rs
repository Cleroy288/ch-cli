use std::sync::atomic::{AtomicBool, AtomicUsize};
use std::sync::{Arc, Mutex};

/// Shared progress state between the indexing thread
/// and the TUI renderer.
pub(crate) struct ProgressState {
	pub files_done: Arc<AtomicUsize>,
	pub total: Arc<AtomicUsize>,
	pub current_file: Arc<Mutex<String>>,
	pub done: Arc<AtomicBool>,
}

impl ProgressState {
	/// Allocate a fresh ProgressState with zeroed counters.
	pub fn new() -> Self {
		Self {
			files_done: Arc::new(AtomicUsize::new(0)),
			total: Arc::new(AtomicUsize::new(0)),
			current_file: Arc::new(Mutex::new(
				String::new(),
			)),
			done: Arc::new(AtomicBool::new(false)),
		}
	}

	/// Snapshot the current file name under the lock.
	pub fn current_file_name(&self) -> String {
		self.current_file
			.lock()
			.ok()
			.map(|g| g.clone())
			.unwrap_or_default()
	}
}
