use crate::domain::claude::{
	ClaudeResponse, StreamChunk, ToolActivity,
};

use super::App;

impl App {
	/// Active backend display name
	pub fn backend_name(&self) -> &str {
		self.backend.name()
	}

	/// Context usage as percentage (0..100).
	/// None if no response yet or window unknown.
	pub fn context_percent(&self) -> Option<u8> {
		let resp = self.last_claude_response.as_ref()?;
		let window = self.backend
			.context_window(&self.model_name);
		if window == 0 {
			return None;
		}
		let used = resp.usage.input_tokens;
		let pct = (used * 100) / window;
		Some(pct.min(100) as u8)
	}

	/// Most recent completed Claude response
	pub fn last_claude_response(
		&self,
	) -> Option<&ClaudeResponse> {
		self.last_claude_response.as_ref()
	}

	/// True while a Claude stream is in progress
	pub fn is_claude_loading(&self) -> bool {
		self.claude_rx.is_some()
	}

	/// Partial markdown accumulated during streaming
	pub fn streaming_text(&self) -> &str {
		&self.streaming_text
	}

	/// Active tool invocation shown in status bar
	pub fn tool_status(
		&self,
	) -> Option<&ToolActivity> {
		self.tool_status.as_ref()
	}

	/// Inject a stream receiver (testing only)
	pub fn set_claude_rx(
		&mut self,
		recv: Option<
			std::sync::mpsc::Receiver<StreamChunk>,
		>,
	) {
		self.claude_rx = recv;
	}
}
