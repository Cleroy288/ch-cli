use std::mem;

use ratatui::{
	style::{Color, Modifier, Style},
	text::Span,
};

const LANE_COLORS: &[Color] = &[
	Color::Cyan,
	Color::Green,
	Color::Yellow,
	Color::Magenta,
	Color::Red,
	Color::LightBlue,
	Color::LightGreen,
	Color::LightRed,
];

/// Ref label span (branch/tag).
pub fn ref_span(name: &str) -> Span<'static> {
	let style = Style::default()
		.fg(Color::Cyan)
		.add_modifier(Modifier::BOLD);
	Span::styled(
		format!(" {name}"), style,
	)
}

pub fn lane_color(lane: usize) -> Color {
	LANE_COLORS[lane % LANE_COLORS.len()]
}

/// Merge consecutive same-style cells into spans.
pub(super) fn cells_to_spans(
	cells: &[(char, Style)],
) -> Vec<Span<'static>> {
	let mut spans = Vec::new();
	let mut buf = String::new();
	let mut cur = Style::default();
	for &(ch, style) in cells {
		if style != cur && !buf.is_empty() {
			let text = mem::take(&mut buf);
			spans.push(
				Span::styled(text, cur),
			);
		}
		cur = style;
		buf.push(ch);
	}
	if !buf.is_empty() {
		spans.push(Span::styled(buf, cur));
	}
	spans
}
