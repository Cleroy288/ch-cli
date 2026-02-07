use super::state::Picker;
use super::mode::PickerMode;

impl Picker {
    /// Activate the picker in choosing type mode
    pub fn activate(&mut self, position: usize) {
        self.mode = PickerMode::ChoosingType;
        self.query.clear();
        self.trigger_position = position;
    }

    /// Deactivate the picker
    pub fn deactivate(&mut self) {
        self.mode = PickerMode::Inactive;
        self.query.clear();
    }

    /// Select file mode
    pub fn select_file_mode(&mut self) {
        self.mode = PickerMode::File;
        self.query.clear();
    }

    /// Select folder mode
    pub fn select_folder_mode(&mut self) {
        self.mode = PickerMode::Folder;
        self.query.clear();
    }
}
