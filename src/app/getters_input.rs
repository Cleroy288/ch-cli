use crate::domain::agent_suggest::AgentSuggestion;

use super::App;

impl App {
	pub fn input(&self) -> &str {
		&self.input
	}

	pub fn cursor_position(&self) -> usize {
		self.cursor_position.get()
	}

	pub fn should_quit(&self) -> bool {
		self.should_quit
	}

	pub fn model_name(&self) -> &str {
		&self.model_name
	}

	pub fn effort_level(&self) -> &str {
		&self.effort_level
	}

	pub fn project_root(&self) -> &str {
		&self.project_root
	}

	pub fn agent_suggestions(
		&self,
	) -> &[AgentSuggestion] {
		&self.agent_suggestions
	}
}
