//! Block-level markdown parser.
//!
//! Walks lines with a state machine, dispatching
//! headings, code blocks, tables, lists, quotes,
//! and rules to specialized renderers.

use ratatui::text::Line;

use super::{
	blockquote, code_block, heading, inline, list,
	rule, table,
};

/// Parse markdown text into styled ratatui lines.
pub fn render_markdown(
	text: &str,
) -> Vec<Line<'static>> {
	let lines: Vec<&str> = text.lines().collect();
	let mut result = Vec::new();
	let mut idx = 0;

	while idx < lines.len() {
		let trimmed = lines[idx].trim_start();
		if trimmed.starts_with("```") {
			idx = process_code_block(
				&lines, idx, &mut result,
			);
		} else if trimmed.starts_with('#') {
			result.push(heading::render(lines[idx]));
			idx += 1;
		} else if trimmed.starts_with('|') {
			idx = process_table(
				&lines, idx, &mut result,
			);
		} else if rule::is_rule(trimmed) {
			result.push(rule::render());
			idx += 1;
		} else if trimmed.starts_with('>') {
			result.push(
				blockquote::render(lines[idx]),
			);
			idx += 1;
		} else if list::is_item(trimmed) {
			result.push(
				list::render_item(lines[idx]),
			);
			idx += 1;
		} else {
			result.push(render_text_line(lines[idx]));
			idx += 1;
		}
	}
	result
}

/// Collect code block lines between ``` fences.
fn process_code_block(
	lines: &[&str],
	start: usize,
	out: &mut Vec<Line<'static>>,
) -> usize {
	let fence = lines[start].trim_start();
	let lang =
		fence.trim_start_matches('`').trim();
	out.push(code_block::render_open(lang));

	let mut idx = start + 1;
	while idx < lines.len() {
		let trimmed = lines[idx].trim_start();
		if trimmed.starts_with("```") {
			out.push(code_block::render_close());
			return idx + 1;
		}
		out.push(code_block::render_line(lines[idx]));
		idx += 1;
	}
	out.push(code_block::render_close());
	idx
}

/// Collect consecutive table rows.
fn process_table(
	lines: &[&str],
	start: usize,
	out: &mut Vec<Line<'static>>,
) -> usize {
	let mut rows = Vec::new();
	let mut idx = start;

	while idx < lines.len() {
		let trimmed = lines[idx].trim_start();
		if !trimmed.starts_with('|') {
			break;
		}
		rows.push(lines[idx]);
		idx += 1;
	}
	out.extend(table::render(&rows));
	idx
}

/// Render a text line with inline styles.
fn render_text_line(
	line: &str,
) -> Line<'static> {
	if line.is_empty() {
		return Line::from("");
	}
	Line::from(inline::parse(line))
}
