//! Non-blocking poll for Claude CLI response.
//!
//! Called each event loop tick (~100ms).
//! Receives the result from the background thread
//! and updates app state.

use std::sync::mpsc::TryRecvError;

use crate::domain::claude::ClaudeResponse;

use super::memory_save;
use super::App;

/// Poll for Claude CLI response.
///
/// Non-blocking: uses try_recv on the channel.
/// On success, stores the response for rendering.
pub fn tick_claude_response(app: &mut App) {
	let Some(recv) = &app.claude_rx else {
		return;
	};
	match recv.try_recv() {
		Ok(Ok(resp)) => {
			persist_if_pending(app, &resp);
			app.last_claude_response = Some(resp);
			app.scroll_offset = 0;
			app.continue_session = true;
			app.claude_rx = None;
		}
		Ok(Err(err)) => {
			app.set_status_message(Some(format!(
				"Claude: {err}",
			)));
			app.claude_rx = None;
		}
		Err(TryRecvError::Empty) => {}
		Err(TryRecvError::Disconnected) => {
			app.claude_rx = None;
		}
	}
}

/// Save pending input + response to memory
fn persist_if_pending(
	app: &mut App,
	resp: &ClaudeResponse,
) {
	if let Some(input) =
		app.pending_user_input.take()
	{
		memory_save::save_to_memory(
			&app.memory_session_id, input, resp,
		);
	}
}
