use ratatui::{
	style::{Modifier, Style},
	text::Span,
};

use crate::domain::git_graph::GraphNode;
use crate::ui::styles::colors;

use super::render_git_spans::ref_span;

/// Truncated message span.
pub fn message_span(
	node: &GraphNode,
	max_w: usize,
	selected: bool,
) -> Span<'static> {
	let msg = truncate(
		&node.commit.message, max_w,
	);
	let style = if selected {
		Style::default()
			.fg(colors::ACCENT)
			.add_modifier(Modifier::BOLD)
	} else {
		Style::default().fg(colors::TEXT_LIGHT)
	};
	Span::styled(msg, style)
}

/// Author + date dim span.
pub fn meta_span(
	node: &GraphNode,
) -> Span<'static> {
	let author = truncate(
		&node.commit.author, 12,
	);
	let dim =
		Style::default().fg(colors::DIM_TEXT);
	Span::styled(
		format!(
			"  {author}  {}",
			node.commit.date,
		),
		dim,
	)
}

/// First ref label (if any).
pub fn first_ref_span(
	node: &GraphNode,
) -> Option<Span<'static>> {
	node.commit.refs
		.first()
		.map(|r| ref_span(r))
}

fn truncate(s: &str, max: usize) -> String {
	if s.len() <= max {
		return s.to_string();
	}
	if max <= 3 {
		return s.chars().take(max).collect();
	}
	let end = max - 3;
	let t: String =
		s.chars().take(end).collect();
	format!("{t}...")
}
