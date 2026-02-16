//! Horizontal rule renderer.
//!
//! Detects and renders --- / *** / ___ separators
//! as styled box-drawing lines.

use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

/// Divider string for horizontal rules
const DIVIDER: &str =
	"\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\
	\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}";

/// Check if a trimmed line is a horizontal rule.
///
/// Matches 3+ of the same character (-, *, _)
/// with optional spaces between them.
pub fn is_rule(trimmed: &str) -> bool {
	let stripped: String = trimmed
		.chars()
		.filter(|chr| *chr != ' ')
		.collect();
	if stripped.len() < 3 {
		return false;
	}
	stripped.chars().all(|chr| chr == '-')
		|| stripped.chars().all(|chr| chr == '*')
		|| stripped.chars().all(|chr| chr == '_')
}

/// Render a horizontal rule line.
pub fn render() -> Line<'static> {
	Line::from(Span::styled(
		format!("  {}", DIVIDER),
		Style::new().fg(Color::DarkGray),
	))
}
