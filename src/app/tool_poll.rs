use std::sync::mpsc::TryRecvError;

use crate::domain::errors::AtlassianError;
use crate::domain::tool_ref::{
    ToolFetchResult, ToolKind,
};
use crate::picker::mode::PickerMode;
use crate::ui::strings::tui_labels;

use super::App;

/// Poll for tool fetch results (non-blocking)
pub fn tick_tool_results(app: &mut App) {
    let Some(recv) = &app.tool_rx else {
        return;
    };
    let tool = extract_loading_tool(app);
    match recv.try_recv() {
        Ok(Ok(items)) => {
            handle_success(app, tool, items);
        }
        Ok(Err(err)) => {
            handle_error(app, tool, err);
        }
        Err(TryRecvError::Empty) => {}
        Err(TryRecvError::Disconnected) => {
            handle_disconnect(app, tool);
        }
    }
}

/// Route fetch result to the appropriate picker state
fn handle_success(
    app: &mut App,
    tool: Option<ToolKind>,
    result: ToolFetchResult,
) {
    let Some(kind) = tool else {
        app.tool_rx = None;
        return;
    };
    match result {
        ToolFetchResult::Items(items) => {
            app.picker
                .set_tool_results(kind, items);
        }
        ToolFetchResult::JiraAssignees(names) => {
            app.picker
                .activate_assignee_picker(names);
        }
        ToolFetchResult::JiraBoard(board) => {
            app.picker.set_jira_board(board);
        }
        ToolFetchResult::JiraDetail(detail) => {
            app.picker.set_jira_detail(detail);
        }
    }
    app.tool_rx = None;
}

/// Surface fetch error in the picker UI
fn handle_error(
    app: &mut App,
    tool: Option<ToolKind>,
    err: AtlassianError,
) {
    if let Some(kind) = tool {
        app.picker
            .set_tool_error(kind, err.to_string());
    }
    app.tool_rx = None;
}

fn handle_disconnect(
    app: &mut App,
    tool: Option<ToolKind>,
) {
    if let Some(kind) = tool {
        app.picker.set_tool_error(
            kind,
            tui_labels::TOOL_FETCH_DISCONNECTED
                .into(),
        );
    }
    app.tool_rx = None;
}

/// Extract the ToolKind from ToolLoading mode
fn extract_loading_tool(
    app: &App,
) -> Option<ToolKind> {
    match app.picker.mode() {
        PickerMode::ToolLoading { tool } => {
            Some(*tool)
        }
        _ => None,
    }
}
