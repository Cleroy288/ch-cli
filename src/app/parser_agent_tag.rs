/// Tag review blocks with their agent source.
///
/// Finds each code fence's byte offset in the
/// response, then matches it to an agent section.

use crate::domain::review::ReviewBlock;

use super::parser_agent_response::{
	agent_at_offset, parse_agent_sections,
};

/// Parse agent sections from response and tag
/// each review block with its agent source.
pub fn tag_blocks_with_agents(
	blocks: &mut [ReviewBlock],
	response: &str,
) {
	let sections = parse_agent_sections(response);
	if sections.is_empty() {
		return;
	}
	let offsets = find_fence_offsets(response);
	for (i, block) in blocks.iter_mut().enumerate()
	{
		if let Some(off) = offsets.get(i) {
			block.agent_source =
				agent_at_offset(&sections, *off)
					.map(|s| s.model.clone());
		}
	}
}

/// Find byte offsets of opening code fences.
/// Returns one offset per code block in order.
fn find_fence_offsets(
	response: &str,
) -> Vec<usize> {
	let mut offsets = Vec::new();
	let mut offset: usize = 0;
	let mut in_code = false;

	for line in response.lines() {
		let trimmed = line.trim_start();
		if trimmed.starts_with("```") {
			if !in_code {
				offsets.push(offset);
			}
			in_code = !in_code;
		}
		offset += line.len() + 1;
	}
	offsets
}
