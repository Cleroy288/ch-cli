/// Extract doc comment (///) above a symbol.
/// Walks backwards collecting consecutive doc comments.
pub fn extract_item_doc(
	source: &str,
	symbol_line: usize,
) -> Option<String> {
	let lines: Vec<&str> =
		source.lines().collect();

	if symbol_line <= 1
		|| symbol_line > lines.len()
	{
		return None;
	}

	let doc_lines =
		collect_doc_lines_above(&lines, symbol_line);

	(!doc_lines.is_empty())
		.then(|| doc_lines.join("\n"))
}

/// Walk backwards from symbol line
fn collect_doc_lines_above(
	lines: &[&str],
	symbol_line: usize,
) -> Vec<String> {
	let mut doc_lines: Vec<String> = Vec::new();
	// 0-indexed, one above
	let mut idx = symbol_line - 2;

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

/// Process a single line during backward extraction.
/// Returns true to continue, false to stop.
fn process_doc_line(
	line: &str,
	doc_lines: &mut Vec<String>,
) -> bool {
	if line.starts_with("///") {
		let rest = &line[3..]; // skip "///"
		doc_lines.insert(
			0, rest.trim().to_string(),
		);
		return true;
	}
	if line.starts_with("#[") {
		return true; // attribute, skip
	}
	// Non-doc line: stop if docs started
	line.is_empty() && doc_lines.is_empty()
}
