use crate::picker::mode::PickerMode;

use super::state::Picker;

const MAX_GIT_DETAIL_SCROLL: usize = 200;

impl Picker {
	/// Current detail scroll offset.
	pub fn git_detail_scroll(&self) -> usize {
		self.git_detail_scroll
	}

	/// Enter detail view for selected commit.
	pub fn enter_git_detail(&mut self) {
		if self.selected_git_node().is_some() {
			self.git_detail_scroll = 0;
			self.mode = PickerMode::GitDetail;
		}
	}

	/// Return from detail to list.
	pub fn back_to_git_list(&mut self) {
		self.mode = PickerMode::GitHistory;
	}

	/// Scroll detail view down.
	pub fn scroll_git_detail_down(
		&mut self,
	) {
		if self.git_detail_scroll
			< MAX_GIT_DETAIL_SCROLL
		{
			self.git_detail_scroll += 1;
		}
	}

	/// Scroll detail view up.
	pub fn scroll_git_detail_up(&mut self) {
		self.git_detail_scroll =
			self.git_detail_scroll
				.saturating_sub(1);
	}
}
