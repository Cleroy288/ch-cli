use ratatui::{
    layout::{
        Alignment, Constraint, Layout, Rect,
    },
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, List, ListItem,
        Paragraph,
    },
    Frame,
};

use crate::domain::jira::JiraBoardData;
use crate::picker::Picker;
use crate::ui::strings::tui_labels;
use crate::ui::styles::colors;

use super::render_jira_helpers::format_sprint_info;
use super::render_jira_items::build_ticket_items;

pub fn render_jira_board(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let Some(board) = picker.jira_board() else {
        render_no_data(frame, area);
        return;
    };
    if board.tickets.is_empty() {
        render_no_data(frame, area);
        return;
    }
    let filter =
        picker.jira_assignee_filter();
    let header =
        build_sprint_header(board, filter);
    let items = build_ticket_items(
        &board.tickets, picker, area,
    );
    render_board(frame, area, header, items);
}

fn render_no_data(
    frame: &mut Frame,
    area: Rect,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(tui_labels::JIRA_BOARD_TITLE)
        .style(
            Style::default().fg(colors::PICKER),
        );
    let msg = Paragraph::new(
        tui_labels::JIRA_NO_TICKETS,
    )
    .style(
        Style::default().fg(colors::PLACEHOLDER),
    )
    .alignment(Alignment::Center)
    .block(block);
    frame.render_widget(msg, area);
}

fn build_sprint_header(
    board: &JiraBoardData,
    assignee_filter: Option<&str>,
) -> Line<'static> {
    let sprint_text = match &board.sprint {
        Some(spr) => format_sprint_info(spr),
        None => tui_labels::JIRA_NO_SPRINT
            .to_string(),
    };
    let sp_text = format!(
        " {} {}",
        board.total_points,
        tui_labels::JIRA_SP_LABEL,
    );
    let bold = Style::default()
        .fg(colors::TEXT_LIGHT)
        .add_modifier(Modifier::BOLD);
    let dim =
        Style::default().fg(colors::DIM_TEXT);
    let mut spans = vec![
        Span::styled(sprint_text, bold),
        Span::styled(sp_text, dim),
    ];
    if let Some(name) = assignee_filter {
        spans.push(Span::styled(
            format!(" | @{name}"),
            Style::default().fg(colors::TEXT_LIGHT),
        ));
    }
    Line::from(spans)
}

fn render_board(
    frame: &mut Frame,
    area: Rect,
    header: Line<'static>,
    items: Vec<ListItem<'static>>,
) {
    let chunks = Layout::vertical([
        Constraint::Length(1),
        Constraint::Min(0),
    ])
    .split(area);
    frame.render_widget(
        Paragraph::new(header),
        chunks[0],
    );
    let block = Block::default()
        .borders(Borders::ALL)
        .title(tui_labels::JIRA_BOARD_TITLE)
        .style(
            Style::default().fg(colors::PICKER),
        );
    let list = List::new(items).block(block);
    frame.render_widget(list, chunks[1]);
}
