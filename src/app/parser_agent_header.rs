/// Detect agent section headers in Claude response.
///
/// Supports patterns:
///   `## 1. [model] description`
///   `### [model] description`
///   `## Agent 1: [model] description`

/// Check if a line is an agent section header.
/// Returns (model, description) if matched.
pub fn is_agent_header(
	line: &str,
) -> Option<(String, String)> {
	let trimmed = line.trim();
	try_numbered_bracket(trimmed)
		.or_else(|| try_agent_prefix(trimmed))
		.or_else(|| try_h3_bracket(trimmed))
}

/// `## 1. [model] description`
fn try_numbered_bracket(
	trimmed: &str,
) -> Option<(String, String)> {
	let rest = trimmed.strip_prefix("## ")?;
	let rest = rest.trim_start_matches(|c: char| {
		c.is_ascii_digit() || c == '.'
	});
	extract_bracket_model(rest.trim_start())
}

/// `## Agent 1: [model] description`
fn try_agent_prefix(
	trimmed: &str,
) -> Option<(String, String)> {
	let rest = trimmed.strip_prefix("## Agent")?;
	let rest = rest.trim_start();
	let rest = rest.trim_start_matches(|c: char| {
		c.is_ascii_digit()
	});
	let rest = rest.strip_prefix(':')?.trim_start();
	extract_bracket_model(rest)
}

/// `### [model] description`
fn try_h3_bracket(
	trimmed: &str,
) -> Option<(String, String)> {
	let rest = trimmed.strip_prefix("### ")?;
	extract_bracket_model(rest.trim_start())
}

/// Extract `[model] description` from remaining.
fn extract_bracket_model(
	s: &str,
) -> Option<(String, String)> {
	let rest = s.strip_prefix('[')?;
	let bracket_end = rest.find(']')?;
	let model = rest[..bracket_end].trim();
	if model.is_empty() {
		return None;
	}
	let desc = rest[bracket_end + 1..].trim();
	if desc.is_empty() {
		return None;
	}
	Some((model.to_string(), desc.to_string()))
}
