use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
	pub id: String,
	pub timestamp: u64,
	pub session_id: String,
	pub input: UserInput,
	pub response: AiResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInput {
	pub text: String,
	pub command: Option<String>,
	pub files: Vec<String>,
}

#[derive(
	Debug, Clone, Serialize, Deserialize,
)]
#[serde(tag = "type")]
pub enum AiResponse {
	Answer { text: String },
	Question { text: String },
	CodeChange {
		text: String,
		changes: Vec<FileChange>,
	},
}

#[derive(
	Debug, Clone, Serialize, Deserialize,
	PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
pub enum FileAction {
	Create,
	Modify,
	Delete,
	Rename,
}

impl fmt::Display for FileAction {
	fn fmt(
		&self,
		f: &mut fmt::Formatter<'_>,
	) -> fmt::Result {
		match self {
			Self::Create => f.write_str("create"),
			Self::Modify => f.write_str("modify"),
			Self::Delete => f.write_str("delete"),
			Self::Rename => f.write_str("rename"),
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
	pub file: String,
	pub action: FileAction,
	pub diff: Option<String>,
}
