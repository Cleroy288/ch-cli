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
		let manager = self.clone_with_persistence();

		let mut result = manager.index_project(&root)?;
		let mut watcher = FileWatcher::new(&root)?;
		watcher.start()?;

		result = watch_loop(
			&manager, &root, &stop_flag,
			&on_change, &watcher, result,
		)?;

		let _ = watcher.stop();
		Ok(result)
	}

	/// Create a copy of self with persistence enabled
	fn clone_with_persistence(&self) -> IndexManager {
		let mut flags = self.flags.clone();
		flags.persistence = true;
		IndexManager {
			crawler_config: self.crawler_config.clone(),
			progress_callback: None,
			flags,
		}
	}
}

/// Run the watch loop with debouncing
#[allow(
	clippy::too_many_lines,
	clippy::type_complexity,
	clippy::too_many_arguments
)]
fn watch_loop(
	manager: &IndexManager,
	root: &Path,
	stop_flag: &dyn Fn() -> bool,
	on_change: &dyn Fn(&[PathBuf]),
	watcher: &FileWatcher,
	mut result: IndexResult,
) -> IndexManagerResult<IndexResult> {
	let debounce = Duration::from_millis(500);
	let mut pending: Vec<PathBuf> = Vec::new();
	let mut last_change: Option<Instant> = None;

	while !stop_flag() {
		match watcher.wait(Duration::from_millis(100)) {
			Ok(Some(evt)) => {
				accumulate_changes(&mut pending, evt.paths);
				last_change = Some(Instant::now());
				continue;
			}
			Ok(None) => {}
			Err(WatcherError::Timeout) => continue,
			Err(err) => return Err(err.into()),
		}
		if !should_debounce_flush(last_change, debounce, &pending) {
			continue;
		}
		let changed = std::mem::take(&mut pending);
		last_change = None;
		result = manager.index_project(root)?;
		on_change(&changed);
	}
	Ok(result)
}

/// Accumulate unique changed paths
fn accumulate_changes(
	pending: &mut Vec<PathBuf>,
	paths: Vec<PathBuf>,
) {
	for path in paths {
		if !pending.contains(&path) {
			pending.push(path);
		}
	}
}

/// Check if debounce period has elapsed with pending changes
fn should_debounce_flush(
	last_change: Option<Instant>,
	debounce: Duration,
	pending: &[PathBuf],
) -> bool {
	let Some(last) = last_change else {
		return false;
	};
	last.elapsed() >= debounce && !pending.is_empty()
}
