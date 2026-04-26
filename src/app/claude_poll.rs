use std::sync::mpsc::TryRecvError;

use crate::domain::claude::{
	ClaudeResponse, StreamChunk, SUBTYPE_SUCCESS,
	detect_intent,
};
use crate::domain::review_path::extract_review_blocks;
use crate::review::InlineBlocks;
use crate::ui::strings::tui_labels;

use super::memory_save;
use super::parser_agent_tag::tag_blocks_with_agents;
use super::App;

/// Poll for streaming Claude CLI chunks.
///
/// Drains all available chunks in one tick.
/// Delta -> replace streaming_text (snapshot).
/// Done  -> finalize response, clear streaming state.
pub fn tick_claude_response(app: &mut App) {
	let Some(recv) = app.claude_rx.take() else {
		return;
	};
	loop {
		match recv.try_recv() {
			Ok(chunk) => {
				if apply_chunk(app, chunk) {
					return;
				}
			}
			Err(TryRecvError::Empty) => {
				app.claude_rx = Some(recv);
				return;
			}
			Err(TryRecvError::Disconnected) => {
				handle_disconnect(app);
				return;
			}
		}
	}
}

fn apply_chunk(
	app: &mut App,
	chunk: StreamChunk,
) -> bool {
	match chunk {
		StreamChunk::Delta(text) => {
			app.streaming_text = text;
			app.tool_status = None;
			false
		}
		StreamChunk::ToolUse(activity) => {
			app.tool_status = Some(activity);
			false
		}
		StreamChunk::Done(resp) => {
			finalize_response(app, resp);
			true
		}
		StreamChunk::Error(msg) => {
			app.pending_block_question = None;
			app.pending_user_input = None;
			app.set_status_message(Some(msg));
			true
		}
	}
}

/// Finalize: persist, warn, store response, clear
fn finalize_response(
	app: &mut App,
	mut resp: ClaudeResponse,
) {
	if let Some(input) =
		app.pending_user_input.take()
	{
		memory_save::save_to_memory(
			&app.memory_session_id, input, &resp,
		);
	}
	if resp.subtype != SUBTYPE_SUCCESS {
		let msg = format!(
			"{}{}",
			tui_labels::CLAUDE_STOPPED_PREFIX,
			resp.subtype.replace('_', " "),
		);
		app.set_status_message(Some(msg));
	}
	app.claude_session_id =
		Some(resp.session_id.clone());
	app.last_claude_response = None;
	app.streaming_text.clear();
	app.tool_status = None;
	app.claude_rx = None;

	if let Some(idx) =
		app.pending_block_question.take()
	{
		if let Some(ib) = &mut app.inline_blocks {
			ib.set_answer(
				idx, resp.result.clone(),
			);
		}
		app.history.set_last_response(resp.result);
		return;
	}

	app.history.set_last_response(
		resp.result.clone(),
	);
	app.scroll_offset = 0;
	try_auto_activate(app, &mut resp);
	app.last_claude_response = Some(resp);
}

/// Auto-activate inline blocks when the response
/// contains code fences. Sets intent on response.
fn try_auto_activate(
	app: &mut App,
	resp: &mut ClaudeResponse,
) {
	let mut blocks =
		extract_review_blocks(&resp.result);
	if blocks.is_empty() {
		return;
	}
	tag_blocks_with_agents(
		&mut blocks, &resp.result,
	);
	let intent = detect_intent(&blocks);
	resp.intent = intent;
	app.inline_blocks = Some(
		InlineBlocks::activate_with_intent(
			blocks, resp.result.clone(), intent,
		),
	);
}


/// Handle disconnected channel (no Done received)
fn handle_disconnect(app: &mut App) {
	app.pending_block_question = None;
	app.pending_user_input = None;
	let msg = if app.streaming_text.is_empty() {
		tui_labels::CLAUDE_NO_RESPONSE
	} else {
		tui_labels::CLAUDE_INTERRUPTED
	};
	app.streaming_text.clear();
	app.tool_status = None;
	app.scroll_offset = 0;
	app.set_status_message(Some(msg.into()));
}
