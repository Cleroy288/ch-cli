use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crate::fs::FileCache;
use crate::indexer::{FileWatcher, IndexManager};

use super::watcher::{
	WatcherMsg, set_watcher_msg,
};

/// Debounce delay before triggering reindex
const DEBOUNCE_MS: u64 = 500;

fn should_flush_changes(
	last_change: &Option<Instant>,
	debounce: Duration,
	has_changes: bool,
) -> bool {
	last_change
		.is_some_and(|t| t.elapsed() >= debounce)
		&& has_changes
}

pub(super) fn run_watch_loop(
	watcher: &mut FileWatcher,
	project_path: &Path,
	stop_flag: &Arc<AtomicBool>,
	file_cache: &FileCache,
	msg: &WatcherMsg,
) {
	let mut last_change: Option<Instant> = None;
	let mut changed: Vec<PathBuf> = Vec::new();
	let debounce =
		Duration::from_millis(DEBOUNCE_MS);

	loop {
		if stop_flag.load(Ordering::Relaxed) {
			break;
		}
		poll_events(
			watcher, &mut changed,
			&mut last_change, msg,
		);
		let flush = should_flush_changes(
			&last_change, debounce,
			!changed.is_empty(),
		);
		if flush {
			flush_changes(
				project_path, file_cache,
				&mut changed,
				&mut last_change, msg,
			);
		}
	}
}

/// Reindex and refresh file cache
fn flush_changes(
	project_path: &Path,
	file_cache: &FileCache,
	changed: &mut Vec<PathBuf>,
	last_change: &mut Option<Instant>,
	msg: &WatcherMsg,
) {
	set_watcher_msg(msg, &format!(
		"{} file(s) changed, reindexing...",
		changed.len()
	));
	reindex_project(project_path, msg);
	file_cache.refresh();
	changed.clear();
	*last_change = None;
}

/// Poll for file events from the watcher
fn poll_events(
	watcher: &mut FileWatcher,
	changed_paths: &mut Vec<PathBuf>,
	last_change: &mut Option<Instant>,
	msg: &WatcherMsg,
) {
	let timeout =
		Duration::from_millis(super::watcher::POLL_MS);
	match watcher.wait(timeout) {
		Ok(Some(event)) => {
			changed_paths.extend(event.paths);
			*last_change = Some(Instant::now());
		}
		Ok(None) => {}
		Err(err) => {
			set_watcher_msg(msg, &format!(
				"Watcher error: {}", err
			));
		}
	}
}

fn reindex_project(
	project_path: &Path,
	msg: &WatcherMsg,
) {
	let manager = IndexManager::new()
		.with_persistence()
		.with_semantic_analysis()
		.with_reference_extraction();

	match manager.index_project(project_path) {
		Ok(result) => {
			set_watcher_msg(msg, &format!(
				"Reindexed: {} symbols",
				result.symbols.len()
			));
		}
		Err(err) => {
			set_watcher_msg(msg, &format!(
				"Reindex error: {}", err
			));
		}
	}
}
