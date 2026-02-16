//! Build a prompt string from message segments.
//!
//! Converts MessageSegment variants into a text prompt
//! suitable for `claude -p`. File/symbol references
//! become XML-tagged context blocks.

use crate::message::MessageSegment;

/// XML open tag for file context
const FILE_OPEN: &str = "<file path=\"";
/// XML close tag for file context
const FILE_CLOSE: &str = "</file>";
/// XML open tag for code context
const CODE_OPEN: &str = "<code symbol=\"";
/// XML close tag for code context
const CODE_CLOSE: &str = "</code>";

/// Build a prompt string from message segments.
///
/// - `Text` segments → literal text
/// - `FileReference` → `<file path="...">content</file>`
/// - `SymbolReference` → `<code symbol="...">src</code>`
/// - `FolderReference` → plain path mention
pub fn build_prompt(
	segments: &[MessageSegment],
) -> String {
	let mut parts: Vec<String> = Vec::new();

	for seg in segments {
		parts.push(format_segment(seg));
	}

	parts.join("")
}

/// Format a single segment to prompt text
fn format_segment(seg: &MessageSegment) -> String {
	match seg {
		MessageSegment::Text(text) => text.clone(),
		MessageSegment::FileReference {
			full_path, ..
		} => format_file_ref(full_path),
		MessageSegment::SymbolReference {
			symbol_path,
			source_code,
			..
		} => format_symbol_ref(symbol_path, source_code),
		MessageSegment::FolderReference {
			full_path, ..
		} => format!("[folder: {}]", full_path),
	}
}

/// Read file and wrap in XML tags
fn format_file_ref(path: &str) -> String {
	let content =
		std::fs::read_to_string(path).unwrap_or_else(
			|_| "[file not readable]".to_string(),
		);
	format!(
		"\n{FILE_OPEN}{path}\">\n{content}\n\
		{FILE_CLOSE}\n"
	)
}

/// Wrap source code in XML tags
fn format_symbol_ref(
	symbol: &str,
	source: &Option<String>,
) -> String {
	let code = source
		.as_deref()
		.unwrap_or("[source not available]");
	format!(
		"\n{CODE_OPEN}{symbol}\">\n{code}\n\
		{CODE_CLOSE}\n"
	)
}
