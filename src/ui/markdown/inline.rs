//! Inline markdown parser.
//!
//! Converts bold (**), italic (*), and inline
//! code (`) markers into styled ratatui Spans.

use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Span;

/// Bold text style
const BOLD: Style =
	Style::new().add_modifier(Modifier::BOLD);
/// Italic text style
const ITALIC: Style =
	Style::new().add_modifier(Modifier::ITALIC);
/// Inline code: light text on dark background
const CODE: Style = Style::new()
	.fg(Color::White)
	.bg(Color::Rgb(50, 50, 50));

/// Parse inline markdown into styled spans.
pub fn parse(text: &str) -> Vec<Span<'static>> {
	let mut spans = Vec::new();
	let mut buf = String::new();
	let chars: Vec<char> =
		text.chars().collect();
	let mut idx = 0;

	while idx < chars.len() {
		idx = handle_char(
			&chars, idx, &mut buf, &mut spans,
		);
	}
	flush(&mut buf, &mut spans);

	if spans.is_empty() {
		vec![Span::from(text.to_string())]
	} else {
		spans
	}
}

/// Route a character to the right scanner.
fn handle_char(
	chars: &[char],
	idx: usize,
	buf: &mut String,
	spans: &mut Vec<Span<'static>>,
) -> usize {
	if chars[idx] == '`' {
		flush(buf, spans);
		let (res, end) =
			scan_delimited(chars, idx, '`', CODE);
		spans.push(res);
		return end;
	}
	if chars[idx] == '*'
		&& chars.get(idx + 1) == Some(&'*')
	{
		flush(buf, spans);
		let (res, end) =
			scan_paired(chars, idx, BOLD);
		spans.push(res);
		return end;
	}
	if chars[idx] == '*' {
		flush(buf, spans);
		let (res, end) =
			scan_delimited(chars, idx, '*', ITALIC);
		spans.push(res);
		return end;
	}
	buf.push(chars[idx]);
	idx + 1
}

/// Flush accumulated plain text as a span.
fn flush(
	buf: &mut String,
	spans: &mut Vec<Span<'static>>,
) {
	if !buf.is_empty() {
		spans.push(Span::from(buf.clone()));
		buf.clear();
	}
}

/// Scan for single-char delimited content.
fn scan_delimited(
	chars: &[char],
	start: usize,
	delim: char,
	style: Style,
) -> (Span<'static>, usize) {
	let mut idx = start + 1;
	let mut content = String::new();

	while idx < chars.len() {
		if chars[idx] == delim {
			let res = Span::styled(content, style);
			return (res, idx + 1);
		}
		content.push(chars[idx]);
		idx += 1;
	}
	let raw = format!("{}{}", delim, content);
	(Span::from(raw), idx)
}

/// Scan for ** delimited bold content.
fn scan_paired(
	chars: &[char],
	start: usize,
	style: Style,
) -> (Span<'static>, usize) {
	let mut idx = start + 2;
	let mut content = String::new();

	while idx + 1 < chars.len() {
		if chars[idx] == '*'
			&& chars[idx + 1] == '*'
		{
			let res = Span::styled(content, style);
			return (res, idx + 2);
		}
		content.push(chars[idx]);
		idx += 1;
	}
	// Consume remaining chars
	while idx < chars.len() {
		content.push(chars[idx]);
		idx += 1;
	}
	(Span::from(format!("**{}", content)), idx)
}
