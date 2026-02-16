//! Code block renderer.
//!
//! Renders fenced code blocks with styled
//! separators instead of raw backtick fences.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// Style for fence separators
const FENCE_STYLE: Style =
	Style::new().fg(Color::DarkGray);

/// Style for code content lines
const CODE_STYLE: Style =
	Style::new().fg(Color::Rgb(180, 180, 180));

/// Separator line for code blocks
const SEPARATOR: &str =
	"\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}";

/// Render a code block opening with language.
///
/// Shows a styled separator with the language
/// name, or a plain separator if no language.
pub fn render_open(lang: &str) -> Line<'static> {
	let text = if lang.is_empty() {
		format!("  {}", SEPARATOR)
	} else {
		format!(
			"  \u{2500}\u{2500} {} \u{2500}\u{2500}",
			lang,
		)
	};
	Line::from(Span::styled(text, FENCE_STYLE))
}

/// Render a code block closing separator.
pub fn render_close() -> Line<'static> {
	Line::from(Span::styled(
		format!("  {}", SEPARATOR),
		FENCE_STYLE,
	))
}

/// Render a code content line with indent.
pub fn render_line(line: &str) -> Line<'static> {
	Line::from(Span::styled(
		format!("  {}", line),
		CODE_STYLE,
	))
}
