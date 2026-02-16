//! Caller/callee query detection
//!
//! Detects queries like "who calls X" or "callers of X"

/// Result of caller query detection
#[derive(Debug, Clone)]
pub struct CallerQuery {
	/// the symbol name being queried
	pub symbol_name: String,
	/// direction of the query (callers or callees)
	pub direction: CallerDirection,
}

/// Direction of caller query
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallerDirection {
	/// who calls X
	Callers,
	/// what does X call
	Callees,
}

/// Detect if query is asking about callers/callees
pub fn detect_caller_query(
	query: &str,
) -> Option<CallerQuery> {
	let lower = query.to_lowercase();

	if let Some(result) = detect_callers(&lower) {
		return Some(result);
	}
	detect_callees(&lower)
}

/// Check caller-direction patterns.
fn detect_callers(lower: &str) -> Option<CallerQuery> {
	// "who calls X" or "what calls X"
	let has_calls = lower.contains("who calls")
		|| lower.contains("what calls");
	if has_calls {
		if let Some(sym) =
			extract_symbol_after(lower, "calls ")
		{
			return Some(CallerQuery {
				symbol_name: sym,
				direction: CallerDirection::Callers,
			});
		}
	}

	// "callers of X"
	if lower.contains("callers of") {
		if let Some(sym) =
			extract_symbol_after(lower, "callers of ")
		{
			return Some(CallerQuery {
				symbol_name: sym,
				direction: CallerDirection::Callers,
			});
		}
	}

	detect_callers_suffix(lower)
}

/// Check suffix/infix caller patterns.
fn detect_callers_suffix(
	lower: &str,
) -> Option<CallerQuery> {
	// "X callers" or "X's callers"
	if lower.contains(" callers") {
		if let Some(sym) =
			extract_symbol_before(lower, " callers")
		{
			return Some(CallerQuery {
				symbol_name: sym,
				direction: CallerDirection::Callers,
			});
		}
	}

	// "functions that call X"
	if lower.contains("that call ") {
		if let Some(sym) =
			extract_symbol_after(lower, "that call ")
		{
			return Some(CallerQuery {
				symbol_name: sym,
				direction: CallerDirection::Callers,
			});
		}
	}

	None
}

/// Check callee-direction patterns.
fn detect_callees(lower: &str) -> Option<CallerQuery> {
	let has_what_does = lower.contains("what does");
	let has_call = lower.contains("call");
	if !(has_what_does && has_call) {
		return None;
	}
	let sym = extract_symbol_between(
		lower, "what does ", " call",
	)?;
	Some(CallerQuery {
		symbol_name: sym,
		direction: CallerDirection::Callees,
	})
}

/// Extract symbol name after a keyword
fn extract_symbol_after(
	query: &str,
	keyword: &str,
) -> Option<String> {
	let idx = query.find(keyword)?;
	let rest = &query[idx + keyword.len()..];
	let symbol = rest.split_whitespace().next()?;
	let cleaned = symbol.trim_matches(|chr: char| {
		!chr.is_alphanumeric() && chr != '_'
	});
	if cleaned.len() > 1 {
		Some(cleaned.to_string())
	} else {
		None
	}
}

/// Extract symbol name before a keyword
fn extract_symbol_before(
	query: &str,
	keyword: &str,
) -> Option<String> {
	let idx = query.find(keyword)?;
	let before = &query[..idx];
	let symbol = before.split_whitespace().last()?;
	let cleaned = symbol.trim_matches(|chr: char| {
		!chr.is_alphanumeric() && chr != '_'
	});
	if cleaned.len() > 1 {
		Some(cleaned.to_string())
	} else {
		None
	}
}

/// Extract symbol name between two keywords
fn extract_symbol_between(
	query: &str,
	start: &str,
	end: &str,
) -> Option<String> {
	let start_idx = query.find(start)?;
	let after = &query[start_idx + start.len()..];
	let end_idx = after.find(end)?;
	let between = &after[..end_idx];
	let symbol = between.trim();
	if symbol.len() > 1 {
		Some(symbol.to_string())
	} else {
		None
	}
}
