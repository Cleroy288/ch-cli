//! Documentation Status Handlers
//!
//! Extracted from doc_handlers.rs for norm compliance.

use std::path::{Path, PathBuf};

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::DaemonResponse;
use crate::retrieval::docgen::{DocStore, DocStoreStats};

/// Build DocGenStatus response from store stats
fn status_from_stats(
	stats: DocStoreStats,
	in_progress: bool,
) -> DaemonResponse {
	DaemonResponse::DocGenStatus {
		total: stats.total,
		completed: stats.ready,
		pending: stats.pending,
		is_ready: stats.is_complete,
		in_progress,
	}
}

/// Build in-progress status from live progress data
fn in_progress_status(
	daemon: &ModelDaemon,
	canonical: &PathBuf,
	prog: &super::DocGenProgress,
) -> DaemonResponse {
	let total = daemon
		.doc_stores
		.lock()
		.ok()
		.and_then(|stores| {
			stores
				.get(canonical)
				.map(|store| store.len())
		})
		.unwrap_or(prog.total);
	DaemonResponse::DocGenStatus {
		total,
		completed: prog.completed,
		pending: total.saturating_sub(prog.completed),
		is_ready: false,
		in_progress: true,
	}
}

/// Handle DocGenStatus request
pub fn handle_doc_gen_status(
	daemon: &ModelDaemon,
	project_path: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical = path.canonicalize().unwrap_or(path);

	// read progress (brief lock)
	let Ok(prog_guard) =
		daemon.doc_gen_progress.lock()
	else {
		return DaemonResponse::Error(
			"lock poisoned".into(),
		);
	};
	let prog = prog_guard.clone();
	drop(prog_guard);

	if prog.is_running {
		return in_progress_status(
			daemon, &canonical, &prog,
		);
	}
	// not running - read from store
	status_from_store(daemon, &canonical)
}

/// Try loading status from disk store
fn load_from_disk(canonical: &Path) -> DaemonResponse {
	match DocStore::load(canonical) {
		Ok(store) => {
			status_from_stats(store.stats(), false)
		}
		Err(_) => DaemonResponse::DocGenStatus {
			total: 0,
			completed: 0,
			pending: 0,
			is_ready: false,
			in_progress: false,
		},
	}
}

/// Get status from store (when bg gen is not running)
fn status_from_store(
	daemon: &ModelDaemon,
	canonical: &PathBuf,
) -> DaemonResponse {
	let Ok(stores) = daemon.doc_stores.lock() else {
		return load_from_disk(canonical);
	};
	match stores.get(canonical) {
		Some(store) => {
			status_from_stats(store.stats(), false)
		}
		None => {
			drop(stores);
			load_from_disk(canonical)
		}
	}
}
