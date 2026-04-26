use std::fmt;

/// Thinking effort levels for the Claude CLI.
#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
)]
pub enum Effort {
	Low,
	Medium,
	High,
	Max,
}

pub const DEFAULT_EFFORT: &str = "high";

const VALID: &[&str] =
	&["low", "medium", "high", "max"];

pub fn is_valid(name: &str) -> bool {
	VALID.contains(&name)
}

impl fmt::Display for Effort {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let s = match self {
			Self::Low => "low",
			Self::Medium => "medium",
			Self::High => "high",
			Self::Max => "max",
		};
		f.write_str(s)
	}
}
