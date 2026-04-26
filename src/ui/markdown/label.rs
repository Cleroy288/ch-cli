use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::ui::styles::colors;

const HEADER: Style = Style::new()
	.fg(colors::SEGMENT_LABEL)
	.add_modifier(Modifier::BOLD);
const PREFIX: Style =
	Style::new().fg(colors::SEPARATOR);

const LABELS: &[&str] = &[
	"Files analyzed",
	"Demand",
	"Analysis",
	"Problem",
	"Solution",
	"Summary",
	"Why",
];

pub fn is_format_label(trimmed: &str) -> bool {
	find_label(trimmed).is_some()
}

pub fn render_label(
	raw: &str,
) -> Vec<Line<'static>> {
	let trimmed = raw.trim_start();
	let Some(label) = find_label(trimmed) else {
		return vec![Line::from(
			super::inline::parse(raw),
		)];
	};
	let value = trimmed[label.len()..]
		.trim_start()
		.strip_prefix(':')
		.unwrap_or("")
		.trim_start();
	let header = build_header(label);
	if value.is_empty() {
		return vec![header];
	}
	vec![header, build_content(value)]
}

fn build_header(label: &str) -> Line<'static> {
	Line::from(vec![
		Span::styled(
			"  \u{251C}\u{2500} ",
			PREFIX,
		),
		Span::styled(label.to_owned(), HEADER),
	])
}

fn build_content(
	value: &str,
) -> Line<'static> {
	let mut spans = vec![Span::styled(
		"  \u{2502}  ",
		PREFIX,
	)];
	spans.extend(super::inline::parse(value));
	Line::from(spans)
}

fn find_label(
	trimmed: &str,
) -> Option<&'static str> {
	LABELS.iter().copied().find(|label| {
		trimmed.starts_with(label)
			&& trimmed[label.len()..]
				.trim_start()
				.starts_with(':')
	})
}
