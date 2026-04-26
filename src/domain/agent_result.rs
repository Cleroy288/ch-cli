/// A section of Claude's response attributed to
/// an agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSection {
	pub model: String,
	pub description: String,
	/// Byte range in the full response text
	pub start: usize,
	pub end: usize,
}
