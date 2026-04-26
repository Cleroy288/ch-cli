use serde_json::Value;

use crate::domain::claude::StreamChunk;

use super::tool::extract_tool_use;

/// Extract stream chunks from an "assistant" event.
pub(super) fn extract_assistant(
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
	if let Some(tool) = extract_tool_use(content) {
		chunks.push(tool);
	}
	chunks
}

/// Return the assistant message content array if present.
fn assistant_content(val: &Value) -> Option<&[Value]> {
	val.get("message")?
		.get("content")?
		.as_array()
		.map(|v| v.as_slice())
}

/// Concatenate the text of all "text" content blocks.
fn collect_text_blocks(content: &[Value]) -> String {
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
