use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{
        Block, Borders, List, Paragraph,
    },
    Frame,
};

use crate::domain::tool_ref::ToolKind;
use crate::picker::mode::PickerMode;
use crate::picker::Picker;
use crate::ui::styles::colors;

use super::render_tool_items::{
    build_result_items, filter_items,
};

pub fn render_tool_results(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    match picker.mode() {
        PickerMode::ToolLoading { tool } => {
            render_centered_msg(
                frame, area,
                tool.display_name(), "Loading...",
            );
        }
        PickerMode::ToolResults { tool } => {
            render_results(
                frame, area, picker, *tool,
            );
        }
        _ => {}
    }
}

fn render_results(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
    tool: ToolKind,
) {
    if let Some(err) = &picker.tool_error {
        render_centered_msg(
            frame, area, tool.display_name(), err,
        );
        return;
    }
    let query = picker.query().to_lowercase();
    let filtered = filter_items(
        &picker.tool_results, &query,
    );
    if filtered.is_empty() {
        render_centered_msg(
            frame, area,
            tool.display_name(), "No results",
        );
        return;
    }
    render_items(frame, area, picker, &filtered);
}

fn render_centered_msg(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    msg: &str,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {title} "))
        .title_alignment(Alignment::Left)
        .style(Style::default().fg(colors::PICKER));
    let widget =
        Paragraph::new(Line::from(msg))
            .style(
                Style::default()
                    .fg(colors::PLACEHOLDER),
            )
            .alignment(Alignment::Center)
            .block(block);
    frame.render_widget(widget, area);
}

fn render_items(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
    filtered: &[&crate::domain::tool_ref::ToolItem],
) {
    let selected = picker.selected_index();
    let visible =
        area.height.saturating_sub(2) as usize;
    let items = build_result_items(
        filtered, selected, visible,
    );
    let title = match picker.mode() {
        PickerMode::ToolResults { tool } => format!(
            " {} [{}/{}]",
            tool.display_name(),
            selected + 1,
            filtered.len()
        ),
        _ => " Results ".to_string(),
    };
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Left)
            .style(
                Style::default().fg(colors::PICKER),
            ),
    );
    frame.render_widget(list, area);
}
