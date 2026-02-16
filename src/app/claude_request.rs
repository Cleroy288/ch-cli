//! Spawn background thread for Claude CLI requests.

use std::sync::mpsc;
use std::thread;

use crate::message::MessageSegment;
use crate::service::claude::{
	build_prompt, execute_claude_cli,
	parse_claude_response,
};

use super::App;

/// Spawn a background thread to call Claude CLI.
///
/// Builds the prompt from segments, sends the request,
/// and pipes the result back via mpsc channel.
/// Uses `--continue` to resume the latest session
/// unless the user started a fresh conversation.
pub fn spawn_claude_request(
	app: &mut App,
	segments: &[MessageSegment],
) {
	let prompt = build_prompt(segments);
	let cont = app.continue_session;
	let (sender, recv) = mpsc::channel();
	app.claude_rx = Some(recv);

	thread::spawn(move || {
		let result =
			execute_claude_cli(&prompt, cont)
				.and_then(|json| {
					parse_claude_response(&json)
				});
		let _ = sender.send(result);
	});
}
