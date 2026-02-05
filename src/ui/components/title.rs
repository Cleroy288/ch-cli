use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::ui::styles::colors;

/// Render the ASCII art "tcah" title.
///
/// This is a pure rendering function that creates the title widget.
pub fn render_title(frame: &mut Frame, area: Rect) {
    let text = vec![
        Line::from("████████  ██████   █████  ██   ██"),
        Line::from("   ██    ██       ██   ██ ██   ██"),
        Line::from("   ██    ██       ███████ ███████"),
        Line::from("   ██    ██       ██   ██ ██   ██"),
        Line::from("   ██     ██████  ██   ██ ██   ██"),
    ];

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" tcah ")
                .title_alignment(Alignment::Center),
        )
        .style(Style::default().fg(colors::TITLE).bold())
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}
