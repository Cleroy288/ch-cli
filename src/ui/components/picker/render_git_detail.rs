use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::Line,
	widgets::{Block, Borders},
	Frame,
};

use crate::picker::Picker;
use crate::ui::styles::colors;

use super::render_git_detail_lines
	::build_detail_lines;

/// Render full commit detail overlay.
pub fn render_git_detail(
	frame: &mut Frame,
	area: Rect,
	picker: &Picker,
) {
	let Some(node) =
		picker.selected_git_node() else { return };
	let lines = build_detail_lines(node);
	let scroll = picker.git_detail_scroll();
	let block = detail_block();
	let inner = block.inner(area);
	frame.render_widget(block, area);
	write_scrolled(
		frame, inner, &lines, scroll,
	);
}

fn detail_block() -> Block<'static> {
	Block::default()
		.borders(Borders::ALL)
		.title(" Commit Detail ")
		.title_alignment(Alignment::Left)
		.style(
			Style::default().fg(colors::PICKER),
		)
}

fn write_scrolled(
	frame: &mut Frame,
	area: Rect,
	lines: &[Line<'_>],
	scroll: usize,
) {
	let buf = frame.buffer_mut();
	let h = area.height as usize;
	let total = lines.len();
	let skip =
		scroll.min(total.saturating_sub(h));
	for i in 0..h {
		let y = area.y + i as u16;
		if let Some(line) = lines.get(skip + i) {
			buf.set_line(
				area.x, y, line, area.width,
			);
		}
	}
}
