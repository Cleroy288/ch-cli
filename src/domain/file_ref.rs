use crate::domain::file_name::FileName;
use crate::domain::file_path::FilePath;

/// File or folder reference in the input
///
/// Uses NewTypes to make the distinction between full paths
/// and display names explicit.
#[derive(Debug, Clone)]
pub struct FileReference {
    /// Start position in input string
    pub start: usize,
    /// End position in input string
    pub end: usize,
    /// Full path of the file/folder (e.g., "./src/main.rs")
    pub full_path: FilePath,
    /// Display name shown in UI (e.g., "main.rs")
    pub display_name: FileName,
    /// Whether it's a directory
    pub is_dir: bool,
}

impl FileReference {
    /// Create a new FileReference
    pub fn new(
        start: usize,
        end: usize,
        full_path: FilePath,
        display_name: FileName,
        is_dir: bool,
    ) -> Self {
        Self {
            start,
            end,
            full_path,
            display_name,
            is_dir,
        }
    }
}
