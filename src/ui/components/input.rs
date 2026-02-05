use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::app::App;
use crate::domain::FileReference;
use crate::ui::styles::{self, colors};

/// Render the input box.
///
/// Shows either a placeholder for empty input or styled text with
/// file/folder references highlighted.
pub fn render_input(frame: &mut Frame, area: Rect, app: &App) {
    let input_text = if app.input().is_empty() {
        Line::from("Type something... (@ for files/folders, ESC or Ctrl+C to quit)")
            .style(Style::default().fg(colors::PLACEHOLDER))
    } else {
        build_styled_input_line(app)
    };

    let paragraph = Paragraph::new(input_text).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Input ")
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(colors::INPUT_TEXT)),
    );

    frame.render_widget(paragraph, area);
}

/// Build a styled line with file/folder references highlighted.
///
/// This is a pure function that transforms the input text and file references
/// into a styled Line for rendering.
fn build_styled_input_line(app: &App) -> Line<'static> {
    let input = app.input();
    let file_refs = app.file_references();

    // Early return for no file references
    if file_refs.is_empty() {
        return Line::from(Span::styled(
            input.to_string(),
            Style::default().fg(colors::INPUT_TEXT),
        ));
    }

    let mut spans = Vec::new();
    let mut last_pos = 0;

    // Sort file references by start position
    let mut sorted_refs: Vec<&FileReference> = file_refs.iter().collect();
    sorted_refs.sort_by_key(|r| r.start);

    for file_ref in sorted_refs {
        // Add normal text before this reference
        if last_pos < file_ref.start {
            let normal_text = &input[last_pos..file_ref.start];
            spans.push(Span::styled(
                normal_text.to_string(),
                Style::default().fg(colors::INPUT_TEXT),
            ));
        }

        // Add styled file/folder reference
        if file_ref.start < input.len() && file_ref.end <= input.len() {
            let ref_text = &input[file_ref.start..file_ref.end];
            let style = if file_ref.is_dir {
                styles::folder_reference_style()
            } else {
                styles::file_reference_style()
            };
            spans.push(Span::styled(ref_text.to_string(), style));
            last_pos = file_ref.end;
        }
    }

    // Add remaining normal text after last reference
    if last_pos < input.len() {
        let remaining_text = &input[last_pos..];
        spans.push(Span::styled(
            remaining_text.to_string(),
            Style::default().fg(colors::INPUT_TEXT),
        ));
    }

    Line::from(spans)
}

/// Set the cursor position in the input box.
///
/// This should be called after rendering the input box to position the cursor.
pub fn set_cursor(frame: &mut Frame, area: Rect, cursor_pos: usize) {
    let cursor_x = area.x + cursor_pos as u16 + 1; // +1 for border
    let cursor_y = area.y + 1; // +1 for border

    // Only set cursor if it's within the input box bounds
    if cursor_x < area.x + area.width - 1 {
        frame.set_cursor_position((cursor_x, cursor_y));
    }
}
