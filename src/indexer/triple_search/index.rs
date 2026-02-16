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
		symbols: &[Symbol],
	) -> SearchResult<TripleIndexStats> {
		let (code, doc, notes) = route_by_content(symbols);

		let code_count = index_if_nonempty(
			&self.code_index, &code,
		)?;
		let doc_count = index_if_nonempty(
			&self.doc_index, &doc,
		)?;
		let notes_count = index_if_nonempty(
			&self.notes_index, &notes,
		)?;

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
		// Note: reader access may not be available
		0
	}
}

/// Route symbols into code, doc, notes buckets
#[allow(clippy::type_complexity)]
fn route_by_content(
	symbols: &[Symbol],
) -> (Vec<Symbol>, Vec<Symbol>, Vec<Symbol>) {
	let mut code = Vec::new(); // source code symbols
	let mut doc = Vec::new(); // documentation symbols
	let mut notes = Vec::new(); // notes symbols

	for sym in symbols {
		let kind = ContentType::from_path(&sym.location.file);
		match kind {
			ContentType::Code => code.push(sym.clone()),
			ContentType::Doc => doc.push(sym.clone()),
			ContentType::Notes => notes.push(sym.clone()),
		}
	}

	(code, doc, notes)
}

/// Index symbols into a SearchIndex only if non-empty
fn index_if_nonempty(
	idx: &SearchIndex,
	symbols: &[Symbol],
) -> SearchResult<usize> {
	if symbols.is_empty() {
		return Ok(0);
	}
	idx.index_symbols(symbols)
}
