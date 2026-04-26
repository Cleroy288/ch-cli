use std::collections::VecDeque;

use crate::domain::DEFAULT_MAX_MESSAGES;
use crate::message::UserMessage;

#[derive(Debug)]
pub struct ConversationHistory {
    pub(crate) messages: VecDeque<UserMessage>,
    pub(crate) max_messages: usize,
}

impl ConversationHistory {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            max_messages: DEFAULT_MAX_MESSAGES,
        }
    }

    pub fn with_capacity(max_messages: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_messages),
            max_messages,
        }
    }

    /// Drops oldest when max_messages exceeded.
    pub fn add_message(&mut self, message: UserMessage) {
        self.messages.push_back(message);

        // Remove oldest messages if we exceed max_messages
        while self.messages.len() > self.max_messages {
            self.messages.pop_front();
        }
    }

    pub fn set_last_response(&mut self, text: String) {
        if let Some(msg) = self.messages.back_mut() {
            msg.response = Some(text);
        }
    }

    pub fn debug_string(&self) -> String {
        let mut result = String::new();
        result.push_str(&format!(
            "Conversation History ({} messages):\n\n",
            self.len()
        ));

        for (idx, message) in self.messages.iter().enumerate() {
            result.push_str(&format!("=== Message {} ===\n", idx + 1));
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
