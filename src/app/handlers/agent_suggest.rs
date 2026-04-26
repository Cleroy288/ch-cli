/// Auto-suggest agents based on input keywords.
///
/// Called after space is typed to update
/// the suggestion list.

use crate::app::App;
use crate::domain::agent_suggest_rules;

impl App {
	/// Refresh agent suggestions from input.
	pub(crate) fn update_agent_suggestions(
		&mut self,
	) {
		self.agent_suggestions =
			agent_suggest_rules::detect_suggestions(
				&self.input,
			);
	}
}
