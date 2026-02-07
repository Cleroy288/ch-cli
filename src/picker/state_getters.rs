use super::state::Picker;
use super::mode::PickerMode;

impl Picker {
    /// Check if the picker is active
    pub fn is_active(&self) -> bool {
        self.mode != PickerMode::Inactive
    }

    /// Get the current mode
    pub fn mode(&self) -> PickerMode {
        self.mode
    }

    /// Get the search query
    pub fn query(&self) -> &str {
        self.query.query()
    }

    /// Get the selected index
    pub fn selected_index(&self) -> usize {
        self.query.selected_index()
    }

    /// Get the trigger position
    pub fn trigger_position(&self) -> usize {
        self.trigger_position
    }
}
