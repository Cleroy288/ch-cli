//! Auto-update watch mode for file changes.
//!
//! Watches for file changes, re-indexes, and triggers doc regeneration.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::indexer::{FileWatcher, IndexManager};
use crate::retrieval::daemon::DaemonClient;

/// Debounce delay before triggering reindex after file changes
const DEBOUNCE_MS: u64 = 500;

/// Poll interval for checking file events
const POLL_TIMEOUT_MS: u64 = 100;

/// Handle to the background watcher thread
pub struct WatcherHandle {
	/// flag to signal the watcher to stop
	stop_flag: Arc<AtomicBool>,
	/// handle to the watcher thread
	thread_handle: Option<std::thread::JoinHandle<()>>,
}

impl WatcherHandle {
	/// Stop the watcher and wait for the thread to finish
	pub fn stop(mut self) {
		self.stop_flag.store(true, Ordering::SeqCst);
		if let Some(handle) = self.thread_handle.take() {
			let _ = handle.join();
		}
	}
}

/// Spawn background watcher thread for auto-reindex
pub fn spawn_watcher(project_path: &Path) -> WatcherHandle {
	// shared stop signal
	let stop_flag = Arc::new(AtomicBool::new(false));
	// clone for the thread
	let stop_clone = stop_flag.clone();
	// owned path for thread
	let path = project_path.to_path_buf();

	let handle = std::thread::spawn(move || {
		watcher_loop(&path, &stop_clone);
	});

	WatcherHandle {
		stop_flag,
		thread_handle: Some(handle),
	}
}

/// Main watcher loop
///
/// Watches for changes, debounces, reindexes, regenerates docs.
fn watcher_loop(
	project_path: &Path,
	stop_flag: &Arc<AtomicBool>,
) {
	let mut watcher = match FileWatcher::new(project_path) {
		Ok(w) => w,
		Err(e) => {
			eprintln!("[watcher] Failed to create watcher: {}", e);
			return;
		}
	};

	if let Err(e) = watcher.start() {
		eprintln!("[watcher] Failed to start watching: {}", e);
		return;
	}

	eprintln!(
		"[watcher] Watching {} for changes...",
		project_path.display()
	);

	// timestamp of last detected change
	let mut last_change: Option<Instant> = None;
	// accumulated changed file paths
	let mut changed_paths: Vec<PathBuf> = Vec::new();
	// debounce delay
	let debounce = Duration::from_millis(DEBOUNCE_MS);

	loop {
		// check stop flag
		if stop_flag.load(Ordering::Relaxed) {
			break;
		}

		// poll for file events
		match watcher
			.wait(Duration::from_millis(POLL_TIMEOUT_MS))
		{
			Ok(Some(event)) => {
				changed_paths.extend(event.paths);
				last_change = Some(Instant::now());
			}
			Ok(None) => {} // timeout, no events
			Err(e) => {
				eprintln!("[watcher] Error: {}", e);
			}
		}

		// check debounce: if changes accumulated and
		// debounce expired, trigger update
		if let Some(last) = last_change {
			if last.elapsed() >= debounce
				&& !changed_paths.is_empty()
			{
				// number of changed files
				let count = changed_paths.len();
				eprintln!(
					"[watcher] {} file(s) changed, \
					reindexing...",
					count
				);

				// reindex project
				let manager = IndexManager::new()
					.with_persistence()
					.with_semantic_analysis()
					.with_reference_extraction();

				match manager.index_project(project_path) {
					Ok(result) => {
						eprintln!(
							"[watcher] Reindexed: {} \
							symbols",
							result.symbols.len()
						);
					}
					Err(e) => {
						eprintln!(
							"[watcher] Reindex error: {}",
							e
						);
					}
				}

				// trigger doc regeneration via daemon
				trigger_doc_update(project_path);

				// reset state
				changed_paths.clear();
				last_change = None;
			}
		}
	}

	let _ = watcher.stop();
	eprintln!("[watcher] Stopped");
}

/// Send StartDocGen request to daemon
fn trigger_doc_update(project_path: &Path) {
	// daemon client for IPC
	let client = DaemonClient::new();
	// path as string
	let path_str = project_path.to_string_lossy().to_string();

	// check if daemon is alive before sending
	if client.ping().is_err() {
		return;
	}

	match client.start_doc_gen(path_str, false) {
		Ok(_) => {
			eprintln!("[watcher] Doc update triggered")
		}
		Err(e) => {
			eprintln!("[watcher] Doc update failed: {}", e)
		}
	}
}
