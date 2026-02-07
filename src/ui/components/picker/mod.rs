use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;
use crate::picker::PickerMode;
use crate::ui::styles::colors;
use crate::ui::{layout, styles};

mod render;

use render::{
    build_picker_title, render_empty_results,
    render_picker_help_text, render_type_chooser,
};

/// Render the picker overlay (type chooser or file list).
///
/// This is the main coordinator that routes to the
/// appropriate picker renderer based on the current picker mode.
pub fn render_picker(
    frame: &mut Frame,
    input_area: Rect,
    bottom_area: Rect,
    app: &App,
) {
    let picker = app.picker();

    match picker.mode() {
        PickerMode::ChoosingType => {
            render_type_chooser(
                frame,
                input_area,
                picker.selected_index(),
            );
        }
        PickerMode::File | PickerMode::Folder => {
            render_file_list(frame, input_area, bottom_area, app);
        }
        PickerMode::Inactive => {}
    }
}

/// Render the file/folder list picker.
///
/// This is split into smaller functions for better testability.
fn render_file_list(
    frame: &mut Frame,
    input_area: Rect,
    bottom_area: Rect,
    app: &App,
) {
    let picker = app.picker();
    let results = picker.get_results();
    let picker_area = layout::calculate_file_list_area(
        input_area,
        bottom_area,
    );

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

/// Render the list of file/folder items with scrolling.
fn render_file_list_items(
    frame: &mut Frame,
    picker_area: Rect,
    picker: &crate::picker::Picker,
    results: &[&crate::fs::FsEntry],
) {
    let selected_index = picker.selected_index();
    let max_height = picker_area.height.saturating_sub(2);
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
