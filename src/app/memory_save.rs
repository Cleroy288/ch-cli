//! Auto-save TUI interactions to memory.
//!
//! Captures user input at handle_enter() time,
//! then persists the full interaction when
//! Claude responds in tick_claude_response().

use std::path::Path;

use crate::domain::claude::ClaudeResponse;
use crate::domain::memory::{AiResponse, UserInput};
use crate::domain::memory_helpers;
use crate::message::segment::MessageSegment;
use crate::service::{
	DefaultMemoryService, MemoryService,
};

/// Extract UserInput from raw text + segments
pub fn extract_user_input(
	raw_input: &str,
	segments: &[MessageSegment],
) -> UserInput {
	let files = extract_file_paths(segments);
	UserInput {
		text: raw_input.to_string(),
		command: None,
		files,
	}
}

/// Collect file paths from segments
fn extract_file_paths(
	segments: &[MessageSegment],
) -> Vec<String> {
	segments
		.iter()
		.filter_map(segment_path)
		.collect()
}

/// Get path from a single segment if it's a ref
fn segment_path(
	seg: &MessageSegment,
) -> Option<String> {
	match seg {
		MessageSegment::FileReference {
			full_path, ..
		} => Some(full_path.clone()),
		MessageSegment::FolderReference {
			full_path, ..
		} => Some(full_path.clone()),
		_ => None,
	}
}

/// Convert ClaudeResponse to AiResponse
pub fn build_ai_response(
	resp: &ClaudeResponse,
) -> AiResponse {
	if resp.is_error {
		AiResponse::Question {
			text: resp.result.clone(),
		}
	} else {
		AiResponse::Answer {
			text: resp.result.clone(),
		}
	}
}

/// Persist interaction (fire-and-forget, logs errors)
#[allow(clippy::print_stderr)]
pub fn save_to_memory(
	session_id: &str,
	user_input: UserInput,
	resp: &ClaudeResponse,
) {
	let ai_resp = build_ai_response(resp);
	let interaction = memory_helpers::new_interaction(
		session_id, user_input, ai_resp,
	);
	let root = Path::new(".");
	let svc = DefaultMemoryService::new();
	if let Err(err) = svc.add(root, &interaction) {
		eprintln!("[memory] auto-save failed: {err}");
	}
}
