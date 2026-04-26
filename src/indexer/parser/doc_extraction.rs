pub use super::doc_extraction_collect
	::extract_item_doc;

/// Extract module-level doc comments (//!)
/// Returns concatenated module documentation
pub fn extract_module_docs(
	source: &str,
) -> Option<String> {
	let mut doc_lines: Vec<String> = Vec::new();

	for line in source.lines() {
		let trimmed = line.trim();
		if let Some(rest) = try_module_doc(trimmed) {
			doc_lines.push(rest);
		} else if should_skip_line(
			trimmed, &doc_lines,
		) {
			continue;
		} else {
			break;
		}
	}

	(!doc_lines.is_empty())
		.then(|| doc_lines.join("\n"))
}

/// Try to extract a module doc comment line (//!)
fn try_module_doc(
	trimmed: &str,
) -> Option<String> {
	if !trimmed.starts_with("//!") {
		return None;
	}
	let rest = &trimmed[3..]; // skip "//!"
	Some(rest.trim().to_string())
}

fn should_skip_line(
	trimmed: &str,
	doc_lines: &[String],
) -> bool {
	trimmed.is_empty() && doc_lines.is_empty()
}
