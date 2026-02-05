use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;
use crate::picker::{Picker, PickerMode};
use crate::ui::styles::colors;
use crate::ui::{layout, styles};

/// Render the picker overlay (type chooser or file list).
///
/// This is the main coordinator that routes to the appropriate picker renderer
/// based on the current picker mode.
pub fn render_picker(frame: &mut Frame, input_area: Rect, bottom_area: Rect, app: &App) {
    let picker = app.picker();

    match picker.mode() {
        PickerMode::ChoosingType => {
            render_type_chooser(frame, input_area, picker.selected_index());
        }
        PickerMode::File | PickerMode::Folder => {
            render_file_list(frame, input_area, bottom_area, app);
        }
        PickerMode::Inactive => {}
    }
}

/// Render the type chooser (folder or file selection).
fn render_type_chooser(frame: &mut Frame, input_area: Rect, selected_index: usize) {
    let picker_area = layout::calculate_type_chooser_area(input_area);

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

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(" Select Type (↑↓ to navigate, Enter to select, f=file, d=folder) ")
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(colors::TITLE)),
    );

    frame.render_widget(list, picker_area);
}

/// Render the file/folder list picker.
///
/// This is split into smaller functions for better readability and testability.
fn render_file_list(frame: &mut Frame, input_area: Rect, bottom_area: Rect, app: &App) {
    let picker = app.picker();
    let results = picker.get_results();
    let picker_area = layout::calculate_file_list_area(input_area, bottom_area);

    // Handle empty results case
    if results.is_empty() {
        render_empty_results(frame, picker_area, picker);
        return;
    }

    // Render the file list
    render_file_list_items(frame, picker_area, picker, &results);

    // Render help text
    render_picker_help_text(frame, picker_area);
}

/// Render a message when no results are found.
fn render_empty_results(frame: &mut Frame, picker_area: Rect, picker: &Picker) {
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

/// Render the list of file/folder items with scrolling.
fn render_file_list_items(
    frame: &mut Frame,
    picker_area: Rect,
    picker: &Picker,
    results: &[&crate::fs::FsEntry],
) {
    let selected_index = picker.selected_index();
    let max_height = picker_area.height.saturating_sub(2); // Subtract borders
    let visible_items = max_height as usize;

    // Calculate scroll offset to keep selected item visible
    let scroll_offset = if selected_index >= visible_items {
        selected_index - visible_items + 1
    } else {
        0
    };

    // Build list items
    let items: Vec<ListItem> = results
        .iter()
        .enumerate()
        .skip(scroll_offset)
        .take(visible_items)
        .map(|(i, entry)| {
            let is_selected = i == selected_index;
            let display_text = entry.display_name();

            let style = if is_selected {
                styles::file_list_selected_style()
            } else {
                Style::default().fg(colors::INPUT_TEXT)
            };

            ListItem::new(Line::from(display_text)).style(style)
        })
        .collect();

    // Build title with count indicator
    let title = format!(
        "{} [{}/{}]",
        build_picker_title(picker),
        selected_index + 1,
        results.len()
    );

    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Left)
            .style(Style::default().fg(colors::PICKER)),
    );

    frame.render_widget(list, picker_area);
}

/// Render help text at the bottom of the picker.
fn render_picker_help_text(frame: &mut Frame, picker_area: Rect) {
    let help_area = layout::calculate_help_text_area(picker_area);

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

    let help_paragraph = Paragraph::new(help_text).style(Style::default().fg(colors::PLACEHOLDER));

    frame.render_widget(help_paragraph, help_area);
}

/// Build the picker title based on mode and query.
fn build_picker_title(picker: &Picker) -> String {
    match picker.mode() {
        PickerMode::File => format!(" Files (query: '{}') ", picker.query()),
        PickerMode::Folder => format!(" Folders (query: '{}') ", picker.query()),
        _ => " Search ".to_string(),
    }
}
