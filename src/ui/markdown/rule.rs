///
/// Matches 3+ of the same character (-, *, _)
/// with optional spaces between them.
pub fn is_rule(trimmed: &str) -> bool {
	let stripped: String = trimmed
		.chars()
		.filter(|chr| *chr != ' ')
		.collect();
	if stripped.len() < 3 {
		return false;
	}
	stripped.chars().all(|chr| chr == '-')
		|| stripped.chars().all(|chr| chr == '*')
		|| stripped.chars().all(|chr| chr == '_')
}
