use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{
        Block, Borders, List, ListItem,
    },
    Frame,
};

use crate::picker::Picker;
use crate::ui::strings::tui_labels;
use crate::ui::styles;
use crate::ui::styles::colors;

pub fn render_jira_filter(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let items =
        build_assignee_items(picker, area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(tui_labels::JIRA_ASSIGNEE_TITLE)
        .style(
            Style::default().fg(colors::PICKER),
        );
    let list = List::new(items).block(block);
    frame.render_widget(list, area);
}

fn build_assignee_items(
    picker: &Picker,
    area: Rect,
) -> Vec<ListItem<'static>> {
    let query = picker.query().to_lowercase();
    let visible =
        area.height.saturating_sub(4) as usize;
    let list = build_full_list(picker, &query);
    let sel = picker.selected_index().min(
        list.len().saturating_sub(1),
    );
    let scroll = if sel >= visible {
        sel - visible + 1
    } else {
        0
    };
    list.into_iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, name)| {
            build_name_item(&name, idx == sel)
        })
        .collect()
}

fn build_full_list(
    picker: &Picker,
    query: &str,
) -> Vec<String> {
    let list: Vec<String> =
        picker.jira_assignees().to_vec();
    if query.is_empty() {
        return list;
    }
    list.into_iter()
        .filter(|name| {
            name.to_lowercase().contains(query)
        })
        .collect()
}

fn build_name_item(
    name: &str,
    selected: bool,
) -> ListItem<'static> {
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let text = format!("  @{name}");
    ListItem::new(Line::from(text)).style(style)
}
