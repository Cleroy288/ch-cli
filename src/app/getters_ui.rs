use ratatui::layout::Rect;

use crate::domain::preflight::PreFlightData;
use crate::message::ConversationHistory;
use crate::review::InlineBlocks;

use super::App;

impl App {
	pub fn history(
		&self,
	) -> &ConversationHistory {
		&self.history
	}

	pub fn status_message(
		&self,
	) -> Option<&str> {
		self.status_message.as_deref()
	}

	pub fn set_status_message(
		&mut self,
		msg: Option<String>,
	) {
		self.status_message = msg;
	}

	pub fn scroll_offset(&self) -> u16 {
		self.scroll_offset
	}

	pub fn output_area(&self) -> Rect {
		self.output_area.get()
	}

	/// Stores Rect in a Cell for later hit-testing.
	pub fn set_output_area(&self, r: Rect) {
		self.output_area.set(r);
	}

	pub fn inline_blocks(
		&self,
	) -> Option<&InlineBlocks> {
		self.inline_blocks.as_ref()
	}

	pub fn set_block_ranges(
		&self,
		ranges: Vec<(u16, u16)>,
	) {
		*self.block_ranges.borrow_mut() = ranges;
	}

	pub fn block_ranges(
		&self,
	) -> Vec<(u16, u16)> {
		self.block_ranges.borrow().clone()
	}

	pub fn preflight(
		&self,
	) -> Option<&PreFlightData> {
		self.preflight.as_ref()
	}

	/// Drains the watcher slot into the status bar.
	pub fn tick_watcher_msg(&mut self) {
		let Some(ref slot) = self.watcher_msg
		else {
			return;
		};
		let taken = slot.lock().ok()
			.and_then(|mut g| g.take());
		if let Some(text) = taken {
			self.status_message = Some(text);
		}
	}
}
