//! Documentation extraction functions.
//!
//! Provides functions to extract module-level (//!) and item-level (///) doc comments.

/// Extract module-level doc comments (//!) from source code
/// Returns concatenated module documentation as a single string
/// Module docs must be at the start of the file (after empty lines)
pub fn extract_module_docs(source: &str) -> Option<String> {
	// collected doc comment lines
	let mut doc_lines: Vec<String> = Vec::new();
	// whether we're still in module docs section
	let in_module_docs: bool = true;

	for line in source.lines() {
		let trimmed = line.trim();

		// Module doc comment: //!
		if trimmed.starts_with("//!") {
			let doc = trimmed.strip_prefix("//!").unwrap_or("").trim();
			doc_lines.push(doc.to_string());
		}
		// Empty lines are OK at the start
		else if trimmed.is_empty() && doc_lines.is_empty() {
			continue;
		}
		// Non-comment line ends module docs section
		else if !trimmed.starts_with("//") && !trimmed.is_empty() {
			break;
		}
		// Regular comment - might be part of module docs
		else if trimmed.starts_with("//") && in_module_docs && !doc_lines.is_empty() {
			// After module docs start, regular comments don't count
			break;
		}
	}
	// Suppress unused variable warning
	let _ = in_module_docs;

	if doc_lines.is_empty() {
		None
	} else {
		Some(doc_lines.join("\n"))
	}
}

/// Extract doc comment (///) above a symbol at a given line.
/// Returns the concatenated documentation.
/// Walks backwards from the symbol line collecting consecutive doc comments.
pub fn extract_item_doc(source: &str, symbol_line: usize) -> Option<String> {
	let lines: Vec<&str> = source.lines().collect(); // all source lines

	if symbol_line == 0 || symbol_line > lines.len() {
		return None;
	}

	let mut doc_lines: Vec<String> = Vec::new(); // collected doc comment lines
	let mut current_line: usize = symbol_line - 2; // 0-indexed, start one line above

	// Walk backwards collecting doc comments
	while current_line < lines.len() {
		let line = lines[current_line].trim();

		if line.starts_with("///") {
			let doc = line.strip_prefix("///").unwrap_or("").trim();
			doc_lines.insert(0, doc.to_string());
		} else if line.starts_with("#[") {
			// Attribute - skip but continue
		} else if line.is_empty() {
			// Empty line - might have more docs above
			if !doc_lines.is_empty() {
				break;
			}
		} else {
			// Non-doc line - stop
			break;
		}

		if current_line == 0 {
			break;
		}
		current_line -= 1;
	}

	if doc_lines.is_empty() {
		None
	} else {
		Some(doc_lines.join("\n"))
	}
}
