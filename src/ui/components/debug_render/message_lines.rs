//! Message-level rendering for debug panel.
//!
//! Builds the styled lines for a single UserMessage:
//! header, raw input, segments label, and stats.

use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

use crate::message::UserMessage;
use crate::ui::styles::colors;

use super::segment_lines::build_segment_line;

/// Build display lines for a single message.
pub(super) fn build_message_lines(
    message: &UserMessage,
    index: usize,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    lines.push(build_msg_header_line(index));
    lines.push(build_raw_input_line(message));
    lines.push(Line::from(Span::styled(
        "Parsed Segments:",
        Style::default().fg(colors::SEGMENT_LABEL),
    )));
    append_segment_lines(&mut lines, message);
    lines.push(build_stats_line(message));
    lines.push(Line::from(""));
    lines
}

/// Build the message header line
fn build_msg_header_line(
    index: usize,
) -> Line<'static> {
    Line::from(Span::styled(
        format!("--- Message {} ---", index),
        Style::default()
            .fg(colors::MESSAGE_HEADER)
            .add_modifier(Modifier::BOLD),
    ))
}

/// Build the raw input display line
fn build_raw_input_line(
    message: &UserMessage,
) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            "Raw: ",
            Style::default().fg(colors::SEGMENT_LABEL),
        ),
        Span::styled(
            message.raw_input.clone(),
            Style::default().fg(colors::INPUT_TEXT),
        ),
    ])
}

/// Append individual segment lines
fn append_segment_lines(
    lines: &mut Vec<Line<'static>>,
    message: &UserMessage,
) {
    for (idx, seg) in
        message.segments.iter().enumerate()
    {
        lines.push(build_segment_line(seg, idx));
    }
}

/// Build the stats line
fn build_stats_line(
    message: &UserMessage,
) -> Line<'static> {
    Line::from(vec![
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
    ])
}
