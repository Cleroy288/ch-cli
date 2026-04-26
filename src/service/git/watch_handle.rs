use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Handle to stop the git watcher thread.
pub struct GitWatchHandle {
	stop: Arc<AtomicBool>,
	thread: Option<std::thread::JoinHandle<()>>,
}

impl GitWatchHandle {
	pub(super) fn new(
		stop: Arc<AtomicBool>,
		thread: std::thread::JoinHandle<()>,
	) -> Self {
		Self {
			stop,
			thread: Some(thread),
		}
	}

	/// Signal the watcher to stop.
	pub fn stop(mut self) {
		self.shutdown();
	}

	fn shutdown(&mut self) {
		self.stop.store(true, Ordering::SeqCst);
		// Don't join — thread exits within ~50ms
		drop(self.thread.take());
	}
}

impl Drop for GitWatchHandle {
	fn drop(&mut self) {
		self.shutdown();
	}
}
