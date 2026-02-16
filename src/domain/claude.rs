//! Domain types for Claude Code CLI integration.
//!
//! Holds the response structure, token usage stats,
//! and active session state.

use serde::{Deserialize, Serialize};

/// Response from Claude Code CLI (JSON output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResponse {
	pub result: String,        // main text output
	pub session_id: String,    // session for continuity
	pub is_error: bool,        // true if Claude errored
	pub num_turns: u32,        // agentic turns used
	pub cost_usd: Option<f64>, // cost in USD
	pub duration_ms: u64,      // wall clock duration
	pub usage: ClaudeUsage,    // token breakdown
}

/// Token usage stats from Claude CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeUsage {
	pub input_tokens: u64,          // prompt tokens
	pub output_tokens: u64,         // completion tokens
	pub cache_read_tokens: u64,     // cache hits
	pub cache_creation_tokens: u64, // cache writes
}
