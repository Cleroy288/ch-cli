use ratatui::{
    layout::{Constraint, Layout, Rect},
    Frame,
};

use crate::app::App;
use crate::domain::{
    STATUS_LINE_HEIGHT, TITLE_BOX_HEIGHT,
};
use super::components;

/// True when conversation has started
fn has_conversation(app: &App) -> bool {
    !app.history().is_empty()
        || app.last_claude_response().is_some()
        || app.is_claude_loading()
}

/// Full-frame layout: title, output, input, status.
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    if area.height < 5 || area.width < 20 {
        return;
    }
    let active = has_conversation(app);
    let top_h = if active { 0 } else {
        TITLE_BOX_HEIGHT
    };
    let input_h = components::compute_input_height(
        app.input(),
        area.width,
        area.height,
    );
    let chunks = Layout::vertical([
        Constraint::Length(top_h),
        Constraint::Fill(1),
        Constraint::Length(input_h),
        Constraint::Length(STATUS_LINE_HEIGHT),
    ]);
    let [top, output, input, bottom] =
        chunks.areas(frame.area());

    if !active {
        components::render_title(
            frame, top, app.model_name(),
        );
    }
    render_output(frame, output, app);
    components::render_input(frame, input, app);
    let inline_focused = app.inline_blocks()
        .is_some_and(|ib| ib.focus().is_some());
    if !app.picker().is_active()
        && !inline_focused
    {
        components::set_cursor(
            frame,
            input,
            app.input(),
            app.cursor_position(),
        );
    }
    components::render_status_line(
        frame, bottom, app,
    );
}

/// Render the main output panel.
fn render_output(
    frame: &mut Frame,
    output: Rect,
    app: &App,
) {
    app.set_output_area(output);
    if let Some(data) = app.preflight() {
        components::render_preflight_panel(
            frame, output, data,
        );
    } else if let Some(inline) =
        app.inline_blocks()
    {
        components::render_inline_panel(
            frame, output, inline,
        );
    } else if app.picker().is_active() {
        components::render_picker(
            frame, output, app,
        );
    } else {
        components::render_debug_panel(
            frame, output, app,
        );
    }
}
