use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use super::code_metrics::digit_width;
use super::syntax;
use crate::ui::styles::colors;

const FRAME: Style =
	Style::new().fg(colors::SEPARATOR);
const LABEL: Style = Style::new()
	.fg(colors::ACCENT)
	.add_modifier(Modifier::BOLD);
const LINE_NUM: Style =
	Style::new().fg(colors::SEPARATOR);
const HDASH: &str = "\u{2500}";
const FRAME_DASHES: usize = 24;

pub const FRAME_TOP_ROWS: u16 = 2;
const FRAME_BOTTOM_ROWS: u16 = 1;
/// Top + bottom frame rows
pub const FRAME_ROWS: u16 =
	FRAME_TOP_ROWS + FRAME_BOTTOM_ROWS;

/// Frame open line with optional language label
pub fn render_open(lang: &str) -> Line<'static> {
	if lang.is_empty() {
		return Line::from(Span::styled(
			format!(
				"  \u{250C}{}",
				HDASH.repeat(FRAME_DASHES),
			),
			FRAME,
		));
	}
	// "─ " + lang + " " = 2+len+1 used dashes
	let used = 2 + lang.len() + 1;
	let rest = FRAME_DASHES.saturating_sub(used);
	Line::from(vec![
		Span::styled("  \u{250C}\u{2500} ", FRAME),
		Span::styled(lang.to_owned(), LABEL),
		Span::styled(
			format!(" {}", HDASH.repeat(rest)),
			FRAME,
		),
	])
}

/// Frame close line
pub fn render_close() -> Line<'static> {
	Line::from(Span::styled(
		format!(
			"  \u{2514}{}",
			HDASH.repeat(FRAME_DASHES),
		),
		FRAME,
	))
}

/// Highlight code and prepend line-number gutters
pub fn render_lines(
	lang: &str,
	lines: &[&str],
) -> Vec<Line<'static>> {
	// Strip trailing whitespace from LLM output
	let code = lines
		.iter()
		.map(|l| l.trim_end())
		.collect::<Vec<_>>()
		.join("\n");
	let highlighted =
		syntax::highlight_lines(lang, &code);
	let width = digit_width(highlighted.len());

	highlighted
		.into_iter()
		.enumerate()
		.map(|(i, line)| {
			prepend_gutter(line, i + 1, width)
		})
		.collect()
}

/// Prepend "  │ N │ " gutter to a highlighted line
fn prepend_gutter(
	line: Line<'static>,
	num: usize,
	width: usize,
) -> Line<'static> {
	let gutter = format!(
		"  \u{2502} {:>w$} \u{2502} ",
		num,
		w = width,
	);
	let mut spans =
		Vec::with_capacity(1 + line.spans.len());
	spans.push(Span::styled(gutter, LINE_NUM));
	spans.extend(line.spans);
	Line::from(spans)
}
