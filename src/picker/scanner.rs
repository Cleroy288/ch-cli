use std::time::{SystemTime, UNIX_EPOCH};

use crate::fs::{FileScanner, FsEntry};
use crate::picker::PickerMode;

/// Filesystem scanner wrapper for the Picker.
///
/// Manages scanning and filtering of files and folders.
pub struct PickerScanner {
    /// File system scanner
    scanner: FileScanner,
    /// Last time the filesystem was scanned
    last_scan_time: u64,
}

impl PickerScanner {
    /// Create a new PickerScanner and scan current directory
    pub fn new() -> Self {
        let mut scanner = FileScanner::new();
        let _ = scanner.scan_directory(".");

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            scanner,
            last_scan_time: now,
        }
    }

    /// Get the last scan time
    pub fn last_scan_time(&self) -> u64 {
        self.last_scan_time
    }

    /// Rescan the file system
    pub fn rescan(&mut self) {
        let _ = self.scanner.scan_directory(".");
        self.last_scan_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Get filtered results based on mode and query
    pub fn get_results(
        &self,
        mode: PickerMode,
        query: &str,
    ) -> Vec<&FsEntry> {
        match mode {
            PickerMode::Inactive | PickerMode::ChoosingType => {
                Vec::new()
            }
            PickerMode::File => {
                self.scanner.search(query, true, false)
            }
            PickerMode::Folder => {
                self.scanner.search(query, false, true)
            }
        }
    }
}

impl Default for PickerScanner {
    fn default() -> Self {
        Self::new()
    }
}
