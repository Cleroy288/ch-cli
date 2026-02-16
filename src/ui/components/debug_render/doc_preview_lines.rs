//! Doc preview rendering for the output panel.
//!
//! Converts a DocEntryResponse into styled lines
//! for display in the debug/output panel.

use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::retrieval::daemon::protocol::DocEntryResponse;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

/// Build styled lines for doc preview display.
///
/// Returns "no doc" message when doc is None.
pub fn build_doc_preview_lines(
	doc: Option<&DocEntryResponse>,
) -> Vec<Line<'static>> {
	let Some(doc) = doc else {
		return vec![
			Line::from(""),
			Line::from(Span::styled(
				tui_labels::DOC_NO_PREVIEW
					.to_string(),
				Style::default()
					.fg(colors::PLACEHOLDER),
			)),
		];
	};

	let mut lines = vec![
		Line::from(""),
		build_header_line(doc),
		build_location_line(doc),
		Line::from(""),
	];
	lines.extend(build_doc_body_lines(doc));
	lines.push(Line::from(""));
	lines.extend(build_deps_lines(doc));
	lines
}

/// Build signature header line (amber + bold)
fn build_header_line(
	doc: &DocEntryResponse,
) -> Line<'static> {
	let text = doc
		.signature
		.clone()
		.unwrap_or_else(|| {
			format!("{} {}", doc.kind, doc.name)
		});
	Line::from(Span::styled(
		format!("  {}", text),
		Style::default()
			.fg(colors::AMBER)
			.add_modifier(Modifier::BOLD),
	))
}

/// Build location line (file:line + kind)
fn build_location_line(
	doc: &DocEntryResponse,
) -> Line<'static> {
	let text = format!(
		"  {}:{}  {}",
		doc.file_path, doc.line, doc.kind,
	);
	Line::from(Span::styled(
		text,
		Style::default().fg(colors::DIM_TEXT),
	))
}

/// Build doc body lines from user comment / LLM doc
fn build_doc_body_lines(
	doc: &DocEntryResponse,
) -> Vec<Line<'static>> {
	let body = doc
		.user_comment
		.as_deref()
		.or(doc.llm_doc.as_deref());

	let Some(text) = body else {
		return Vec::new();
	};

	text.lines()
		.map(|line| {
			Line::from(Span::styled(
				format!("  {}", line),
				Style::default()
					.fg(colors::INPUT_TEXT),
			))
		})
		.collect()
}

/// Build dependency/usage lines
fn build_deps_lines(
	doc: &DocEntryResponse,
) -> Vec<Line<'static>> {
	let style =
		Style::default().fg(colors::DIM_TEXT);
	let none = tui_labels::DOC_NONE_MARKER;
	let dep_items = if doc.depends_on.is_empty() {
		none.to_string()
	} else {
		doc.depends_on.join(", ")
	};
	let use_items = if doc.depended_by.is_empty() {
		none.to_string()
	} else {
		doc.depended_by.join(", ")
	};
	let depends = format!(
		"  {}: {}",
		tui_labels::DOC_DEPENDS_ON, dep_items,
	);
	let used_by = format!(
		"  {}: {}",
		tui_labels::DOC_USED_BY, use_items,
	);
	vec![
		Line::from(Span::styled(depends, style)),
		Line::from(Span::styled(used_by, style)),
	]
}
