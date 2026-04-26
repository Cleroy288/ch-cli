use tui_textarea::CursorMove;

use crate::app::App;
use crate::domain::review::BlockIdx;
use crate::review::InlineFocus;
use crate::ui::components::inline_panel_hit;
use crate::ui::markdown::code_block;
use crate::ui::markdown::code_metrics;

impl App {
	/// Click in inline panel: focus the hit block,
	/// or clear focus if clicking outside any block.
	pub(crate) fn handle_inline_click(
		&mut self,
		col: u16,
		row: u16,
	) {
		let hit = match self
			.inline_blocks.as_ref()
		{
			None => return,
			Some(ib) => inline_panel_hit
				::hit_test_inline(ib, col, row),
		};
		match hit {
			Some((focus, vy)) => {
				apply_focus(
					self, focus, col, row, vy,
				);
			}
			None => {
				if let Some(ib) =
					&mut self.inline_blocks
				{
					ib.clear_focus();
				}
			}
		}
	}
}

fn apply_focus(
	app: &mut App,
	focus: InlineFocus,
	col: u16,
	row: u16,
	vy: i32,
) {
	let Some(ib) = app.inline_blocks.as_mut()
	else {
		return;
	};
	ib.set_focus(focus);
	if let InlineFocus::CodeBlock(idx) = focus {
		jump_cursor(ib, idx, col, row, vy);
	}
}

/// Position cursor at the clicked line.
/// `vy` is the signed virtual-Y from hit test.
fn jump_cursor(
	ib: &mut crate::review::InlineBlocks,
	idx: BlockIdx,
	col: u16,
	row: u16,
	vy: i32,
) {
	let Some(block) = ib.block_mut(idx) else {
		return;
	};
	let rect = block.render_rect.get();
	let count =
		block.edited.split('\n').count();
	let gutter =
		code_metrics::cursor_x_offset(count);
	let top = code_block::FRAME_TOP_ROWS as i32;
	let target_row =
		(row as i32 - vy - top).max(0) as u16;
	let target_col =
		col.saturating_sub(rect.x + gutter);
	block.editor.move_cursor(
		CursorMove::Jump(target_row, target_col),
	);
}
