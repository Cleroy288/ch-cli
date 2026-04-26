use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::time::Duration;

use crate::domain::errors::git::GitResult;
use crate::domain::git_graph::GitHistory;

use super::watch_handle::GitWatchHandle;
use super::watch_mtime::git_mtime;
use super::{build_graph, fetch_git_log};

const POLL_INTERVAL_MS: u64 = 2000;
const SLEEP_SLICE_MS: u64 = 50;
const DEBOUNCE_MS: u64 = 500;

/// Spawn a git watcher thread for a repo.
pub fn spawn_git_watcher(
	repo: PathBuf,
	tx: mpsc::Sender<GitResult<GitHistory>>,
) -> GitWatchHandle {
	let stop = Arc::new(AtomicBool::new(false));
	let flag = stop.clone();
	let thread = std::thread::spawn(move || {
		watch_loop(&repo, &flag, &tx);
	});
	GitWatchHandle::new(stop, thread)
}

fn watch_loop(
	repo: &Path,
	stop: &Arc<AtomicBool>,
	tx: &mpsc::Sender<GitResult<GitHistory>>,
) {
	let mut snapshot = git_mtime(repo);
	loop {
		if !interruptible_sleep(
			stop, POLL_INTERVAL_MS,
		) {
			break;
		}
		let current = git_mtime(repo);
		if current <= snapshot {
			continue;
		}
		if !interruptible_sleep(
			stop, DEBOUNCE_MS,
		) {
			break;
		}
		snapshot = git_mtime(repo);
		let result = fetch_and_build(repo);
		if tx.send(result).is_err() {
			break;
		}
	}
}

fn fetch_and_build(
	repo: &Path,
) -> GitResult<GitHistory> {
	let commits = fetch_git_log(repo)?;
	Ok(build_graph(repo.to_path_buf(), commits))
}

fn interruptible_sleep(
	stop: &Arc<AtomicBool>,
	total_ms: u64,
) -> bool {
	let mut remaining = total_ms;
	while remaining > 0 {
		if stop.load(Ordering::Relaxed) {
			return false;
		}
		let ms = remaining.min(SLEEP_SLICE_MS);
		std::thread::sleep(
			Duration::from_millis(ms),
		);
		remaining =
			remaining.saturating_sub(ms);
	}
	!stop.load(Ordering::Relaxed)
}
