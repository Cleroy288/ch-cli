use ratatui::{
    layout::{Alignment, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders},
    Frame,
};

use crate::ui::wrap::pre_wrap;

use crate::domain::jira_detail::JiraIssueDetail;
use crate::ui::styles::colors;

fn heading_style() -> Style {
    Style::default()
        .fg(colors::TEXT_LIGHT)
        .add_modifier(Modifier::BOLD)
}

pub fn append_description(
    lines: &mut Vec<Line<'static>>,
    detail: &JiraIssueDetail,
) {
    lines.push(Line::from(Span::styled(
        "Description:", heading_style(),
    )));
    if detail.description.is_empty() {
        let dim =
            Style::default().fg(colors::DIM_TEXT);
        lines.push(Line::from(Span::styled(
            "(no description)", dim,
        )));
    } else {
        for line in detail.description.lines() {
            lines.push(Line::from(
                line.to_string(),
            ));
        }
    }
    lines.push(Line::from(""));
}

pub fn append_comments(
    lines: &mut Vec<Line<'static>>,
    detail: &JiraIssueDetail,
) {
    let count = detail.comments.len();
    let dim =
        Style::default().fg(colors::DIM_TEXT);
    lines.push(Line::from(Span::styled(
        format!("Comments ({count}):"),
        heading_style(),
    )));
    if detail.comments.is_empty() {
        lines.push(Line::from(Span::styled(
            "(no comments)", dim,
        )));
        return;
    }
    for comment in &detail.comments {
        let date = comment
            .created
            .get(..10)
            .unwrap_or(&comment.created);
        lines.push(Line::from(vec![
            Span::styled(
                format!("@{}", comment.author),
                heading_style(),
            ),
            Span::styled(
                format!(" ({date})"), dim,
            ),
        ]));
        for line in comment.body.lines() {
            lines.push(Line::from(
                format!("  {line}"),
            ));
        }
        lines.push(Line::from(""));
    }
}

pub fn render_scrolled(
    frame: &mut Frame,
    area: Rect,
    lines: Vec<Line<'static>>,
    scroll: usize,
) {
    if area.width == 0 || area.height == 0 {
        return;
    }
    let block = detail_block();
    let inner = block.inner(area);
    frame.render_widget(block, area);
    let wrapped = pre_wrap(lines, inner.width);
    let total = wrapped.len();
    let h = inner.height as usize;
    let skip = scroll.min(
        total.saturating_sub(h),
    );
    write_lines(frame, inner, &wrapped, skip);
}

fn detail_block() -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .title(" Ticket Detail ")
        .title_alignment(Alignment::Left)
        .style(
            Style::default().fg(colors::PICKER),
        )
}

fn write_lines(
    frame: &mut Frame,
    area: Rect,
    lines: &[Line<'_>],
    skip: usize,
) {
    let buf = frame.buffer_mut();
    let h = area.height as usize;
    let w = area.width;
    for i in 0..h {
        let y = area.y + i as u16;
        if let Some(line) = lines.get(skip + i) {
            buf.set_line(area.x, y, line, w);
        }
    }
}
