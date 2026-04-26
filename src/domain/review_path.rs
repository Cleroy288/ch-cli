use super::content_section::{
	parse_sections, ContentSection,
};

use super::review::ReviewBlock;
use super::review_path_match;

pub fn infer_file_path(
	preceding: &str,
) -> Option<String> {
	preceding
		.lines()
		.rev()
		.filter(|l| !l.trim().is_empty())
		.find_map(|l| {
			review_path_match::try_line(l.trim())
		})
}

pub fn extract_review_blocks(
	response: &str,
) -> Vec<ReviewBlock> {
	let sections = parse_sections(response);
	let mut blocks = Vec::new();
	let mut last_text = String::new();

	for section in &sections {
		match section {
			ContentSection::Text(text) => {
				last_text = text.clone();
			}
			ContentSection::Code { lang, code } => {
				let path =
					infer_file_path(&last_text);
				blocks.push(ReviewBlock::new(
					blocks.len(),
					lang.clone(),
					code.clone(),
					path,
				));
			}
		}
	}
	blocks
}
