//! Label helpers for memory response types.

use crate::domain::memory::AiResponse;

// Re-export ID functions so callers don't break
pub use super::memory_id::{
	current_timestamp, generate_id,
	new_interaction, new_session_id,
};

/// Response type label: plain answer
pub const LABEL_ANSWER: &str = "answer";
/// Response type label: follow-up question
pub const LABEL_QUESTION: &str = "question";
/// Response type label: code change
pub const LABEL_CODE_CHANGE: &str = "code_change";

/// Label for the response type variant
pub fn response_type_label(
	response: &AiResponse,
) -> &str {
	match response {
		AiResponse::Answer { .. } => LABEL_ANSWER,
		AiResponse::Question { .. } => LABEL_QUESTION,
		AiResponse::CodeChange { .. } => {
			LABEL_CODE_CHANGE
		}
	}
}

/// Build AiResponse from a type label string
pub fn response_from_label(
	label: &str,
	text: String,
) -> AiResponse {
	match label {
		LABEL_QUESTION => {
			AiResponse::Question { text }
		}
		LABEL_CODE_CHANGE => AiResponse::CodeChange {
			text,
			changes: Vec::new(),
		},
		_ => AiResponse::Answer { text },
	}
}
