use ratatui::{
	layout::{Alignment, Rect},
	style::Style,
	text::{Line, Span},
	widgets::Paragraph,
	Frame,
};

use crate::ui::styles::colors;

/// Title area — app name + model on one line.
pub fn render_title(
	frame: &mut Frame,
	area: Rect,
	model_name: &str,
) {
	let line = build_title_line(model_name);
	let paragraph = Paragraph::new(line)
		.alignment(Alignment::Center);
	frame.render_widget(paragraph, area);
}

fn build_title_line(
	model_name: &str,
) -> Line<'static> {
	Line::from(vec![
		Span::styled(
			"\u{25C8} rustean",
			Style::default()
				.fg(colors::ACCENT)
				.bold(),
		),
		Span::styled(
			" \u{00b7} ",
			Style::default().fg(colors::SEPARATOR),
		),
		Span::styled(
			model_name.to_owned(),
			Style::default().fg(colors::DIM_TEXT),
		),
	])
}
