use super::backend::{BackendResponse, BackendUsage};

/// Backward-compat aliases
pub type ClaudeResponse = BackendResponse;
pub type ClaudeUsage = BackendUsage;

/// Whether the response proposes code changes
/// or just answers a question.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResponseIntent {
	#[default]
	Question,
	Implementation,
}

pub const SUBTYPE_SUCCESS: &str = "success";

#[derive(Debug, Clone)]
pub struct ToolActivity {
	pub tool_name: String,
	pub summary: String,
}

#[derive(Debug, Clone)]
pub enum StreamChunk {
	Delta(String),
	ToolUse(ToolActivity),
	Done(ClaudeResponse),
	Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryMode {
	Question,
	Action,
	Plan,
}

/// Heuristic: code fences preceded by a file
/// path line → implementation intent.
pub fn detect_intent(
	blocks: &[crate::domain::review::ReviewBlock],
) -> ResponseIntent {
	let has_file_path =
		blocks.iter().any(|b| b.file_path.is_some());
	if has_file_path {
		ResponseIntent::Implementation
	} else {
		ResponseIntent::Question
	}
}
