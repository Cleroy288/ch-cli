use ratatui::{
	style::{Modifier, Style},
	text::{Line, Span},
};

use crate::domain::git_graph::GraphNode;
use crate::ui::styles::colors;

/// Build detail lines for a commit.
pub fn build_detail_lines(
	node: &GraphNode,
) -> Vec<Line<'static>> {
	let bold = bold_style();
	let dim = dim_style();
	let normal = normal_style();
	let mut lines = vec![
		detail_row("Commit  ", &node.commit.hash, dim, bold),
		detail_row("Author  ", &node.commit.author, dim, normal),
		detail_row("Date    ", &node.commit.date, dim, normal),
	];
	append_refs(&mut lines, node, dim, bold);
	append_parents(&mut lines, node, dim);
	lines.push(Line::from(""));
	lines.push(Line::from(Span::styled(
		node.commit.message.clone(), normal,
	)));
	lines
}

fn detail_row(
	label: &str,
	value: &str,
	dim: Style,
	val_style: Style,
) -> Line<'static> {
	Line::from(vec![
		Span::styled(label.to_string(), dim),
		Span::styled(value.to_string(), val_style),
	])
}

fn append_refs(
	lines: &mut Vec<Line<'static>>,
	node: &GraphNode,
	dim: Style,
	bold: Style,
) {
	if node.commit.refs.is_empty() {
		return;
	}
	let joined = node.commit.refs.join(", ");
	lines.push(Line::from(vec![
		Span::styled("Refs    ".to_string(), dim),
		Span::styled(joined, bold),
	]));
}

fn append_parents(
	lines: &mut Vec<Line<'static>>,
	node: &GraphNode,
	dim: Style,
) {
	if node.commit.parent_hashes.is_empty() {
		return;
	}
	let joined = node.commit.parent_hashes
		.iter()
		.map(|h| &h[..7.min(h.len())])
		.collect::<Vec<_>>()
		.join(" ");
	lines.push(Line::from(vec![
		Span::styled(
			"Parents ".to_string(), dim,
		),
		Span::styled(joined, normal_style()),
	]));
}

fn bold_style() -> Style {
	Style::default()
		.fg(colors::TEXT_LIGHT)
		.add_modifier(Modifier::BOLD)
}

fn dim_style() -> Style {
	Style::default().fg(colors::DIM_TEXT)
}

fn normal_style() -> Style {
	Style::default().fg(colors::INPUT_TEXT)
}
