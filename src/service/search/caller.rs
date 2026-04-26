use super::caller_extract::{
	extract_after, extract_before, extract_between,
};

/// Parsed "who calls X" / "what does X call" query
#[derive(Debug, Clone)]
pub struct CallerQuery {
	pub symbol_name: String,
	pub direction: CallerDirection,
}

/// Callers (inbound) vs callees (outbound)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallerDirection {
	Callers,
	Callees,
}

pub fn detect_caller_query(
	query: &str,
) -> Option<CallerQuery> {
	let lower = query.to_lowercase();
	detect_callers(&lower)
		.or_else(|| detect_callees(&lower))
}

fn detect_callers(
	lower: &str,
) -> Option<CallerQuery> {
	let sym = try_extract_caller_sym(lower)?;
	Some(CallerQuery {
		symbol_name: sym,
		direction: CallerDirection::Callers,
	})
}

/// Try each caller pattern in order
fn try_extract_caller_sym(
	lower: &str,
) -> Option<String> {
	// "who calls X" / "what calls X"
	if lower.contains("who calls")
		|| lower.contains("what calls")
	{
		return extract_after(lower, "calls ");
	}
	// "callers of X"
	if lower.contains("callers of") {
		return extract_after(lower, "callers of ");
	}
	// "X callers"
	if lower.contains(" callers") {
		return extract_before(lower, " callers");
	}
	// "functions that call X"
	if lower.contains("that call ") {
		return extract_after(lower, "that call ");
	}
	None
}

fn detect_callees(
	lower: &str,
) -> Option<CallerQuery> {
	if !(lower.contains("what does")
		&& lower.contains("call"))
	{
		return None;
	}
	let sym = extract_between(
		lower,
		"what does ",
		" call",
	)?;
	Some(CallerQuery {
		symbol_name: sym,
		direction: CallerDirection::Callees,
	})
}
