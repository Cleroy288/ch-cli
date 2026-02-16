//! Core data types for the memory system.
//!
//! Stores user interactions: input, AI responses,
//! file changes, and session metadata.

use serde::{Deserialize, Serialize};

/// A single user-AI interaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
	pub id: String,           // unique interaction id
	pub timestamp: u64,       // unix epoch seconds
	pub session_id: String,   // groups interactions
	pub input: UserInput,     // what the user said
	pub response: AiResponse, // what the AI returned
}

/// What the user provided as input
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInput {
	pub text: String,          // user's query/command
	pub command: Option<String>, // CLI command if any
	pub files: Vec<String>,    // referenced file paths
}

/// The AI's response, tagged by type
#[derive(
	Debug, Clone, Serialize, Deserialize,
)]
#[serde(tag = "type")]
pub enum AiResponse {
	/// Plain text answer
	Answer { text: String },

	/// Clarifying question back to user
	Question { text: String },

	/// Code modification with optional diff
	CodeChange {
		text: String,
		changes: Vec<FileChange>,
	},
}

/// A single file modification record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileChange {
	pub file: String,   // path to changed file
	pub action: String, // create, modify, delete
	pub diff: Option<String>, // optional diff content
}
