use crate::domain::claude::QueryMode;
use crate::service::skills::CODING_NORM_INSTRUCTION;

use super::mode_prompts_text::{
	FORMAT_PREFIX, FORMAT_SUFFIX, MCP_INSTRUCTION,
	PROMPT_ACTION, PROMPT_PLAN, PROMPT_QUESTION,
};

const PREFIX_Q: &str = "/Q ";
const PREFIX_A: &str = "/A ";
const PREFIX_P: &str = "/P ";

pub fn detect_query_mode(
	text: &str,
) -> (Option<QueryMode>, &str) {
	if let Some(rest) = text.strip_prefix(PREFIX_Q) {
		return (Some(QueryMode::Question), rest);
	}
	if let Some(rest) = text.strip_prefix(PREFIX_A) {
		return (Some(QueryMode::Action), rest);
	}
	if let Some(rest) = text.strip_prefix(PREFIX_P) {
		return (Some(QueryMode::Plan), rest);
	}
	(None, text)
}

pub fn wrap_prompt(text: &str) -> String {
	format!("{FORMAT_PREFIX}{text}{FORMAT_SUFFIX}")
}

/// Default mode: MCP + clean code reminder
pub fn default_instruction() -> String {
	format!(
		"{MCP_INSTRUCTION}\
		 {CODING_NORM_INSTRUCTION}",
	)
}

/// Mode-specific system instruction
pub fn mode_instruction(
	mode: QueryMode,
) -> String {
	match mode {
		QueryMode::Question => format!(
			"{MCP_INSTRUCTION}{PROMPT_QUESTION}",
		),
		QueryMode::Action => format!(
			"{MCP_INSTRUCTION}\
			 {CODING_NORM_INSTRUCTION}\
			 {PROMPT_ACTION}",
		),
		QueryMode::Plan => format!(
			"{MCP_INSTRUCTION}{PROMPT_PLAN}",
		),
	}
}
