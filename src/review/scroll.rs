/// Scroll methods for the inline panel viewport.

use crate::domain::review::ScrollOffset;

use super::state::InlineBlocks;

const SCROLL_STEP: u16 = 3;
const PAGE_STEP: u16 = 20;

impl InlineBlocks {
	fn scroll_by(&self, delta: i32) {
		let cur =
			self.scroll_offset.get().val() as i32;
		let max = self.max_scroll.get() as i32;
		let next =
			cur.saturating_add(delta).clamp(0, max);
		self.scroll_offset.set(
			ScrollOffset(next as u16),
		);
	}

	pub fn set_max_scroll(&self, max: u16) {
		self.max_scroll.set(max);
	}

	pub fn scroll_up(&self) {
		self.scroll_by(-(SCROLL_STEP as i32));
	}

	pub fn scroll_down(&self) {
		self.scroll_by(SCROLL_STEP as i32);
	}

	pub fn scroll_page_up(&self) {
		self.scroll_by(-(PAGE_STEP as i32));
	}

	pub fn scroll_page_down(&self) {
		self.scroll_by(PAGE_STEP as i32);
	}

	pub fn scroll_offset(&self) -> ScrollOffset {
		self.scroll_offset.get()
	}

	pub fn set_scroll(&self, offset: ScrollOffset) {
		self.scroll_offset.set(offset);
	}

	pub fn clamp_scroll(&self, max: u16) {
		let v = self.scroll_offset.get().val();
		if v > max {
			self.scroll_offset.set(
				ScrollOffset(max),
			);
		}
	}
}
