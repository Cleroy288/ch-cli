//! Code extraction and doc cleaning utilities.

use std::path::Path;

use crate::retrieval::docgen::generator::CONTEXT_LINES;
use crate::retrieval::models::{ModelError, ModelResult};

/// Extract code snippet from file around a given line.
#[doc(hidden)]
pub fn extract_code_snippet(
	file_path: &Path,
	line: usize,
) -> ModelResult<String> {
	let content =
		std::fs::read_to_string(file_path).map_err(
			|err| {
				ModelError::WeightLoad(format!(
					"Failed to read file {}: {}",
					file_path.display(),
					err
				))
			},
		)?;

	let lines: Vec<&str> = content.lines().collect();

	if line == 0 || line > lines.len() {
		return Ok(String::new());
	}

	extract_from_lines(&lines, line)
}

/// Extract the snippet range from pre-split lines.
fn extract_from_lines(
	lines: &[&str],
	line: usize,
) -> ModelResult<String> {
	let line_idx = line - 1;
	let half_context = CONTEXT_LINES / 2;
	let start = line_idx.saturating_sub(half_context);
	let end = (line_idx + half_context).min(lines.len());

	let actual_end =
		find_construct_end(lines, line_idx, end);
	let end = actual_end.min(start + CONTEXT_LINES);
	let snippet: String = lines[start..end].join("\n");

	Ok(snippet)
}

/// Find end of current construct by matching braces.
fn find_construct_end(
	lines: &[&str],
	line_idx: usize,
	default_end: usize,
) -> usize {
	let mut count: i32 = 0;
	let mut found = false;

	for (idx, line_content) in lines
		.iter()
		.enumerate()
		.skip(line_idx)
		.take(CONTEXT_LINES * 2)
	{
		(count, found) =
			count_braces(line_content, count, found);
		if found && count == 0 {
			return (idx + 1).min(lines.len());
		}
	}

	default_end
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

/// Clean up generated documentation text.
#[doc(hidden)]
pub fn clean_generated_doc(doc: &str) -> String {
	let mut cleaned = doc.trim().to_string();
	cleaned = strip_prefixes(cleaned);
	cleaned = strip_code_fences(cleaned);
	truncate_if_long(&mut cleaned);
	cleaned
}

/// Remove known doc prefixes.
fn strip_prefixes(mut text: String) -> String {
	let prefixes =
		["Description:", "description:", "Doc:", "doc:"];
	for prefix in prefixes {
		if text.starts_with(prefix) {
			text =
				text[prefix.len()..].trim().to_string();
		}
	}
	text
}

/// Remove surrounding code fences if present.
fn strip_code_fences(mut text: String) -> String {
	if !text.starts_with("```") {
		return text;
	}
	if let Some(end) = text.rfind("```") {
		if end > 3 {
			let start = text
				.find('\n')
				.map(|idx| idx + 1)
				.unwrap_or(3);
			text = text[start..end].trim().to_string();
		}
	}
	text
}

/// Truncate text to ~500 chars on sentence boundary.
fn truncate_if_long(text: &mut String) {
	if text.len() <= 500 {
		return;
	}
	let boundary = (0..=500)
		.rev()
		.find(|&idx| text.is_char_boundary(idx))
		.unwrap_or(0);
	if let Some(pos) = text[..boundary].rfind(". ") {
		*text = text[..=pos].to_string();
	} else {
		text.truncate(boundary);
		text.push_str("...");
	}
}
