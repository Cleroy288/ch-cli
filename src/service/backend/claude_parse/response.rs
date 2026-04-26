use serde_json::Value;

use crate::domain::backend::{
	BackendResponse, BackendUsage,
};
use crate::domain::errors::{
	BackendError, BackendResult,
};

/// Raw top-level response payload as returned by the
/// Claude CLI.
#[derive(serde::Deserialize)]
pub(super) struct RawResponse {
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

/// Raw token usage payload.
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

/// Parse a full JSON response string from Claude CLI.
pub fn parse_response(
	raw_json: &str,
) -> BackendResult<BackendResponse> {
	let raw: RawResponse = serde_json::from_str(raw_json)
		.map_err(|e| {
			BackendError::Json(e.to_string())
		})?;
	let resp = convert_raw(raw);
	if resp.is_error {
		return Err(BackendError::Api(
			resp.result.clone(),
		));
	}
	Ok(resp)
}

/// Parse a serde_json::Value into a BackendResponse.
pub(super) fn parse_value(
	val: Value,
) -> BackendResult<BackendResponse> {
	let raw: RawResponse = serde_json::from_value(val)
		.map_err(|e| {
			BackendError::Json(e.to_string())
		})?;
	Ok(convert_raw(raw))
}

/// Convert a RawResponse into the domain BackendResponse.
fn convert_raw(raw: RawResponse) -> BackendResponse {
	BackendResponse {
		result: raw.result.unwrap_or_default(),
		session_id: raw.session_id.unwrap_or_default(),
		subtype: raw
			.subtype
			.unwrap_or_else(|| "success".to_string()),
		is_error: raw.is_error.unwrap_or(false),
		num_turns: raw.num_turns.unwrap_or(0),
		cost_usd: raw.total_cost_usd,
		duration_ms: raw.duration_ms.unwrap_or(0),
		duration_api_ms: raw.duration_api_ms.unwrap_or(0),
		usage: BackendUsage {
			input_tokens: raw.usage.input_tokens,
			output_tokens: raw.usage.output_tokens,
			cache_read_tokens: raw
				.usage
				.cache_read_input_tokens,
			cache_creation_tokens: raw
				.usage
				.cache_creation_input_tokens,
		},
		intent: Default::default(),
	}
}
