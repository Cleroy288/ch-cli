/// Keyword-based agent suggestion rules.
///
/// Scans input for known patterns and proposes
/// relevant subagents the user may want to add.

use super::agent_suggest::AgentSuggestion;

/// Minimum input length to trigger suggestions.
const MIN_INPUT_LEN: usize = 10;

/// Maximum suggestions returned at once.
const MAX_SUGGESTIONS: usize = 2;

/// All known suggestion rules.
const RULES: &[AgentSuggestion] = &[
	AgentSuggestion {
		model: "sonnet",
		description: "Review security",
		trigger: "api",
	},
	AgentSuggestion {
		model: "haiku",
		description: "Write tests",
		trigger: "refactor",
	},
	AgentSuggestion {
		model: "sonnet",
		description: "Analyze root cause",
		trigger: "bug",
	},
	AgentSuggestion {
		model: "haiku",
		description: "Add test coverage",
		trigger: "feature",
	},
	AgentSuggestion {
		model: "haiku",
		description: "Check quality",
		trigger: "test",
	},
];

/// Detect agent suggestions from input text.
///
/// Returns up to 2 suggestions. Skips if input
/// is too short or already contains agent lines.
pub fn detect_suggestions(
	input: &str,
) -> Vec<AgentSuggestion> {
	if input.len() < MIN_INPUT_LEN {
		return Vec::new();
	}
	if input.contains("- agent(") {
		return Vec::new();
	}
	let lower = input.to_lowercase();
	RULES
		.iter()
		.filter(|r| lower.contains(r.trigger))
		.take(MAX_SUGGESTIONS)
		.cloned()
		.collect()
}
