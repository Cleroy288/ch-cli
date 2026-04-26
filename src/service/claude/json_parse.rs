use crate::domain::claude::{
	ClaudeResponse, ClaudeUsage, SUBTYPE_SUCCESS,
};
use crate::domain::errors::ClaudeError;

#[derive(serde::Deserialize)]
struct RawResponse {
	result: Option<String>,
	session_id: Option<String>,
	subtype: Option<String>,
	is_error: Option<bool>,
	num_turns: Option<u32>,
	total_cost_usd: Option<f64>,
	duration_ms: Option<u64>,
	duration_api_ms: Option<u64>,
	#[serde(default)]
	usage: RawUsage,
}

#[derive(serde::Deserialize, Default)]
struct RawUsage {
	#[serde(default)]
	input_tokens: u64,
	#[serde(default)]
	output_tokens: u64,
	#[serde(default)]
	cache_read_input_tokens: u64,
	#[serde(default)]
	cache_creation_input_tokens: u64,
}

pub fn parse_claude_response(
	raw_json: &str,
) -> Result<ClaudeResponse, ClaudeError> {
	let resp = parse_response_permissive(raw_json)?;
	if resp.is_error {
		return Err(ClaudeError::Api(resp.result));
	}
	Ok(resp)
}

pub fn parse_response_permissive(
	raw_json: &str,
) -> Result<ClaudeResponse, ClaudeError> {
	let raw: RawResponse =
		serde_json::from_str(raw_json).map_err(
			|e| ClaudeError::Json(e.to_string()),
		)?;
	Ok(convert_raw(raw))
}

pub fn parse_value_permissive(
	val: serde_json::Value,
) -> Result<ClaudeResponse, ClaudeError> {
	let raw: RawResponse =
		serde_json::from_value(val).map_err(
			|e| ClaudeError::Json(e.to_string()),
		)?;
	Ok(convert_raw(raw))
}

fn convert_raw(raw: RawResponse) -> ClaudeResponse {
	ClaudeResponse {
		result: raw.result.unwrap_or_default(),
		session_id: raw
			.session_id
			.unwrap_or_default(),
		subtype: raw
			.subtype
			.unwrap_or(SUBTYPE_SUCCESS.to_string()),
		is_error: raw.is_error.unwrap_or(false),
		num_turns: raw.num_turns.unwrap_or(0),
		cost_usd: raw.total_cost_usd,
		duration_ms: raw.duration_ms.unwrap_or(0),
		duration_api_ms: raw
			.duration_api_ms
			.unwrap_or(0),
		usage: convert_usage(raw.usage),
		intent: Default::default(),
	}
}

fn convert_usage(raw: RawUsage) -> ClaudeUsage {
	ClaudeUsage {
		input_tokens: raw.input_tokens,
		output_tokens: raw.output_tokens,
		cache_read_tokens: raw
			.cache_read_input_tokens,
		cache_creation_tokens: raw
			.cache_creation_input_tokens,
	}
}
