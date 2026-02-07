//! Configuration for Context Expansion
//!
//! Defines ContextConfig which controls how much context
//! is gathered for each symbol (callers, callees, usages).

/// Configuration for context expansion
#[derive(Debug, Clone)]
pub struct ContextConfig {
	/// max number of callers to include
	pub max_callers: usize,
	/// max number of callees to include
	pub max_callees: usize,
	/// max number of usages per symbol to include
	pub max_usages_per_symbol: usize,
	/// context lines before symbol
	pub context_lines_before: usize,
	/// context lines after symbol
	pub context_lines_after: usize,
	/// include parent context
	pub include_parent: bool,
	/// include related types
	pub include_related_types: bool,
}

impl Default for ContextConfig {
	fn default() -> Self {
		Self {
			max_callers: 15,
			max_callees: 15,
			max_usages_per_symbol: 30,
			context_lines_before: 3,
			context_lines_after: 15,
			include_parent: true,
			include_related_types: true,
		}
	}
}

