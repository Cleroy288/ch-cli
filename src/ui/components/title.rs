use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::{Line, Span},
	widgets::{Block, Borders, Paragraph},
	Frame,
};

use crate::app::doc_progress::DocProgressState;
use crate::ui::styles::colors;

/// Build the "RUSTEAN" ASCII art lines.
///
/// Returns styled spans with gradient from bright
/// orange to darker orange across the letters.
fn build_ascii_lines() -> Vec<Line<'static>> {
	let bright =
		Style::default().fg(colors::RUST_ORANGE).bold();
	let dim =
		Style::default().fg(colors::DARK_ORANGE);

	let lines_raw = [
		"█▀█ █ █ █▀▀ ▀█▀ █▀▀ ▄▀█ █▄ █",
		"█▀▄ █▄█ ▄▄█  █  ██▄ █▀█ █ ▀█",
	];

	lines_raw
		.iter()
		.enumerate()
		.map(|(idx, &line_str)| {
			let style = if idx == 0 { bright } else { dim };
			Line::from(Span::styled(
				line_str.to_string(),
				style,
			))
		})
		.collect()
}

/// Build a styled status message span
fn build_status_span(msg: &str) -> Line<'static> {
	let style = Style::default().fg(colors::AMBER);
	Line::from(Span::styled(
		msg.to_string(), style,
	))
	.left_aligned()
}

/// Build the title block with optional progress/status.
fn build_title_block(
	doc_progress: &DocProgressState,
	status_msg: Option<&str>,
) -> Block<'static> {
	let title_style = Style::default()
		.fg(colors::RUST_ORANGE)
		.bold();
	let mut block = Block::default()
		.borders(Borders::ALL)
		.border_style(Style::default().fg(colors::BORDER))
		.title(Span::styled(" rustean ", title_style))
		.title_alignment(Alignment::Center);
	if let Some(msg) = status_msg {
		block = block
			.title_bottom(build_status_span(msg));
	}
	let progress =
		super::title_progress::build_progress_span(
			doc_progress,
		);
	if let Some(span) = progress {
		block = block.title_bottom(
			Line::from(span).right_aligned(),
		);
	}
	block
}

/// Render the "RUSTEAN" title block.
///
/// Compact 2-line ASCII art with dark border.
/// Shows status message (bottom-left) and doc
/// gen progress (bottom-right) when present.
pub fn render_title(
	frame: &mut Frame,
	area: Rect,
	doc_progress: &DocProgressState,
	status_msg: Option<&str>,
) {
	let text = build_ascii_lines();
	let block =
		build_title_block(doc_progress, status_msg);
	let paragraph = Paragraph::new(text)
		.block(block)
		.alignment(Alignment::Center);
	frame.render_widget(paragraph, area);
}
