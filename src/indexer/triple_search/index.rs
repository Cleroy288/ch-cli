//! Triple Search Index
//!
//! Main index struct managing separate code, doc, and notes indexes.

use std::path::{Path, PathBuf};

use crate::indexer::search::{SearchIndex, SearchResult};
use crate::indexer::symbols::{ContentType, Symbol};

use super::types::TripleIndexStats;

/// Triple search index with separate code, doc, and notes indexes.
/// Enables parallel search without content type interference.
pub struct TripleSearchIndex {
	/// index for code symbols (SourceCode + Test)
	pub(crate) code_index: SearchIndex,
	/// index for documentation (doc/*.md)
	pub(crate) doc_index: SearchIndex,
	/// index for notes (notes/*.md)
	pub(crate) notes_index: SearchIndex,
	/// base path for persistence (if enabled)
	pub(crate) base_path: Option<PathBuf>,
}

impl TripleSearchIndex {
	/// Create new in-memory triple index
	pub fn in_memory() -> SearchResult<Self> {
		let code_index = SearchIndex::in_memory()?; // in-memory code index
		let doc_index = SearchIndex::in_memory()?; // in-memory doc index
		let notes_index = SearchIndex::in_memory()?; // in-memory notes index

		Ok(Self {
			code_index,
			doc_index,
			notes_index,
			base_path: None,
		})
	}

	/// Create persistent triple index at given path
	pub fn open_or_create(base_path: &Path) -> SearchResult<Self> {
		// path for code index
		let code_path = base_path.join("code").join("tantivy");
		// path for doc index
		let doc_path = base_path.join("docs").join("tantivy");
		// path for notes index
		let notes_path = base_path.join("notes").join("tantivy");

		let code_index = SearchIndex::open_or_create(&code_path)?;
		let doc_index = SearchIndex::open_or_create(&doc_path)?;
		let notes_index = SearchIndex::open_or_create(&notes_path)?;

		Ok(Self {
			code_index,
			doc_index,
			notes_index,
			base_path: Some(base_path.to_path_buf()),
		})
	}

	/// Index symbols, routing each to the appropriate index
	pub fn index_symbols(
		&self,
		symbols: &[Symbol]
	) -> SearchResult<TripleIndexStats> {
		let mut code_symbols = Vec::new(); // symbols for code index
		let mut doc_symbols = Vec::new(); // symbols for doc index
		let mut notes_symbols = Vec::new(); // symbols for notes index

		// Route symbols by content type
		for symbol in symbols {
			let content_type = ContentType::from_path(&symbol.location.file);
			match content_type {
				ContentType::Code => code_symbols.push(symbol.clone()),
				ContentType::Doc => doc_symbols.push(symbol.clone()),
				ContentType::Notes => notes_symbols.push(symbol.clone()),
			}
		}

		// Index each set
		let code_count = if !code_symbols.is_empty() {
			self.code_index.index_symbols(&code_symbols)?
		} else {
			0
		};

		let doc_count = if !doc_symbols.is_empty() {
			self.doc_index.index_symbols(&doc_symbols)?
		} else {
			0
		};

		let notes_count = if !notes_symbols.is_empty() {
			self.notes_index.index_symbols(&notes_symbols)?
		} else {
			0
		};

		Ok(TripleIndexStats {
			code_count,
			doc_count,
			notes_count,
		})
	}

	/// Get base path (if persistent)
	pub fn path(&self) -> Option<&Path> {
		self.base_path.as_deref()
	}

	/// Get total symbol count across all indexes
	pub fn total_count(&self) -> usize {
		// Note: This requires reader access which may not be available
		// For now, return 0 as a placeholder
		0
	}
}
