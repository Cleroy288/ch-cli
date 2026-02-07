//! File watching logic for auto-reindexing.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use crate::indexer::watcher::{FileWatcher, WatcherError};

use super::builder::IndexManager;
use super::error::IndexManagerResult;
use super::types::IndexResult;

impl IndexManager {
	/// Watch a project directory for changes and auto-reindex
	///
	/// This method:
	/// 1. Performs an initial full/incremental index
	/// 2. Starts watching for file changes
	/// 3. Re-indexes changed files automatically
	/// 4. Calls the optional callback when changes are detected
	///
	/// The watch loop runs until `stop_flag` returns true.
	pub fn watch_project<P, F, C>(
		&self,
		root: P,
		stop_flag: F,
		on_change: C,
	) -> IndexManagerResult<IndexResult>
	where
		P: AsRef<Path>,
		F: Fn() -> bool,
		C: Fn(&[PathBuf]),
	{
		let root = root
			.as_ref()
			.canonicalize()
			.unwrap_or_else(|_| root.as_ref().to_path_buf());

		// Enable persistence for watch mode
		let manager_with_persistence = IndexManager {
			crawler_config: self.crawler_config.clone(),
			progress_callback: None, // Don't use progress in watch mode
			enable_semantic_analysis: self.enable_semantic_analysis,
			enable_reference_extraction: self.enable_reference_extraction,
			enable_persistence: true, // Force persistence for watch mode
		};

		// Initial index
		let mut result = manager_with_persistence.index_project(&root)?;

		// Start file watcher
		let mut watcher = FileWatcher::new(&root)?;
		watcher.start()?;

		// Watch loop with debouncing
		let debounce_duration = Duration::from_millis(500);
		let mut pending_changes: Vec<PathBuf> = Vec::new();
		let mut last_change_time: Option<Instant> = None;

		while !stop_flag() {
			// Poll for changes with timeout
			match watcher.wait(Duration::from_millis(100)) {
				Ok(Some(event)) => {
					// Accumulate changes
					for path in event.paths {
						if !pending_changes.contains(&path) {
							pending_changes.push(path);
						}
					}
					last_change_time = Some(Instant::now());
				}
				Ok(None) => {
					// Check if we should process pending changes (debounce)
					if let Some(last_time) = last_change_time {
						if last_time.elapsed() >= debounce_duration && !pending_changes.is_empty()
						{
							// Process accumulated changes
							let changed_paths = std::mem::take(&mut pending_changes);
							last_change_time = None;

							// Re-index the project (incremental)
							result = manager_with_persistence.index_project(&root)?;

							// Notify callback
							on_change(&changed_paths);
						}
					}
				}
				Err(WatcherError::Timeout) => continue,
				Err(e) => return Err(e.into()),
			}
		}

		// Stop watcher
		let _ = watcher.stop();

		Ok(result)
	}
}
