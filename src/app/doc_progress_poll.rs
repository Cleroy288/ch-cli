//! Polling logic for doc generation progress.
//!
//! Handles tick-based polling of the daemon and
//! initial doc gen trigger on TUI startup.

use super::App;
use crate::retrieval::daemon::client::DaemonClient;

/// Tick the doc progress state, polling daemon if due.
///
/// Called every event loop iteration (~100ms).
/// Only polls daemon every 2 seconds to avoid overhead.
pub fn tick_doc_progress(
	app: &mut App,
	project_path: &str,
) {
	let progress = app.doc_progress_mut();
	if !progress.should_poll() {
		return;
	}
	poll_daemon_status(app, project_path);
}

/// Poll daemon for current doc gen status.
///
/// Silently ignores errors (daemon may be down).
fn poll_daemon_status(
	app: &mut App,
	project_path: &str,
) {
	let client = DaemonClient::new();
	let Ok(status) = client
		.doc_gen_status(project_path.to_string())
	else {
		return;
	};
	let progress = app.doc_progress_mut();
	progress.update(
		status.total,
		status.completed,
		status.in_progress,
	);
}

/// Trigger initial doc generation on TUI startup.
///
/// Non-blocking, force=false (skips if already done).
/// Silently ignores errors if daemon is not running.
pub fn trigger_initial_doc_gen(project_path: &str) {
	let client = DaemonClient::new();
	let _ = client.start_doc_gen(
		project_path.to_string(),
		false,
	);
}
