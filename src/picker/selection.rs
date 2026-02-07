use super::queries::PickerQuery;

impl PickerQuery {
    /// Get the selected index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Move selection up
    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn move_down(&mut self, max_items: usize) {
        if self.selected_index + 1 < max_items {
            self.selected_index += 1;
        }
    }

    /// Set the selected index directly
    pub fn set_selected_index(&mut self, index: usize) {
        self.selected_index = index;
    }

    /// Reset the selected index to the first item
    pub(super) fn reset_selection(&mut self) {
        self.selected_index = 0;
    }
}
