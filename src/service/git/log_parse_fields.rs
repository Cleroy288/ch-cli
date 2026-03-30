/// Parse space-separated parent hashes.
pub fn parse_parents(raw: &str) -> Vec<String> {
	if raw.is_empty() {
		return Vec::new();
	}
	raw.split(' ')
		.filter(|s| !s.is_empty())
		.map(|s| s.to_string())
		.collect()
}

/// Parse comma-separated ref decorations.
pub fn parse_refs(raw: &str) -> Vec<String> {
	if raw.is_empty() {
		return Vec::new();
	}
	raw.split(", ")
		.filter_map(|r| normalise_ref(r.trim()))
		.collect()
}

/// Strip git log decoration prefixes.
fn normalise_ref(s: &str) -> Option<String> {
	let s = s
		.strip_prefix("HEAD -> ")
		.unwrap_or(s);
	let s = s
		.strip_prefix("tag: ")
		.unwrap_or(s);
	let s = s
		.strip_prefix("origin/")
		.unwrap_or(s);
	if s.is_empty() {
		None
	} else {
		Some(s.to_string())
	}
}
