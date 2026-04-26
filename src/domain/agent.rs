use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum AgentKind {
	ClaudeCode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentPreference {
	pub kind: AgentKind,
}

impl AgentKind {
	pub fn label(&self) -> &'static str {
		match self {
			Self::ClaudeCode => "Claude Code",
		}
	}
}
