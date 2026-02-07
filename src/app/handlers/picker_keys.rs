use crossterm::event::KeyCode;

use crate::app::App;
use crate::domain::PICKER_TYPE_OPTIONS;
use crate::picker::PickerMode;

impl App {
    /// Handle picker-specific keyboard events.
    ///
    /// Routes to specific handlers based on picker mode,
    /// using guard clauses to avoid deep nesting.
    /// Returns true if the app should quit.
    pub(crate) fn handle_picker_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        // Route to appropriate handler based on mode
        match self.picker.mode() {
            PickerMode::Inactive => false,
            PickerMode::ChoosingType => {
                self.handle_type_chooser_key(key)
            }
            PickerMode::File | PickerMode::Folder => {
                self.handle_file_picker_key(key)
            }
        }
    }

    /// Handle keyboard input when in type chooser mode.
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
                self.picker.select_file_mode();
            }
            KeyCode::Char('d') => {
                self.picker.select_folder_mode();
            }
            _ => {}
        }
        false
    }

    /// Handle keyboard input when in file/folder picker mode.
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
}
