use serde::{Deserialize, Serialize};

/// Generic response from any CLI backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendResponse {
	pub result: String,
	pub session_id: String,
	/// "success" or error kind
	pub subtype: String,
	pub is_error: bool,
	pub num_turns: u32,
	pub cost_usd: Option<f64>,
	/// Wall-clock ms
	pub duration_ms: u64,
	/// API-side ms
	pub duration_api_ms: u64,
	pub usage: BackendUsage,
	/// Whether the response is an implementation
	#[serde(skip)]
	pub intent: super::claude::ResponseIntent,
}

/// Token usage from any backend.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendUsage {
	pub input_tokens: u64,
	pub output_tokens: u64,
	pub cache_read_tokens: u64,
	pub cache_creation_tokens: u64,
}
