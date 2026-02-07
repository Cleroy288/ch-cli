//! Source code display for info command.

use std::fs;

use crate::indexer::Symbol;

use crate::cli::commands::error::CommandResult;

/// Display source code snippet around the symbol
#[doc(hidden)]
pub fn display_source_code(
	symbol: &Symbol,
) -> CommandResult {
	let path = &symbol.location.file;
	if !path.exists() {
		return Ok(());
	}

	let content = fs::read_to_string(path)?;
	let lines: Vec<&str> =
		content.lines().collect();
	// 0-indexed from 1-indexed line number
	let start =
		symbol.location.line.saturating_sub(1);
	let end =
		find_symbol_end(&lines, start).min(
			lines.len(),
		);

	if start >= lines.len() {
		return Ok(());
	}

	println!("\n\u{1F4BB} Source Code:");

	for (i, line) in
		lines[start..end].iter().enumerate()
	{
		let line_num = start + i + 1;
		let marker =
			if line_num == symbol.location.line {
				">"
			} else {
				" "
			};
		println!(
			"{} {:4} \u{2502} {}",
			marker, line_num, line
		);
	}

	Ok(())
}

/// Find the end of a symbol definition
/// Uses brace counting to find the closing brace
#[doc(hidden)]
pub fn find_symbol_end(
	lines: &[&str],
	start: usize,
) -> usize {
	let mut brace_count = 0; // brace nesting
	let mut found_open = false; // opening brace

	for (i, line) in
		lines.iter().enumerate().skip(start)
	{
		for ch in line.chars() {
			if ch == '{' {
				brace_count += 1;
				found_open = true;
			} else if ch == '}' {
				brace_count -= 1;
				if found_open && brace_count == 0 {
					return i + 1;
				}
			}
		}

		// Limit to ~50 lines max
		if i - start > 50 {
			return i;
		}
	}

	// No braces found — show ~10 lines
	(start + 10).min(lines.len())
}

