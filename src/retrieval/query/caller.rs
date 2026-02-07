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
/// Returns Some(CallerQuery) if detected, None otherwise
pub fn detect_caller_query(query: &str) -> Option<CallerQuery> {
	let query_lower = query.to_lowercase(); // lowercase for matching

	// Pattern 1: "who calls X" or "what calls X"
	let has_who_calls = query_lower.contains("who calls");
	let has_what_calls = query_lower.contains("what calls");
	if has_who_calls || has_what_calls {
		let symbol =
			extract_symbol_after(&query_lower, "calls ");
		if let Some(symbol) = symbol {
			return Some(CallerQuery {
				symbol_name: symbol,
				direction: CallerDirection::Callers,
			});
		}
	}

	// Pattern 2: "callers of X"
	if query_lower.contains("callers of") {
		let symbol = extract_symbol_after(
			&query_lower,
			"callers of ",
		);
		if let Some(symbol) = symbol {
			return Some(CallerQuery {
				symbol_name: symbol,
				direction: CallerDirection::Callers,
			});
		}
	}

	// Pattern 3: "X callers" or "X's callers"
	if query_lower.contains(" callers") {
		let symbol =
			extract_symbol_before(&query_lower, " callers");
		if let Some(symbol) = symbol {
			return Some(CallerQuery {
				symbol_name: symbol,
				direction: CallerDirection::Callers,
			});
		}
	}

	// Pattern 4: "functions that call X"
	if query_lower.contains("that call ") {
		let symbol =
			extract_symbol_after(&query_lower, "that call ");
		if let Some(symbol) = symbol {
			return Some(CallerQuery {
				symbol_name: symbol,
				direction: CallerDirection::Callers,
			});
		}
	}

	// Pattern 5: "what does X call" or "what X calls"
	let has_what_does = query_lower.contains("what does");
	let has_call = query_lower.contains("call");
	if has_what_does && has_call {
		let symbol = extract_symbol_between(
			&query_lower,
			"what does ",
			" call",
		);
		if let Some(symbol) = symbol {
			return Some(CallerQuery {
				symbol_name: symbol,
				direction: CallerDirection::Callees,
			});
		}
	}

	None
}

/// Extract symbol name after a keyword
fn extract_symbol_after(
	query: &str,
	keyword: &str,
) -> Option<String> {
	let idx = query.find(keyword)?;
	let rest = &query[idx + keyword.len()..];
	let symbol = rest.split_whitespace().next()?;
	let cleaned = symbol.trim_matches(|c: char| {
		!c.is_alphanumeric() && c != '_'
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
	let cleaned = symbol.trim_matches(|c: char| {
		!c.is_alphanumeric() && c != '_'
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
	let after_start = &query[start_idx + start.len()..];
	let end_idx = after_start.find(end)?;
	let between = &after_start[..end_idx];
	let symbol = between.trim();
	if symbol.len() > 1 {
		Some(symbol.to_string())
	} else {
		None
	}
}

