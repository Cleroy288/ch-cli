use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::ui::styles::colors;

pub fn render(rows: &[&str]) -> Vec<Line<'static>> {
	let parsed: Vec<Vec<String>> = rows
		.iter()
		.filter(|row| {
			!row.trim().chars().all(|chr| {
				matches!(chr, '|' | '-' | ':' | ' ')
			})
		})
		.map(|row| parse_cells(row))
		.collect();

	if parsed.is_empty() {
		return Vec::new();
	}

	let widths = compute_widths(&parsed);
	let mut lines = Vec::new();

	for (idx, row) in parsed.iter().enumerate() {
		let hdr = idx == 0;
		lines.push(render_row(row, &widths, hdr));
		if hdr && parsed.len() > 1 {
			lines.push(render_divider(&widths));
		}
	}
	lines
}

/// Split a pipe-delimited row into cells.
fn parse_cells(row: &str) -> Vec<String> {
	let inner = row.trim().trim_matches('|');
	inner
		.split('|')
		.map(|cell| cell.trim().to_string())
		.collect()
}

fn compute_widths(
	rows: &[Vec<String>],
) -> Vec<usize> {
	let cols = rows
		.iter()
		.map(|row| row.len())
		.max()
		.unwrap_or(0);

	(0..cols)
		.map(|col| {
			rows.iter()
				.filter_map(|row| row.get(col))
				.map(|cell| cell.len())
				.max()
				.unwrap_or(0)
		})
		.collect()
}

fn render_row(
	cells: &[String],
	widths: &[usize],
	is_header: bool,
) -> Line<'static> {
	let style = if is_header {
		Style::new().add_modifier(Modifier::BOLD)
	} else {
		Style::default()
	};

	let text: String = cells
		.iter()
		.enumerate()
		.map(|(idx, cell)| {
			let wid =
				widths.get(idx).copied().unwrap_or(0);
			format!("{:<width$}", cell, width = wid)
		})
		.collect::<Vec<_>>()
		.join("  ");

	Line::from(Span::styled(
		format!("  {}", text),
		style,
	))
}

fn render_divider(
	widths: &[usize],
) -> Line<'static> {
	let text: String = widths
		.iter()
		.map(|wid| "\u{2500}".repeat(*wid))
		.collect::<Vec<_>>()
		.join("  ");

	Line::from(Span::styled(
		format!("  {}", text),
		Style::default().fg(colors::DEBUG),
	))
}
