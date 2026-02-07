use ratatui::{
    layout::{Constraint, Layout},
    Frame,
};

use crate::app::App;
use crate::domain::{INPUT_BOX_HEIGHT, TITLE_BOX_HEIGHT};

pub mod components;
pub mod layout;
pub mod strings;
pub mod styles;

// Re-export the goodbye message function for convenience
pub use styles::show_goodbye_message;

/// Render the entire UI.
///
/// This is the main coordinator function that:
/// 1. Defines the overall layout
/// 2. Delegates rendering to component modules
/// 3. Handles conditional rendering (picker vs debug panel)
/// 4. Manages cursor positioning
pub fn render(frame: &mut Frame, app: &App) {
    let area = frame.area();

    // Create main vertical layout using domain constants
    let vertical = Layout::vertical([
        Constraint::Length(TITLE_BOX_HEIGHT),
        Constraint::Length(INPUT_BOX_HEIGHT),
        Constraint::Fill(1),
    ]);
    let [title_area, input_area, bottom_area] = vertical.areas(area);

    // Render title and input (always visible)
    components::render_title(frame, title_area);
    components::render_input(frame, input_area, app);

    // Render picker overlay if active, otherwise show debug panel
    if app.picker().is_active() {
        components::render_picker(frame, input_area, bottom_area, app);
    } else {
        components::render_debug_panel(frame, bottom_area, app);
        // Set cursor position only when picker is not active
        components::set_cursor(frame, input_area, app.cursor_position());
    }
}
