use ratatui::{
    style::Style,
    text::Span,
};

use crate::domain::jira::JiraTicketDetail;
use crate::ui::styles::colors;

use super::render_jira_helpers::{
    short_name, status_to_color, truncate,
};

/// Column width for ticket key
const COL_KEY: usize = 10;
/// Column width for summary text
const COL_SUMMARY: usize = 30;
/// Column width for status label
const COL_STATUS: usize = 14;
/// Column width for story points
const COL_POINTS: usize = 6;
/// Column width for assignee name
const COL_ASSIGNEE: usize = 12;

/// Assemble key + summary + detail spans
pub fn build_ticket_spans(
    ticket: &JiraTicketDetail,
    base: Style,
) -> Vec<Span<'static>> {
    let key = format!(
        "{:<w$}", ticket.key, w = COL_KEY,
    );
    let summary =
        truncate(&ticket.summary, COL_SUMMARY);
    let mut spans = vec![
        Span::styled(key, base),
        Span::styled(summary, base),
    ];
    spans.extend(build_detail_spans(ticket));
    spans
}

fn build_detail_spans(
    ticket: &JiraTicketDetail,
) -> Vec<Span<'static>> {
    let dim =
        Style::default().fg(colors::DIM_TEXT);
    let status_fg =
        status_to_color(&ticket.status_category);
    let pts = format_points(ticket.story_points);
    let status = format!(
        " {:<w$}", ticket.status, w = COL_STATUS,
    );
    let name = short_name(&ticket.assignee);
    let assignee = format!(
        " @{:<w$}", name, w = COL_ASSIGNEE,
    );
    vec![
        Span::styled(
            status,
            Style::default().fg(status_fg),
        ),
        Span::styled(pts, dim),
        Span::styled(assignee, dim),
    ]
}

fn format_points(points: Option<f64>) -> String {
    let label = match points {
        Some(val) => format!("{val}SP"),
        None => "\u{2014}".into(),
    };
    format!(" {:<w$}", label, w = COL_POINTS)
}
