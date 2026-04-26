use ratatui::{
    style::Style,
    text::Line,
    widgets::ListItem,
};

use crate::domain::jira::JiraTicketDetail;
use crate::ui::styles;
use crate::ui::styles::colors;

use super::render_jira_groups::BoardRow;
use super::render_jira_spans::build_ticket_spans;

pub fn render_board_row(
    row: BoardRow<'_>,
    selected: bool,
) -> ListItem<'static> {
    match row {
        BoardRow::ProjectHeader(key) => {
            render_project_header(&key, selected)
        }
        BoardRow::SprintHeader(name) => {
            render_sprint_header(&name, selected)
        }
        BoardRow::Ticket(ticket) => {
            render_ticket_row(ticket, selected)
        }
    }
}

fn render_project_header(
    key: &str,
    selected: bool,
) -> ListItem<'static> {
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::TEXT_LIGHT)
    };
    let label = format!(
        "\u{2550}\u{2550} {key} \u{2550}\u{2550}"
    );
    ListItem::new(Line::from(label)).style(style)
}

fn render_sprint_header(
    name: &str,
    selected: bool,
) -> ListItem<'static> {
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::DIM_TEXT)
    };
    let label = format!(
        "  \u{2500}\u{2500} {name} \u{2500}\u{2500}"
    );
    ListItem::new(Line::from(label)).style(style)
}

fn render_ticket_row(
    ticket: &JiraTicketDetail,
    selected: bool,
) -> ListItem<'static> {
    let base = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::INPUT_TEXT)
    };
    let spans = build_ticket_spans(ticket, base);
    ListItem::new(Line::from(spans))
}
