use std::collections::VecDeque;

use crate::domain::DEFAULT_MAX_MESSAGES;
use crate::message::UserMessage;

/// Stores the conversation history.
///
/// Uses a VecDeque for efficient FIFO operations with automatic
/// size limiting to prevent unbounded memory growth.
#[derive(Debug)]
pub struct ConversationHistory {
    /// Messages in chronological order (newest at the back)
    messages: VecDeque<UserMessage>,
    /// Maximum number of messages to keep
    max_messages: usize,
}

impl ConversationHistory {
    /// Create a new conversation history with default capacity
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: DEFAULT_MAX_MESSAGES,
        }
    }

    /// Create with a specific max size
    pub fn with_capacity(max_messages: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_messages),
            max_messages,
        }
    }

    /// Add a new message to the history.
    ///
    /// Automatically removes oldest messages if max_messages is exceeded.
    pub fn add_message(&mut self, message: UserMessage) {
        self.messages.push_back(message);

        // Remove oldest messages if we exceed max_messages
        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    /// Get all messages in chronological order
    pub fn messages(&self) -> &VecDeque<UserMessage> {
        &self.messages
    }

    /// Get the most recent message
    pub fn last_message(&self) -> Option<&UserMessage> {
        self.messages.back()
    }

    /// Get the number of messages
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// Check if history is empty
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Clear all messages
    pub fn clear(&mut self) {
        self.messages.clear()
    }

    /// Get a debug string showing all messages
    pub fn debug_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!(
            "Conversation History ({} messages):\n\n",
            self.len()
        ));

        for (i, message) in self.messages.iter().enumerate() {
            result.push_str(&format!("=== Message {} ===\n", i + 1));
            result.push_str(&format!("Raw: \"{}\"\n", message.raw_input));
            result.push_str(&message.debug_string());
            result.push_str(&format!(
                "Files: {}, Folders: {}\n\n",
                message.file_count(),
                message.folder_count()
            ));
        }

        result
    }
}

impl Default for ConversationHistory {
    fn default() -> Self {
        Self::new()
    }
}
