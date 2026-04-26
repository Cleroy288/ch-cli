use std::collections::HashMap;

use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem, Paragraph,
    },
    Frame,
};

use crate::picker::Picker;
use crate::ui::styles;
use crate::ui::styles::colors;

pub fn render_jira_board_select(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let keys = filter_keys(picker);
    if keys.is_empty() {
        render_empty(frame, area);
        return;
    }
    let sel = picker.selected_index();
    let vis =
        area.height.saturating_sub(2) as usize;
    let scroll = sel
        .saturating_sub(vis.saturating_sub(1));
    let items = build_key_items(
        picker, &keys, sel, scroll, vis,
    );
    let block = space_block(sel, keys.len());
    frame.render_widget(
        List::new(items).block(block),
        area,
    );
}

fn filter_keys(picker: &Picker) -> Vec<&str> {
    let query = picker.query().to_lowercase();
    picker
        .jira_project_keys()
        .iter()
        .filter(|key| {
            query.is_empty()
                || key
                    .to_lowercase()
                    .contains(&query)
        })
        .map(|key| key.as_str())
        .collect()
}

fn render_empty(frame: &mut Frame, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Select Space ")
        .title_alignment(Alignment::Left)
        .style(Style::default().fg(colors::PICKER));
    let msg = Paragraph::new(Line::from(
        "No spaces found",
    ))
    .style(Style::default().fg(colors::PLACEHOLDER))
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(msg, area);
}

fn space_block(
    sel: usize,
    total: usize,
) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " Select Space [{}/{}] ",
            sel + 1,
            total,
        ))
        .title_alignment(Alignment::Left)
        .style(Style::default().fg(colors::PICKER))
}

fn build_key_items(
    picker: &Picker,
    keys: &[&str],
    selected: usize,
    scroll: usize,
    visible: usize,
) -> Vec<ListItem<'static>> {
    let tickets = picker
        .jira_board()
        .map(|brd| &brd.tickets[..])
        .unwrap_or(&[]);
    let mut counts: HashMap<&str, usize> =
        HashMap::new();
    for tkt in tickets {
        *counts
            .entry(tkt.project_key.as_str())
            .or_insert(0) += 1;
    }
    let dim =
        Style::default().fg(colors::PLACEHOLDER);
    keys.iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, key)| {
            let count =
                counts.get(*key).copied().unwrap_or(0);
            let style = if idx == selected {
                styles::file_list_selected_style()
            } else {
                Style::default()
                    .fg(colors::INPUT_TEXT)
            };
            let line = Line::from(vec![
                Span::styled(
                    format!("  {key}"), style,
                ),
                Span::styled(
                    format!("  ({count} tickets)"),
                    dim,
                ),
            ]);
            ListItem::new(line)
        })
        .collect()
}
