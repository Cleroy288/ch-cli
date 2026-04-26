use serde_json::Value;

use crate::domain::claude::{StreamChunk, ToolActivity};

const SUMMARY_MAX_LEN: usize = 60;

/// Extract a ToolUse chunk from assistant content blocks.
pub(super) fn extract_tool_use(
	content: &[Value],
) -> Option<StreamChunk> {
	let block = content.iter().find(|blk| {
		blk.get("type").and_then(Value::as_str)
			== Some("tool_use")
	})?;
	let name = block
		.get("name")
		.and_then(Value::as_str)
		.unwrap_or("unknown");
	let summary = summarize_input(name, block);
	Some(StreamChunk::ToolUse(ToolActivity {
		tool_name: name.to_string(),
		summary,
	}))
}

/// Summarize the relevant input field for a given tool.
fn summarize_input(
	name: &str,
	block: &Value,
) -> String {
	let key = field_key_for_tool(name);
	let raw = block
		.get("input")
		.and_then(|v| v.get(key))
		.and_then(Value::as_str)
		.unwrap_or("");
	truncate(raw)
}

/// Map a tool name to the input field most relevant
/// for a short summary.
fn field_key_for_tool(name: &str) -> &str {
	match name {
		"Bash" => "command",
		"Grep" | "Glob" => "pattern",
		"WebSearch" => "query",
		"Task" => "description",
		_ => "file_path",
	}
}

/// Truncate a raw string to SUMMARY_MAX_LEN on a safe
/// char boundary, appending an ellipsis when truncated.
fn truncate(raw: &str) -> String {
	if raw.len() <= SUMMARY_MAX_LEN {
		return raw.to_string();
	}
	let end = raw.floor_char_boundary(SUMMARY_MAX_LEN);
	format!("{}...", &raw[..end])
}
