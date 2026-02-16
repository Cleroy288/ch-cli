//! Documentation extraction functions.
//!
//! Provides functions to extract module-level (//!)
//! and item-level (///) doc comments.

/// Extract module-level doc comments (//!)
/// Returns concatenated module documentation
pub fn extract_module_docs(source: &str) -> Option<String> {
	let mut doc_lines: Vec<String> = Vec::new();

	for line in source.lines() {
		let trimmed = line.trim();
		if let Some(rest) = try_module_doc(trimmed) {
			doc_lines.push(rest);
		} else if should_skip_line(trimmed, &doc_lines) {
			continue;
		} else {
			break;
		}
	}

	if doc_lines.is_empty() {
		None
	} else {
		Some(doc_lines.join("\n"))
	}
}

/// Try to extract a module doc comment line (//!)
fn try_module_doc(trimmed: &str) -> Option<String> {
	if !trimmed.starts_with("//!") {
		return None;
	}
	let doc = trimmed
		.strip_prefix("//!")
		.unwrap_or("")
		.trim();
	Some(doc.to_string())
}

/// Check if a line should be skipped (empty at start)
fn should_skip_line(
	trimmed: &str,
	doc_lines: &[String],
) -> bool {
	trimmed.is_empty() && doc_lines.is_empty()
}

/// Extract doc comment (///) above a symbol at a given line.
/// Walks backwards collecting consecutive doc comments.
pub fn extract_item_doc(
	source: &str,
	symbol_line: usize,
) -> Option<String> {
	let lines: Vec<&str> = source.lines().collect();

	if symbol_line <= 1 || symbol_line > lines.len() {
		return None;
	}

	let doc_lines =
		collect_doc_lines_above(&lines, symbol_line);

	if doc_lines.is_empty() {
		None
	} else {
		Some(doc_lines.join("\n"))
	}
}

/// Walk backwards from symbol line collecting doc comments
fn collect_doc_lines_above(
	lines: &[&str],
	symbol_line: usize,
) -> Vec<String> {
	let mut doc_lines: Vec<String> = Vec::new();
	let mut idx = symbol_line - 2; // 0-indexed, one above

	while idx < lines.len() {
		let line = lines[idx].trim();
		if !process_doc_line(line, &mut doc_lines) {
			break;
		}
		if idx == 0 { break; }
		idx -= 1;
	}

	doc_lines
}

/// Process a single line during backward doc extraction.
/// Returns true to continue, false to stop.
fn process_doc_line(
	line: &str,
	doc_lines: &mut Vec<String>,
) -> bool {
	if line.starts_with("///") {
		let doc = line
			.strip_prefix("///")
			.unwrap_or("")
			.trim();
		doc_lines.insert(0, doc.to_string());
		return true;
	}
	if line.starts_with("#[") {
		return true; // attribute, skip
	}
	// Non-doc line: stop if docs started or non-empty
	line.is_empty() && doc_lines.is_empty()
}
