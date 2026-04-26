use serde::{Deserialize, Serialize};

#[derive(
	Debug, Clone, Copy, PartialEq, Eq,
	Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum McpServerKind {
	Stdio,
	Http,
}

#[derive(Debug, Clone)]
pub struct McpServerConfig {
	pub name: String,
	pub kind: McpServerKind,
	pub command: Option<String>,
	pub args: Vec<String>,
	pub url: Option<String>,
	pub source: String,
}

impl McpServerConfig {
	pub fn is_stdio(&self) -> bool {
		self.kind == McpServerKind::Stdio
	}
}
