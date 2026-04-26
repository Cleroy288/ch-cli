/// Extract description from YAML frontmatter
pub fn extract_description(
	content: &str,
) -> Option<String> {
	let body = strip_frontmatter(content)?;
	extract_field(body, "description")
}

/// Strip the --- delimited frontmatter block
fn strip_frontmatter(
	content: &str,
) -> Option<&str> {
	let trimmed = content.trim_start();
	if !trimmed.starts_with("---") {
		return None;
	}
	let after_open = &trimmed[3..];
	after_open.find("---").map(|end| {
		&after_open[..end]
	})
}

/// Find `key: value` in frontmatter text
fn extract_field(
	frontmatter: &str,
	key: &str,
) -> Option<String> {
	let prefix = format!("{key}:");
	for line in frontmatter.lines() {
		let trimmed = line.trim();
		if trimmed.starts_with(&prefix) {
			let value = trimmed[prefix.len()..]
				.trim()
				.to_string();
			if !value.is_empty() {
				return Some(value);
			}
		}
	}
	None
}
