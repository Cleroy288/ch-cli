use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::ui::styles::colors;

/// Left bar style
const BAR_STYLE: Style =
	Style::new().fg(colors::BLOCKQUOTE_BAR);

/// Quoted text style — gray italic
const QUOTE_STYLE: Style = Style::new()
	.fg(colors::SEGMENT_LABEL)
	.add_modifier(Modifier::ITALIC);

///
/// Strips the > prefix and applies italic
/// styling with inline formatting support.
pub fn render(line: &str) -> Line<'static> {
	let text = strip_prefix(line);
	let prefix =
		Span::styled("  \u{2502} ", BAR_STYLE);
	let mut spans = vec![prefix];
	for src in super::inline::parse(text) {
		spans.push(Span::styled(
			src.content.into_owned(),
			QUOTE_STYLE.patch(src.style),
		));
	}
	Line::from(spans)
}

/// Strip the > prefix from a quote line.
fn strip_prefix(line: &str) -> &str {
	let trimmed = line.trim_start();
	let after = trimmed
		.strip_prefix('>')
		.unwrap_or(trimmed);
	after.strip_prefix(' ').unwrap_or(after)
}
