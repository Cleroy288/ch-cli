//! Documentation Status Handlers
//!
//! Extracted from doc_handlers.rs for norm compliance.

use std::path::PathBuf;

use super::ModelDaemon;
use crate::retrieval::daemon::protocol::DaemonResponse;
use crate::retrieval::docgen::DocStore;

/// Handle DocGenStatus request
pub fn handle_doc_gen_status(
	daemon: &ModelDaemon,
	project_path: &str,
) -> DaemonResponse {
	let path = PathBuf::from(project_path);
	let canonical = path.canonicalize().unwrap_or(path);

	// read progress (brief lock)
	let bg = daemon
		.doc_gen_progress
		.lock()
		.unwrap()
		.clone();

	if bg.is_running {
		let total = daemon
			.doc_stores
			.lock()
			.ok()
			.and_then(|s| {
				s.get(&canonical).map(|st| st.len())
			})
			.unwrap_or(bg.total);
		return DaemonResponse::DocGenStatus {
			total,
			completed: bg.completed,
			pending: total.saturating_sub(bg.completed),
			is_ready: false,
			in_progress: true,
		};
	}

	// not running - read from store
	status_from_store(daemon, &canonical)
}

/// Get status from store (when bg gen is not running)
fn status_from_store(
	daemon: &ModelDaemon,
	canonical: &PathBuf,
) -> DaemonResponse {
	let stores = daemon.doc_stores.lock().unwrap();
	match stores.get(canonical) {
		Some(store) => {
			let stats = store.stats();
			DaemonResponse::DocGenStatus {
				total: stats.total,
				completed: stats.ready,
				pending: stats.pending,
				is_ready: stats.is_complete,
				in_progress: false,
			}
		}
		None => {
			drop(stores);
			match DocStore::load(canonical) {
				Ok(store) => {
					let stats = store.stats();
					DaemonResponse::DocGenStatus {
						total: stats.total,
						completed: stats.ready,
						pending: stats.pending,
						is_ready: stats.is_complete,
						in_progress: false,
					}
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
	}
}
