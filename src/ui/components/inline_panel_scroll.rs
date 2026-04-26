use ratatui::layout::Rect;
use ratatui::widgets::{
	Scrollbar, ScrollbarOrientation,
	ScrollbarState,
};
use ratatui::Frame;

use crate::review::InlineBlocks;

pub(super) fn render_scrollbar(
	frame: &mut Frame,
	area: Rect,
	inline: &InlineBlocks,
	total_h: usize,
	vp_h: usize,
) {
	let max = total_h.saturating_sub(vp_h);
	let pos =
		inline.scroll_offset().val() as usize;
	let mut state = ScrollbarState::new(max)
		.position(pos)
		.viewport_content_length(vp_h);
	let sb = Scrollbar::new(
		ScrollbarOrientation::VerticalRight,
	)
	.begin_symbol(None)
	.end_symbol(None);
	frame.render_stateful_widget(
		sb, area, &mut state,
	);
}
