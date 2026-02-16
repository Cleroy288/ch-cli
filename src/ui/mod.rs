use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;
use crate::domain::{INPUT_BOX_HEIGHT, TITLE_BOX_HEIGHT};

pub mod components;
pub mod layout;
pub mod markdown;
pub mod strings;
pub mod styles;

// Re-export the goodbye message function for convenience
pub use styles::show_goodbye_message;

/// True when conversation has started (title should hide)
fn has_conversation(app: &App) -> bool {
    !app.history().is_empty()
        || app.last_claude_response().is_some()
        || app.is_claude_loading()
}

/// Render the entire UI.
///
/// Collapses the title after the first message
/// to maximize output panel space.
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();
    let title_h = if has_conversation(app) {
        0
    } else {
        TITLE_BOX_HEIGHT
    };

    let vertical = Layout::vertical([
        Constraint::Length(title_h),
        Constraint::Length(INPUT_BOX_HEIGHT),
        Constraint::Fill(1),
    ]);
    let [title_area, input_area, bottom_area] =
        vertical.areas(area);

    // Render title only when visible
    if title_h > 0 {
        components::render_title(
            frame,
            title_area,
            app.doc_progress(),
            app.status_message(),
        );
    }
    components::render_input(frame, input_area, app);

    if app.picker().is_active() {
        components::render_picker(
            frame, input_area, bottom_area, app,
        );
    } else {
        components::render_debug_panel(
            frame, bottom_area, app,
        );
        components::set_cursor(
            frame,
            input_area,
            app.cursor_position(),
        );
    }
}
