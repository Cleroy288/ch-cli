use std::collections::VecDeque;

use crate::message::UserMessage;

use super::ConversationHistory;

impl ConversationHistory {
	pub fn messages(&self) -> &VecDeque<UserMessage> {
		&self.messages
	}

	pub fn last_message(&self) -> Option<&UserMessage> {
		self.messages.back()
	}

	pub fn len(&self) -> usize {
		self.messages.len()
	}

	pub fn is_empty(&self) -> bool {
		self.messages.is_empty()
	}

	pub fn clear(&mut self) {
		self.messages.clear()
	}
}
