use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::domain::skill::SkillEntry;
use crate::picker::mode::PickerMode;
use crate::picker::slash_items::{
	self, DisplayItem, ItemKind,
};
use crate::picker::Picker;
use crate::ui::strings::tui_labels;
use crate::ui::styles;
use crate::ui::styles::colors;

const BORDER_HEIGHT: u16 = 2;
const ITEM_SEPARATOR: &str = " \u{2014} ";
const LABEL_SLASH: &str = " Slash ";

use super::render::render_empty_results;

pub fn render_slash_list(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
    skills: &[SkillEntry],
) {
    let items = slash_items::all_display_items(
        picker.mode(),
        picker.query(),
        skills,
    );
    if items.is_empty() {
        render_empty_results(frame, area, picker);
        return;
    }
    let selected = picker.selected_index();
    let list_items =
        build_slash_items(&items, selected, area);
    let list = build_slash_widget(
        picker, items.len(), list_items,
    );
    frame.render_widget(list, area);
}

fn build_slash_items(
    items: &[DisplayItem],
    selected: usize,
    area: Rect,
) -> Vec<ListItem<'static>> {
    let visible = area
        .height
        .saturating_sub(BORDER_HEIGHT) as usize;
    if visible == 0 {
        return vec![];
    }
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
            build_one_item(item, idx == selected)
        })
        .collect()
}

fn build_one_item(
    item: &DisplayItem,
    selected: bool,
) -> ListItem<'static> {
    let style = if selected {
        styles::file_list_selected_style()
    } else if item.kind == ItemKind::Skill {
        Style::default().fg(colors::ACCENT)
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let dim = Style::default().fg(colors::DIM_TEXT);
    let tag = kind_label(item.kind);
    let tag_style = Style::default()
        .fg(colors::STATUS_DIM);
    let spans = vec![
        Span::styled(tag, tag_style),
        Span::styled(
            item.name.clone(), style,
        ),
        Span::styled(ITEM_SEPARATOR, dim),
        Span::styled(
            item.description.clone(), dim,
        ),
    ];
    ListItem::new(Line::from(spans))
}

fn kind_label(kind: ItemKind) -> String {
    match kind {
        ItemKind::Mode => "[mode] ".to_string(),
        ItemKind::Command => "[cmd]  ".to_string(),
        ItemKind::Skill => "[skill]".to_string(),
        ItemKind::Arg => "       ".to_string(),
    }
}

fn build_slash_widget(
    picker: &Picker,
    count: usize,
    list_items: Vec<ListItem<'static>>,
) -> List<'static> {
    let label = match picker.mode() {
        PickerMode::SlashCommand => {
            tui_labels::PICKER_COMMANDS
        }
        PickerMode::SlashArg { command }
            if command == "effort" =>
        {
            tui_labels::PICKER_EFFORT_ARG
        }
        PickerMode::SlashArg { .. } => {
            tui_labels::PICKER_MODEL_ARG
        }
        _ => LABEL_SLASH,
    };
    let title = format!(
        "{} [{}/{}]",
        label,
        picker.selected_index() + 1,
        count,
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
