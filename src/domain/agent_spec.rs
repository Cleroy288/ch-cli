/// A subagent specification parsed from input.
///
/// Format: `- agent(model): description`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AgentSpec {
    pub model: String,
    pub description: String,
}

/// Default model when none specified.
pub const DEFAULT_AGENT_MODEL: &str = "sonnet";
