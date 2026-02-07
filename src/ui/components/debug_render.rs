//! Debug Rendering Helpers
//!
//! Pure functions that transform conversation history
//! data into styled lines for the debug panel display.

use ratatui::{
	style::{Color, Modifier, Style},
	text::{Line, Span},
};

use crate::domain::HISTORY_DISPLAY_COUNT;
use crate::message::{
	ConversationHistory, MessageSegment, UserMessage,
};
use crate::ui::styles::colors;

/// Build lines for the message history display.
///
/// Transforms conversation history into styled lines.
/// Handles both empty and populated history.
#[doc(hidden)]
pub fn build_message_history_lines(
	history: &ConversationHistory,
) -> Vec<Line<'static>> {
	if history.is_empty() {
		return build_empty_history_lines();
	}

	let mut lines = Vec::new();

	// Header
	lines.push(Line::from(Span::styled(
		format!(
			"Message History ({} messages)",
			history.len(),
		),
		Style::default()
			.fg(colors::TITLE)
			.add_modifier(Modifier::BOLD),
	)));
	lines.push(Line::from(""));

	// Show last N messages (oldest first)
	let msgs: Vec<&UserMessage> = history
		.messages()
		.iter()
		.rev()
		.take(HISTORY_DISPLAY_COUNT)
		.collect();

	for (i, msg) in msgs.iter().rev().enumerate() {
		lines.extend(build_message_lines(msg, i + 1));
	}

	// Truncation indicator
	if history.len() > HISTORY_DISPLAY_COUNT {
		let count =
			history.len() - HISTORY_DISPLAY_COUNT;
		lines.push(Line::from(Span::styled(
			format!("... and {} more messages", count),
			Style::default().fg(colors::PLACEHOLDER),
		)));
	}

	lines
}

/// Build placeholder lines for empty history
fn build_empty_history_lines() -> Vec<Line<'static>> {
	vec![
		Line::from(""),
		Line::from(Span::styled(
			"No messages yet. Type something \
			and press Enter!",
			Style::default().fg(colors::PLACEHOLDER),
		)),
	]
}

/// Build display lines for a single message.
///
/// Creates header, raw input, segments, and stats.
fn build_message_lines(
	message: &UserMessage,
	index: usize,
) -> Vec<Line<'static>> {
	let mut lines = Vec::new();

	// Message header
	lines.push(Line::from(Span::styled(
		format!("--- Message {} ---", index),
		Style::default()
			.fg(colors::MESSAGE_HEADER)
			.add_modifier(Modifier::BOLD),
	)));

	// Raw input
	lines.push(Line::from(vec![
		Span::styled(
			"Raw: ",
			Style::default().fg(colors::SEGMENT_LABEL),
		),
		Span::styled(
			message.raw_input.clone(),
			Style::default().fg(colors::INPUT_TEXT),
		),
	]));

	// Parsed segments header
	lines.push(Line::from(Span::styled(
		"Parsed Segments:",
		Style::default().fg(colors::SEGMENT_LABEL),
	)));

	// Each segment
	for (idx, seg) in
		message.segments.iter().enumerate()
	{
		lines.push(build_segment_line(seg, idx));
	}

	// Stats
	lines.push(Line::from(vec![
		Span::styled(
			"  Stats: ",
			Style::default().fg(colors::SEGMENT_LABEL),
		),
		Span::styled(
			format!(
				"{} files, {} folders",
				message.file_count(),
				message.folder_count(),
			),
			Style::default().fg(colors::PLACEHOLDER),
		),
	]));
	lines.push(Line::from(""));

	lines
}

/// Build a styled line for a single segment.
fn build_segment_line(
	segment: &MessageSegment,
	idx: usize,
) -> Line<'static> {
	let mut spans = vec![
		Span::raw("  "),
		Span::styled(
			format!("[{}] ", idx),
			Style::default().fg(colors::PLACEHOLDER),
		),
	];

	match segment {
		MessageSegment::Text(text) => {
			spans.push(Span::styled(
				"Text: ",
				Style::default()
					.fg(colors::MESSAGE_HEADER),
			));
			spans.push(Span::styled(
				format!("\"{}\"", text),
				Style::default()
					.fg(colors::INPUT_TEXT),
			));
		}
		MessageSegment::FileReference {
			full_path,
			display_name,
		} => {
			append_ref_spans(
				&mut spans,
				"File: ",
				colors::FILE_REF_BG,
				display_name,
				full_path,
			);
		}
		MessageSegment::FolderReference {
			full_path,
			display_name,
		} => {
			append_ref_spans(
				&mut spans,
				"Folder: ",
				colors::FOLDER_REF_BG,
				display_name,
				full_path,
			);
		}
	}

	Line::from(spans)
}

/// Append file/folder reference spans to a vec.
fn append_ref_spans(
	spans: &mut Vec<Span<'static>>,
	label: &str,
	label_color: Color,
	display_name: &str,
	full_path: &str,
) {
	spans.push(Span::styled(
		label.to_string(),
		Style::default()
			.fg(label_color)
			.add_modifier(Modifier::BOLD),
	));
	spans.push(Span::styled(
		display_name.to_string(),
		Style::default()
			.fg(colors::INPUT_TEXT)
			.add_modifier(Modifier::BOLD),
	));
	spans.push(Span::styled(
		" -> ",
		Style::default().fg(colors::SEPARATOR),
	));
	spans.push(Span::styled(
		full_path.to_string(),
		Style::default().fg(colors::PATH_DISPLAY),
	));
}
