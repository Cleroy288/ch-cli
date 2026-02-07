use crate::app::parser;
use crate::app::App;
use crate::domain::{FileName, FilePath, FileReference};

impl App {
    /// Jump cursor to start
    pub(crate) fn handle_home(&mut self) {
        self.cursor_position.jump_to_start();
    }

    /// Jump cursor to end
    pub(crate) fn handle_end(&mut self) {
        self.cursor_position.jump_to_end(self.input.len());
    }

    /// Handle enter key - parse and store message
    pub(crate) fn handle_enter(&mut self) {
        // Use the parser module for pure parsing logic
        let refs = &self.file_references;
        if let Some(message) =
            parser::parse_input_to_message(
                self.input.clone(),
                refs,
            )
        {
            self.history.add_message(message);
        }

        self.input.clear();
        self.cursor_position.jump_to_start();
        self.file_references.clear();
    }

    /// Update file references when input changes.
    /// Removes references that are no longer valid due to edits.
    pub(crate) fn update_file_references(&mut self) {
        let input_len = self.input.len();
        self.file_references.retain(|r| r.end <= input_len);
    }

    /// Insert a selected file/folder path at the trigger position.
    ///
    /// Called when the user selects a file/folder from the picker.
    /// Removes the @ symbol and inserts the display name.
    pub(crate) fn insert_selected_path(
        &mut self,
        full_path_str: String,
        name_only: String,
        is_dir: bool,
    ) {
        let trigger_pos = self.picker.trigger_position();

        // Remove the @ symbol
        let trigger_char = self.input.chars().nth(trigger_pos);
        if trigger_pos < self.input.len()
            && trigger_char == Some('@')
        {
            self.input.remove(trigger_pos);
            self.cursor_position.set(trigger_pos);
        }

        // Track the start position
        let start_pos = self.cursor_position.get();

        // Insert only the filename/foldername
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
