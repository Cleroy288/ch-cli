use super::App;

/// Start the background git watcher.
pub fn start_watcher(
	app: &mut App,
	history: &crate::domain::git_graph::GitHistory,
) {
	use crate::service::git::watcher
		::spawn_git_watcher;
	use std::sync::mpsc;

	stop_watcher(app);
	let repo = history.repo_path.clone();
	let (tx, rx) = mpsc::channel();
	let handle = spawn_git_watcher(repo, tx);
	app.git_watch_rx = Some(rx);
	app.git_watch_handle = Some(handle);
}

/// Stop and clean up the git watcher.
pub fn stop_watcher(app: &mut App) {
	app.git_watch_rx = None;
	if let Some(handle) =
		app.git_watch_handle.take()
	{
		handle.stop();
	}
}
