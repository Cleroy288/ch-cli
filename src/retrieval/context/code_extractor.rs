//! Code Extraction Utilities
//!
//! Helper functions for extracting code snippets from source files.

use crate::indexer::{Symbol, SymbolKind};

/// Find the end of a symbol's definition
pub fn find_symbol_end(
	lines: &[&str],
	start_idx: usize,
	symbol: &Symbol,
) -> usize {
	let mut brace_count = 0;
	let mut found_opening = false;

	for (i, line) in
		lines[start_idx..].iter().enumerate()
	{
		(brace_count, found_opening) =
			count_braces(
				line, brace_count, found_opening,
			);
		if found_opening && brace_count == 0 {
			return start_idx + i + 1;
		}
		if i > 100 {
			break;
		}
	}

	estimate_symbol_end(start_idx, symbol, lines.len())
}

/// Count braces in a line, returning updated state
fn count_braces(
	line: &str,
	mut count: i32,
	mut found: bool,
) -> (i32, bool) {
	for chr in line.chars() {
		if chr == '{' {
			count += 1;
			found = true;
		} else if chr == '}' {
			count -= 1;
		}
	}
	(count, found)
}

/// Estimate symbol end based on kind
fn estimate_symbol_end(
	start_idx: usize,
	symbol: &Symbol,
	max_lines: usize,
) -> usize {
	let estimated_lines = match symbol.kind {
		SymbolKind::Function | SymbolKind::Method => 20,
		SymbolKind::Struct | SymbolKind::Enum => 15,
		SymbolKind::Impl => 30,
		SymbolKind::Trait => 25,
		_ => 5,
	};

	(start_idx + estimated_lines).min(max_lines)
}

