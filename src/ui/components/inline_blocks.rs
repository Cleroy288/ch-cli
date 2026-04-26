use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::Frame;

use crate::review::InlineBlock;
use crate::ui::markdown::code_block;
use crate::ui::markdown::code_metrics::{
	self, SatU16,
};
use crate::ui::markdown::inline;
use crate::ui::markdown::render_markdown;
use crate::ui::styles::colors;

const BLOCK_CURSOR: &str = "\u{2588}";

/// Overlay reverse-video cursor on focused code.
/// `skip` = lines scrolled off the top.
pub fn overlay_cursor(
	frame: &mut Frame,
	rect: Rect,
	block: &InlineBlock,
	skip: u16,
) {
	let (row, col) = block.editor.cursor();
	let abs_row = code_block::FRAME_TOP_ROWS
		+ row.sat_u16();
	if abs_row < skip { return; }

	let count =
		block.edited.split('\n').count();
	let gutter =
		code_metrics::cursor_x_offset(count);
	let x = rect.x
		.saturating_add(gutter)
		.saturating_add(col.sat_u16());
	let y = rect.y + (abs_row - skip);

	if x >= rect.x + rect.width
		|| y >= rect.y + rect.height
	{
		return;
	}
	if let Some(cell) = frame.buffer_mut().cell_mut(
		ratatui::layout::Position::new(x, y),
	) {
		cell.set_style(
			Style::default()
				.add_modifier(Modifier::REVERSED),
		);
	}
}

pub fn render_text_section(
	frame: &mut Frame,
	rect: Rect,
	text: &str,
	skip: u16,
) {
	let lines: Vec<Line> = text
		.lines()
		.map(|l| Line::from(inline::parse(l)))
		.collect();
	let buf = frame.buffer_mut();
	for i in 0..rect.height as usize {
		let y = rect.y + i as u16;
		let src = skip as usize + i;
		if let Some(line) = lines.get(src) {
			buf.set_line(
				rect.x, y, line, rect.width,
			);
		}
	}
}

pub fn render_question_input(
	frame: &mut Frame,
	rect: Rect,
	question: &str,
	focused: bool,
) {
	let cursor =
		if focused { BLOCK_CURSOR } else { "" };
	let fg = if focused {
		Color::White
	} else {
		colors::DEBUG
	};
	let line = Line::from(vec![
		Span::styled(
			"  Ask: ",
			Style::default().fg(colors::DEBUG),
		),
		Span::styled(
			question.to_owned(),
			Style::default().fg(fg),
		),
		Span::styled(
			cursor,
			Style::default().fg(colors::FOCUS),
		),
	]);
	let buf = frame.buffer_mut();
	buf.set_line(rect.x, rect.y, &line, rect.width);
}

pub fn render_answer(
	frame: &mut Frame,
	rect: Rect,
	answer: &str,
	skip: u16,
) {
	let lines = render_markdown(answer);
	let buf = frame.buffer_mut();
	let sk = skip as usize;
	for i in 0..rect.height as usize {
		let y = rect.y + i as u16;
		if let Some(line) = lines.get(sk + i) {
			buf.set_line(
				rect.x, y, line, rect.width,
			);
		}
	}
}
