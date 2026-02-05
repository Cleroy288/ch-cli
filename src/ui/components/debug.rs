use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::domain::HISTORY_DISPLAY_COUNT;
use crate::message::{ConversationHistory, MessageSegment, UserMessage};
use crate::ui::styles::colors;

/// Types of debug information that can be displayed in the debug section.
///
/// This enum allows the debug panel to show different kinds of information.
/// To add a new debug type:
/// 1. Add a variant to this enum
/// 2. Add a corresponding match arm in `build_debug_lines()`
/// 3. Implement a `build_*_lines()` function for your debug type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugInfoType {
    /// Display parsed message history with segments and stats
    MessageHistory,
    // Add more debug types as needed:
    // AppState,        // Show current app state
    // PickerState,     // Show picker mode and selections
    // FileReferences,  // Show all file references
    // Performance,     // Show performance metrics
}

impl Default for DebugInfoType {
    fn default() -> Self {
        Self::MessageHistory
    }
}

impl DebugInfoType {
    /// Get a human-readable name for this debug type
    pub fn name(&self) -> &'static str {
        match self {
            Self::MessageHistory => "Message History",
        }
    }
}

/// Render the debug section with various debug information.
///
/// This is the main rendering function that coordinates building and displaying
/// different types of debug information based on the current debug mode.
pub fn render_debug_panel(frame: &mut Frame, area: Rect, app: &App) {
    // Currently defaulting to message history, but this could be made configurable
    let debug_type = DebugInfoType::default();
    let lines = build_debug_lines(app, debug_type);
    let widget = create_debug_panel_widget(lines);
    frame.render_widget(widget, area);
}

/// Build the debug lines based on the selected debug info type.
///
/// This function routes to the appropriate builder based on the debug type.
fn build_debug_lines(app: &App, debug_type: DebugInfoType) -> Vec<Line<'static>> {
    match debug_type {
        DebugInfoType::MessageHistory => build_message_history_lines(app.history()),
        // Add more cases as needed
    }
}

/// Build the lines for the message history display.
///
/// This is a pure function that transforms the conversation history into
/// styled lines for rendering. It handles both empty and populated history.
///
/// This is one of the debug info builders. To add more debug types,
/// create similar `build_*_lines()` functions and add them to `build_debug_lines()`.
fn build_message_history_lines(history: &ConversationHistory) -> Vec<Line<'static>> {
    // Handle empty history case
    if history.is_empty() {
        return vec![
            Line::from(""),
            Line::from(Span::styled(
                "No messages yet. Type something and press Enter!",
                Style::default().fg(colors::PLACEHOLDER),
            )),
        ];
    }

    let mut lines = Vec::new();

    // Add header
    lines.push(Line::from(Span::styled(
        format!("📋 Message History ({} messages)", history.len()),
        Style::default()
            .fg(colors::TITLE)
            .add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Show last N messages (most recent first)
    let messages_to_show: Vec<&UserMessage> = history
        .messages()
        .iter()
        .rev()
        .take(HISTORY_DISPLAY_COUNT)
        .collect();

    // Reverse to show oldest first
    for (i, message) in messages_to_show.iter().rev().enumerate() {
        lines.extend(build_message_display_lines(message, i + 1));
    }

    // Add truncation indicator if needed
    if history.len() > HISTORY_DISPLAY_COUNT {
        lines.push(Line::from(Span::styled(
            format!(
                "... and {} more messages",
                history.len() - HISTORY_DISPLAY_COUNT
            ),
            Style::default().fg(colors::PLACEHOLDER),
        )));
    }

    lines
}

/// Build the display lines for a single message.
///
/// Creates the header, raw input, segments, and stats for one message.
fn build_message_display_lines(message: &UserMessage, index: usize) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    // Message header
    lines.push(Line::from(Span::styled(
        format!("─── Message {} ───", index),
        Style::default()
            .fg(colors::MESSAGE_HEADER)
            .add_modifier(Modifier::BOLD),
    )));

    // Raw input
    lines.push(Line::from(vec![
        Span::styled("Raw: ", Style::default().fg(colors::SEGMENT_LABEL)),
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

    // Add each segment
    for (seg_idx, segment) in message.segments.iter().enumerate() {
        lines.push(build_segment_line(segment, seg_idx));
    }

    // Stats line
    lines.push(Line::from(vec![
        Span::styled("  Stats: ", Style::default().fg(colors::SEGMENT_LABEL)),
        Span::styled(
            format!(
                "{} files, {} folders",
                message.file_count(),
                message.folder_count()
            ),
            Style::default().fg(colors::PLACEHOLDER),
        ),
    ]));

    lines.push(Line::from(""));

    lines
}

/// Build a styled line for a single message segment.
///
/// Formats the segment based on its type (Text, FileReference, FolderReference).
fn build_segment_line(segment: &MessageSegment, seg_idx: usize) -> Line<'static> {
    let mut spans = vec![
        Span::raw("  "),
        Span::styled(
            format!("[{}] ", seg_idx),
            Style::default().fg(colors::PLACEHOLDER),
        ),
    ];

    match segment {
        MessageSegment::Text(text) => {
            spans.push(Span::styled(
                "Text: ",
                Style::default().fg(colors::MESSAGE_HEADER),
            ));
            spans.push(Span::styled(
                format!("\"{}\"", text),
                Style::default().fg(colors::INPUT_TEXT),
            ));
        }
        MessageSegment::FileReference {
            full_path,
            display_name,
        } => {
            spans.push(Span::styled(
                "File: ",
                Style::default()
                    .fg(colors::FILE_REF_BG)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                display_name.clone(),
                Style::default()
                    .fg(colors::INPUT_TEXT)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(" → ", Style::default().fg(colors::SEPARATOR)));
            spans.push(Span::styled(
                full_path.clone(),
                Style::default().fg(colors::PATH_DISPLAY),
            ));
        }
        MessageSegment::FolderReference {
            full_path,
            display_name,
        } => {
            spans.push(Span::styled(
                "Folder: ",
                Style::default()
                    .fg(colors::FOLDER_REF_BG)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(
                display_name.clone(),
                Style::default()
                    .fg(colors::INPUT_TEXT)
                    .add_modifier(Modifier::BOLD),
            ));
            spans.push(Span::styled(" → ", Style::default().fg(colors::SEPARATOR)));
            spans.push(Span::styled(
                full_path.clone(),
                Style::default().fg(colors::PATH_DISPLAY),
            ));
        }
    }

    Line::from(spans)
}

/// Create the debug panel widget with the given lines.
///
/// Pure UI construction function.
fn create_debug_panel_widget(lines: Vec<Line<'static>>) -> Paragraph<'static> {
    Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Debug Section ")
                .title_alignment(Alignment::Left)
                .style(Style::default().fg(colors::DEBUG)),
        )
        .scroll((0, 0))
}
