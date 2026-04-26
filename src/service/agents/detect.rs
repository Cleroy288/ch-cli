use std::path::PathBuf;

use crate::domain::agent::AgentKind;

pub fn detect_agents() -> Vec<AgentKind> {
	let mut agents = Vec::new();
	if is_claude_installed() {
		agents.push(AgentKind::ClaudeCode);
	}
	agents
}

pub fn is_claude_installed() -> bool {
	claude_config_dir().is_some()
}

/// Path to `~/.claude/` if it exists
pub fn claude_config_dir() -> Option<PathBuf> {
	let home = std::env::var("HOME").ok()?;
	let dir = PathBuf::from(home).join(".claude");
	if dir.is_dir() { Some(dir) } else { None }
}
