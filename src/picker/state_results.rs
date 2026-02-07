use crate::fs::FsEntry;
use super::state::Picker;

impl Picker {
    /// Get filtered results based on current mode and query
    pub fn get_results(&self) -> Vec<&FsEntry> {
        self.scanner.get_results(self.mode, self.query.query())
    }

    /// Get the type options when in ChoosingType mode
    pub fn get_type_options(&self) -> Vec<String> {
        vec!["▸ folder".to_string(), "◆ file".to_string()]
    }

    /// Get the currently selected entry (if any)
    pub fn get_selected_entry(&self) -> Option<&FsEntry> {
        let results = self.get_results();
        results.get(self.selected_index()).copied()
    }

    /// Rescan the file system
    pub fn rescan(&mut self) {
        self.scanner.rescan();
    }

    /// Get the last scan time
    pub fn last_scan_time(&self) -> u64 {
        self.scanner.last_scan_time()
    }
}
