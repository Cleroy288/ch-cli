use crossterm::event::KeyCode;

use crate::app::App;

impl App {
    /// Handle regular input keyboard events (when not active).
    ///
    /// Processes character input, backspace, delete,
    /// cursor movement, and Enter.
    /// Returns true if the app should quit.
    pub(crate) fn handle_input_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::Char(c) => self.handle_char_input(c),
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Delete => self.handle_delete(),
            KeyCode::Left => self.handle_cursor_left(),
            KeyCode::Right => self.handle_cursor_right(),
            KeyCode::Home => self.handle_home(),
            KeyCode::End => self.handle_end(),
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Esc => {
                self.should_quit = true;
                return true;
            }
            _ => {}
        }
        false
    }

    /// Insert a character at the cursor position
    fn handle_char_input(&mut self, c: char) {
        let cursor_pos = self.cursor_position.get();
        self.input.insert(cursor_pos, c);
        self.cursor_position.move_right(self.input.len());

        // Check if @ was typed to activate picker
        if c == '@' {
            self.picker.rescan();
            self.picker.activate(cursor_pos);
        }
    }

    /// Handle backspace key - delete character before cursor
    fn handle_backspace(&mut self) {
        let cursor_pos = self.cursor_position.get();
        if cursor_pos > 0 {
            self.cursor_position.move_left();
            self.input.remove(cursor_pos - 1);
            self.update_file_references();
        }
    }

    /// Handle delete key - delete character at cursor
    fn handle_delete(&mut self) {
        let cursor_pos = self.cursor_position.get();
        if cursor_pos < self.input.len() {
            self.input.remove(cursor_pos);
            self.update_file_references();
        }
    }

}

