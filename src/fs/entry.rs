use std::path::PathBuf;

use crate::domain::{DIR_SYMBOL, FILE_SYMBOL};

/// Represents a file system entry.
///
/// Stores the path, name, and type (file or directory) of a filesystem item.
#[derive(Debug, Clone)]
pub struct FsEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
}

impl FsEntry {
    /// Create a new FsEntry
    pub fn new(path: PathBuf, is_dir: bool) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Self { path, name, is_dir }
    }

    /// Get the display name for this entry with appropriate icon.
    ///
    /// Directories are prefixed with DIR_SYMBOL, files with FILE_SYMBOL.
    pub fn display_name(&self) -> String {
        if self.is_dir {
            format!("{} {}", DIR_SYMBOL, self.name)
        } else {
            format!("{} {}", FILE_SYMBOL, self.name)
        }
    }

    /// Get the relative path as a string
    pub fn path_string(&self) -> String {
        self.path.to_str().unwrap_or("").to_string()
    }

    /// Get just the filename or foldername (last component of path)
    pub fn name_only(&self) -> String {
        self.name.clone()
    }
}

