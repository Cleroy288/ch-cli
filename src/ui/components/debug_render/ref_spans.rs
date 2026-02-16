//! Reference span helpers for debug panel.
//!
//! Shared types and functions for rendering file
//! and folder reference spans in the debug display.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::ui::styles::colors;

/// Info for rendering a file/folder reference span
pub(super) struct RefSpanInfo<'ref_info> {
    /// label text (e.g. "File: " or "Folder: ")
    pub label: &'ref_info str,
    /// color for the label
    pub label_color: Color,
    /// short display name
    pub display_name: &'ref_info str,
    /// full filesystem path
    pub full_path: &'ref_info str,
}

/// Append file/folder reference spans to a vec.
pub(super) fn append_ref_spans<'ref_info>(
    spans: &mut Vec<Span<'static>>,
    info: &RefSpanInfo<'ref_info>,
) {
    spans.push(Span::styled(
        info.label.to_string(),
        Style::default()
            .fg(info.label_color)
            .add_modifier(Modifier::BOLD),
    ));
    spans.push(Span::styled(
        info.display_name.to_string(),
        Style::default()
            .fg(colors::INPUT_TEXT)
            .add_modifier(Modifier::BOLD),
    ));
    spans.push(Span::styled(
        " -> ",
        Style::default().fg(colors::SEPARATOR),
    ));
    spans.push(Span::styled(
        info.full_path.to_string(),
        Style::default().fg(colors::PATH_DISPLAY),
    ));
}

/// Append text segment spans
pub(super) fn append_text_spans(
    spans: &mut Vec<Span<'static>>,
    text: &str,
) {
    spans.push(Span::styled(
        "Text: ",
        Style::default().fg(colors::MESSAGE_HEADER),
    ));
    spans.push(Span::styled(
        format!("\"{}\"", text),
        Style::default().fg(colors::INPUT_TEXT),
    ));
}
