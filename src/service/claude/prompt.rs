use crate::message::MessageSegment;

const FILE_OPEN: &str = "<file path=\"";
const FILE_CLOSE: &str = "</file>";
const CODE_OPEN: &str = "<code symbol=\"";
const CODE_CLOSE: &str = "</code>";

pub fn build_prompt(
	segments: &[MessageSegment],
) -> String {
	segments
		.iter()
		.map(format_segment)
		.collect()
}

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

fn format_file_ref(path: &str) -> String {
	let content =
		std::fs::read_to_string(path).unwrap_or_else(
			|_| "[file not readable]".into(),
		);
	format!(
		"\n{FILE_OPEN}{path}\">\n{content}\n\
		{FILE_CLOSE}\n"
	)
}

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
