/// Format agent instructions for the system prompt.

use crate::domain::agent_spec::AgentSpec;
use crate::ui::strings::tui_labels;

/// Build the agent instruction block.
///
/// Returns `None` if the list is empty.
pub fn format_agent_instruction(
    agents: &[AgentSpec],
) -> Option<String> {
    if agents.is_empty() {
        return None;
    }
    let mut lines = Vec::with_capacity(
        agents.len() + 2,
    );
    lines.push(
        tui_labels::AGENT_INSTRUCTION_HEADER
            .to_string(),
    );
    for (i, agent) in agents.iter().enumerate() {
        lines.push(format!(
            "{}. [{}] {}",
            i + 1,
            agent.model,
            agent.description,
        ));
    }
    lines.push(
        tui_labels::AGENT_INSTRUCTION_FOOTER
            .to_string(),
    );
    Some(lines.join("\n"))
}

/// Merge agent block into existing system prompt.
pub fn merge_agent_sys(
    base: Option<String>,
    agents: &[AgentSpec],
) -> Option<String> {
    let agent_block =
        format_agent_instruction(agents);
    match (base, agent_block) {
        (Some(b), Some(a)) => {
            Some(format!("{}\n\n{}", b, a))
        }
        (Some(b), None) => Some(b),
        (None, Some(a)) => Some(a),
        (None, None) => None,
    }
}
