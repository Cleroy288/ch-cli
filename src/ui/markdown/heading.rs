//! Heading renderer.
//!
//! Strips # prefix and applies level-based styles
//! with inline formatting support.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};

use crate::ui::styles::colors;

/// Render a markdown heading line.
///
/// Strips `#` markers, applies heading style,
/// and parses inline bold/italic/code within.
pub fn render(line: &str) -> Line<'static> {
	let trimmed = line.trim_start();
	let level = trimmed
		.chars()
		.take_while(|chr| *chr == '#')
		.count();
	let text = trimmed[level..].trim_start();
	let style = heading_style(level);
	let spans = apply_style(text, style);
	Line::from(spans)
}

/// Get style for heading level.
fn heading_style(level: usize) -> Style {
	match level {
		1 => Style::new()
			.fg(Color::Cyan)
			.add_modifier(
				Modifier::BOLD
					.union(Modifier::UNDERLINED),
			),
		2 => Style::new()
			.fg(Color::Cyan)
			.add_modifier(Modifier::BOLD),
		3 => Style::new()
			.fg(colors::AMBER)
			.add_modifier(Modifier::BOLD),
		_ => Style::new()
			.fg(colors::AMBER)
			.add_modifier(Modifier::ITALIC),
	}
}

/// Parse inline markers and merge heading style.
fn apply_style(
	text: &str,
	base: Style,
) -> Vec<Span<'static>> {
	super::inline::parse(text)
		.into_iter()
		.map(|src| {
			Span::styled(
				src.content.into_owned(),
				base.patch(src.style),
			)
		})
		.collect()
}
