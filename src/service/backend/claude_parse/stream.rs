use serde_json::Value;

use crate::domain::claude::StreamChunk;

use super::assistant::extract_assistant;
use super::response::parse_value;

/// Parse one stream-json line from Claude CLI into
/// zero or more StreamChunk events.
pub fn parse_stream_line(
	line: &str,
) -> Vec<StreamChunk> {
	let Some(val) = parse_json(line) else {
		return Vec::new();
	};
	match extract_type(&val) {
		Some("assistant") => extract_assistant(&val),
		Some("result") => extract_result(val),
		_ => Vec::new(),
	}
}

/// Parse a single JSON line; ignore parse errors.
fn parse_json(line: &str) -> Option<Value> {
	serde_json::from_str(line).ok()
}

/// Read the top-level "type" discriminator.
fn extract_type(val: &Value) -> Option<&str> {
	val.get("type")?.as_str()
}

/// Convert a terminal "result" event into a Done or
/// Error stream chunk.
fn extract_result(val: Value) -> Vec<StreamChunk> {
	let chunk = match parse_value(val) {
		Ok(r) => StreamChunk::Done(r),
		Err(e) => StreamChunk::Error(e.to_string()),
	};
	vec![chunk]
}
