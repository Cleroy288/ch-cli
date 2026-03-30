use std::sync::mpsc::TryRecvError;

use crate::picker::PickerMode;

use super::App;
use super::git_poll_watcher::start_watcher;

const GIT_ERROR_PREFIX: &str = "Git: ";
const GIT_DISCONNECTED: &str =
	"Git fetch disconnected";
const GIT_WATCH_ERROR: &str = "Git watch: ";

/// Poll for git history results (non-blocking).
pub fn tick_git_results(app: &mut App) {
	poll_initial_fetch(app);
	poll_watcher_updates(app);
}

/// Poll the one-shot git fetch channel.
fn poll_initial_fetch(app: &mut App) {
	let Some(recv) = &app.git_rx else {
		return;
	};
	match recv.try_recv() {
		Ok(Ok(history)) => {
			start_watcher(app, &history);
			app.picker
				.activate_git_history(history);
			app.status_message = None;
			app.git_rx = None;
		}
		Ok(Err(err)) => {
			app.picker.clear_git_history();
			app.status_message = Some(format!(
				"{GIT_ERROR_PREFIX}{err}",
			));
			app.git_rx = None;
		}
		Err(TryRecvError::Empty) => {}
		Err(TryRecvError::Disconnected) => {
			app.picker.clear_git_history();
			app.status_message = Some(
				GIT_DISCONNECTED.to_string(),
			);
			app.git_rx = None;
		}
	}
}

/// Poll the watcher channel for live updates.
fn poll_watcher_updates(app: &mut App) {
	let Some(recv) = &app.git_watch_rx else {
		return;
	};
	let is_git = matches!(
		app.picker.mode(),
		PickerMode::GitHistory
			| PickerMode::GitDetail,
	);
	match recv.try_recv() {
		Ok(Ok(history)) if is_git => {
			app.picker
				.update_git_history(history);
		}
		Ok(Ok(_)) => {}
		Ok(Err(err)) => {
			app.status_message = Some(format!(
				"{GIT_WATCH_ERROR}{err}",
			));
		}
		Err(TryRecvError::Empty) => {}
		Err(TryRecvError::Disconnected) => {
			app.git_watch_rx = None;
		}
	}
}
