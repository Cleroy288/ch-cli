use std::path::Path;

use crate::indexer::{
	ByteSpan, CodeLocation, Symbol,
	SymbolKind, Visibility,
};

use super::chunk::DocChunk;
use super::chunk_extract::{
	flush_chunk, process_line, ChunkState,
};

/// Parser for markdown documentation files
pub struct DocParser;

impl DocParser {
	pub fn parse(
		path: &Path,
		content: &str,
	) -> Vec<Symbol> {
		let chunks = Self::extract_chunks(content);
		Self::chunks_to_symbols(path, chunks)
	}

	/// Extract documentation chunks from content
	fn extract_chunks(
		content: &str,
	) -> Vec<DocChunk> {
		let mut state = ChunkState::default();

		for (idx, line) in content.lines().enumerate()
		{
			process_line(line, idx + 1, &mut state);
		}

		flush_chunk(&mut state);
		state.chunks
	}

	fn chunks_to_symbols(
		path: &Path,
		chunks: Vec<DocChunk>,
	) -> Vec<Symbol> {
		chunks
			.into_iter()
			.map(|chunk| chunk_to_symbol(path, chunk))
			.collect()
	}
}

fn chunk_to_symbol(
	path: &Path,
	chunk: DocChunk,
) -> Symbol {
	let bytes = ByteSpan {
		offset: 0,
		length: chunk.content.len(),
	};
	let location = CodeLocation::new(
		path.to_path_buf(),
		chunk.line,
		1,
		bytes,
	);

	let sig =
		format!("h{}: {}", chunk.level, chunk.title);
	let mut symbol = Symbol::new(
		chunk.title,
		SymbolKind::DocumentChunk,
		location,
	)
	.with_visibility(Visibility::Public)
	.with_content(chunk.content)
	.with_signature(sig);

	if let Some(parent) = chunk.parent {
		symbol = symbol.with_parent(parent);
	}

	symbol
}
