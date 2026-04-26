use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::Line,
	widgets::{Block, Borders},
	Frame,
};

use crate::domain::git_graph::GitHistory;
use crate::picker::Picker;
use crate::ui::styles::colors;

use super::render_git_graph::build_compact_line;

/// Render "Loading..." placeholder.
pub fn render_git_loading(
	frame: &mut Frame,
	area: Rect,
) {
	let block = git_block(" Git History ");
	let inner = block.inner(area);
	frame.render_widget(block, area);
	let buf = frame.buffer_mut();
	let line =
		Line::from(" Loading git history...");
	buf.set_line(
		inner.x, inner.y, &line, inner.width,
	);
}

/// Render git history with Unicode graph.
pub fn render_git_history(
	frame: &mut Frame,
	area: Rect,
	picker: &Picker,
) {
	let Some(history) =
		picker.git_history()
	else {
		return;
	};
	let selected = picker.git_selected();
	let total = history.nodes.len();
	let title = format!(
		" Git History [{}/{}] ",
		if total == 0 { 0 } else { selected + 1 },
		total,
	);
	let block = git_block(&title);
	let inner = block.inner(area);
	frame.render_widget(block, area);
	if inner.width == 0
		|| inner.height == 0
	{
		return;
	}
	let h = inner.height as usize;
	let scroll = auto_scroll(selected, h);
	write_rows(
		frame, inner, history, selected,
		scroll, h,
	);
}

fn write_rows(
	frame: &mut Frame,
	area: Rect,
	history: &GitHistory,
	selected: usize,
	scroll: usize,
	visible: usize,
) {
	let buf = frame.buffer_mut();
	let w = area.width as usize;
	for i in 0..visible {
		let idx = scroll + i;
		let Some(node) =
			history.nodes.get(idx)
		else {
			break;
		};
		let is_sel = idx == selected;
		let line = build_compact_line(
			node, is_sel, w,
			history.max_lanes,
		);
		let y = area.y + i as u16;
		buf.set_line(
			area.x, y, &line, area.width,
		);
	}
}

fn auto_scroll(
	selected: usize,
	visible: usize,
) -> usize {
	if selected >= visible {
		selected - visible + 1
	} else {
		0
	}
}

fn git_block(title: &str) -> Block<'static> {
	Block::default()
		.borders(Borders::ALL)
		.title(title.to_string())
		.title_alignment(Alignment::Left)
		.style(
			Style::default().fg(colors::PICKER),
		)
}
