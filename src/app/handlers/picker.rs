use crossterm::event::KeyCode;

use crate::app::App;
use crate::domain::PICKER_TYPE_OPTIONS;
use crate::picker::PickerMode;

impl App {
    /// Handle picker-specific keyboard events.
    ///
    /// Routes to specific handlers based on picker mode, using guard clauses
    /// to avoid deep nesting. Returns true if the app should quit.
    pub(crate) fn handle_picker_key(&mut self, key: KeyCode) -> bool {
        // Guard clause: Route to appropriate handler based on mode
        match self.picker.mode() {
            PickerMode::Inactive => false,
            PickerMode::ChoosingType => self.handle_type_chooser_key(key),
            PickerMode::File | PickerMode::Folder => self.handle_file_picker_key(key),
        }
    }

    /// Handle keyboard input when in type chooser mode (choosing between file/folder).
    ///
    /// Supports:
    /// - Arrow keys for navigation
    /// - Enter to confirm selection
    /// - Quick shortcuts: 'f' for file, 'd' for folder
    /// - ESC to cancel
    fn handle_type_chooser_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Up => {
                self.picker.move_up();
            }
            KeyCode::Down => {
                self.picker.move_down(PICKER_TYPE_OPTIONS);
            }
            KeyCode::Enter => {
                self.select_picker_type_by_index();
            }
            KeyCode::Esc => {
                self.cancel_picker();
            }
            KeyCode::Char('f') => {
                // Quick shortcut: 'f' for file
                self.picker.select_file_mode();
            }
            KeyCode::Char('d') => {
                // Quick shortcut: 'd' for directory/folder
                self.picker.select_folder_mode();
            }
            _ => {}
        }
        false
    }

    /// Handle keyboard input when in file/folder picker mode (searching and selecting).
    ///
    /// Supports:
    /// - Arrow keys for navigation
    /// - Enter to select current item
    /// - Typing to filter results
    /// - Backspace to edit query or go back to type selection
    /// - F5 to refresh filesystem
    /// - ESC to cancel
    fn handle_file_picker_key(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::F(5) => {
                // F5 to refresh/rescan filesystem
                self.picker.rescan();
            }
            KeyCode::Up => {
                self.picker.move_up();
            }
            KeyCode::Down => {
                let results = self.picker.get_results();
                self.picker.move_down(results.len());
            }
            KeyCode::Enter => {
                self.select_current_picker_entry();
            }
            KeyCode::Esc => {
                self.cancel_picker();
            }
            KeyCode::Backspace => {
                self.handle_picker_backspace();
            }
            KeyCode::Char(c) => {
                self.picker.push_query(c);
            }
            _ => {}
        }
        false
    }

    /// Select picker type based on current selected index (0 = folder, 1 = file)
    fn select_picker_type_by_index(&mut self) {
        if self.picker.selected_index() == 0 {
            self.picker.select_folder_mode();
        } else {
            self.picker.select_file_mode();
        }
    }

    /// Select the currently highlighted entry in the file/folder picker
    fn select_current_picker_entry(&mut self) {
        if let Some(entry) = self.picker.get_selected_entry() {
            self.insert_selected_path(entry.path_string(), entry.name_only(), entry.is_dir);
        }
        self.picker.deactivate();
    }

    /// Handle backspace in picker mode
    fn handle_picker_backspace(&mut self) {
        if self.picker.query().is_empty() {
            // If query is empty, go back to type selection
            let trigger_pos = self.picker.trigger_position();
            self.picker.activate(trigger_pos);
        } else {
            self.picker.pop_query();
        }
    }

    /// Cancel picker and remove the @ trigger from input
    fn cancel_picker(&mut self) {
        self.remove_at_trigger();
        self.picker.deactivate();
    }

    /// Remove the @ trigger symbol from input
    fn remove_at_trigger(&mut self) {
        let trigger_pos = self.picker.trigger_position();
        if trigger_pos < self.input.len() && self.input.chars().nth(trigger_pos) == Some('@') {
            self.input.remove(trigger_pos);
            if self.cursor_position.get() > trigger_pos {
                self.cursor_position.move_left();
            }
        }
    }
}
