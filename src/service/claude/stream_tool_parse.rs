use serde_json::Value;

use crate::domain::claude::{StreamChunk, ToolActivity};

const SUMMARY_MAX_LEN: usize = 60;

pub fn extract_tool_use(
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
	let input = block.get("input");
	let summary = summarize_tool_input(name, input);
	Some(StreamChunk::ToolUse(ToolActivity {
		tool_name: name.to_string(),
		summary,
	}))
}

fn summarize_tool_input(
	name: &str,
	input: Option<&Value>,
) -> String {
	let raw = extract_field(name, input);
	truncate_summary(raw)
}

fn extract_field<'a>(
	name: &str,
	input: Option<&'a Value>,
) -> &'a str {
	let key = field_key_for_tool(name);
	input
		.and_then(|v| v.get(key))
		.and_then(Value::as_str)
		.unwrap_or("")
}

fn field_key_for_tool(name: &str) -> &str {
	match name {
		"Bash" => "command",
		"Grep" | "Glob" => "pattern",
		"WebSearch" => "query",
		"Task" => "description",
		_ => "file_path",
	}
}

fn truncate_summary(raw: &str) -> String {
	if raw.len() <= SUMMARY_MAX_LEN {
		return raw.to_string();
	}
	let end = raw.floor_char_boundary(SUMMARY_MAX_LEN);
	format!("{}...", &raw[..end])
}
