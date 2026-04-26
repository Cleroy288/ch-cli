use crate::app::App;
use crate::app::git_poll_watcher::stop_watcher;

impl App {
	/// Insert commit ref into input and close.
	pub(super) fn insert_git_reference(
		&mut self,
	) {
		let Some(node) =
			self.picker.selected_git_node()
		else { return };
		let text = format!(
			"[git:{} \"{}\"]",
			node.commit.short_hash,
			node.commit.message,
		);
		self.picker.clear_git_history();
		stop_watcher(self);
		self.esc_pressed_at = None;
		self.insert_text_at_cursor(&text);
	}

	/// Re-fetch git history for current repo.
	pub(super) fn refresh_git_history(
		&mut self,
	) {
		let repo = self.picker
			.git_history()
			.map(|h| h.repo_path.clone());
		if let Some(path) = repo {
			self.spawn_git_fetch(path);
		}
	}

	/// Close git history and stop watcher.
	pub(super) fn close_git_history(
		&mut self,
	) {
		stop_watcher(self);
		self.picker.clear_git_history();
		self.esc_pressed_at = None;
	}
}
