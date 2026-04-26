use ratatui::{
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    Frame,
};

use crate::domain::jira_detail::JiraIssueDetail;
use crate::picker::Picker;
use crate::ui::styles::colors;

use super::render_jira_detail_parts::{
    append_comments, append_description,
    render_scrolled,
};

pub fn render_jira_detail(
    frame: &mut Frame,
    area: Rect,
    picker: &Picker,
) {
    let Some(detail) = picker.jira_detail() else {
        return;
    };
    let lines = build_all_lines(detail);
    let scroll = picker.jira_detail_scroll();
    render_scrolled(frame, area, lines, scroll);
}

fn build_all_lines(
    detail: &JiraIssueDetail,
) -> Vec<Line<'static>> {
    let mut lines = build_header_lines(detail);
    lines.push(Line::from(SEPARATOR));
    append_description(&mut lines, detail);
    append_comments(&mut lines, detail);
    lines
}

const SEPARATOR: &str =
    "\u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}\
    \u{2500}\u{2500}\u{2500}\u{2500}";

fn build_header_lines(
    detail: &JiraIssueDetail,
) -> Vec<Line<'static>> {
    let bold = Style::default()
        .fg(colors::TEXT_LIGHT)
        .add_modifier(Modifier::BOLD);
    let dim =
        Style::default().fg(colors::DIM_TEXT);
    let key_line = Line::from(vec![
        Span::styled(detail.key.clone(), bold),
        Span::styled(
            format!("  {}", detail.summary),
            Style::default().fg(colors::INPUT_TEXT),
        ),
    ]);
    let meta_line = build_meta_line(detail, dim);
    vec![key_line, meta_line]
}

fn build_meta_line(
    detail: &JiraIssueDetail,
    dim: Style,
) -> Line<'static> {
    let bold = Style::default()
        .fg(colors::TEXT_LIGHT)
        .add_modifier(Modifier::BOLD);
    let points = detail
        .story_points
        .map(|pts| format!("  SP:{pts}"))
        .unwrap_or_default();
    Line::from(vec![
        Span::styled(
            detail.status.clone(), bold,
        ),
        Span::styled(
            format!("  {}", detail.priority), dim,
        ),
        Span::styled(
            format!("  {}", detail.issue_type), dim,
        ),
        Span::styled(points, dim),
        Span::styled(
            format!("  @{}", detail.assignee), dim,
        ),
    ])
}
