use ratatui::{
	style::{Color, Style},
	text::{Line, Span},
};

use crate::domain::git_graph::{
	Connector, GraphNode,
};

use super::render_git_compact::{
	first_ref_span, message_span, meta_span,
};
use super::render_git_spans::{
	cells_to_spans, lane_color,
};

/// Minimum chars reserved for commit message.
const MIN_MSG_BUDGET: usize = 15;

/// Compute effective lane count and message width.
fn compute_line_widths(
	node: &GraphNode,
	max_lanes: usize,
	width: usize,
) -> (usize, usize) {
	let meta = node.commit.author.len().min(12)
		+ node.commit.date.len() + 4;
	let ref_w = node.commit.refs
		.first()
		.map(|r| r.len() + 1)
		.unwrap_or(0);
	let lanes = max_lanes
		.min(width.saturating_sub(
			meta + ref_w + MIN_MSG_BUDGET,
		) / 2)
		.max(1);
	let msg_w = width
		.saturating_sub(lanes * 2 + 1)
		.saturating_sub(meta)
		.saturating_sub(ref_w);
	(lanes, msg_w)
}

/// Build a compact line: graph + msg + meta + ref.
pub fn build_compact_line(
	node: &GraphNode,
	selected: bool,
	width: usize,
	max_lanes: usize,
) -> Line<'static> {
	let (lanes, msg_w) =
		compute_line_widths(node, max_lanes, width);
	let cells =
		build_graph_cells(node, lanes, selected);
	let mut spans = cells_to_spans(&cells);
	spans.push(Span::raw(" "));
	spans.push(message_span(node, msg_w, selected));
	spans.push(meta_span(node));
	if let Some(r) = first_ref_span(node) {
		spans.push(r);
	}
	Line::from(spans)
}

/// 2-char-per-lane cell buffer with connectors.
fn build_graph_cells(
	node: &GraphNode,
	max_lanes: usize,
	selected: bool,
) -> Vec<(char, Style)> {
	let size = max_lanes * 2;
	let mut cells =
		vec![(' ', Style::default()); size];
	for col in 0..max_lanes {
		let pos = col * 2;
		if col == node.column {
			let color = if selected {
				Color::White
			} else {
				lane_color(col)
			};
			cells[pos] = (
				'\u{25cf}',
				Style::default()
					.fg(color)
					.add_modifier(
						ratatui::style::Modifier::BOLD,
					),
			);
			continue;
		}
		let Some(conn) =
			node.connectors.get(col)
		else {
			continue;
		};
		let sty =
			Style::default().fg(lane_color(col));
		match conn {
			Connector::Pipe => {
				cells[pos] =
					('\u{2502}', sty);
			}
			Connector::Fork
			| Connector::Merge => {
				let f = matches!(
					conn, Connector::Fork,
				);
				let r = col > node.column;
				let ch = match (f, r) {
					(true, true) => '\u{256e}',
					(true, _) => '\u{256d}',
					(_, true) => '\u{256f}',
					_ => '\u{2570}',
				};
				cells[pos] = (ch, sty);
				fill_horizontal(
					&mut cells, node.column,
					col, sty,
				);
			}
			Connector::Empty => {}
		}
	}
	apply_merge_to(&mut cells, node);
	cells
}

/// Draw junction + horizontal for each merge_to lane.
fn apply_merge_to(
	cells: &mut [(char, Style)],
	node: &GraphNode,
) {
	for &target in &node.merge_to {
		let t_pos = target * 2;
		if t_pos >= cells.len() {
			continue;
		}
		let sty = Style::default()
			.fg(lane_color(node.column));
		let ch = if target < node.column {
			'\u{251c}'
		} else {
			'\u{2524}'
		};
		cells[t_pos] = (ch, sty);
		fill_horizontal(
			cells, node.column, target, sty,
		);
	}
}

/// Fill empty cells between two lanes with ─.
fn fill_horizontal(
	cells: &mut [(char, Style)],
	from_col: usize,
	to_col: usize,
	style: Style,
) {
	let left = from_col.min(to_col);
	let right = from_col.max(to_col);
	let start = left * 2 + 1;
	let end = right * 2;
	for pos in start..end {
		if pos < cells.len()
			&& cells[pos].0 == ' '
		{
			cells[pos] = ('\u{2500}', style);
		}
	}
}
