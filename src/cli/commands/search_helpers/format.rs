use std::io::Write;

use crate::service::search::types::SearchResultHit;

/// Lines shown after target in --full snippets
const SNIPPET_CONTEXT_LINES: usize = 195;

/// Print full source content for search hits
pub fn print_full_content(
	hits: &[SearchResultHit],
) {
	let mut out = std::io::stdout().lock();
	writeln!(out, "\n--- Full Content ---\n").ok();
	for hit in hits {
		let path = &hit.symbol.location.file;
		let target = hit.symbol.location.line;
		writeln!(
			out,
			"=== {} (line {}) ===\n",
			path.display(),
			target
		)
		.ok();
		if let Ok(content) =
			std::fs::read_to_string(path)
		{
			print_snippet(
				&mut out, &content, target,
			);
		}
		writeln!(out).ok();
	}
}

/// Print source snippet around a target line
fn print_snippet(
	out: &mut impl Write,
	content: &str,
	target: usize,
) {
	let lines: Vec<&str> =
		content.lines().collect();
	let start = target.saturating_sub(5);
	let end =
		(target + SNIPPET_CONTEXT_LINES).min(lines.len());

	print_snippet_lines(
		out, &lines[start..end], start, target,
	);
	if end < lines.len() {
		writeln!(
			out,
			"      ... ({} more lines)",
			lines.len() - end
		)
		.ok();
	}
}

/// Print snippet lines with markers
fn print_snippet_lines(
	out: &mut impl Write,
	lines: &[&str],
	start: usize,
	target: usize,
) {
	for (idx, line) in lines.iter().enumerate() {
		let line_num = start + idx + 1;
		let marker = if line_num == target {
			">"
		} else {
			" "
		};
		writeln!(
			out,
			"{}{:4} | {}",
			marker, line_num, line
		)
		.ok();
	}
}
