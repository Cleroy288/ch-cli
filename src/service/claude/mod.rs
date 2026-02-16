//! Claude Code CLI integration service.
//!
//! Provides process execution, JSON parsing, and
//! prompt building for the Claude CLI.

pub mod json_parse;
pub mod process;
pub mod prompt;

pub use json_parse::parse_claude_response;
pub use process::execute_claude_cli;
pub use prompt::build_prompt;
