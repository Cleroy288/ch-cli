#[derive(Debug, PartialEq)]
pub enum ContentSection {
	Text(String),
	Code { lang: String, code: String },
}

/// Unclosed fences become Code (streaming support).
pub fn parse_sections(
	text: &str,
) -> Vec<ContentSection> {
	let mut sections = Vec::new();
	let mut buf: Vec<&str> = Vec::new();
	let mut in_code = false;
	let mut lang = String::new();

	for line in text.lines() {
		let trimmed = line.trim_start();
		if trimmed.starts_with("```") {
			if in_code {
				flush_code(
					&lang, &buf, &mut sections,
				);
				buf.clear();
				in_code = false;
			} else {
				flush_text(&buf, &mut sections);
				buf.clear();
				lang = extract_lang(trimmed);
				in_code = true;
			}
			continue;
		}
		buf.push(line);
	}
	if !buf.is_empty() {
		if in_code {
			flush_code(&lang, &buf, &mut sections);
		} else {
			flush_text(&buf, &mut sections);
		}
	}
	sections
}

pub fn extract_lang(fence: &str) -> String {
	fence
		.trim_start_matches('`')
		.trim()
		.to_owned()
}

fn flush_text(
	buf: &[&str],
	out: &mut Vec<ContentSection>,
) {
	if buf.is_empty() {
		return;
	}
	out.push(ContentSection::Text(
		buf.join("\n"),
	));
}

fn flush_code(
	lang: &str,
	buf: &[&str],
	out: &mut Vec<ContentSection>,
) {
	out.push(ContentSection::Code {
		lang: lang.to_owned(),
		code: buf.join("\n"),
	});
}
