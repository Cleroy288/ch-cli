//! Code extraction and doc cleaning utilities.

use std::fs;
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
		fs::read_to_string(file_path).map_err(|e| {
			ModelError::WeightLoad(format!(
				"Failed to read file {}: {}",
				file_path.display(),
				e
			))
		})?;

	let lines: Vec<&str> = content.lines().collect();

	if line == 0 || line > lines.len() {
		return Ok(String::new());
	}

	let line_idx = line - 1;
	let half_context = CONTEXT_LINES / 2;
	let start = line_idx.saturating_sub(half_context);
	let end = (line_idx + half_context).min(lines.len());

	let actual_end =
		find_construct_end(&lines, line_idx, end);
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
	let mut brace_count = 0;
	let mut found_open = false;
	let mut actual_end = default_end;

	for (i, line_content) in lines
		.iter()
		.enumerate()
		.skip(line_idx)
		.take(CONTEXT_LINES * 2)
	{
		for ch in line_content.chars() {
			if ch == '{' {
				brace_count += 1;
				found_open = true;
			} else if ch == '}' {
				brace_count -= 1;
				if found_open && brace_count == 0 {
					actual_end = (i + 1).min(lines.len());
					return actual_end;
				}
			}
		}
	}

	actual_end
}

/// Clean up generated documentation text.
#[doc(hidden)]
pub fn clean_generated_doc(doc: &str) -> String {
	let mut cleaned = doc.trim().to_string();

	let prefixes =
		["Description:", "description:", "Doc:", "doc:"];
	for prefix in prefixes {
		if cleaned.starts_with(prefix) {
			cleaned =
				cleaned[prefix.len()..].trim().to_string();
		}
	}

	if cleaned.starts_with("```") {
		if let Some(end) = cleaned.rfind("```") {
			if end > 3 {
				let start = cleaned
					.find('\n')
					.map(|i| i + 1)
					.unwrap_or(3);
				cleaned =
					cleaned[start..end].trim().to_string();
			}
		}
	}

	if cleaned.len() > 500 {
		if let Some(pos) = cleaned[..500].rfind(". ") {
			cleaned = cleaned[..=pos].to_string();
		} else {
			cleaned.truncate(500);
			cleaned.push_str("...");
		}
	}

	cleaned
}

