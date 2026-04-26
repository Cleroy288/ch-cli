use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::picker::{Picker, PickerMode};
use crate::ui::styles::colors;

pub fn render_empty_results(
    frame: &mut Frame,
    picker_area: Rect,
    picker: &Picker,
) {
    let title = build_picker_title(picker);
    let message = if picker.query().is_empty() {
        "Start typing to search..."
    } else {
        "No results found"
    };

    let paragraph = Paragraph::new(Line::from(message))
        .style(
            Style::default().fg(colors::PLACEHOLDER),
        )
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .title(title)
                .title_alignment(Alignment::Left)
                .style(
                    Style::default().fg(colors::PICKER),
                ),
        );

    frame.render_widget(paragraph, picker_area);
}

pub fn render_picker_help_text(
    frame: &mut Frame,
    picker_area: Rect,
) {
    let help_area = Rect {
        x: picker_area.x.saturating_add(2),
        y: picker_area
            .y
            .saturating_add(picker_area.height),
        width: picker_area.width.saturating_sub(4),
        height: 1,
    };
    let help_text = build_help_spans();
    let help_style =
        Style::default().fg(colors::PLACEHOLDER);
    let paragraph =
        Paragraph::new(help_text).style(help_style);
    frame.render_widget(paragraph, help_area);
}

fn build_help_spans() -> Line<'static> {
    let bold = |key: &'static str, color: Color| {
        Span::styled(
            key,
            Style::default()
                .fg(color)
                .add_modifier(Modifier::BOLD),
        )
    };
    Line::from(vec![
        bold("↑↓", colors::TITLE),
        Span::raw(" navigate  "),
        bold("Enter", colors::PICKER),
        Span::raw(" select  "),
        bold("Tab", colors::TITLE),
        Span::raw(" folder  "),
        bold("F5", colors::MESSAGE_HEADER),
        Span::raw(" refresh  "),
        bold("ESC", colors::FILE_REF_BG),
        Span::raw(" cancel"),
    ])
}

pub fn build_picker_title(
    picker: &Picker,
) -> String {
    match picker.mode() {
        PickerMode::Browse { .. } => format!(
            " Browse (query: '{}') ",
            picker.query()
        ),
        PickerMode::Symbols { .. } => {
            let parent = picker
                .symbol_browser()
                .and_then(|b| b.current_parent())
                .unwrap_or("top-level");
            format!(
                " Symbols [{}] (query: '{}') ",
                parent,
                picker.query()
            )
        }
        PickerMode::ToolResults { tool } => {
            format!(" {} ", tool.display_name())
        }
        PickerMode::Tools => " Tools ".into(),
        PickerMode::ToolLoading { .. } => {
            " Loading... ".into()
        }
        PickerMode::RepoSelect { .. } => {
            " Select Repo ".into()
        }
        PickerMode::McpToolBrowse => {
            " MCP Tools ".into()
        }
        PickerMode::SlashCommand => {
            " Commands ".into()
        }
        PickerMode::SlashArg { .. } => {
            " Model ".into()
        }
        PickerMode::JiraTicketDetail => {
            " Ticket Detail ".into()
        }
        _ => " Search ".into(),
    }
}
