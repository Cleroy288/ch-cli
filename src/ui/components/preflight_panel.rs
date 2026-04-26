/// Render the pre-flight confirmation panel.
///
/// Bordered box showing agents, mode, and counts
/// before sending to Claude.

use ratatui::{
	layout::Rect,
	text::Text,
	widgets::{Block, Borders, Paragraph},
	Frame,
};

use super::preflight_lines;
use crate::domain::preflight::PreFlightData;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

/// Draw the preflight panel centered in `area`.
pub fn render_preflight_panel(
	frame: &mut Frame,
	area: Rect,
	data: &PreFlightData,
) {
	let lines = preflight_lines::build_lines(data);
	let text = Text::from(lines);
	let block = Block::default()
		.borders(Borders::ALL)
		.border_style(
			ratatui::style::Style::default()
				.fg(colors::ACCENT),
		)
		.title(tui_labels::PREFLIGHT_TITLE);
	let paragraph =
		Paragraph::new(text).block(block);
	frame.render_widget(paragraph, area);
}
