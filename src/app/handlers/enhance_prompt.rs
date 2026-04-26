use std::sync::{mpsc, Arc};
use std::thread;

use crate::app::parser_agent;
use crate::domain::errors::BackendResult;
use crate::service::backend::CliBackend;
use crate::service::claude::enhance;
use crate::ui::strings::tui_labels;

use crate::app::App;

impl App {
	/// Ctrl+E: enhance the current input text.
	///
	/// Strips agent lines, enhances main text,
	/// then reattaches agents in enhance_poll.
	pub(crate) fn start_enhance(&mut self) {
		if !self.can_enhance() {
			return;
		}
		let (main, agent_block) =
			parser_agent::extract_agent_suffix(
				self.input.trim(),
			);
		self.pre_enhance_input =
			Some(self.input.trim().to_string());
		self.pre_enhance_agents = if
			agent_block.is_empty()
		{
			None
		} else {
			Some(agent_block)
		};
		self.input.clear();
		self.cursor_position.set(0);
		self.status_message = Some(
			tui_labels::ENHANCE_WORKING.into(),
		);
		let backend = Arc::clone(&self.backend);
		self.enhance_rx =
			Some(spawn_enhance(main, backend));
	}

	/// /e-prompt <text>: enhance provided text.
	pub(crate) fn start_enhance_prefix(
		&mut self,
		text: &str,
	) {
		if !self.can_enhance() {
			return;
		}
		let owned = text.trim().to_string();
		self.pre_enhance_input =
			Some(owned.clone());
		self.pre_enhance_agents = None;
		self.input.clear();
		self.cursor_position.set(0);
		self.status_message = Some(
			tui_labels::ENHANCE_WORKING.into(),
		);
		let backend = Arc::clone(&self.backend);
		self.enhance_rx =
			Some(spawn_enhance(owned, backend));
	}

	/// Guard: refuse if empty, busy, or streaming.
	fn can_enhance(&mut self) -> bool {
		if self.input.trim().is_empty() {
			self.status_message = Some(
				tui_labels::ENHANCE_EMPTY.into(),
			);
			return false;
		}
		if self.enhance_rx.is_some() {
			self.status_message = Some(
				tui_labels::ENHANCE_BUSY.into(),
			);
			return false;
		}
		if self.claude_rx.is_some() {
			self.status_message = Some(
				tui_labels::ENHANCE_BUSY.into(),
			);
			return false;
		}
		true
	}
}

fn spawn_enhance(
	text: String,
	backend: Arc<dyn CliBackend>,
) -> mpsc::Receiver<BackendResult<String>> {
	let (tx, rx) = mpsc::channel();
	thread::spawn(move || {
		let result =
			enhance::enhance_with_backend(&text, &*backend);
		let _ = tx.send(result);
	});
	rx
}
