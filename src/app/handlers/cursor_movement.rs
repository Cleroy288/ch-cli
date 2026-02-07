use crate::app::App;

impl App {
    /// Move cursor left
    pub(super) fn handle_cursor_left(&mut self) {
        self.cursor_position.move_left();
    }

    /// Move cursor right
    pub(super) fn handle_cursor_right(&mut self) {
        self.cursor_position.move_right(self.input.len());
    }
}
