use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::picker::{Picker, PickerMode};
use crate::ui::styles::colors;

/// Render a message when no results are found.
pub fn render_empty_results(
    frame: &mut Frame,
    picker_area: Rect,
    picker: &Picker,
) {
    let title = build_picker_title(picker);
    let message = if picker.query().is_empty() {
        "Start typing to search..."
    } else {
        "No results found"
    };

    let paragraph = Paragraph::new(Line::from(message))
        .style(
            Style::default().fg(colors::PLACEHOLDER),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Left)
                .style(
                    Style::default().fg(colors::PICKER),
                ),
        );

    frame.render_widget(paragraph, picker_area);
}

/// Render help text at the bottom of the picker.
pub fn render_picker_help_text(
    frame: &mut Frame,
    picker_area: Rect,
) {
    let help_area = Rect {
        x: picker_area.x + 2,
        y: picker_area.y + picker_area.height,
        width: picker_area.width.saturating_sub(4),
        height: 1,
    };
    let help_text = build_help_spans();
    let help_style =
        Style::default().fg(colors::PLACEHOLDER);
    let paragraph =
        Paragraph::new(help_text).style(help_style);
    frame.render_widget(paragraph, help_area);
}

/// Build help text spans
fn build_help_spans() -> Line<'static> {
    Line::from(vec![
        bold_key_span("↑↓", colors::TITLE),
        Span::raw(" navigate  "),
        bold_key_span("Enter", colors::PICKER),
        Span::raw(" select  "),
        bold_key_span("F5", colors::MESSAGE_HEADER),
        Span::raw(" refresh  "),
        bold_key_span("ESC", colors::FILE_REF_BG),
        Span::raw(" cancel"),
    ])
}

/// Build a bold key hint span
fn bold_key_span(
    key: &'static str,
    color: Color,
) -> Span<'static> {
    Span::styled(
        key,
        Style::default()
            .fg(color)
            .add_modifier(Modifier::BOLD),
    )
}

/// Build the picker title based on mode and query.
pub fn build_picker_title(picker: &Picker) -> String {
    let label = match picker.mode() {
        PickerMode::Browse { .. } => "Browse",
        PickerMode::Symbols { .. } => {
            return build_symbols_title(picker);
        }
        PickerMode::Tools => return " Tools ".into(),
        PickerMode::DocBrowser => "Doc Browser",
        PickerMode::Inactive => return " Search ".into(),
    };
    format!(" {} (query: '{}') ", label, picker.query())
}

/// Build title for Symbols mode with parent info
fn build_symbols_title(picker: &Picker) -> String {
    let parent = picker
        .symbol_browser()
        .and_then(|brow| brow.current_parent())
        .unwrap_or("top-level");
    format!(
        " Symbols [{}] (query: '{}') ",
        parent,
        picker.query()
    )
}
