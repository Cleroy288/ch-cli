use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::picker::tool_items;
use crate::picker::Picker;
use crate::ui::styles;
use crate::ui::styles::colors;

use super::render::render_empty_results;

pub fn render_tools_list(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let items = get_tool_items(picker);
    if items.is_empty() {
        render_empty_results(frame, area, picker);
        return;
    }
    let selected = picker.selected_index();
    let list_items =
        build_tool_items(&items, selected, area);
    let list =
        build_tools_widget(&items, selected, list_items);
    frame.render_widget(list, area);
}

fn get_tool_items(
    picker: &Picker,
) -> Vec<(&'static str, &'static str)> {
    tool_items::filter_tools(picker.query())
        .into_iter()
        .map(|i| (i.name, i.description))
        .collect()
}

fn build_tool_items(
    items: &[(&'static str, &'static str)],
    selected: usize,
    area: Rect,
) -> Vec<ListItem<'static>> {
    let visible =
        area.height.saturating_sub(2) as usize;
    let scroll = if selected >= visible {
        selected - visible + 1
    } else {
        0
    };
    items
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, (name, desc))| {
            build_one_item(name, desc, idx == selected)
        })
        .collect()
}

fn build_one_item(
    name: &'static str,
    desc: &'static str,
    selected: bool,
) -> ListItem<'static> {
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let dim = Style::default().fg(colors::DIM_TEXT);
    let spans = vec![
        Span::styled(name, style),
        Span::styled(" — ", dim),
        Span::styled(desc, dim),
    ];
    ListItem::new(Line::from(spans))
}

fn build_tools_widget(
    items: &[(&'static str, &'static str)],
    selected: usize,
    list_items: Vec<ListItem<'static>>,
) -> List<'static> {
    let title = format!(
        " Tools [{}/{}]",
        selected + 1,
        items.len()
    );
    List::new(list_items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Left)
            .style(
                Style::default().fg(colors::PICKER),
            ),
    )
}
