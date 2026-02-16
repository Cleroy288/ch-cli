//! Debug Rendering Helpers
//!
//! Pure functions that transform conversation history
//! data into styled lines for the debug panel display.

mod claude_lines;
mod doc_preview_lines;
mod message_lines;
mod ref_spans;
mod segment_lines;

pub use claude_lines::{
	build_claude_loading_lines,
	build_claude_response_lines,
};
pub use doc_preview_lines::build_doc_preview_lines;

use ratatui::text::Line;

use crate::message::ConversationHistory;

use message_lines::build_message_lines;

/// Build lines for the message history display.
#[doc(hidden)]
pub fn build_message_history_lines(
    history: &ConversationHistory,
) -> Vec<Line<'static>> {
    if history.is_empty() {
        return build_empty_history_lines();
    }

    let mut lines = vec![
        build_history_header(history.len()),
        Line::from(""),
    ];

    append_recent_messages(&mut lines, history);
    append_truncation_hint(&mut lines, history);
    lines
}

/// Build placeholder lines for empty history
fn build_empty_history_lines() -> Vec<Line<'static>> {
    use ratatui::{
        style::Style,
        text::Span,
    };
    use crate::ui::styles::colors;

    vec![
        Line::from(""),
        Line::from(Span::styled(
            "No messages yet. Type something \
            and press Enter!",
            Style::default().fg(colors::PLACEHOLDER),
        )),
    ]
}

/// Build the history header line
fn build_history_header(
    count: usize,
) -> Line<'static> {
    use ratatui::{
        style::{Modifier, Style},
        text::Span,
    };
    use crate::ui::styles::colors;

    Line::from(Span::styled(
        format!("Message History ({} messages)", count),
        Style::default()
            .fg(colors::TITLE)
            .add_modifier(Modifier::BOLD),
    ))
}

/// Append recent messages to lines
fn append_recent_messages(
    lines: &mut Vec<Line<'static>>,
    history: &ConversationHistory,
) {
    use crate::domain::HISTORY_DISPLAY_COUNT;
    use crate::message::UserMessage;

    let msgs: Vec<&UserMessage> = history
        .messages()
        .iter()
        .rev()
        .take(HISTORY_DISPLAY_COUNT)
        .collect();

    for (idx, msg) in msgs.iter().rev().enumerate() {
        lines.extend(build_message_lines(msg, idx + 1));
    }
}

/// Append truncation indicator if needed
fn append_truncation_hint(
    lines: &mut Vec<Line<'static>>,
    history: &ConversationHistory,
) {
    use ratatui::{style::Style, text::Span};
    use crate::domain::HISTORY_DISPLAY_COUNT;
    use crate::ui::styles::colors;

    if history.len() <= HISTORY_DISPLAY_COUNT {
        return;
    }
    let count =
        history.len() - HISTORY_DISPLAY_COUNT;
    lines.push(Line::from(Span::styled(
        format!("... and {} more messages", count),
        Style::default().fg(colors::PLACEHOLDER),
    )));
}
