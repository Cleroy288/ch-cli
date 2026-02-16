use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;
use crate::fs::FsEntry;
use crate::picker::PickerMode;
use crate::ui::styles::colors;
use crate::ui::{layout, styles};

mod render;
mod render_doc_browser;
#[doc(hidden)]
pub mod render_symbols;
mod render_tools;

use render::{
    build_picker_title, render_empty_results,
    render_picker_help_text,
};
use render_doc_browser::render_doc_list;
use render_symbols::render_symbol_list;
use render_tools::render_tools_list;

/// Render the picker overlay (browse or symbol list).
pub fn render_picker(
    frame: &mut Frame,
    input_area: Rect,
    bottom_area: Rect,
    app: &App,
) {
    let picker = app.picker();

    let area = layout::calculate_file_list_area(
        input_area, bottom_area,
    );
    match picker.mode() {
        PickerMode::Browse { .. } => {
            render_file_list(
                frame, input_area, bottom_area, app,
            );
        }
        PickerMode::Symbols { .. } => {
            render_symbol_list(frame, area, picker);
        }
        PickerMode::Tools => {
            render_tools_list(frame, area, picker);
        }
        PickerMode::DocBrowser => {
            render_doc_list(frame, area, picker);
        }
        PickerMode::Inactive => {}
    }
}

/// Render the file/folder list picker.
fn render_file_list(
    frame: &mut Frame,
    input_area: Rect,
    bottom_area: Rect,
    app: &App,
) {
    let picker = app.picker();
    let results = picker.get_results();
    let picker_area = layout::calculate_file_list_area(
        input_area, bottom_area,
    );

    if results.is_empty() {
        render_empty_results(
            frame, picker_area, picker,
        );
        return;
    }

    render_file_list_items(
        frame, picker_area, picker, &results,
    );
    render_picker_help_text(frame, picker_area);
}

/// Render the list of file/folder items with scrolling
fn render_file_list_items(
    frame: &mut Frame,
    picker_area: Rect,
    picker: &crate::picker::Picker,
    results: &[&FsEntry],
) {
    let selected = picker.selected_index();
    let visible =
        picker_area.height.saturating_sub(2) as usize;
    let scroll = if selected >= visible {
        selected - visible + 1
    } else {
        0
    };
    let items = build_file_items(
        results, selected, scroll, visible,
    );
    let title = format!(
        "{} [{}/{}]",
        build_picker_title(picker),
        selected + 1,
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

/// Build list items for the visible range
fn build_file_items(
    results: &[&FsEntry],
    selected: usize,
    scroll: usize,
    visible: usize,
) -> Vec<ListItem<'static>> {
    results
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, entry)| {
            let style = if idx == selected {
                styles::file_list_selected_style()
            } else {
                Style::default().fg(colors::INPUT_TEXT)
            };
            ListItem::new(Line::from(
                entry.display_name(),
            ))
            .style(style)
        })
        .collect()
}
