use ratatui::{
    layout::{Alignment, Rect},
    style::Style,
    text::Line,
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;
use crate::fs::FsEntry;
use crate::domain::tool_ref::ToolKind;
use crate::picker::PickerMode;
use crate::ui::styles::{self, colors};

use super::render::{
    build_picker_title, render_empty_results,
    render_picker_help_text,
};
use super::render_mcp_tools::render_mcp_tools_list;
use super::render_repo_select::render_repo_select;
use super::render_slash::render_slash_list;
use super::render_symbols::render_symbol_list;
use super::render_jira_board::render_jira_board;
use super::render_jira_board_select
    ::render_jira_board_select;
use super::render_git_detail::render_git_detail;
use super::render_git_history::{
    render_git_history, render_git_loading,
};
use super::render_git_select
    ::render_git_repo_select;
use super::render_jira_detail::render_jira_detail;
use super::render_jira_filter::render_jira_filter;
use super::render_tool_results::render_tool_results;
use super::render_tools::render_tools_list;

/// Picker overlay dispatch by mode.
pub fn render_picker(
    frame: &mut Frame,
    input_area: Rect,
    app: &App,
) {
    if let PickerMode::Browse { .. } =
        app.picker().mode()
    {
        render_file_list(frame, input_area, app);
        return;
    }
    let picker = app.picker();
    match picker.mode() {
        PickerMode::SlashCommand
        | PickerMode::SlashArg { .. } => {
            render_slash_list(
                frame, input_area, picker,
                &app.skills,
            )
        }
        _ => render_mode(
            frame, input_area, picker,
        ),
    }
}

fn render_mode(
    frame: &mut Frame,
    area: Rect,
    picker: &crate::picker::Picker,
) {
    match picker.mode() {
        PickerMode::Symbols { .. } => {
            render_symbol_list(frame, area, picker)
        }
        PickerMode::Tools => {
            render_tools_list(frame, area, picker)
        }
        PickerMode::ToolResults { .. }
        | PickerMode::ToolLoading { .. } => {
            render_tool_mode(frame, area, picker)
        }
        PickerMode::RepoSelect { .. } => {
            render_repo_select(frame, area, picker)
        }
        PickerMode::McpToolBrowse => {
            render_mcp_tools_list(
                frame, area, picker,
            )
        }
        PickerMode::JiraAssigneeFilter => {
            render_jira_filter(frame, area, picker)
        }
        PickerMode::JiraBoardSelect => {
            render_jira_board_select(
                frame, area, picker,
            )
        }
        PickerMode::JiraTicketDetail => {
            render_jira_detail(frame, area, picker)
        }
        PickerMode::GitRepoSelect => {
            render_git_repo_select(
                frame, area, picker,
            )
        }
        PickerMode::GitLoading => {
            render_git_loading(frame, area)
        }
        PickerMode::GitHistory => {
            render_git_history(frame, area, picker)
        }
        PickerMode::GitDetail => {
            render_git_detail(frame, area, picker)
        }
        _ => {}
    }
}

/// Route tool results to Jira board or generic view
fn render_tool_mode(
    frame: &mut Frame,
    area: Rect,
    picker: &crate::picker::Picker,
) {
    let is_jira = matches!(
        picker.mode(),
        PickerMode::ToolResults { tool }
            if *tool == ToolKind::Jira
    );
    if is_jira && picker.jira_board().is_some() {
        render_jira_board(frame, area, picker);
    } else {
        render_tool_results(frame, area, picker);
    }
}

/// File/folder browse list with help bar.
fn render_file_list(
    frame: &mut Frame,
    area: Rect,
    app: &App,
) {
    let picker = app.picker();
    let results = picker.get_results();
    if results.is_empty() {
        render_empty_results(
            frame, area, picker,
        );
        return;
    }
    let selected = picker.selected_index();
    let visible =
        area.height.saturating_sub(2) as usize;
    let scroll = if selected >= visible {
        selected - visible + 1
    } else {
        0
    };
    let items = build_file_items(
        &results, selected, scroll, visible,
    );
    let title = format!(
        "{} [{}/{}]",
        build_picker_title(picker),
        selected + 1,
        results.len()
    );
    let list = List::new(items).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .title_alignment(Alignment::Left)
            .style(
                Style::default().fg(colors::PICKER),
            ),
    );
    frame.render_widget(list, area);
    render_picker_help_text(frame, area);
}

fn build_file_items(
    results: &[&FsEntry],
    selected: usize,
    scroll: usize,
    visible: usize,
) -> Vec<ListItem<'static>> {
    results
        .iter()
        .enumerate()
        .skip(scroll)
        .take(visible)
        .map(|(idx, entry)| {
            let style = if idx == selected {
                styles::file_list_selected_style()
            } else {
                Style::default()
                    .fg(colors::INPUT_TEXT)
            };
            ListItem::new(Line::from(
                entry.display_name(),
            ))
            .style(style)
        })
        .collect()
}
