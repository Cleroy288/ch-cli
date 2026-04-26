use std::io::Write;

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

	let content =
		std::fs::read_to_string(path)?;
	let lines: Vec<&str> =
		content.lines().collect();
	let start =
		symbol.location.line.saturating_sub(1);
	let end =
		find_symbol_end(&lines, start).min(
			lines.len(),
		);

	if start >= lines.len() {
		return Ok(());
	}
	print_source_lines(
		&lines[start..end], start,
		symbol.location.line,
	)?;
	Ok(())
}

/// Print source lines with line numbers
fn print_source_lines(
	lines: &[&str],
	start: usize,
	highlight: usize,
) -> std::io::Result<()> {
	let mut out = std::io::stdout().lock();
	writeln!(out, "\n\u{1F4BB} Source Code:")?;
	for (idx, line) in lines.iter().enumerate() {
		let line_num = start + idx + 1;
		let marker = if line_num == highlight {
			">"
		} else {
			" "
		};
		writeln!(
			out,
			"{} {:4} \u{2502} {}",
			marker, line_num, line
		)?;
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

	for (idx, line) in
		lines.iter().enumerate().skip(start)
	{
		(brace_count, found_open) = count_braces(
			line, brace_count, found_open,
		);
		if found_open && brace_count == 0 {
			return idx + 1;
		}

		// Limit to ~50 lines max
		if idx - start > 50 {
			return idx;
		}
	}

	// No braces found — show ~10 lines
	(start + 10).min(lines.len())
}

/// returning updated state
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
