use std::path::Path;

use crate::domain::claude::ClaudeResponse;
use crate::domain::memory::{AiResponse, UserInput};
use crate::service::memory::id_gen;
use crate::message::segment::MessageSegment;
use crate::service::{
	DefaultMemoryService, MemoryService,
};

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

fn extract_file_paths(
	segments: &[MessageSegment],
) -> Vec<String> {
	segments
		.iter()
		.filter_map(segment_path)
		.collect()
}

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
	let interaction = id_gen::new_interaction(
		session_id, user_input, ai_resp,
	);
	let root = Path::new(".");
	let svc = DefaultMemoryService::default();
	if let Err(err) = svc.add(root, &interaction) {
		eprintln!("[memory] auto-save failed: {err}");
	}
}
