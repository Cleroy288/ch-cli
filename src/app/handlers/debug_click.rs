/// Click handler for the debug output panel.
///
/// Detects clicks on code blocks and activates
/// inline mode focused on the clicked block.

use ratatui::layout::Position;

use crate::app::App;
use crate::domain::content_section::ContentSection;
use crate::domain::review::{BlockIdx, ScrollOffset};
use crate::domain::review_path::extract_review_blocks;
use crate::review::{InlineBlocks, InlineFocus};
use crate::ui::components::debug_hit;
use crate::ui::markdown::code_block;

impl App {
	/// Click in the debug panel: if on a code
	/// block, enter inline mode focused on it.
	pub(crate) fn handle_debug_click(
		&mut self,
		col: u16,
		row: u16,
	) {
		let area = self.output_area.get();
		if area.width == 0 || area.height == 0 {
			return;
		}
		if !area.contains(
			Position::new(col, row),
		) {
			return;
		}
		let idx = debug_hit::hit_test_code_block(
			self, area, row,
		);
		let idx = match idx {
			Some(i) => i,
			None => return,
		};
		self.activate_inline_for(idx);
	}

	fn activate_inline_for(
		&mut self,
		block_idx: usize,
	) {
		let resp = match &self.last_claude_response
		{
			Some(r) => r,
			None => return,
		};
		let blocks = extract_review_blocks(
			&resp.result,
		);
		if blocks.is_empty() {
			return;
		}
		let result = resp.result.clone();
		self.inline_blocks = Some(
			InlineBlocks::activate(blocks, result),
		);
		if let Some(ib) = &mut self.inline_blocks {
			ib.set_focus(
				InlineFocus::CodeBlock(
					BlockIdx(block_idx),
				),
			);
			scroll_to_block(ib, block_idx);
		}
	}
}

/// Compute Y offset of block_idx in section
/// layout, then set scroll so it's visible.
fn scroll_to_block(
	ib: &InlineBlocks,
	target: usize,
) {
	let mut y: usize = 0;
	let mut ci: usize = 0;
	for (i, sec) in
		ib.cached_sections().iter().enumerate()
	{
		if i > 0 { y += 1; }
		match sec {
			ContentSection::Text(t) => {
				y += t.lines().count().max(1);
			}
			ContentSection::Code { .. } => {
				if ci == target {
					let off = y.saturating_sub(2);
					ib.set_scroll(
						ScrollOffset(off as u16),
					);
					return;
				}
				y += block_height(ib, ci);
				ci += 1;
			}
		}
	}
}

fn block_height(
	ib: &InlineBlocks,
	ci: usize,
) -> usize {
	let Some(block) = ib.blocks().get(ci) else {
		return 0;
	};
	let mut h = block.edited.split('\n').count()
		+ code_block::FRAME_ROWS as usize;
	if !ib.is_frozen() { h += 1; }
	if let Some(ref ans) = block.answer {
		h += ans.lines().count().max(1);
	}
	h
}
