//! Output panel scroll handlers.

use crate::app::App;

/// Number of lines to scroll per key press
const SCROLL_STEP: u16 = 3;

impl App {
	/// Scroll the output panel up
	pub(crate) fn scroll_up(&mut self) {
		self.scroll_offset =
			self.scroll_offset.saturating_sub(
				SCROLL_STEP,
			);
	}

	/// Scroll the output panel down
	pub(crate) fn scroll_down(&mut self) {
		self.scroll_offset =
			self.scroll_offset.saturating_add(
				SCROLL_STEP,
			);
	}
}
