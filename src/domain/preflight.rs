/// Pre-flight recap shown before sending to Claude.
///
/// Displayed when input contains agent lines.
/// User presses Enter to confirm, Esc to cancel.
#[derive(Debug, Clone)]
pub struct PreFlightData {
	/// Main prompt text (agents stripped)
	pub main_prompt: String,
	/// Parsed agent specifications
	pub agents: Vec<super::agent_spec::AgentSpec>,
	/// Detected query mode label (Q/A/P or default)
	pub mode_label: String,
	/// Number of attached file references
	pub file_count: usize,
	/// Number of attached symbol selectors
	pub symbol_count: usize,
}
