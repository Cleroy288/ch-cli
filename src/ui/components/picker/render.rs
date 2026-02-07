use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::picker::Picker;
use crate::ui::styles::colors;
use crate::ui::{layout, styles};

/// Render the type chooser (folder or file selection).
pub fn render_type_chooser(
    frame: &mut Frame,
    input_area: Rect,
    selected_index: usize,
) {
    let picker_area = layout::calculate_type_chooser_area(
        input_area,
    );

    let options = ["▸ folder", "◆ file"];
    let items: Vec<ListItem> = options
        .iter()
        .enumerate()
        .map(|(i, option)| {
            let style = if i == selected_index {
                styles::picker_selected_style()
            } else {
                Style::default().fg(colors::INPUT_TEXT)
            };
            ListItem::new(Line::from(*option)).style(style)
        })
        .collect();

    let chooser_title = " Select Type (↑↓, Enter, f/d) ";
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(chooser_title)
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(colors::TITLE)),
    );

    frame.render_widget(list, picker_area);
}

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
        .style(Style::default().fg(colors::PLACEHOLDER))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .title_alignment(Alignment::Left)
                .style(Style::default().fg(colors::PICKER)),
        );

    frame.render_widget(paragraph, picker_area);
}

/// Render help text at the bottom of the picker.
pub fn render_picker_help_text(
    frame: &mut Frame,
    picker_area: Rect,
) {
    let help_area = layout::calculate_help_text_area(
        picker_area,
    );

    let help_text = Line::from(vec![
        Span::styled(
            "↑↓",
            Style::default()
                .fg(colors::TITLE)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" navigate  "),
        Span::styled(
            "Enter",
            Style::default()
                .fg(colors::PICKER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" select  "),
        Span::styled(
            "F5",
            Style::default()
                .fg(colors::MESSAGE_HEADER)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" refresh  "),
        Span::styled(
            "ESC",
            Style::default()
                .fg(colors::FILE_REF_BG)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" cancel"),
    ]);

    let help_style = Style::default().fg(colors::PLACEHOLDER);
    let help_paragraph = Paragraph::new(help_text)
        .style(help_style);

    frame.render_widget(help_paragraph, help_area);
}

/// Build the picker title based on mode and query.
pub fn build_picker_title(picker: &Picker) -> String {
    use crate::picker::PickerMode;

    match picker.mode() {
        PickerMode::File => {
            format!(" Files (query: '{}') ", picker.query())
        }
        PickerMode::Folder => {
            format!(" Folders (query: '{}') ", picker.query())
        }
        _ => " Search ".to_string(),
    }
}
