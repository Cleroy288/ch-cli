use ratatui::text::{Line, Span};

use super::{
	blockquote, code_block, heading, inline,
	list, rule, table,
	sections::{self, ContentSection},
};

///
/// Splits into sections, then renders each section
/// with the appropriate renderer. Adds 1-space
/// left margin so content aligns with role headers.
pub fn render_markdown(
	text: &str,
) -> Vec<Line<'static>> {
	let secs = sections::parse_sections(text);
	let mut lines = render_sections(&secs);
	for line in &mut lines {
		if !line.spans.is_empty() {
			line.spans.insert(
				0, Span::from("  "),
			);
		}
	}
	lines
}

///
/// Adds blank lines around code blocks for
/// visual separation from surrounding text.
fn render_sections(
	sections: &[ContentSection],
) -> Vec<Line<'static>> {
	let mut out = Vec::new();
	for (i, section) in sections.iter().enumerate() {
		match section {
			ContentSection::Text(text) => {
				out.extend(
					render_text_lines(text),
				);
			}
			ContentSection::Code { lang, code } => {
				if i > 0 { ensure_blank(&mut out); }
				let lines: Vec<&str> =
					code.split('\n').collect();
				out.push(
					code_block::render_open(lang),
				);
				out.extend(
					code_block::render_lines(
						lang, &lines,
					),
				);
				out.push(code_block::render_close());
				ensure_blank(&mut out);
			}
		}
	}
	out
}

fn ensure_blank(lines: &mut Vec<Line<'static>>) {
	let blank = lines
		.last()
		.is_some_and(|l| l.spans.is_empty());
	if !blank {
		lines.push(Line::from(""));
	}
}

/// blockquotes, lists, and plain inline text.
fn render_text_lines(
	text: &str,
) -> Vec<Line<'static>> {
	let lines: Vec<&str> =
		text.split('\n').collect();
	let mut out = Vec::new();
	let mut idx = 0;

	while idx < lines.len() {
		let trimmed = lines[idx].trim_start();
		if trimmed.starts_with('#') {
			out.push(heading::render(lines[idx]));
			idx += 1;
		} else if trimmed.starts_with('|') {
			let start = idx;
			while idx < lines.len()
				&& lines[idx]
					.trim_start()
					.starts_with('|')
			{
				idx += 1;
			}
			out.extend(
				table::render(&lines[start..idx]),
			);
		} else if rule::is_rule(trimmed) {
			idx += 1;
		} else {
			out.push(
				render_leaf(trimmed, lines[idx]),
			);
			idx += 1;
		}
	}
	out
}

fn render_leaf(
	trimmed: &str,
	raw: &str,
) -> Line<'static> {
	if trimmed.starts_with('>') {
		return blockquote::render(raw);
	}
	if list::is_item(trimmed) {
		return list::render_item(raw);
	}
	if raw.is_empty() {
		return Line::from("");
	}
	Line::from(inline::parse(raw))
}
