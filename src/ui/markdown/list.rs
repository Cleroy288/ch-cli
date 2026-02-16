//! List item renderer.
//!
//! Handles bullet (- * +) and numbered (1.)
//! list items with inline formatting support.

use ratatui::text::{Line, Span};

use super::inline;

/// Bullet character for unordered lists
const BULLET: &str = "\u{2022}";

/// Check if a trimmed line is a list item.
///
/// Matches: `- item`, `* item`, `+ item`,
/// and `1. item` style numbered lists.
pub fn is_item(trimmed: &str) -> bool {
	if trimmed.starts_with("- ")
		|| trimmed.starts_with("* ")
		|| trimmed.starts_with("+ ")
	{
		return true;
	}
	let dot = trimmed.find(". ");
	dot.map_or(false, |pos| {
		pos > 0
			&& trimmed[..pos]
				.chars()
				.all(|chr| chr.is_ascii_digit())
	})
}

/// Render a list item with marker and indent.
pub fn render_item(line: &str) -> Line<'static> {
	let trimmed = line.trim_start();
	let (marker, text) = split_marker(trimmed);
	let indent = line.len() - trimmed.len();
	let mut spans = vec![Span::from(format!(
		"  {}{} ",
		" ".repeat(indent),
		marker,
	))];
	spans.extend(inline::parse(text));
	Line::from(spans)
}

/// Split a list marker from its content text.
fn split_marker(trimmed: &str) -> (&str, &str) {
	if trimmed.starts_with("- ")
		|| trimmed.starts_with("* ")
		|| trimmed.starts_with("+ ")
	{
		return (BULLET, &trimmed[2..]);
	}
	let dot = trimmed.find(". ").unwrap_or(1);
	(&trimmed[..dot + 1], &trimmed[dot + 2..])
}
