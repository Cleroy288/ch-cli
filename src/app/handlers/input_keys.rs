use crossterm::event::KeyCode;

use crate::app::App;

impl App {
    /// Handle regular input keyboard events.
    ///
    /// Processes character input, backspace, delete,
    /// cursor movement, and Enter.
    /// Returns true if the app should quit.
    pub(crate) fn handle_input_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::Char(chr) => {
                self.handle_char_input(chr)
            }
            KeyCode::Backspace => self.handle_backspace(),
            KeyCode::Delete => self.handle_delete(),
            KeyCode::Left => self.handle_cursor_left(),
            KeyCode::Right => self.handle_cursor_right(),
            KeyCode::Up => self.scroll_up(),
            KeyCode::Down => self.scroll_down(),
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
    fn handle_char_input(&mut self, chr: char) {
        let cursor_pos = self.cursor_position.get();
        self.input.insert(cursor_pos, chr);
        self.cursor_position.move_right(
            self.input.len(),
        );

        if chr == '@' {
            self.picker.activate(cursor_pos);
            return;
        }

        if chr == '#' {
            self.picker.activate_tools(cursor_pos);
            return;
        }

        // Check if ( typed right after a file ref
        if chr == '(' {
            self.try_open_symbol_picker();
        }
    }

    /// Check if cursor is right after a FileReference
    /// and open symbol picker if so.
    pub(crate) fn try_open_symbol_picker(&mut self) {
        let cursor = self.cursor_position.get();
        // The ( was inserted at cursor-1
        let paren_pos = cursor - 1;

        // Find a file ref that ends exactly at paren_pos
        let ref_idx = self
            .file_references
            .iter()
            .position(|fref| fref.end == paren_pos && !fref.is_dir);

        if let Some(idx) = ref_idx {
            self.activate_symbol_picker(idx);
        }
    }

    /// Handle backspace key
    fn handle_backspace(&mut self) {
        let cursor_pos = self.cursor_position.get();
        if cursor_pos > 0 {
            self.cursor_position.move_left();
            self.input.remove(cursor_pos - 1);
            self.update_file_references();
        }
    }

    /// Handle delete key
    fn handle_delete(&mut self) {
        let cursor_pos = self.cursor_position.get();
        if cursor_pos < self.input.len() {
            self.input.remove(cursor_pos);
            self.update_file_references();
        }
    }
}
