use ratatui::{
    style::Style,
    text::{Line, Span},
    widgets::ListItem,
};

use crate::domain::tool_ref::ToolItem;
use crate::ui::styles;
use crate::ui::styles::colors;

pub fn build_result_items(
    items: &[&ToolItem],
    selected: usize,
    visible: usize,
) -> Vec<ListItem<'static>> {
    let name_w = items
        .iter()
        .map(|i| i.display.chars().count())
        .max()
        .unwrap_or(0);
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
        .map(|(idx, item)| {
            build_result_item(
                item, idx == selected, name_w,
            )
        })
        .collect()
}

fn build_result_item(
    item: &ToolItem,
    selected: bool,
    name_w: usize,
) -> ListItem<'static> {
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let dim = Style::default().fg(colors::DIM_TEXT);
    let name = format!(
        "{:<w$}", item.display, w = name_w,
    );
    let mut spans =
        vec![Span::styled(name, style)];
    if !item.description.is_empty() {
        spans.push(Span::styled(
            format!("  {}", item.description),
            dim,
        ));
    }
    ListItem::new(Line::from(spans))
}

pub fn filter_items<'a>(
    items: &'a [ToolItem],
    query: &str,
) -> Vec<&'a ToolItem> {
    if query.is_empty() {
        return items.iter().collect();
    }
    items
        .iter()
        .filter(|item| {
            item.display
                .to_lowercase()
                .contains(query)
        })
        .collect()
}
