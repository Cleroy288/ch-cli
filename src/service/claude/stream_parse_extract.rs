use serde_json::Value;

use crate::domain::claude::StreamChunk;
use crate::service::claude::json_parse;
use crate::service::claude::stream_tool_parse;

/// Returns both Delta and ToolUse when both exist.
pub(crate) fn extract_assistant(
	val: &Value,
) -> Vec<StreamChunk> {
	let Some(content) = assistant_content(val) else {
		return Vec::new();
	};
	let mut chunks = Vec::new();
	let text = collect_text_blocks(content);
	if !text.is_empty() {
		chunks.push(StreamChunk::Delta(text));
	}
	if let Some(tool) =
		stream_tool_parse::extract_tool_use(content)
	{
		chunks.push(tool);
	}
	chunks
}

fn assistant_content(
	val: &Value,
) -> Option<&[Value]> {
	val.get("message")?
		.get("content")?
		.as_array()
		.map(|v| v.as_slice())
}

fn collect_text_blocks(
	content: &[Value],
) -> String {
	content
		.iter()
		.filter(|b| {
			b.get("type").and_then(Value::as_str)
				== Some("text")
		})
		.filter_map(|b| {
			b.get("text").and_then(Value::as_str)
		})
		.collect()
}

pub(crate) fn extract_result(
	val: Value,
) -> Vec<StreamChunk> {
	let chunk =
		match json_parse::parse_value_permissive(val)
		{
			Ok(r) => StreamChunk::Done(r),
			Err(e) => {
				StreamChunk::Error(e.to_string())
			}
		};
	vec![chunk]
}
