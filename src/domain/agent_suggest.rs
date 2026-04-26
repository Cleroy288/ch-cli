/// A suggestion to add a subagent to the prompt.
///
/// Triggered by keyword detection in the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSuggestion {
	/// Model to use (e.g. "sonnet", "haiku")
	pub model: &'static str,
	/// Short description of the agent's role
	pub description: &'static str,
	/// Keyword that triggered this suggestion
	pub trigger: &'static str,
}
