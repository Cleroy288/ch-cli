use std::fs;
use std::path::Path;

use crate::domain::MAX_RECURSION_DEPTH;
use crate::fs::FsEntry;

/// File system scanner for indexing files and directories.
///
/// Recursively scans directories up to MAX_RECURSION_DEPTH and provides
/// methods for filtering and searching entries.
pub struct FileScanner {
    entries: Vec<FsEntry>,
}

impl FileScanner {
    /// Create a new FileScanner
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Scan a directory and index all files and folders recursively.
    ///
    /// Entries are sorted with directories first, then files, alphabetically.
    pub fn scan_directory<P: AsRef<Path>>(&mut self, path: P) -> std::io::Result<()> {
        self.entries.clear();
        self.scan_recursive(path.as_ref(), 0)?;

        // Sort entries: directories first, then files, alphabetically
        self.entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        });

        Ok(())
    }

    /// Recursively scan a directory.
    ///
    /// Uses let-else pattern for clean error handling. Entries that can't be
    /// read are silently skipped.
    fn scan_recursive(&mut self, path: &Path, depth: usize) -> std::io::Result<()> {
        // Limit recursion depth to avoid infinite loops and performance issues
        if depth > MAX_RECURSION_DEPTH {
            return Ok(());
        }

        let entries = fs::read_dir(path)?;

        for entry in entries {
            // Use let-else pattern (Rust 1.65+) for cleaner error handling
            let Ok(entry) = entry else { continue };

            let entry_path = entry.path();

            // Don't skip any files or folders - show everything

            let Ok(metadata) = entry.metadata() else {
                continue;
            };

            if metadata.is_dir() {
                self.entries.push(FsEntry::new(entry_path.clone(), true));
                // Recursively scan subdirectories
                let _ = self.scan_recursive(&entry_path, depth + 1);
            } else if metadata.is_file() {
                self.entries.push(FsEntry::new(entry_path, false));
            }
        }

        Ok(())
    }

    /// Get all entries
    pub fn entries(&self) -> &[FsEntry] {
        &self.entries
    }

    /// Get only directories
    pub fn directories(&self) -> Vec<&FsEntry> {
        self.entries.iter().filter(|e| e.is_dir).collect()
    }

    /// Get only files
    pub fn files(&self) -> Vec<&FsEntry> {
        self.entries.iter().filter(|e| !e.is_dir).collect()
    }

    /// Search entries by name (case-insensitive).
    ///
    /// Can filter by files only or directories only. Empty query returns all entries.
    pub fn search(&self, query: &str, files_only: bool, dirs_only: bool) -> Vec<&FsEntry> {
        let query_lower = query.to_lowercase();

        self.entries
            .iter()
            .filter(|e| {
                // Filter by type if specified
                if files_only && e.is_dir {
                    return false;
                }
                if dirs_only && !e.is_dir {
                    return false;
                }

                // Filter by name
                if query_lower.is_empty() {
                    return true;
                }

                e.name.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
}

impl Default for FileScanner {
    fn default() -> Self {
        Self::new()
    }
}
