//! Parse Claude Code CLI JSON output into domain types.

use crate::domain::claude::{
	ClaudeResponse, ClaudeUsage,
};
use crate::domain::errors::ClaudeError;

/// Intermediate serde target for Claude CLI output.
///
/// Maps the raw JSON keys to Rust fields before
/// converting to the domain ClaudeResponse.
#[derive(serde::Deserialize)]
struct RawResponse {
	result: Option<String>,
	session_id: Option<String>,
	is_error: Option<bool>,
	num_turns: Option<u32>,
	total_cost_usd: Option<f64>,
	duration_ms: Option<u64>,
	#[serde(default)]
	usage: RawUsage,
}

/// Raw usage stats from JSON
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

/// Parse raw JSON string into ClaudeResponse.
///
/// Maps missing fields to sensible defaults.
/// Returns ClaudeError::InvalidJson on parse failure.
pub fn parse_claude_response(
	raw_json: &str,
) -> Result<ClaudeResponse, ClaudeError> {
	let raw: RawResponse =
		serde_json::from_str(raw_json).map_err(
			|err| {
				ClaudeError::InvalidJson(
					err.to_string(),
				)
			},
		)?;

	let response = convert_raw(raw);
	check_api_error(&response)?;
	Ok(response)
}

/// Convert raw serde struct to domain type
fn convert_raw(raw: RawResponse) -> ClaudeResponse {
	ClaudeResponse {
		result: raw.result.unwrap_or_default(),
		session_id: raw
			.session_id
			.unwrap_or_default(),
		is_error: raw.is_error.unwrap_or(false),
		num_turns: raw.num_turns.unwrap_or(0),
		cost_usd: raw.total_cost_usd,
		duration_ms: raw.duration_ms.unwrap_or(0),
		usage: ClaudeUsage {
			input_tokens: raw.usage.input_tokens,
			output_tokens: raw.usage.output_tokens,
			cache_read_tokens: raw
				.usage
				.cache_read_input_tokens,
			cache_creation_tokens: raw
				.usage
				.cache_creation_input_tokens,
		},
	}
}

/// Return ApiError if is_error flag is set
fn check_api_error(
	resp: &ClaudeResponse,
) -> Result<(), ClaudeError> {
	if resp.is_error {
		return Err(ClaudeError::ApiError(
			resp.result.clone(),
		));
	}
	Ok(())
}
