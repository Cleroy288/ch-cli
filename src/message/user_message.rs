use std::time::{SystemTime, UNIX_EPOCH};

use crate::message::MessageSegment;

#[derive(Debug, Clone)]
pub struct UserMessage {
    pub segments: Vec<MessageSegment>,
    /// Unix epoch seconds
    pub timestamp: u64,
    pub raw_input: String,
    pub response: Option<String>,
}

impl UserMessage {
    pub fn new(segments: Vec<MessageSegment>, raw_input: String) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            segments,
            timestamp,
            raw_input,
            response: None,
        }
    }

    pub fn as_display_string(&self) -> String {
        self.segments
            .iter()
            .map(|seg| seg.display_text())
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn debug_string(&self) -> String {
        let mut result = String::new();
        result.push_str("Message Segments:\n");
        for (idx, segment) in self.segments.iter().enumerate() {
            let line = format!(
                "  [{}] {}\n",
                idx,
                segment.debug_string(),
            );
            result.push_str(&line);
        }
        result
    }

    pub fn file_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|seg| matches!(seg, MessageSegment::FileReference { .. }))
            .count()
    }

    pub fn folder_count(&self) -> usize {
        self.segments
            .iter()
            .filter(|seg| matches!(seg, MessageSegment::FolderReference { .. }))
            .count()
    }
}
