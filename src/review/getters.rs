/// Read-only accessors for InlineBlocks.

use ratatui::layout::Rect;

use crate::domain::claude::ResponseIntent;
use crate::domain::review::BlockIdx;
use crate::ui::markdown::sections::ContentSection;

use super::state::{
	BlockMode, InlineBlock, InlineBlocks,
	InlineFocus,
};

impl InlineBlocks {
	pub fn blocks(&self) -> &[InlineBlock] {
		&self.blocks
	}

	pub fn block_mut(
		&mut self,
		idx: BlockIdx,
	) -> Option<&mut InlineBlock> {
		self.blocks.get_mut(idx.val())
	}

	pub fn focus(&self) -> Option<InlineFocus> {
		self.focus
	}

	pub fn mode(&self) -> BlockMode {
		self.mode
	}

	pub fn is_frozen(&self) -> bool {
		matches!(self.mode, BlockMode::Frozen)
	}

	pub fn content_area(&self) -> Rect {
		self.content_area.get()
	}

	pub fn set_content_area(&self, r: Rect) {
		self.content_area.set(r);
	}

	pub fn impl_step(&self) -> Option<BlockIdx> {
		self.impl_step
	}

	pub fn cached_sections(
		&self,
	) -> &[ContentSection] {
		&self.cached_sections
	}

	pub fn intent(&self) -> ResponseIntent {
		self.intent
	}

	pub fn is_implementation(&self) -> bool {
		matches!(
			self.intent,
			ResponseIntent::Implementation,
		)
	}
}
