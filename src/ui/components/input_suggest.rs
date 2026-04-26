/// Format agent suggestions for input display.
///
/// Returns a one-line hint string for the bottom
/// separator of the input box.

use crate::domain::agent_suggest::AgentSuggestion;
use crate::ui::strings::tui_labels;

/// Format suggestions into a display string.
///
/// Returns None if the list is empty.
pub fn format_suggestions(
	suggestions: &[AgentSuggestion],
) -> Option<String> {
	if suggestions.is_empty() {
		return None;
	}
	let parts: Vec<String> = suggestions
		.iter()
		.map(|s| {
			format!(
				"{}{}{}{}",
				tui_labels::AGENT_OPEN.trim_start(),
				s.model,
				tui_labels::AGENT_SEP,
				s.description,
			)
		})
		.collect();
	Some(format!(
		"{} {}",
		tui_labels::SUGGEST_PREFIX,
		parts.join(" | "),
	))
}
