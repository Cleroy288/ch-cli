use crate::domain::file_name::FileName;
use crate::domain::file_path::FilePath;

/// Span of characters in the input string
#[derive(Debug, Clone, Copy)]
pub struct InputSpan {
    /// Start position in input string
    pub start: usize,
    /// End position in input string
    pub end: usize,
}

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
    /// Full path of the file/folder
    pub full_path: FilePath,
    /// Display name shown in UI (e.g., "main.rs")
    pub display_name: FileName,
    /// Whether it's a directory
    pub is_dir: bool,
}

impl FileReference {
    /// Create a new FileReference
    pub fn new(
        span: InputSpan,
        full_path: FilePath,
        display_name: FileName,
        is_dir: bool,
    ) -> Self {
        Self {
            start: span.start,
            end: span.end,
            full_path,
            display_name,
            is_dir,
        }
    }
}
