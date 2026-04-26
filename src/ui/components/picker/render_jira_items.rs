use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::ListItem,
};

use crate::domain::jira::JiraTicketDetail;
use crate::picker::Picker;
use crate::ui::strings::tui_labels;
use crate::ui::styles;
use crate::ui::styles::colors;

use super::render_jira_filter_logic::filter_tickets;
use super::render_jira_groups::{
    group_by_project_sprint, group_by_sprint,
};
use super::render_jira_rows::render_board_row;

///
/// Index 0 = filter action row.
/// Index 1..N = project/sprint headers + tickets.
pub fn build_ticket_items(
    tickets: &[JiraTicketDetail],
    picker: &Picker,
    area: Rect,
) -> Vec<ListItem<'static>> {
    let sel = picker.selected_index();
    let filter_item =
        build_filter_row(picker, sel == 0);
    let grouped = build_scrolled_rows(
        tickets, picker, area, sel,
    );
    let mut items = vec![filter_item];
    items.extend(grouped);
    items
}

fn build_filter_row(
    picker: &Picker,
    selected: bool,
) -> ListItem<'static> {
    let label = match picker.jira_assignee_filter()
    {
        Some(name) => format!(
            "{}@{name}",
            tui_labels::JIRA_FILTER_ROW,
        ),
        None => tui_labels::JIRA_FILTER_ROW
            .to_string(),
    };
    let style = if selected {
        styles::file_list_selected_style()
    } else {
        Style::default().fg(colors::TEXT_LIGHT)
    };
    ListItem::new(Line::from(label)).style(style)
}

fn build_scrolled_rows(
    tickets: &[JiraTicketDetail],
    picker: &Picker,
    area: Rect,
    sel: usize,
) -> Vec<ListItem<'static>> {
    let filtered = filter_tickets(
        tickets,
        &picker.query().to_lowercase(),
        picker.jira_assignee_filter(),
        picker.jira_selected_project(),
    );
    let rows = build_grouped_rows(filtered);
    let visible =
        area.height.saturating_sub(5) as usize;
    let local_sel = sel
        .saturating_sub(1)
        .min(rows.len().saturating_sub(1));
    let scroll = local_sel.saturating_sub(
        visible.saturating_sub(1),
    );
    rows.into_iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, row)| {
            let active =
                sel > 0 && idx == local_sel;
            render_board_row(row, active)
        })
        .collect()
}

///
/// If a project is selected: sprint-only grouping.
/// If no project selected: project + sprint.
fn build_grouped_rows<'a>(
    filtered: Vec<&'a JiraTicketDetail>,
) -> Vec<super::render_jira_groups::BoardRow<'a>>
{
    let has_sprint = filtered
        .iter()
        .any(|tkt| !tkt.sprint_name.is_empty());
    if has_sprint {
        group_by_sprint(filtered)
    } else {
        group_by_project_sprint(filtered)
    }
}
