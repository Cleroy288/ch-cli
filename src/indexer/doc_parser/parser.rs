//! DocParser for markdown documentation files

use std::path::Path;

use crate::indexer::{CodeLocation, Symbol, SymbolKind, Visibility};

use super::chunk::DocChunk;

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

				let mut symbol = Symbol::new(
					chunk.title.clone(),
					SymbolKind::DocumentChunk,
					location
				)
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
