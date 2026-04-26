const EXCLUDED_EXTS: &[&str] = &[
	".com", ".org", ".net", ".io",
];

/// (open, close, inner): inner=true extracts
/// between delimiters, false extracts after close.
const DELIMITERS: &[(&str, &str, bool)] = &[
	("`", "`", true),
	("**", "**", true),
	("[", "]", false),
];

/// Try to extract a file path from a single line.
pub(super) fn try_line(
	line: &str,
) -> Option<String> {
	try_prefix_strip(line)
		.or_else(|| try_delimited(line))
		.or_else(|| try_bare_path(line))
}

/// Strip box-drawing or heading prefix, then
/// extract the first path token from the rest.
fn try_prefix_strip(
	line: &str,
) -> Option<String> {
	let stripped = line.trim_start_matches([
		'\u{250C}', '\u{2500}', '\u{251C}',
		'\u{2502}', '\u{2514}', '#', ' ',
	]);
	if stripped.len() == line.len() {
		return None;
	}
	let tok =
		stripped.split_whitespace().next()?;
	is_path(tok).then(|| tok.to_string())
}

/// Try each delimiter pair from the table.
fn try_delimited(
	line: &str,
) -> Option<String> {
	DELIMITERS.iter().find_map(
		|&(open, close, inner)| {
			let (_, rest) =
				line.split_once(open)?;
			if inner {
				let (mid, _) =
					rest.split_once(close)?;
				return is_path(mid)
					.then(|| mid.to_string());
			}
			let (_, after) =
				rest.split_once(close)?;
			let tok = after
				.trim()
				.split_whitespace()
				.next()?;
			is_path(tok)
				.then(|| tok.to_string())
		},
	)
}

/// Scan whitespace tokens in reverse for a bare
/// path (must contain `/` and a valid extension).
fn try_bare_path(line: &str) -> Option<String> {
	line.split_whitespace()
		.rev()
		.find(|t| t.contains('/') && is_path(t))
		.map(|s| s.to_string())
}

/// Check whether `s` looks like a file path by
/// validating its extension suffix.
fn is_path(s: &str) -> bool {
	let ext = s.rfind('.').map(|d| &s[d..]);
	ext.is_some_and(|e| {
		(2..=11).contains(&e.len())
			&& e[1..].chars().all(
				|c| c.is_ascii_alphanumeric(),
			)
			&& !EXCLUDED_EXTS.contains(&e)
	})
}
