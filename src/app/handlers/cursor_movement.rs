use crate::app::App;

impl App {
    /// Step one character left (UTF-8 aware)
    pub(super) fn handle_cursor_left(&mut self) {
        self.cursor_position.move_left(&self.input);
    }

    /// Step one character right (UTF-8 aware)
    pub(super) fn handle_cursor_right(&mut self) {
        self.cursor_position.move_right(&self.input);
    }
}
