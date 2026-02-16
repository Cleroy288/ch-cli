//! Input box rendering for the TUI.
//!
//! Renders the input field with placeholder text
//! or styled content, and manages cursor position.

use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::input_styling;
use crate::app::App;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

/// Render the input box.
///
/// Shows loading indicator when waiting for Claude,
/// placeholder when empty, or styled content.
pub fn render_input(
    frame: &mut Frame,
    area: Rect,
    app: &App,
) {
    let loading = app.is_claude_loading();
    let empty = app.input().is_empty();
    let input_text = match (empty, loading) {
        (true, true) => build_loading_line(),
        (true, false) => build_placeholder_line(),
        _ => input_styling::build_styled_input_line(app),
    };

    let paragraph = Paragraph::new(input_text)
        .block(input_block(loading));
    frame.render_widget(paragraph, area);
}

/// Build loading indicator for Claude requests
fn build_loading_line() -> Line<'static> {
    Line::from(
        tui_labels::INPUT_WAITING.to_string(),
    )
    .style(Style::default().fg(colors::AMBER).bold())
}

/// Build the placeholder text for empty input
fn build_placeholder_line() -> Line<'static> {
    Line::from(
        "Type something... (@ for files/folders, \
        ESC or Ctrl+C to quit)",
    )
    .style(Style::default().fg(colors::PLACEHOLDER))
}

/// Build the input box block decoration
fn input_block(loading: bool) -> Block<'static> {
    let border_fg = if loading {
        colors::AMBER
    } else {
        colors::BORDER
    };
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_fg))
        .title(" > ")
        .title_alignment(Alignment::Left)
        .title_style(
            Style::default()
                .fg(colors::RUST_ORANGE)
                .bold(),
        )
}

/// Set the cursor position in the input box.
pub fn set_cursor(
    frame: &mut Frame,
    area: Rect,
    cursor_pos: usize,
) {
    let cursor_x = area.x + cursor_pos as u16 + 1;
    let cursor_y = area.y + 1;

    if cursor_x < area.x + area.width - 1 {
        frame.set_cursor_position((
            cursor_x, cursor_y,
        ));
    }
}
