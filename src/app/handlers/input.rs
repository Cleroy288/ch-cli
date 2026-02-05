use crossterm::event::KeyCode;

use crate::app::parser;
use crate::app::App;
use crate::domain::{FileName, FilePath, FileReference};

impl App {
    /// Handle regular input keyboard events (when picker is not active).
    ///
    /// Processes character input, backspace, delete, cursor movement, and Enter.
    /// Returns true if the app should quit.
    pub(crate) fn handle_input_key(&mut self, key: KeyCode) -> bool {
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
            // Rescan filesystem to get latest files/folders
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

    /// Move cursor left
    fn handle_cursor_left(&mut self) {
        self.cursor_position.move_left();
    }

    /// Move cursor right
    fn handle_cursor_right(&mut self) {
        self.cursor_position.move_right(self.input.len());
    }

    /// Jump cursor to start
    fn handle_home(&mut self) {
        self.cursor_position.jump_to_start();
    }

    /// Jump cursor to end
    fn handle_end(&mut self) {
        self.cursor_position.jump_to_end(self.input.len());
    }

    /// Handle enter key - parse and store message
    fn handle_enter(&mut self) {
        // Use the parser module for pure parsing logic
        if let Some(message) =
            parser::parse_input_to_message(self.input.clone(), &self.file_references)
        {
            self.history.add_message(message);
        }

        // Clear input for next message
        self.input.clear();
        self.cursor_position.jump_to_start();
        self.file_references.clear();
    }

    /// Update file references when input changes.
    /// Removes references that are no longer valid due to edits.
    pub(crate) fn update_file_references(&mut self) {
        self.file_references.retain(|r| r.end <= self.input.len());
    }

    /// Insert a selected file/folder path at the trigger position.
    ///
    /// This is called when the user selects a file/folder from the picker.
    /// It removes the @ symbol and inserts the display name (not full path).
    pub(crate) fn insert_selected_path(
        &mut self,
        full_path_str: String,
        name_only: String,
        is_dir: bool,
    ) {
        let trigger_pos = self.picker.trigger_position();

        // Remove the @ symbol
        if trigger_pos < self.input.len() && self.input.chars().nth(trigger_pos) == Some('@') {
            self.input.remove(trigger_pos);
            self.cursor_position.set(trigger_pos);
        }

        // Track the start position
        let start_pos = self.cursor_position.get();

        // Insert only the filename/foldername (not the full path)
        for c in name_only.chars() {
            let cursor_pos = self.cursor_position.get();
            self.input.insert(cursor_pos, c);
            self.cursor_position.move_right(self.input.len());
        }

        // Track this as a file reference
        let file_ref = FileReference::new(
            start_pos,
            self.cursor_position.get(),
            FilePath::from(full_path_str),
            FileName::from(name_only),
            is_dir,
        );
        self.file_references.push(file_ref);
    }
}
