/// Parse agent sections from Claude's response.
///
/// Recognises header patterns:
///   `## 1. [model] description`
///   `### [model] description`
///   `## Agent 1: [model] description`

use crate::domain::agent_result::AgentSection;

use super::parser_agent_header::is_agent_header;

/// Parse agent sections from Claude response.
/// Each section runs from its header to the next
/// header (or end of text).
pub fn parse_agent_sections(
	response: &str,
) -> Vec<AgentSection> {
	let mut sections = Vec::new();
	let mut offset: usize = 0;

	for line in response.lines() {
		let line_start = offset;
		offset += line.len() + 1;

		if let Some((model, desc)) =
			is_agent_header(line)
		{
			close_previous(
				&mut sections, line_start,
			);
			sections.push(AgentSection {
				model,
				description: desc,
				start: line_start,
				end: response.len(),
			});
		}
	}
	sections
}

/// Close previous section at boundary offset.
fn close_previous(
	sections: &mut [AgentSection],
	boundary: usize,
) {
	if let Some(last) = sections.last_mut() {
		last.end = boundary;
	}
}

/// Find which agent section a byte offset falls in.
pub fn agent_at_offset(
	sections: &[AgentSection],
	offset: usize,
) -> Option<&AgentSection> {
	sections.iter().find(|s| {
		offset >= s.start && offset < s.end
	})
}
