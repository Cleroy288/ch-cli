use serde_json::Value;

use crate::domain::claude::StreamChunk;

use super::stream_parse_extract::{
	extract_assistant, extract_result,
};

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

fn parse_json(line: &str) -> Option<Value> {
	serde_json::from_str(line).ok()
}

fn extract_type(val: &Value) -> Option<&str> {
	val.get("type")?.as_str()
}
