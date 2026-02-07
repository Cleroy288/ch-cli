use crate::app::App;

impl App {
    /// Select picker type based on index (0 = folder, 1 = file)
    pub(crate) fn select_picker_type_by_index(&mut self) {
        if self.picker.selected_index() == 0 {
            self.picker.select_folder_mode();
        } else {
            self.picker.select_file_mode();
        }
    }

    /// Select the currently highlighted entry in picker
    pub(crate) fn select_current_picker_entry(&mut self) {
        if let Some(entry) = self.picker.get_selected_entry() {
            let path = entry.path_string();
            let name = entry.name_only();
            let is_dir = entry.is_dir;
            self.insert_selected_path(path, name, is_dir);
        }
        self.picker.deactivate();
    }

    /// Handle backspace in picker mode
    pub(crate) fn handle_picker_backspace(&mut self) {
        if self.picker.query().is_empty() {
            // If query is empty, go back to type selection
            let trigger_pos = self.picker.trigger_position();
            self.picker.activate(trigger_pos);
        } else {
            self.picker.pop_query();
        }
    }

    /// Cancel picker and remove the @ trigger from input
    pub(crate) fn cancel_picker(&mut self) {
        self.remove_at_trigger();
        self.picker.deactivate();
    }

    /// Remove the @ trigger symbol from input
    fn remove_at_trigger(&mut self) {
        let trigger_pos = self.picker.trigger_position();
        let trigger_char = self.input.chars().nth(trigger_pos);
        if trigger_pos < self.input.len()
            && trigger_char == Some('@')
        {
            self.input.remove(trigger_pos);
            if self.cursor_position.get() > trigger_pos {
                self.cursor_position.move_left();
            }
        }
    }
}
