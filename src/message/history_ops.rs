//! History operations for ConversationHistory.

use std::collections::VecDeque;

use crate::message::UserMessage;

use super::ConversationHistory;

impl ConversationHistory {
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
}
