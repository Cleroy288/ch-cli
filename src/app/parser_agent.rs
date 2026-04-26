/// Parse agent lines from multi-line input.
///
/// Format: `- agent(model): description`
/// Default model: sonnet.

use crate::domain::agent_spec::{
    AgentSpec, DEFAULT_AGENT_MODEL,
};

/// Split input into (main_text, agents).
pub fn parse_agent_lines(
    input: &str,
) -> (String, Vec<AgentSpec>) {
    let (main, agent_strs) = partition_lines(input);
    let agents = agent_strs
        .into_iter()
        .filter_map(|l| parse_one_agent(l))
        .collect();
    (main, agents)
}

/// Check if a line matches the agent pattern.
pub fn is_agent_line(line: &str) -> bool {
    parse_one_agent(line).is_some()
}

/// Extract agent suffix: (main, agent_block).
///
/// agent_block is the raw lines starting with
/// `- agent(`. Used by enhance to strip/reattach.
pub fn extract_agent_suffix(
    input: &str,
) -> (String, String) {
    let (main, agent_strs) = partition_lines(input);
    (main, agent_strs.join("\n"))
}

/// Separate non-agent lines from agent lines.
fn partition_lines(
    input: &str,
) -> (String, Vec<&str>) {
    let (main, agents): (Vec<&str>, Vec<&str>) =
        input.split('\n').partition(|l| {
            parse_one_agent(l).is_none()
        });
    (main.join("\n"), agents)
}

/// Parse `- agent(model): description`.
fn parse_one_agent(line: &str) -> Option<AgentSpec> {
    let trimmed = line.trim();
    let rest = trimmed.strip_prefix("- agent(")?;
    let paren = rest.find(')')?;
    let model_raw = rest[..paren].trim();
    let after = rest[paren + 1..].trim();
    let desc = after.strip_prefix(':')?.trim();
    if desc.is_empty() {
        return None;
    }
    let model = if model_raw.is_empty() {
        DEFAULT_AGENT_MODEL.to_string()
    } else {
        model_raw.to_string()
    };
    Some(AgentSpec {
        model,
        description: desc.to_string(),
    })
}
