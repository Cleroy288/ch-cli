use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported CLI backends.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum BackendKind {
	ClaudeCode,
	GeminiCli,
}

impl BackendKind {
	pub fn binary_name(&self) -> &'static str {
		match self {
			Self::ClaudeCode => "claude",
			Self::GeminiCli => "gemini",
		}
	}
}

impl Default for BackendKind {
	fn default() -> Self {
		Self::ClaudeCode
	}
}

impl fmt::Display for BackendKind {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			Self::ClaudeCode => f.write_str("claude"),
			Self::GeminiCli => f.write_str("gemini"),
		}
	}
}
