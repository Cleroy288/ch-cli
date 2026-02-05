//! Documentation Parser for Markdown Files
//!
//! Parses markdown files into semantic chunks for search indexing.
//! Each header section becomes a searchable DocumentChunk symbol.

use std::path::Path;

use crate::indexer::{CodeLocation, Symbol, SymbolKind, Visibility};

/// A parsed documentation chunk from a markdown file
#[derive(Debug, Clone)]
pub struct DocChunk {
	/// section title (header text)
	pub title: String,
	/// full content of the section
	pub content: String,
	/// header level (1, 2, 3, etc.)
	pub level: usize,
	/// line number where section starts
	pub line: usize,
	/// parent section title (if nested)
	pub parent: Option<String>,
}

/// Parser for markdown documentation files
pub struct DocParser;

impl DocParser {
	/// Parse a markdown file into documentation chunks
	pub fn parse(path: &Path, content: &str) -> Vec<Symbol> {
		let chunks = Self::extract_chunks(content);
		Self::chunks_to_symbols(path, chunks)
	}

	/// Extract documentation chunks from markdown content
	fn extract_chunks(content: &str) -> Vec<DocChunk> {
		let mut chunks = Vec::new(); // collected chunks
		let mut current_chunk: Option<DocChunk> = None; // chunk being built
		let mut parent_stack: Vec<(usize, String)> = Vec::new(); // track hierarchy
		let mut content_buffer = String::new(); // accumulate section content

		for (line_idx, line) in content.lines().enumerate() {
			let line_num = line_idx + 1; // 1-indexed

			// check if line is a header
			if let Some((level, title)) = Self::parse_header(line) {
				// save previous chunk if exists
				if let Some(mut chunk) = current_chunk.take() {
					chunk.content = content_buffer.trim().to_string();
					if !chunk.content.is_empty() || !chunk.title.is_empty() {
						chunks.push(chunk);
					}
				}
				content_buffer.clear();

				// update parent stack
				while let Some((parent_level, _)) = parent_stack.last() {
					if *parent_level >= level {
						parent_stack.pop();
					} else {
						break;
					}
				}

				// get current parent
				let parent = parent_stack.last().map(|(_, name)| name.clone());

				// start new chunk
				current_chunk = Some(DocChunk {
					title: title.clone(),
					content: String::new(),
					level,
					line: line_num,
					parent,
				});

				// add to parent stack
				parent_stack.push((level, title));
			} else if current_chunk.is_some() {
				// add line to current chunk content
				content_buffer.push_str(line);
				content_buffer.push('\n');
			} else if !line.trim().is_empty() {
				// content before first header - create intro chunk
				current_chunk = Some(DocChunk {
					title: "Introduction".to_string(),
					content: String::new(),
					level: 0,
					line: line_num,
					parent: None,
				});
				content_buffer.push_str(line);
				content_buffer.push('\n');
			}
		}

		// save final chunk
		if let Some(mut chunk) = current_chunk.take() {
			chunk.content = content_buffer.trim().to_string();
			if !chunk.content.is_empty() || !chunk.title.is_empty() {
				chunks.push(chunk);
			}
		}

		chunks
	}

	/// Parse a markdown header line
	fn parse_header(line: &str) -> Option<(usize, String)> {
		let trimmed = line.trim();

		// ATX-style headers: # Header
		if trimmed.starts_with('#') {
			let level = trimmed.chars().take_while(|&c| c == '#').count();
			if level <= 6 {
				let title = trimmed[level..].trim().trim_end_matches('#').trim();
				if !title.is_empty() {
					return Some((level, title.to_string()));
				}
			}
		}

		None
	}

	/// Convert documentation chunks to Symbol instances
	fn chunks_to_symbols(path: &Path, chunks: Vec<DocChunk>) -> Vec<Symbol> {
		chunks
			.into_iter()
			.map(|chunk| {
				let location = CodeLocation::new(
					path.to_path_buf(),
					chunk.line,
					1,
					0,
					chunk.content.len(),
				);

				let mut symbol = Symbol::new(chunk.title.clone(), SymbolKind::DocumentChunk, location)
					.with_visibility(Visibility::Public)
					.with_content(chunk.content);

				if let Some(parent) = chunk.parent {
					symbol = symbol.with_parent(parent);
				}

				// add signature showing hierarchy
				let sig = format!("h{}: {}", chunk.level, chunk.title);
				symbol = symbol.with_signature(sig);

				symbol
			})
			.collect()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::path::PathBuf;

	#[test]
	fn test_parse_simple_markdown() {
		let content = r#"# Main Title

This is the introduction.

## Section One

Content of section one.

## Section Two

Content of section two.

### Subsection

Nested content.
"#;

		let path = PathBuf::from("test.md");
		let symbols = DocParser::parse(&path, content);

		assert_eq!(symbols.len(), 4);
		assert_eq!(symbols[0].name, "Main Title");
		assert_eq!(symbols[1].name, "Section One");
		assert_eq!(symbols[2].name, "Section Two");
		assert_eq!(symbols[3].name, "Subsection");
		assert_eq!(symbols[3].parent, Some("Section Two".to_string()));
	}

	#[test]
	fn test_parse_header_levels() {
		assert_eq!(DocParser::parse_header("# Title"), Some((1, "Title".to_string())));
		assert_eq!(DocParser::parse_header("## Title"), Some((2, "Title".to_string())));
		assert_eq!(DocParser::parse_header("### Title"), Some((3, "Title".to_string())));
		assert_eq!(DocParser::parse_header("Not a header"), None);
	}

	#[test]
	fn test_content_extraction() {
		let content = r#"# Header

Line 1
Line 2
Line 3
"#;

		let path = PathBuf::from("test.md");
		let symbols = DocParser::parse(&path, content);

		assert_eq!(symbols.len(), 1);
		assert!(symbols[0].content.as_ref().unwrap().contains("Line 1"));
		assert!(symbols[0].content.as_ref().unwrap().contains("Line 2"));
	}

	#[test]
	fn test_intro_without_header() {
		let content = r#"This is intro content without a header.

# First Header

Content after header.
"#;

		let path = PathBuf::from("test.md");
		let symbols = DocParser::parse(&path, content);

		assert_eq!(symbols.len(), 2);
		assert_eq!(symbols[0].name, "Introduction");
		assert_eq!(symbols[1].name, "First Header");
	}
}
