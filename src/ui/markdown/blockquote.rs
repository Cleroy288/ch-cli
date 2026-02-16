//! Blockquote renderer.
//!
//! Renders > prefixed lines with a styled left
//! bar indicator and italic text.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

/// Left bar style
const BAR_STYLE: Style =
	Style::new().fg(Color::DarkGray);

/// Quoted text style — gray italic
const QUOTE_STYLE: Style = Style::new()
	.fg(Color::Gray)
	.add_modifier(Modifier::ITALIC);

/// Render a blockquote line with left bar.
///
/// Strips the > prefix and applies italic
/// styling with inline formatting support.
pub fn render(line: &str) -> Line<'static> {
	let text = strip_prefix(line);
	let bar =
		Span::styled("  \u{2502} ".to_string(), BAR_STYLE);
	let mut spans = vec![bar];
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
