/// Represents a segment of a user message.
///
/// Messages are composed of multiple segments that can be plain text,
/// file references, or folder references.
#[derive(Debug, Clone)]
pub enum MessageSegment {
    /// Plain text segment
    Text(String),
    /// File reference with full path and display name
    FileReference {
        /// Full path to the file
        full_path: String,
        /// Display name (filename only)
        display_name: String,
    },
    /// Folder reference with full path and display name
    FolderReference {
        /// Full path to the folder
        full_path: String,
        /// Display name (foldername only)
        display_name: String,
    },
}

impl MessageSegment {
    /// Get a debug string representation of the segment
    pub fn debug_string(&self) -> String {
        match self {
            MessageSegment::Text(text) => format!("Text: \"{}\"", text),
            MessageSegment::FileReference {
                full_path,
                display_name,
            } => {
                format!("File: {} ({})", display_name, full_path)
            }
            MessageSegment::FolderReference {
                full_path,
                display_name,
            } => {
                format!("Folder: {} ({})", display_name, full_path)
            }
        }
    }

    /// Get the display text for this segment
    pub fn display_text(&self) -> String {
        match self {
            MessageSegment::Text(text) => text.clone(),
            MessageSegment::FileReference { display_name, .. } => display_name.clone(),
            MessageSegment::FolderReference { display_name, .. } => display_name.clone(),
        }
    }
}
