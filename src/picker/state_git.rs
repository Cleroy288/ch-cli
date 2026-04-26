use crate::domain::git_graph::GitHistory;
use crate::picker::mode::PickerMode;

use super::state::Picker;

impl Picker {
	/// Enter loading state while git log runs.
	pub fn activate_git_loading(&mut self) {
		self.mode = PickerMode::GitLoading;
	}

	/// Store history and enter list mode.
	pub fn activate_git_history(
		&mut self,
		history: GitHistory,
	) {
		self.git_history = Some(history);
		self.git_selected = 0;
		self.mode = PickerMode::GitHistory;
	}

	/// Update history, preserve selection.
	pub fn update_git_history(
		&mut self,
		history: GitHistory,
	) {
		let max = history.nodes.len();
		self.git_history = Some(history);
		if max == 0 {
			self.git_selected = 0;
		} else if self.git_selected >= max {
			self.git_selected = max - 1;
		}
	}

	/// Clear git history and return to Inactive.
	pub fn clear_git_history(&mut self) {
		self.git_history = None;
		self.git_selected = 0;
		self.mode = PickerMode::Inactive;
	}
}
