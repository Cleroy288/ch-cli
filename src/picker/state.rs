use std::time::{SystemTime, UNIX_EPOCH};

use crate::fs::{FileScanner, FsEntry};
use crate::picker::PickerMode;

/// File/folder picker state.
///
/// Manages the picker's current mode, search query, file system scanner,
/// and selection state.
pub struct Picker {
    /// Current picker mode
    mode: PickerMode,
    /// Search query for filtering
    query: String,
    /// Currently selected index in the results
    selected_index: usize,
    /// File system scanner
    scanner: FileScanner,
    /// The position in the input where @ was typed
    trigger_position: usize,
    /// Last time the filesystem was scanned
    last_scan_time: u64,
}

impl Picker {
    /// Create a new Picker
    pub fn new() -> Self {
        let mut scanner = FileScanner::new();
        // Scan current directory on initialization
        let _ = scanner.scan_directory(".");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            mode: PickerMode::Inactive,
            query: String::new(),
            selected_index: 0,
            scanner,
            trigger_position: 0,
            last_scan_time: now,
        }
    }

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
        &self.query
    }

    /// Get the selected index
    pub fn selected_index(&self) -> usize {
        self.selected_index
    }

    /// Get the trigger position
    pub fn trigger_position(&self) -> usize {
        self.trigger_position
    }

    /// Get the last scan time
    pub fn last_scan_time(&self) -> u64 {
        self.last_scan_time
    }

    /// Activate the picker in choosing type mode
    pub fn activate(&mut self, position: usize) {
        self.mode = PickerMode::ChoosingType;
        self.query.clear();
        self.selected_index = 0;
        self.trigger_position = position;
    }

    /// Deactivate the picker
    pub fn deactivate(&mut self) {
        self.mode = PickerMode::Inactive;
        self.query.clear();
        self.selected_index = 0;
    }

    /// Select file mode
    pub fn select_file_mode(&mut self) {
        self.mode = PickerMode::File;
        self.query.clear();
        self.reset_selection();
    }

    /// Select folder mode
    pub fn select_folder_mode(&mut self) {
        self.mode = PickerMode::Folder;
        self.query.clear();
        self.reset_selection();
    }

    /// Reset the selected index to the first item.
    ///
    /// This helper eliminates duplication across query manipulation methods.
    fn reset_selection(&mut self) {
        self.selected_index = 0;
    }

    /// Add a character to the search query
    pub fn push_query(&mut self, c: char) {
        self.query.push(c);
        self.reset_selection();
    }

    /// Remove the last character from the query
    pub fn pop_query(&mut self) {
        self.query.pop();
        self.reset_selection();
    }

    /// Clear the query
    pub fn clear_query(&mut self) {
        self.query.clear();
        self.reset_selection();
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

    /// Get the filtered results based on current mode and query
    pub fn get_results(&self) -> Vec<&FsEntry> {
        match self.mode {
            PickerMode::Inactive | PickerMode::ChoosingType => Vec::new(),
            PickerMode::File => self.scanner.search(&self.query, true, false),
            PickerMode::Folder => self.scanner.search(&self.query, false, true),
        }
    }

    /// Get the type options when in ChoosingType mode
    pub fn get_type_options(&self) -> Vec<String> {
        vec!["▸ folder".to_string(), "◆ file".to_string()]
    }

    /// Get the currently selected entry (if any)
    pub fn get_selected_entry(&self) -> Option<&FsEntry> {
        let results = self.get_results();
        results.get(self.selected_index).copied()
    }

    /// Rescan the file system
    pub fn rescan(&mut self) {
        let _ = self.scanner.scan_directory(".");
        self.last_scan_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

impl Default for Picker {
    fn default() -> Self {
        Self::new()
    }
}
