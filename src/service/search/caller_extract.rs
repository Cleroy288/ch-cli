/// Extract symbol name after a keyword
pub(super) fn extract_after(
	query: &str,
	keyword: &str,
) -> Option<String> {
	let idx = query.find(keyword)?;
	let rest = &query[idx + keyword.len()..];
	let symbol = rest.split_whitespace().next()?;
	let cleaned = symbol.trim_matches(|chr: char| {
		!chr.is_alphanumeric() && chr != '_'
	});
	(cleaned.len() > 1)
		.then(|| cleaned.to_string())
}

/// Extract symbol name before a keyword
pub(super) fn extract_before(
	query: &str,
	keyword: &str,
) -> Option<String> {
	let idx = query.find(keyword)?;
	let before = &query[..idx];
	let symbol = before.split_whitespace().last()?;
	let cleaned = symbol.trim_matches(|chr: char| {
		!chr.is_alphanumeric() && chr != '_'
	});
	(cleaned.len() > 1)
		.then(|| cleaned.to_string())
}

/// Extract symbol name between two keywords
pub(super) fn extract_between(
	query: &str,
	start: &str,
	end: &str,
) -> Option<String> {
	let start_idx = query.find(start)?;
	let after = &query[start_idx + start.len()..];
	let end_idx = after.find(end)?;
	let sym = after[..end_idx]
		.trim()
		.trim_end_matches(['.', ',', ';', ':']);
	(sym.len() > 1).then(|| sym.to_string())
}
