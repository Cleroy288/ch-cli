use ratatui::layout::Position;

use crate::domain::review::BlockIdx;
use crate::review::{InlineBlocks, InlineFocus};
use crate::ui::markdown::code_block::{
	FRAME_ROWS, FRAME_TOP_ROWS,
};
use crate::ui::markdown::code_metrics::LineCount;

use super::inline_panel::{
	section_layout, SectionSlot,
};

/// Returns `(focus, vy)` where `vy` is the
/// signed virtual-Y of the hit block.
pub fn hit_test_inline(
	inline: &InlineBlocks,
	col: u16,
	row: u16,
) -> Option<(InlineFocus, i32)> {
	let area = inline.content_area();
	if !area.contains(Position::new(col, row)) {
		return None;
	}
	let layout = section_layout(inline);
	let scroll =
		inline.scroll_offset().val() as i32;
	let btm = (area.y + area.height) as i32;
	let mut vy: i32 = area.y as i32 - scroll;
	for (i, slot) in layout.iter().enumerate() {
		if vy >= btm { break; }
		if i > 0 { vy += 1; }
		match slot {
			SectionSlot::Text { height, .. } => {
				vy += *height as i32;
			}
			SectionSlot::Code { idx, height } => {
				let slot_end =
					vy + *height as i32;
				if slot_end > area.y as i32 {
					let hit = hit_test_block(
						inline, *idx,
						vy, row,
					);
					if let Some(f) = hit {
						return Some((f, vy));
					}
				}
				vy += *height as i32;
			}
		}
	}
	None
}

fn hit_test_block(
	inline: &InlineBlocks,
	idx: BlockIdx,
	y: i32,
	row: u16,
) -> Option<InlineFocus> {
	let block =
		inline.blocks().get(idx.val())?;
	let frame_h = block.edited.line_count()
		+ FRAME_ROWS;
	let btm_rows =
		FRAME_ROWS - FRAME_TOP_ROWS;
	let r = row as i32;
	let code_top = y + FRAME_TOP_ROWS as i32;
	let code_end =
		y + frame_h as i32 - btm_rows as i32;
	if r >= code_top && r < code_end {
		return Some(
			InlineFocus::CodeBlock(idx),
		);
	}
	if !inline.is_frozen()
		&& r == y + frame_h as i32
	{
		return Some(InlineFocus::Question(idx));
	}
	None
}
