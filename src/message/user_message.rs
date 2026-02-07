use std::time::{SystemTime, UNIX_EPOCH};

use crate::message::MessageSegment;

/// Represents a complete user message.
///
/// Contains the parsed segments, timestamp, and raw input for reference.
#[derive(Debug, Clone)]
pub struct UserMessage {
    /// The segments that make up this message
    pub segments: Vec<MessageSegment>,
    /// When this message was created
    pub timestamp: u64,
    /// The raw input text (for reference)
    pub raw_input: String,
}

impl UserMessage {
    /// Create a new user message
    pub fn new(segments: Vec<MessageSegment>, raw_input: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            segments,
            timestamp,
            raw_input,
        }
    }

    /// Get a human-readable representation of this message
    pub fn as_display_string(&self) -> String {
        self.segments
            .iter()
            .map(|s| s.display_text())
            .collect::<Vec<_>>()
            .join("")
    }

    /// Get a debug representation showing the parsed structure
    pub fn debug_string(&self) -> String {
        let mut result = String::new();
        result.push_str("Message Segments:\n");
        for (i, segment) in self.segments.iter().enumerate() {
            result.push_str(&format!("  [{}] {}\n", i, segment.debug_string()));
        }
        result
    }

    /// Count the number of file references in this message
    pub fn file_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| matches!(s, MessageSegment::FileReference { .. }))
            .count()
    }

    /// Count the number of folder references in this message
    pub fn folder_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|s| matches!(s, MessageSegment::FolderReference { .. }))
            .count()
    }
}
